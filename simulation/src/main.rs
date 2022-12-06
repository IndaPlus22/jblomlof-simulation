/*
TEMPLATE FROM PISTON GETTING STARTED PAGE
A ROTATING SQUARE
*/

const AMOUNT_PENDULUMS: u64 = 2;           // sets the amount of arms.
const STARING_ANGLE: f64 = 3.0*PI / 2.0;    // sets the first arms angle in radians
const ANGLE_DIFF_PER: f64 = -0.01 * PI;     // how much each arm differs from its parent

const SLOWDOWN_SPEED: f64 = 1.0;            // the rate at which we keep angle
                                            // set to  less than 1 if you want it to die. greater than 1 makes it increase in speed.

const SIZE_SCALE: f64 = 25.0;               // the value of each Pendulums length / Width

const TRACE_SIZE: f64 = 2.5;                // the size of each trace point.
const TRACE_POINTS: usize = 500;            // the amount of trace points at once

const HEIGHT: u32 = 500;                    //init window height and width. Window is resizable.
const WIDTH: u32 = 600;

mod physics;                                //contains a constant that can alter "gravity".
                                            
/*
Notice
The length of each pendulum is set window size.
 */

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f64::consts::PI;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    pendulums: Vec<physics::Pendulum>,
    influence: f64,
    trace_points: Vec<(f64,f64)>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        //center point of window
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        //making the size of pendulums resized so they perfectly fit in window
        let size =
            args.window_size[0].min(args.window_size[1]) / (2.0 * (self.pendulums.len() + 1) as f64);
        for pend in self.pendulums.iter_mut() {
            pend.stick_length = size;
        }

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);
            
            //drawing squares for each trace point.
            for i in self.trace_points.iter() {
                let rect = rectangle::square(0.0, 0.0, TRACE_SIZE);
                let transform = c.transform
                .trans(i.0, i.1).trans(-TRACE_SIZE/2.0, -TRACE_SIZE/2.0);
                rectangle(GREEN, rect, transform, gl)
            }
            
            //the diff help me keep track of the ending point of my pendulum chain.
            // so x + x_diff is the x coord of the end of the chain.
            let mut x_diff: f64 = 0.0;
            let mut y_diff: f64 = 0.0;
            //drawing each pendulum as a line
            for index_pend in 0..self.pendulums.len() {
                let current_ref = self.pendulums.get(index_pend).unwrap();
                let transform = c
                    .transform
                    .trans(x, y)
                    .trans(x_diff, y_diff)
                    .rot_rad(current_ref.current_angle);
                line(
                    if index_pend % 2 == 0 { BLUE } else { RED },
                    current_ref.stick_length / SIZE_SCALE,
                    [0.0, 0.0, 0.0, current_ref.stick_length],
                    transform,
                    gl,
                );
                x_diff -= current_ref.current_angle.sin() * current_ref.stick_length;
                y_diff += current_ref.current_angle.cos() * current_ref.stick_length;
            }

            //adding and removing trace points
            if self.trace_points.len() > TRACE_POINTS {
                self.trace_points.remove(0);
            }
            self.trace_points.push((x + x_diff, y + y_diff));
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        //each pendulum is affecting the next one and the previous one aswell as making the maths on itself.
        //see physics.rs for physics
        //NOTICE pendulums are influencing each other, but its implemented so the pendulum above is getting half of the influence
        //than the bottom one.
        for index_pend in 0..self.pendulums.len() {
            let influence_on_prev: f64 = {
                let pend = self.pendulums.get_mut(index_pend).unwrap();
                pend.angle_speed *= SLOWDOWN_SPEED;
                self.influence = pend.update_angle(args.dt);
                self.influence / 2.0 // <- here half.
            };
            if index_pend != 0 {
                self.pendulums
                    .get_mut(index_pend - 1)
                    .unwrap()
                    .angle_influence(influence_on_prev);
            }
            if index_pend + 1 != self.pendulums.len() {
                self.pendulums
                    .get_mut(index_pend + 1)
                    .unwrap()
                    .angle_influence(self.influence);
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("n:TH-pendulum", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut pends: Vec<physics::Pendulum> = vec![];
    for i in 0..AMOUNT_PENDULUMS {
        pends.push(physics::Pendulum { current_angle: (STARING_ANGLE - i as f64 *ANGLE_DIFF_PER), angle_speed: (0.0), stick_length: (25.0) })
    }

    let mut app = App {
        gl: GlGraphics::new(opengl),
        pendulums: pends,
        influence: 0.0,
        trace_points: vec![],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
