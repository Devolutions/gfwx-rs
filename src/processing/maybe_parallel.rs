#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[cfg(feature = "rayon")]
#[inline(always)]
pub fn process_maybe_parallel_for_each<'a, T, I, A>(items: I, action: A, hint_do_parallel: bool)
where
    T: 'a + Send,
    I: Iterator<Item = T> + Send,
    A: Fn(I::Item) + Sync + Send,
{
    if cfg!(not(feature = "adaptive_multithreading")) || hint_do_parallel {
        items.par_bridge().for_each(action);
    } else {
        items.for_each(action);
    }
}

#[cfg(not(feature = "rayon"))]
#[inline(always)]
pub fn process_maybe_parallel_for_each<'a, T, I, A>(items: I, action: A, _hint_do_parallel: bool)
where
    T: 'a,
    I: Iterator<Item = T>,
    A: Fn(I::Item),
{
    items.for_each(action);
}

#[cfg(feature = "rayon")]
#[inline(always)]
pub fn process_maybe_parallel_map_collect<'a, T, I, A, R>(
    items: I,
    action: A,
    hint_do_parallel: bool,
) -> Vec<R>
where
    T: 'a + Send,
    I: Iterator<Item = T> + Send,
    A: Fn(I::Item) -> R + Sync + Send,
    R: Send,
{
    if cfg!(not(feature = "adaptive_multithreading")) || hint_do_parallel {
        items.par_bridge().map(action).collect()
    } else {
        items.map(action).collect()
    }
}

#[cfg(not(feature = "rayon"))]
#[inline(always)]
pub fn process_maybe_parallel_map_collect<'a, T, I, A, R>(
    items: I,
    action: A,
    _hint_do_parallel: bool,
) -> Vec<R>
where
    T: 'a + Send,
    I: Iterator<Item = T> + Send,
    A: Fn(I::Item) -> R + Sync + Send,
    R: Send,
{
    items.map(action).collect()
}
