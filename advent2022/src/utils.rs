use std::{
    cmp::{max, min},
    ops::Range,
};

use easy_ext::ext;

#[macro_export]
macro_rules! parse {
    ($value:ident, $ty:ty) => {
        $value
            .parse::<$ty>()
            .map_err(|_| anyhow::anyhow!(concat!("Failed to parse", stringify!($value))))
    };
    ($value:ident) => {
        $value
            .parse()
            .map_err(|_| anyhow::anyhow!(concat!("Failed to parse", stringify!($value))))
    };
}

#[ext(RangeExt)]
pub impl<T> Range<T>
where
    T: Ord + Clone,
{
    fn contains_range(&self, other: &Range<T>) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range<T>) -> bool {
        min(self.end.clone(), other.end.clone()) >= max(self.start.clone(), other.start.clone())
    }
}
