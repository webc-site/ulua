//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1617:to_string_detailed`
//! Source: `Analysis/src/ToString.cpp:1617-1720` (hand-ported)
//!
//! Differences from the TypeId overload, preserved 1:1: no `follow` of the
//! root, the where-clause guard checks the raw cycle SETS (not the name
//! maps), and truncation does NOT set `result.truncated`.

use alloc::{collections::BTreeSet, string::String, vec::Vec};

use crate::{
  functions::{
    assign_cycle_names::assign_cycle_names,
    find_cyclic_types::find_cyclic_types_type_pack_id,
    to_string_detailed_to_string::{visit_pack_arms, visit_type_arms},
  },
  records::{
    stringifier_state::StringifierState, to_string_options::ToStringOptions,
    to_string_result::ToStringResult, type_pack_stringifier::TypePackStringifier,
    type_stringifier::TypeStringifier,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// C++ `ToStringResult toStringDetailed(TypePackId tp, ToStringOptions& opts)`.
pub fn to_string_detailed_type_pack_id_to_string_options(
  tp: TypePackId,
  opts: &mut ToStringOptions,
) -> ToStringResult {
  let mut result = ToStringResult::default();
  let mut state = unsafe {
    StringifierState::stringifier_state_stringifier_state(
      opts as *mut ToStringOptions,
      &mut result as *mut ToStringResult,
    )
  };

  let mut cycles: BTreeSet<TypeId> = BTreeSet::new();
  let mut cycle_tps: BTreeSet<TypePackId> = BTreeSet::new();

  find_cyclic_types_type_pack_id(&mut cycles, &mut cycle_tps, tp, opts.exhaustive);

  assign_cycle_names(
    &cycles,
    &cycle_tps,
    &mut state.cycle_names,
    &mut state.cycle_tp_names,
    opts.exhaustive,
  );

  let mut tvs = TypeStringifier {
    state: &mut state as *mut StringifierState,
  };

  /* If the root itself is a cycle, we special case a little.
   * We go out of our way to print the following:
   *
   * t1 where t1 = the_whole_root_type
   */
  if let Some(p) = state.cycle_tp_names.find(&tp) {
    let name = p.clone();
    state.emit(name.as_str());
  } else {
    tvs.stringify_type_pack_id(tp);
  }

  if !cycles.is_empty() || !cycle_tps.is_empty() {
    result.cycle = true;
    state.emit(" where ");
  }

  state.exhaustive = true;

  let mut sorted_cycle_names: Vec<(TypeId, String)> = state
    .cycle_names
    .iter()
    .map(|(k, v)| (*k, v.clone()))
    .collect();
  sorted_cycle_names.sort_unstable_by(|a, b| a.1.cmp(&b.1));

  let mut semi = false;
  for (cycle_ty, name) in sorted_cycle_names.iter() {
    if semi {
      state.emit(" ; ");
    }

    state.emit(name.as_str());
    state.emit(" = ");
    visit_type_arms(&mut tvs, *cycle_ty);

    semi = true;
  }

  let mut sorted_cycle_tp_names: Vec<(TypePackId, String)> = state
    .cycle_tp_names
    .iter()
    .map(|(k, v)| (*k, v.clone()))
    .collect();
  sorted_cycle_tp_names.sort_unstable_by(|a, b| a.1.cmp(&b.1));

  let mut tps = TypePackStringifier::type_pack_stringifier_stringifier_state(
    &mut state as *mut StringifierState,
  );

  for (cycle_tp, name) in sorted_cycle_tp_names.iter() {
    if semi {
      state.emit(" ; ");
    }

    state.emit(name.as_str());
    state.emit(" = ");
    visit_pack_arms(&mut tps, *cycle_tp);

    semi = true;
  }

  if opts.max_type_length > 0 && result.name.len() > opts.max_type_length {
    result.name.push_str("... *TRUNCATED*");
  }

  result
}
