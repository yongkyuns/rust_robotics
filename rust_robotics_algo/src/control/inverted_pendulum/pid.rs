/// Simple PID controller
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PID {
    pub P: f32,
    pub I: f32,
    pub D: f32,
    err_integral: f32,
    err_prev: f32,
}

impl Default for PID {
    fn default() -> Self {
        Self {
            P: 1.0,
            I: 0.0,
            D: 0.0,
            err_integral: 0.0,
            err_prev: 0.0,
        }
    }
}

impl PID {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reset_state(&mut self) {
        self.err_integral = 0.0;
        self.err_prev = 0.0;
    }
    pub fn control(&mut self, err: f32, dt: f32) -> f32 {
        self.err_integral += err * dt;
        let u = self.P * err + self.I * self.err_integral + self.D * (err - self.err_prev) / dt;
        self.err_prev = err;
        u
    }
}
