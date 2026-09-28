use alloc::{collections::BTreeMap, string::String};
use core::fmt::Debug;

pub fn get<T: Clone + Debug>(map: &BTreeMap<String, T>, name: &str) -> Option<T> {
  map.get(name).cloned()
}
