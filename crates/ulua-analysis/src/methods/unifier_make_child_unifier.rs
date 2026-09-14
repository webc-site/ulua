use alloc::{boxed::Box, vec::Vec};
use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{txn_log::TxnLog, unifier::Unifier};
impl Unifier {
  pub fn unifier_make_child_unifier(&mut self) -> Box<Unifier> {
    let parent_log: *mut TxnLog = &mut self.log;

    Box::new(Unifier {
      types: self.types,
      builtin_types: self.builtin_types,
      normalizer: self.normalizer,
      scope: self.scope,
      log: TxnLog {
        type_var_changes: DenseHashMap::new(null()),
        type_pack_changes: DenseHashMap::new(null()),
        parent: parent_log,
        owned_seen: Vec::new(),
        // Child borrows the parent's seen set; it does not own/free it.
        shared_seen: self.log.shared_seen,
        owned_seen_box: None,
        radioactive: false,
      },
      failure: false,
      errors: Vec::new(),
      location: self.location,
      variance: self.variance,
      normalize: self.normalize,
      check_inhabited: self.check_inhabited,
      ctx: self.ctx,
      shared_state: self.shared_state,
      blocked_types: Vec::new(),
      blocked_type_packs: Vec::new(),
      first_pack_error_pos: None,
    })
  }
}
