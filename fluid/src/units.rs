#![allow(dead_code)]
#![allow(unused_imports)]

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Meter(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Second(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Newton(pub f32);

impl Add for Meter {
    type Output = Meter;

    fn add(self, other: Meter) -> Meter {
        Meter(self.0 + other.0)
    }
}

impl Mul<Second> for Meter {
    type Output = f32;

    fn mul(self, other: Second) -> f32 {
        self.0 * other.0
    }
}

impl Mul<Second> for Newton {
    type Output = f32;

    fn mul(self, other: Second) -> f32 {
        self.0 * other.0
    }
}

trait MeterUnit {}
trait SecondUnit {}
trait NewtonUnit {}

impl MeterUnit for Meter {}
impl SecondUnit for Second {}
impl NewtonUnit for Newton {}

struct Quantity<U> {
    value: f32,
    _unit: std::marker::PhantomData<U>,
}

impl<U> Quantity<U> {
    fn new(value: f32) -> Self {
        Quantity {
            value,
            _unit: std::marker::PhantomData,
        }
    }
}

impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;

    fn add(self, other: Quantity<U>) -> Quantity<U> {
        Quantity::new(self.value + other.value)
    }
}

impl<U1, U2> Mul<Quantity<U2>> for Quantity<U1> {
    type Output = Quantity<(U1, U2)>;

    fn mul(self, other: Quantity<U2>) -> Quantity<(U1, U2)> {
        Quantity::new(self.value * other.value)
    }
}

type MetersPerSecond = Quantity<(Meter, Second)>;

impl Quantity<(Meter, Second)> {
    fn divide(self, other: Quantity<Second>) -> Quantity<Meter> {
        Quantity::new(self.value / other.value)
    }
}
