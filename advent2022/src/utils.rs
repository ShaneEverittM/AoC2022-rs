use std::{
    cmp::{max, min},
    ops::{Add, Range, RangeInclusive},
};

use easy_ext::ext;
use num_traits::identities::One;

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

#[ext(InclusiveRangeExt)]
pub impl<T> RangeInclusive<T>
where
    T: Ord + Clone,
{
    fn contains_range(&self, other: &RangeInclusive<T>) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps(&self, other: &RangeInclusive<T>) -> bool {
        min(self.end(), other.end()) >= max(self.start(), other.start())
    }

    fn extend_by(&self, other: &RangeInclusive<T>) -> RangeInclusive<T> {
        if self.contains_range(other) {
            self.clone()
        } else {
            self.start().min(other.start()).clone()..=self.end().max(other.end()).clone()
        }
    }

    fn precedes(&self, other: &RangeInclusive<T>) -> bool
    where
        T: Add<T, Output = T>,
        T: One,
    {
        self.end().clone() + T::one() == other.start().clone()
    }
}
