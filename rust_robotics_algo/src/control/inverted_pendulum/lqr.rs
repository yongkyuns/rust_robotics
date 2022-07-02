use super::*;

impl LQR<NX, NU> for Model {
    fn Q(&self) -> QMat {
        self.Q
    }
    fn R(&self) -> RMat {
        self.R
    }
    fn epsilon(&self) -> f32 {
        self.eps
    }
    fn max_iter(&self) -> u32 {
        self.max_iter
    }
}
