//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnify(TypePackId,...), L1394-1399)
use crate::{records::unifier::Unifier, type_aliases::type_pack_id::TypePackId};

impl Unifier {
  /// `void Unifier::tryUnify(TypePackId subTp, TypePackId superTp, bool isFunctionCall)`
  pub fn try_unify_type_pack_id_type_pack_id_bool_entry(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    is_function_call: bool,
  ) {
    unsafe {
      (*self.shared_state).counters.iteration_count = 0;
    }

    self.try_unify_type_pack_id_type_pack_id_bool(sub_tp, super_tp, is_function_call);
  }
}
