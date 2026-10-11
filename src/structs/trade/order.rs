use crate::structs::trade::common::ts::UnixTs;
use crate::structs::trade::common::{id::*, price::Price, qty::Qty, side::*};
use crate::structs::trade::{mech::Mech, time_in_force::TimeInForce};

pub struct Order {
    pub q: Qty,
    pub time_in_force: TimeInForce,
    pub id: Id,
    pub id_user: Hash32,
    pub ts: UnixTs,
}

pub struct OrderExecute {
    pub order: Order,
    pub side: Side,
    pub price: Option<Price>,
    pub price_trigger: Option<Price>,
    pub mech: Option<Mech>,
}

pub struct OrderWrap {
    pub order: Order,
    pub mech: Option<Mech>,
}
