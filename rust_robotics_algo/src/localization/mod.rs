use crate::prelude::*;

pub mod particle_filter;

/// Getter methods for state vector
pub trait StateVector {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn phi(&self) -> f32;
    fn v(&self) -> f32;
}

impl StateVector for Vector4 {
    fn x(&self) -> f32 {
        *self.get(0).expect("Cannot get 1st element of Vector4")
    }
    fn y(&self) -> f32 {
        *self.get(1).expect("Cannot get 2nd element of Vector4")
    }
    fn phi(&self) -> f32 {
        *self.get(2).expect("Cannot get 3rd element of Vector4")
    }
    fn v(&self) -> f32 {
        *self.get(3).expect("Cannot get 4th element of Vector4")
    }
}
