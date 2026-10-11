use crate::prelude::*;

pub trait W {
    fn w(&self) -> usize;
}

pub fn w_src(used_src: &[SettingsUsedUsize]) -> usize {
    used_src
        .iter()
        .map(|s_src| s_src.sub_from_last_i)
        .max()
        .unwrap_or_default()
}

pub fn w_sum(used: &[String], w: &MAP_LINK<&str, usize>) -> usize {
    used.iter().map(|used| w[used.as_str()]).sum::<usize>()
}

pub fn w_scan<'a, 'b, S, T, STRUCT>(
    self_: impl Iterator<Item = (&'b &'a str, &'b STRUCT)>,
    s: impl Iterator<Item = (&'a String, &'a S)>,
    init_func: impl Fn(&STRUCT) -> usize,
    iter_func: impl Fn(&'a S, &MAP_LINK<&'a str, usize>, &'a String) -> T,
) -> MAP_LINK<&'a str, usize>
where
    T: IntoIterator<Item = usize>,
    'a: 'b,
    S: 'a,
    STRUCT: 'b,
{
    s.scan(
        self_
            .map(|(k, ind)| (*k, init_func(ind)))
            .collect::<MAP_LINK<&str, usize>>(),
        |init, (k, v)| {
            let res = iter_func(v, init, k).into_iter().sum();
            init[k.as_str()] = res;
            Some((k.as_str(), res))
        },
    )
    .collect()
}
