


use crate::utils;



use utils::Vector;



#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Clone {
    pub master: Vector<isize>,
}

impl Clone {
    /// master sets the relative index of cell Clone looks at to steal data
    pub fn construct(mx: isize, my: isize) -> Clone {
        Clone { 
            master: Vector::construct(mx, my), 
        }
    }
}

