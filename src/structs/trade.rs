use std::cell::RefCell;

use crate::types::maps::MAP;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StateValues {
    pub qty_percent_of_position: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub symbol: String,
    pub side: String,
    pub qty: f64,
    pub commission: f64,
    pub leverage: f64,
    pub price: Option<f64>,
    pub type_: String,
    pub is_reduce: bool,
    pub type_price_cross: String,
    pub order_link_id: String,
    pub position_idx: usize,
    pub is_active: bool,
    pub state_values: Option<StateValues>,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            symbol: Default::default(),
            side: Default::default(),
            qty: Default::default(),
            commission: Default::default(),
            leverage: 1.,
            price: Default::default(),
            type_: Default::default(),
            is_reduce: Default::default(),
            type_price_cross: "last".to_string(),
            order_link_id: Default::default(),
            position_idx: Default::default(),
            is_active: true,
            state_values: Default::default(),
        }
    }
}

impl Order {
    pub fn new(
        symbol: String,
        side: String,
        qty: f64,
        commission: f64,
        leverage: f64,
        price: Option<f64>,
        type_: String,
        is_reduce: bool,
        type_price_cross: String,
        order_link_id: String,
        position_idx: usize,
        is_active: bool,
        state_values: Option<StateValues>,
    ) -> Self {
        Self {
            symbol,
            side,
            qty,
            commission,
            leverage,
            price,
            type_,
            is_reduce,
            type_price_cross,
            order_link_id,
            position_idx,
            is_active,
            state_values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub symbol: String,
    pub side: String,
    pub qty: f64,
    pub leverage: f64,
    pub avg_open_price: f64,
    pub position_idx: usize,
    pub is_active: bool,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            symbol: Default::default(),
            side: Default::default(),
            qty: Default::default(),
            leverage: 1.,
            avg_open_price: Default::default(),
            position_idx: Default::default(),
            is_active: true,
        }
    }
}

impl Position {
    pub fn new(
        symbol: String,
        side: String,
        qty: f64,
        leverage: f64,
        avg_open_price: f64,
        position_idx: usize,
        is_active: bool,
    ) -> Self {
        Self {
            symbol,
            side,
            qty,
            leverage,
            avg_open_price,
            position_idx,
            is_active,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Trigger {
    pub price: f64,
    pub trigger_by: String,
    pub direction: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TradeState<'a> {
    pub capital: f64,
    pub orders: RefCell<MAP<&'a str, Order>>,
    pub orders_storage: RefCell<MAP<&'a str, (Order, Trigger)>>,
    pub positions: RefCell<MAP<usize, Position>>,
}

impl TradeState<'_> {
    pub fn new(capital: f64) -> Self {
        Self {
            capital,
            ..Self::default()
        }
    }
}

pub trait IsActive {
    fn is_active(&self) -> bool;
}

impl IsActive for Position {
    fn is_active(&self) -> bool {
        self.is_active
    }
}

impl IsActive for Order {
    fn is_active(&self) -> bool {
        self.is_active
    }
}
