use nautilus_model::data::{
        Bar, IndexPriceUpdate, MarkPriceUpdate,
        OrderBookDepth,
    };
use std::collections::BTreeSet;

pub fn bar_serialization(bar: &Bar) -> Vec<f64> {
    vec![
        bar.ts_init.to_be() as f64,
        bar.open.as_f64(),
        bar.high.as_f64(),
        bar.low.as_f64(),
        bar.close.as_f64(),
        bar.volume.as_f64(),
    ]
}

pub fn mark_price_serialization(price: &MarkPriceUpdate) -> f64 {
    price.value.as_f64()
}

pub fn index_price_serialization(price: &IndexPriceUpdate) -> f64 {
    price.value.as_f64()
}

pub fn depth_serizlization(depth: &OrderBookDepth) -> Vec<f64> {
    let mut res = Vec::with_capacity(2 + 20);
    let mut asks = BTreeSet::new();
    let mut bids = BTreeSet::new();
    let best_ask = depth
        .asks
        .iter()
        .map(|v| {
            asks.insert(v.price);
            v.price
        })
        .max_by(|v2c, v2c2| v2c.partial_cmp(v2c2).unwrap_or(std::cmp::Ordering::Equal))
        .map(|v| v.as_f64())
        .unwrap_or_default();
    let best_bid = depth
        .bids
        .iter()
        .map(|v| {
            bids.insert(v.price);
            v.price
        })
        .min_by(|v2c, v2c2| v2c.partial_cmp(v2c2).unwrap_or(std::cmp::Ordering::Equal))
        .map(|v| v.as_f64())
        .unwrap_or_default();
    res.push(best_ask);
    res.push(best_bid);
    res.extend(asks.into_iter().map(|v| v.as_f64()));
    res.extend(bids.into_iter().map(|v| v.as_f64()));
    res
}
