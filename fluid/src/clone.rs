


use crate::utils;



use utils::Vector;



#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Duplicate {
    pub x: usize,
    pub y: usize,
    pub master: Vector<isize>,
}

impl Duplicate {
    pub fn construct(x: usize, y: usize, mx: isize, my: isize) -> Duplicate {
        let master: Vector<isize> = Vector::construct(mx, my);
        Duplicate { x, y, master }
    }
}

