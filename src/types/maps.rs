#![allow(non_camel_case_types)]

use core::hash::BuildHasherDefault;
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHasher};

pub type MAP<K, V> = FxHashMap<K, V>;
pub type MAP_LINK<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

pub type FUNCS_EXTRACT_ARGS_TYPE<S, T> = MAP<&'static str, fn(&S) -> T>;

pub trait MapTrait<'a, K, V>
where 
    K: 'a,
    V: 'a,
{
    fn keys(&'a self) -> impl IntoIterator<Item = &'a K>;
    fn values(&'a self) -> impl IntoIterator<Item = &'a V>;
}

impl<'a, K, V> MapTrait<'a, K, V> for MAP<K, V>
where 
    K: 'a,
    V: 'a,
{
    fn keys(&'a self) -> impl IntoIterator<Item = &'a K> {
        self.keys()
    }
    fn values(&'a self) -> impl IntoIterator<Item = &'a V> {
        self.values()
    }
}
impl<'a, K, V> MapTrait<'a, K, V> for MAP_LINK<K, V>
where 
    K: 'a,
    V: 'a,
{
    fn keys(&'a self) -> impl IntoIterator<Item = &'a K> {
        self.keys()
    }
    fn values(&'a self) -> impl IntoIterator<Item = &'a V> {
        self.values()
    }
}
