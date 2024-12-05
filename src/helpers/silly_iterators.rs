use std::collections::HashMap;

pub trait Countable {
    type K;
    fn frequencies(self) -> HashMap<Self::K, u32>;
}

impl<T, I> Countable for I
where
    I: IntoIterator<Item = T>,
    T: std::hash::Hash + Eq,
{
    type K = T;
    fn frequencies(self) -> HashMap<T, u32> {
        let mut counts: HashMap<T, u32> = HashMap::new();
        for thing in self {
            counts.entry(thing).and_modify(|x| *x += 1).or_insert(1);
        }
        counts
    }
}
