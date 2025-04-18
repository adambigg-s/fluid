use crate::utils;

use utils::Vector;

/// used to define (typically) a border bc which asserts some flux.
/// this can be used to carefully control inflow/outflow
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Source {
    pub velocity: Vector<f32>,
}

impl Source {
    /// defines the velocity leaving source cell on x and y sides
    pub fn construct(x: f32, y: f32) -> Source {
        Source { velocity: Vector::construct(x, y) }
    }
}
