//! Node: `cxx:Method:Luau.Analysis:Analysis/src/Unifier2.cpp:143:unifier_2_unify`
//! Source: `Analysis/src/Unifier2.cpp:143-147` — `Unifier2::unify(TypePackId, TypePackId)`.
//!
//! The C++ overloads `unify` on (TypeId, TypeId) vs (TypePackId, TypePackId);
//! Rust requires distinct method names, so the pack entry point is `unify_pack`.

use crate::{
  enums::unify_result::UnifyResult, records::unifier_2::Unifier2,
  type_aliases::type_pack_id::TypePackId,
};

impl Unifier2 {
  pub fn unify_pack(&mut self, sub_tp: TypePackId, super_tp: TypePackId) -> UnifyResult {
    self.iteration_count = 0;
    self.unify_type_pack_id_type_pack_id(sub_tp, super_tp)
  }
}
