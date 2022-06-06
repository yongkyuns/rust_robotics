pub mod lqr;
#[cfg(not(feature = "libm"))]
pub mod mpc;

pub use lqr::*;
#[cfg(not(feature = "libm"))]
pub use mpc::*;

use crate::prelude::*;

pub const g: f32 = 9.81; // [m/s^2] Gravity

pub const NX: usize = 4; // Number of states
pub const NU: usize = 1; // Number of input

/// Model parameters for inverted pendulum
pub struct Model {
    pub l_bar: f32, // [m] Length of bar
    pub M: f32,     // [kg] Mass of cart
    pub m: f32,     // [kg] Mass of ball
                    // pub dt: f32,    // [s] Sample time
}

impl Default for Model {
    fn default() -> Self {
        Self {
            l_bar: 2.0,
            M: 1.0,
            m: 1.0,
            // dt: 0.01,
        }
    }
}

impl Model {
    pub fn get_model_matrix(&self, dt: f32) -> (Matrix4, Vector4) {
        let Self { l_bar, M, m } = *self;

        let A = matrix![0., 1.,    0., 0.;
						0., 0., m*g / M, 0.;
						0., 0., 0., 1.;
						0., 0., g*(M+m)/(l_bar*M), 0.];

        let B = vector![0., 1. / M, 0., 1. / (l_bar * M)];

        (eye!(NX) + A * dt, B * dt)
    }
}
