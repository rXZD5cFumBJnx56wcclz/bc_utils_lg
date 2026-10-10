use bc_utils_lg::prelude::*;

use crate::common::id::Id;
use crate::orders::order::OrderWrap;

pub struct OrderQueue {
    orders: DashMap<Id, (u64, OrderWrap)>,
    index: SkipMap<u64, Id>,
    next_seq: AtomicU64,
}
