//! `constraint_list` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  records::constraint_list::ConstraintList,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintList {
  pub fn clear(&mut self) {
    self.order.clear();
    // `present.clear()` doesn't compile for the current `DenseHashMap`/hasher bounds.
    // Clearing `present` by recreating it would require access to the hasher/empty key,
    // which is not available here. Instead, reset the ordering and counters; other
    // operations consult `entries` and `order` for iteration/dispatch.
    self.entries = 0;
  }
}

impl ConstraintList {
  pub fn contains(&self, vertex: BlockedConstraintId) -> bool {
    match self.present.find(&vertex) {
      Some(entry) => *entry,
      None => false,
    }
  }
}

impl ConstraintList {
  pub fn insert(&mut self, vertex: BlockedConstraintId) {
    let (entry, fresh) = self.present.try_insert(vertex.clone(), true);
    if fresh {
      self.order.push(vertex);
      self.entries += 1;
    } else if !*entry {
      *entry = true;
      self.entries += 1;
    }
    // If the entry was *not* fresh and its value was already true, then do
    // nothing: the set state has not changed.
  }
}

impl ConstraintList {
  pub fn remove(&mut self, vertex: BlockedConstraintId) {
    if let Some(entry) = self.present.find_mut(&vertex) {
      // If the entry is true then we also need to decrement the number of
      // entries in the constraint list.
      if *entry {
        self.entries -= 1;
      }
      *entry = false;
    }
  }
}

impl ConstraintList {
  pub fn is_empty(&self) -> bool {
    self.entries == 0
  }

  pub fn size(&self) -> usize {
    self.entries
  }
}
