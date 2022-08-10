/// A trait applies to types that support deep merging.
///
/// `a.merge_from(b)` behaves in the following ways:
///
/// ## `struct`s
///
/// Structs are merged by individually merging each of their fields. For example, given:
///
/// ```rust,ignore
/// struct S {
///     a: i32,
///     b: String,
/// }
/// ```
///
/// ... the expected impl of `DeepMerge` for `S` would be:
///
/// ```rust,ignore
/// impl DeepMerge for S {
///     fn merge_from(&mut self, other: Self) {
///         self.a.merge_from(other.a);
///         self.b.merge_from(other.b);
///     }
/// }
/// ```
///
/// The structs in the `k8s-openapi` crate behave this way. If you are implementing this trait for your own types, it is recommended to impl it in the same way.
///
/// ## `Option`
///
/// - If `b` is a `None`, `a` is unchanged.
///
/// - If `b` is a `Some(b_inner)`:
///
///   - If `a` is a `Some(a_inner)`, `a_inner` is merged with `b_inner`.
///
///   - If `a` is a `None`, `a` becomes `Some(b_inner)`.
///
/// ## `Vec`
///
/// The elements of `b` are appended to `a`.
///
/// ## `BTreeMap`
///
/// For each key `k` in `b`:
///
/// - If `a` contains the key `k` too, the value in `a` is merged with the value in `b`.
///
/// - If `a` does not contain the key `k`, the value in `b` is inserted into `a`.
///
/// ## `serde_json::Value`
///
/// `serde_json::Value` is merged using the JSON merge algorithm (RFC 7396).
///
/// ## Other types
///
/// `self` is just replaced by `other`.
pub trait DeepMerge {
	/// Merge `other` into `self`.
	fn merge_from(&mut self, other: Self);
}

macro_rules! default_overwriting_impl {
	() => {
		fn merge_from(&mut self, other: Self) {
			*self = other;
		}
	};
}

impl DeepMerge for bool { default_overwriting_impl! {} }
impl DeepMerge for i32 { default_overwriting_impl! {} }
impl DeepMerge for i64 { default_overwriting_impl! {} }
impl DeepMerge for f64 { default_overwriting_impl! {} }
impl DeepMerge for String { default_overwriting_impl! {} }
impl DeepMerge for crate::ByteString { default_overwriting_impl! {} }
impl<Tz> DeepMerge for chrono::DateTime<Tz> where Tz: chrono::TimeZone { default_overwriting_impl! {} }

impl DeepMerge for serde_json::Value {
	fn merge_from(&mut self, other: Self) {
		if let serde_json::Value::Object(this) = self {
			if let serde_json::Value::Object(other) = other {
				for (k, v) in other {
					if v.is_null() {
						this.remove(&k);
					}
					else {
						this.entry(k).or_insert(serde_json::Value::Null).merge_from(v);
					}
				}

				return;
			}
		}

		*self = other;
	}
}

impl<T> DeepMerge for Box<T> where T: DeepMerge {
	fn merge_from(&mut self, other: Self) {
		(**self).merge_from(*other);
	}
}

impl<K, V> DeepMerge for std::collections::BTreeMap<K, V> where K: Ord, V: DeepMerge {
	fn merge_from(&mut self, other: Self) {
		for (k, v) in other {
			match self.entry(k) {
				std::collections::btree_map::Entry::Vacant(e) => { e.insert(v); },
				std::collections::btree_map::Entry::Occupied(e) => e.into_mut().merge_from(v),
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

impl<T> DeepMerge for Vec<T> {
	fn merge_from(&mut self, other: Self) {
		self.extend(other);
	}
}
