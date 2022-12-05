

pub const GRAVITY : f64 = 9.82;
pub struct Pendulum{
    pub current_angle: f64,
    pub angle_speed: f64,
    pub stick_length: f64
}

impl Pendulum {
    pub fn update_angle(&mut self, time_in_seconds: f64) -> f64{
        //uses differential equation from https://en.wikipedia.org/wiki/Pendulum_(mechanics)
        let angle_acceleration = self.current_angle.sin() * GRAVITY / self.stick_length * time_in_seconds;
        self.angle_speed += angle_acceleration;
        self.current_angle -= self.angle_speed;
        -angle_acceleration
    }

    pub fn angle_influence (&mut self, influence :f64) -> f64 {
        self.angle_speed += influence / 2.0;
        -influence / 2.0
    }
}