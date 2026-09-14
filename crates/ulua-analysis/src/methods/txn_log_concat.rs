use crate::records::txn_log::TxnLog;

impl TxnLog {
  pub fn concat(&mut self, rhs: TxnLog) {
    for (ty, rep) in rhs.type_var_changes.iter() {
      if rep.dead {
        continue;
      }

      if let Some(existing) = self.type_var_changes.find_mut(ty) {
        *existing = rep.clone();
      } else {
        self.type_var_changes.try_insert(*ty, rep.clone());
      }
    }

    for (tp, rep) in rhs.type_pack_changes.iter() {
      if let Some(existing) = self.type_pack_changes.find_mut(tp) {
        *existing = rep.clone();
      } else {
        self.type_pack_changes.try_insert(*tp, rep.clone());
      }
    }

    self.radioactive |= rhs.radioactive;
  }
}
