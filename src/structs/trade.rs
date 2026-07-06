use std::cell::RefCell;

use crate::structs::signals::Signal;
use crate::types::maps::MAP;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Order {
    pub symbol: String,
    pub side: String,
    pub signal: Signal,
    pub qty: f64,
    pub qty_percent_of_position: f64,
    pub leverage: f64,
    pub price: Option<f64>,
    pub type_: String,
    pub tp: Vec<Order>,
    pub sl: Vec<Order>,
    pub trigger_by: Option<String>,
    pub trigger_price: Option<f64>,
    pub trigger_direction: Option<usize>,
    pub is_reduce: bool,
    pub order_link_id: String,
    pub position_idx: String,
    pub is_active: bool,
}

impl Order {
    pub fn new(
        symbol: String,
        side: String,
        signal: Signal,
        qty: f64,
        qty_percent_of_position: f64,
        leverage: f64,
        price: Option<f64>,
        type_: String,
        tp: Vec<Order>,
        sl: Vec<Order>,
        trigger_by: Option<String>,
        trigger_price: Option<f64>,
        trigger_direction: Option<usize>,
        is_reduce: bool,
        order_link_id: String,
        position_idx: String,
        is_active: bool,
    ) -> Self {
        Self {
            symbol,
            side,
            signal,
            qty,
            qty_percent_of_position,
            leverage,
            price,
            type_,
            tp,
            sl,
            trigger_by,
            trigger_price,
            trigger_direction,
            is_reduce,
            order_link_id,
            position_idx,
            is_active,
        }
    }
    pub fn is_limit(&self) -> bool {
        self.type_ == "limit"
    }
    pub fn is_market(&self) -> bool {
        self.type_ == "market"
    }
    pub fn is_trigger(&self) -> bool {
        !self.trigger_price.is_none()
    }
    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }
    pub fn get_order_qty(&self, position_qty: f64) -> f64 {
        self.qty_percent_of_position * position_qty + self.qty
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position {
    pub symbol: String,
    pub side: String,
    pub qty: f64,
    pub leverage: f64,
    pub avg_open_price: f64,
    pub position_idx: String,
    pub is_active: bool,
}

impl Position {
    pub fn new(
        symbol: String,
        side: String,
        qty: f64,
        leverage: f64,
        avg_open_price: f64,
        position_idx: String,
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
    pub fn set_qty(&mut self, qty: f64) {
        self.qty = qty;
    }
    pub fn set_avg_open_price(&mut self, avg_open_price: f64) {
        self.avg_open_price = avg_open_price;
    }
    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TradeCell {
    // reffcell
    pub capital: f64,
    pub src: Vec<f64>,
    pub src_l: Vec<f64>,
    // key: order_link_id
    pub trigger_orders: RefCell<MAP<String, Order>>,
    pub limit_orders: RefCell<MAP<String, Order>>,
    pub market_orders: RefCell<MAP<String, Order>>,
    // key: position_idx
    pub positions: RefCell<MAP<String, Position>>,
}

impl TradeCell {
    pub fn new(capital: f64, src: Vec<f64>, src_l: Vec<f64>) -> Self {
        Self {
            capital: capital,
            src: src,
            src_l: src_l,
            ..Self::default()
        }
    }
    pub fn push_position(&mut self, position: Position) {
        self.positions
            .borrow_mut()
            .insert(position.position_idx.clone(), position);
    }
    pub fn push_trigger_order(&mut self, order: Order) {
        self.trigger_orders
            .borrow_mut()
            .insert(order.order_link_id.clone(), order);
    }
    pub fn push_limit_order(&mut self, order: Order) {
        self.limit_orders
            .borrow_mut()
            .insert(order.order_link_id.clone(), order);
    }
    pub fn push_market_order(&mut self, order: Order) {
        self.market_orders
            .borrow_mut()
            .insert(order.order_link_id.clone(), order);
    }
    pub fn push_triggers_orders<T: IntoIterator<Item = Order>>(&mut self, orders: T) {
        for order in orders {
            self.push_trigger_order(order);
        }
    }
    pub fn push_limits_orders<T: IntoIterator<Item = Order>>(&mut self, orders: T) {
        for order in orders {
            self.push_limit_order(order);
        }
    }
    pub fn push_market_orders<T: IntoIterator<Item = Order>>(&mut self, orders: T) {
        for order in orders {
            self.push_market_order(order);
        }
    }
}
