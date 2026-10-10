use std::num::NonZeroU64;

use crate::structs::trade::common::price::Price;
use crate::structs::trade::common::qty::Qty;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegReferenceType {
    BestBid,
    BestAsk,
    MidPrice,
    LastTrade,
}

pub enum Mech {
    Iceberg {
        qty_hidden: Qty,
    },
    PostOnly,
    TrallingStop {
        trail_amount: Qty,
        last_reference_price: Price,
    },
    PeggedOrder {
        reference_price_offset: i64,
        reference_price_type: PegReferenceType,
    },
    MarketToLimit,
    ReserveOrder {
        th: Qty,
        qty_replenish: Option<NonZeroU64>,
        qty_hidden: Qty,
        auto_replenish: bool,
    },
}
