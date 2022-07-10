use std::collections::{BTreeMap, btree_map::Entry};

/// Trait for all the types supporting deep merging
pub trait DeepMerge {
    /// Merge `other` into `self`
    fn merge_from(&mut self, other: Self) where Self: Sized {
        *self = other;
    }
}

impl DeepMerge for bool {}
impl DeepMerge for i32 {}
impl DeepMerge for i64 {}
impl DeepMerge for f64 {}
impl<'a> DeepMerge for &'a str {}
// TODO: a bit unclear on this one
impl<'a> DeepMerge for &'a [std::string::String] {}

impl<'a, T> DeepMerge for &'a T {}

impl DeepMerge for String {}
impl DeepMerge for crate::ByteString {}
impl<T> DeepMerge for chrono::DateTime<T> where T : chrono::TimeZone {}

/// TODO: well... actually we probably have to descend recursively into it or something :/
impl DeepMerge for serde_json::Value {}


impl<T> DeepMerge for Box<T> where T : DeepMerge {
    fn merge_from(&mut self, other: Self) where Self: Sized {
        (**self).merge_from(*other)
    }
}

impl<T> DeepMerge for Vec<T> {
    fn merge_from(&mut self, mut other: Self) {
        self.append(&mut other);
    }
}

impl<K, V> DeepMerge for BTreeMap<K, V> where K : Ord, V : DeepMerge {
    fn merge_from(&mut self, other: Self) {
        for (k, v) in other.into_iter() {
            match self.entry(k) {
                Entry::Vacant(e) => { e.insert(v); },
                Entry::Occupied(e) => { e.into_mut().merge_from(v); },
            }
        }
    }
}

impl<T> DeepMerge for Option<T> where T: DeepMerge {
    fn merge_from(&mut self, other: Self) {
        if let Some(other) = other {
            if let Some(s) = self {
                s.merge_from(other);
            } else {
                *self = Some(other);
            }
        }
    }
}

