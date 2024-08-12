


use crate::utils;



use utils::Vector;



/// used to define border bcs which directly mirror an adjacent cell. this can be used 
/// to the effect of extending simultaion bounds indefinitely with minimal performance 
/// hit 
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

