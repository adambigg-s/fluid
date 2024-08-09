


use crate::utils;



use utils::Vector;



#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Source {
    pub velocity: Vector<f32>,
}

impl Source {
    pub fn construct(x: f32, y: f32) -> Source {
        Source {
            velocity: Vector::construct(x, y),
        }
    }
}
