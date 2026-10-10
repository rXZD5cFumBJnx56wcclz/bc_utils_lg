
use crate::structs::trade::common::ts::UnixTs;
use crate::structs::trade::common::{id::*, price::Price, qty::Qty, side::*};
use crate::structs::trade::{mech::Mech, time_in_force::TimeInForce};

pub struct Order {
    q: Qty,
    time_in_force: TimeInForce,
    id: Id,
    id_user: Hash32,
    ts: UnixTs,
}

pub struct OrderExecute {
    order: Order,
    side: Side,
    price: Option<Price>,
    price_trigger: Option<Price>,
    mech: Option<Mech>,
}

pub struct OrderWrap {
    order: Order,
    mech: Option<Mech>,
}
