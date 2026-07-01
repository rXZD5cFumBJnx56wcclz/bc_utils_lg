#![allow(non_camel_case_types)]

use core::hash::BuildHasherDefault;
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHasher};

pub type MAP<K, V> = FxHashMap<K, V>;
pub type MAP_LINK<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

pub type FUNCS_EXTRACT_ARGS_TYPE<S, T> = MAP<&'static str, fn(&S) -> T>;