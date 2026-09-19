use crate::records::tarjan::Tarjan;

impl Tarjan {
  /// C++ `Tarjan::visitSCC(int)` (`Substitution.cpp:515-549`).
  ///
  /// `isDirty` and `foundDirty` are pure-virtual in C++; here they dispatch to
  /// the subclass through the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable).
  pub fn visit_scc(&mut self, index: i32) {
    let mut d = self.get_dirty(index);

    let owner = self.vtable.owner;
    let is_dirty_ty = self.vtable.is_dirty_ty;
    let is_dirty_tp = self.vtable.is_dirty_tp;

    // Snapshot of the stack iterated rbegin..rend (the C++ reverse iterators).
    // Taken once: it is also reused for the foundDirty pass below, and copying
    // it avoids holding a borrow of `self.stack` across the `&mut self`
    // mutations / vtable callbacks.
    let stack_rev: Vec<i32> = self.stack.iter().copied().rev().collect();

    for &it in stack_rev.iter() {
      if d {
        break;
      }

      let node = &self.nodes[it as usize];
      let nty = node.ty;
      let ntp = node.tp;

      if !nty.is_null() {
        if let Some(f) = is_dirty_ty {
          d = f(owner, nty);
        }
      } else if !ntp.is_null()
        && let Some(f) = is_dirty_tp
      {
        d = f(owner, ntp);
      }

      if it == index {
        break;
      }
    }

    if !d {
      return;
    }

    let found_dirty_ty = self.vtable.found_dirty_ty;
    let found_dirty_tp = self.vtable.found_dirty_tp;

    for &it in stack_rev.iter() {
      self.set_dirty(it, true);

      let node = &self.nodes[it as usize];
      let nty = node.ty;
      let ntp = node.tp;

      if !nty.is_null() {
        if let Some(f) = found_dirty_ty {
          f(owner, nty);
        }
      } else if !ntp.is_null()
        && let Some(f) = found_dirty_tp
      {
        f(owner, ntp);
      }

      if it == index {
        return;
      }
    }
  }
}
