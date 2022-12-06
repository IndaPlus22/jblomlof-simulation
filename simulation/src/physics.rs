pub const GRAVITY : f64 = 9.82; //self explanetory, positive is "normal" gravity.
pub struct Pendulum{
    pub current_angle: f64,
    pub angle_speed: f64,
    pub stick_length: f64
}

impl Pendulum {
    pub fn update_angle(&mut self, time_in_seconds: f64) -> f64{
        //uses differential equation from https://en.wikipedia.org/wiki/Pendulum_(mechanics)
        let delta_angle_speed = self.current_angle.sin() * GRAVITY / self.stick_length * time_in_seconds;
        self.angle_speed += delta_angle_speed;
        self.current_angle -= self.angle_speed;
        //returning the speed difference but negative,
        //why? Well my thought is that each pendulum will not only affect the speed of it self
        //but also the speed of the one above and below
        //its very simplified, but fun 
        -delta_angle_speed
    }

    pub fn angle_influence (&mut self, influence :f64) {
        //not dividing seems to speed thinks up so its just a number i pulled out.
        //but realisticly dividing by 2 is reasonable since we change 2 angles? or smth.
        self.angle_speed += influence / 2.0;
        self.current_angle += influence / 2.0;
    }
}