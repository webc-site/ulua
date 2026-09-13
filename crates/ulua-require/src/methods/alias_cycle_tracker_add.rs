use alloc::string::String;

use ulua_common::{functions::format::format, records::dense_hash_set::DenseHashSet};

use crate::records::alias_cycle_tracker::AliasCycleTracker;

impl Default for AliasCycleTracker {
  fn default() -> Self {
    Self::new()
  }
}

impl AliasCycleTracker {
  /// 空追踪器（对应 C++ `AliasCycleTracker{}`）。
  pub fn new() -> Self {
    Self {
      seen: DenseHashSet::new(String::new()),
      ordered: Vec::new(),
    }
  }

  pub fn add(&mut self, alias: String) -> Option<String> {
    if self.seen.contains(&alias) {
      return Some(format(format_args!(
        "detected alias cycle ({})",
        self.get_stringified_cycle(&alias)
      )));
    }

    self.seen.insert(alias.clone());
    self.ordered.push(alias);
    None
  }
}
