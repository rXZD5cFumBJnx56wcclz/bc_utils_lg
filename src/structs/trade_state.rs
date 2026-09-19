use crate::prelude::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TradeState<'a> {
    pub capital: Capital,
    pub orders: RefCell<MAP<&'a str, Vec<Order>>>,
    pub orders_trigger: RefCell<MAP<&'a str, Vec<(Order, Trigger)>>>,
    pub positions: RefCell<MAP<usize, Position>>,
}

impl TradeState<'_> {
    pub fn new(capital: Capital) -> Self {
        Self {
            capital,
            ..Self::default()
        }
    }
}
