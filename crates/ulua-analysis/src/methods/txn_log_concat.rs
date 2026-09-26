use crate::records::txn_log::TxnLog;

impl TxnLog {
  pub fn concat(&mut self, rhs: TxnLog) {
    // cpp `typeVarChanges[ty] = std::move(rep)`：operator[] 是无条件覆盖；
    // Box 无法从 &mut 迭代器里移出，这里退化成一次 clone。
    for (ty, rep) in rhs.type_var_changes.iter() {
      if rep.dead {
        continue;
      }

      self.type_var_changes.insert(*ty, rep.clone());
    }

    for (tp, rep) in rhs.type_pack_changes.iter() {
      self.type_pack_changes.insert(*tp, rep.clone());
    }

    self.radioactive |= rhs.radioactive;
  }
}
