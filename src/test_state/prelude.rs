pub use std::sync::LazyLock;
pub use std::time::Duration;

pub use crate::prelude::*;

// pub use crate::test_state::settings::data_gen::*;
pub use crate::test_state::settings::exch::*;
// pub use crate::test_state::settings::files_dir::*;
// pub use crate::test_state::settings::global::*;
pub use crate::test_state::settings::ind::*;
// pub use crate::test_state::settings::ind_col::*;
// pub use crate::test_state::settings::ind_val::*;
pub use crate::test_state::settings::order_creator::*;
pub use crate::test_state::settings::order_filters::*;
pub use crate::test_state::settings::orders_collectors::*;
// pub use crate::test_state::settings::other::*;
// pub use crate::test_state::settings::pipeline::*;
// pub use crate::test_state::settings::pipeline_post::*;
// pub use crate::test_state::settings::settings::*;
pub use crate::test_state::settings::signals::*;
pub use crate::test_state::settings::signals_train::*;
// pub use crate::test_state::settings::symbols::*;
// pub use crate::test_state::settings::symbols_all::*;
// pub use crate::test_state::settings::symbols_filters::*;
pub use crate::test_state::settings::trade::*;
pub use crate::test_state::settings::utils_state::*;

pub fn from_u128_number(n: u128) -> ResultAH<Value> {
    Ok(Value::Number(Number::from_u128(n).ok_or(anyhow::anyhow!("from u128 err"))?))
}

pub fn from_f64_number(n: f64) -> ResultAH<Value> {
    Ok(Value::Number(Number::from_f64(n).ok_or(anyhow::anyhow!("from f64 err"))?))
}