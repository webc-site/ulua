//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1397:assign_cycle_names`
//! Source: `Analysis/src/ToString.cpp:1397-1440` (hand-ported)

use alloc::{collections::BTreeSet, format, string::String};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{follow_type::follow, get_type_alt_j::get},
  records::table_type::TableType,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// C++ `static void assignCycleNames(...)`.
pub fn assign_cycle_names(
  cycles: &BTreeSet<TypeId>,
  cycle_tps: &BTreeSet<TypePackId>,
  cycle_names: &mut DenseHashMap<TypeId, String>,
  cycle_tp_names: &mut DenseHashMap<TypePackId, String>,
  exhaustive: bool,
) {
  let mut next_index = 1;

  for &cycle_ty in cycles.iter() {
    // TODO: use the stringified type list if there are no cycles
    if !exhaustive
      && let Some(ttv) = get::<TableType>(follow(cycle_ty))
      && (ttv.synthetic_name.is_some() || ttv.name.is_some())
    {
      // If we have a cycle type in type parameters, assign a cycle name for this named table
      if ttv
        .instantiated_type_params
        .iter()
        .any(|&el| cycles.contains(&follow(el)))
      {
        *cycle_names.get_or_insert(cycle_ty) = match &ttv.name {
          Some(name) => name.clone(),
          None => ttv.synthetic_name.clone().unwrap(),
        };
      }

      continue;
    }

    let name = format!("t{}", next_index);
    next_index += 1;

    *cycle_names.get_or_insert(cycle_ty) = name;
  }

  for &tp in cycle_tps.iter() {
    let name = format!("tp{}", next_index);
    next_index += 1;
    *cycle_tp_names.get_or_insert(tp) = name;
  }
}
