/*
TEMPLATE FROM PISTON GETTING STARTED PAGE
A ROTATING SQUARE
*/
const HEIGHT: u32 = 500;
const WIDTH: u32 = 600;
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

mod physics;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    parent: physics::Pendulum,
    son: physics::Pendulum,
    influence_on_parent: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let size = args.window_size[0].min(args.window_size[1]) / 9.0;
        self.parent.stick_length = size;
        self.son.stick_length = size;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c.transform.trans(x, y).rot_rad(self.parent.current_angle);
            line(
                RED,
                self.parent.stick_length / 20.0,
                [0.0, 0.0, 0.0, self.parent.stick_length],
                transform,
                gl,
            );
            //--
            let x_diff = self.parent.current_angle.sin() * self.parent.stick_length;
            let y_diff = self.parent.current_angle.cos() * self.parent.stick_length;
            let transform = c
                .transform
                .trans(x, y)
                .trans(-x_diff, y_diff)
                .rot_rad(self.son.current_angle);
            line(  
                BLUE,
                self.son.stick_length / 20.0,
                [0.0, 0.0, 0.0, self.son.stick_length],
                transform,
                gl,
            );
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.parent.angle_influence(self.influence_on_parent / 2.0);
        let influence = self.parent.update_angle(args.dt);
        self.son.angle_influence(influence);
        self.influence_on_parent = self.son.update_angle(args.dt);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Double-pendulum", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        parent: physics::Pendulum {
            current_angle: 3.0 * PI / 2.0,
            angle_speed: 0.0,
            stick_length: 25.0,
        },
        son: physics::Pendulum {
            current_angle: PI,
            angle_speed: 0.0,
            stick_length: 25.0,
        },
        influence_on_parent: 0.0,
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
