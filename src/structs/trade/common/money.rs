use crate::prelude::*;
use std::ops::Add;

#[cfg(feature = "high_precision")]
pub type MoneyRaw = i128;

#[cfg(not(feature = "high_precision"))]
pub type MoneyRaw = i64;

pub struct Money {
    pub v: MoneyRaw,
    pub precision: Currency,
}

impl Add for Money {
    type Output = Money;
    fn add(mut self, rhs: Money) -> Self::Output {
        self.v += rhs.v;
        self
    }
}
