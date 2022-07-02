/// Simple PID controller
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PID {
    /// P Gain. `Default = 1.0`
    pub P: f32,
    /// I Gain. `Default = 0.0`
    pub I: f32,
    /// D Gain. `Default = 0.0`
    pub D: f32,
    /// Integral error
    err_int: f32,
    /// Error from previous sample time
    err_prev: f32,
}

impl Default for PID {
    fn default() -> Self {
        Self {
            P: 1.0,
            I: 0.0,
            D: 0.0,
            err_int: 0.0,
            err_prev: 0.0,
        }
    }
}

impl PID {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_gains(P: f32, I: f32, D: f32) -> Self {
        let mut pid = Self::default();
        pid.P = P;
        pid.I = I;
        pid.D = D;
        pid
    }
    pub fn reset_state(&mut self) {
        self.err_int = 0.0;
        self.err_prev = 0.0;
    }
    pub fn control(&mut self, err: f32, dt: f32) -> f32 {
        self.err_int += err * dt;
        let u = self.P * err + self.I * self.err_int + self.D * (err - self.err_prev) / dt;
        self.err_prev = err;
        u
    }
}
