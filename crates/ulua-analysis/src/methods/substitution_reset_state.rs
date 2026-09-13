use crate::records::{substitution::Substitution, txn_log::TxnLog, type_arena::TypeArena};

impl Substitution {
  pub fn reset_state(&mut self, log: *const TxnLog, arena: *mut TypeArena) {
    self.base.clear_tarjan(log);

    self.arena = arena;

    self.new_types.clear();
    self.new_packs.clear();
    self.replaced_types.clear();
    self.replaced_type_packs.clear();

    self.no_traverse_types.clear();
    self.no_traverse_type_packs.clear();
  }
}
