use alloc::vec::Vec;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::alias_cycle_tracker::AliasCycleTracker;

/// cpp 环检测消息的头部。
const CYCLE_PREFIX: &[u8] = b"detected alias cycle (";

impl AliasCycleTracker {
  /// 空追踪器（对应 C++ `AliasCycleTracker{}`）。
  pub fn new() -> Self {
    Self {
      seen: DenseHashSet::new(Vec::new()),
      ordered: Vec::new(),
    }
  }

  /// 记录别名；已在环中出现则返回字节精确的环消息（对应 cpp `add(std::string)`）。
  pub fn add(&mut self, alias: Vec<u8>) -> Option<Vec<u8>> {
    if self.seen.contains(&alias) {
      let mut message = Vec::with_capacity(CYCLE_PREFIX.len() + alias.len() * 2 + 16);
      message.extend_from_slice(CYCLE_PREFIX);
      self.append_cycle(&mut message, &alias);
      message.push(b')');
      return Some(message);
    }

    self.seen.insert(alias.clone());
    self.ordered.push(alias);
    None
  }
}
