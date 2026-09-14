//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:126:find_cyclic_types`
//! Source: `Analysis/src/ToString.cpp:126-136` (hand-ported)
//!
//! C++ `template<typename TID> void findCyclicTypes(...)` — instantiated for
//! `TypeId` and `TypePackId`; the two Rust monomorphs are spelled out.

use alloc::collections::BTreeSet;
use core::mem::take;

use crate::{
  records::{find_cyclic_types::FindCyclicTypes, generic_type_visitor::GenericTypeVisitorTrait},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn find_cyclic_types(
  cycles: &mut BTreeSet<TypeId>,
  cycle_tps: &mut BTreeSet<TypePackId>,
  ty: TypeId,
  exhaustive: bool,
) {
  let mut fct = FindCyclicTypes::new();
  fct.exhaustive = exhaustive;
  fct.traverse_type_id(ty);

  *cycles = take(&mut fct.cycles);
  *cycle_tps = take(&mut fct.cycle_tps);
}

/// The `TID = TypePackId` instantiation.
pub fn find_cyclic_types_type_pack_id(
  cycles: &mut BTreeSet<TypeId>,
  cycle_tps: &mut BTreeSet<TypePackId>,
  tp: TypePackId,
  exhaustive: bool,
) {
  let mut fct = FindCyclicTypes::new();
  fct.exhaustive = exhaustive;
  fct.traverse_type_pack_id(tp);

  *cycles = take(&mut fct.cycles);
  *cycle_tps = take(&mut fct.cycle_tps);
}
