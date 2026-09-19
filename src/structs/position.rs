#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub symbol: String,
    pub side: String,
    pub qty: f64,
    pub leverage: f64,
    pub avg_open_price: f64,
    pub position_idx: usize,
    pub is_active: bool,
    pub pnl_percent: f64,
    pub pnl_qty: f64,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            symbol: Default::default(),
            side: Default::default(),
            qty: Default::default(),
            leverage: 1.,
            avg_open_price: 0.,
            position_idx: Default::default(),
            is_active: true,
            pnl_qty: 0.,
            pnl_percent: 0.,
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
            pnl_qty: 0.,
            pnl_percent: 0.,
        }
    }
}
