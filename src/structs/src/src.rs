use std::slice::SliceIndex;

use nautilus_model::{
    data::{
        Bar, IndexPriceUpdate, MarkPriceUpdate,
        OrderBookDepth, QuoteTick, TradeTick,
    },
    identifiers::InstrumentId,
};

use crate::{
    prelude::*,
    structs::src::serialization::{
        bar_serialization, depth_serizlization, index_price_serialization, mark_price_serialization,
    },
};

pub struct Src<'a> {
    pub id_instument: &'a InstrumentId,
    pub tick_trade: Option<&'a TradeTick>,
    pub tick_quote: Option<&'a QuoteTick>,
    pub bar: Option<&'a Bar>,
    pub index_price: Option<&'a IndexPriceUpdate>,
    pub mark_price: Option<&'a MarkPriceUpdate>,
    pub depth: Option<&'a OrderBookDepth>,
    pub other: Option<MAP<Ustr, f64>>,
}

pub type SrcVec<'a> = Vec<Src<'a>>;

impl<'a> Src<'a> {
    pub fn to_vec(&self) -> Vec<f64> {
        let mut res: Vec<f64> = Vec::new();
        res.extend(bar_serialization(self.bar.unwrap_or(&Bar::default())));
        res.push(mark_price_serialization(self.mark_price.unwrap_or(
            &MarkPriceUpdate::new(
                *self.id_instument,
                Default::default(),
                Default::default(),
                Default::default(),
            ),
        )));
        res.push(index_price_serialization(self.index_price.unwrap_or(
            &IndexPriceUpdate::new(
                *self.id_instument,
                Default::default(),
                Default::default(),
                Default::default(),
            ),
        )));
        res.extend(depth_serizlization(self.depth.unwrap_or(
            &OrderBookDepth::new::<Vec<_>, Vec<_>, Vec<_>, Vec<_>>(
                *self.id_instument,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
        )));
        res
    }

    pub fn to_vec2<R>(src: &[Self], rng: R) -> Vec<Vec<f64>>
    where
        R: SliceIndex<[f64], Output = [f64]>,
        R: Clone,
    {
        src.into_iter()
            .map(|v| Self::to_vec(v)[rng.clone()].to_vec())
            .collect()
    }
}
