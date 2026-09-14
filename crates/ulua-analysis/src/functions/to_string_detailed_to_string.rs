//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1483:to_string_detailed`
//! Source: `Analysis/src/ToString.cpp:1483-1615` (hand-ported)

use alloc::{collections::BTreeSet, string::String, vec::Vec};

/// The where-clause definition printer: C++ `Luau::visit([&tvs](auto&& t){ return tvs(cycleTy, t); }, cycleTy->ty)`
/// — dispatches DIRECTLY to the arms, bypassing `stringify`'s cycle-name
/// lookup so the definition prints expanded.
use crate::type_aliases::bound_type::BoundType;
use crate::{
  enums::ignore_synthetic_name::IgnoreSyntheticName,
  functions::{
    assign_cycle_names::assign_cycle_names, find_cyclic_types::find_cyclic_types,
    follow_type::follow, get_type_alt_j::get,
    table_type_to_string_detailed::table_type_to_string_detailed,
  },
  records::{
    metatable_type::MetatableType, stringifier_state::StringifierState, table_type::TableType,
    to_string_options::ToStringOptions, to_string_result::ToStringResult,
    type_pack_stringifier::TypePackStringifier, type_stringifier::TypeStringifier,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};
pub(crate) fn visit_type_arms(tvs: &mut TypeStringifier, cycle_ty: TypeId) {
  // SAFETY: cycle_ty 指向类型 arena 中的 Type，仅读取 variant
  unsafe {
    match &(*cycle_ty).ty {
      TypeVariant::Bound(b) => {
        let btv = BoundType { bound_to: *b };
        tvs.operator_call_10(cycle_ty, &btv)
      }
      TypeVariant::Error(etv) => tvs.operator_call_11(cycle_ty, etv),
      TypeVariant::Free(ftv) => tvs.operator_call_2(cycle_ty, ftv),
      TypeVariant::Generic(gtv) => tvs.operator_call_3(cycle_ty, gtv),
      TypeVariant::Primitive(ptv) => tvs.operator_call_17(cycle_ty, ptv),
      TypeVariant::Singleton(stv) => tvs.operator_call_18(cycle_ty, stv),
      TypeVariant::Blocked(btv) => tvs.operator_call_9(cycle_ty, btv),
      TypeVariant::PendingExpansion(petv) => tvs.operator_call_6(cycle_ty, petv),
      TypeVariant::Function(ftv) => tvs.operator_call_12(cycle_ty, ftv),
      TypeVariant::Table(ttv) => tvs.operator_call_7(cycle_ty, ttv),
      TypeVariant::Metatable(mtv) => tvs.operator_call_5(cycle_ty, mtv),
      TypeVariant::Extern(etv) => tvs.operator_call(cycle_ty, etv),
      TypeVariant::Any(atv) => tvs.operator_call_8(cycle_ty, atv),
      TypeVariant::Union(utv) => tvs.operator_call_20(cycle_ty, utv),
      TypeVariant::Intersection(itv) => tvs.operator_call_4(cycle_ty, itv),
      TypeVariant::Lazy(ltv) => tvs.operator_call_13(cycle_ty, ltv),
      TypeVariant::Unknown(utv) => tvs.operator_call_21(cycle_ty, utv),
      TypeVariant::Never(ntv) => tvs.operator_call_15(cycle_ty, ntv),
      TypeVariant::Negation(ntv) => tvs.operator_call_14(cycle_ty, ntv),
      TypeVariant::NoRefine(nrt) => tvs.operator_call_16(cycle_ty, nrt),
      TypeVariant::TypeFunctionInstance(tfitv) => tvs.operator_call_19(cycle_ty, tfitv),
    }
  }
}

/// Pack analog of [`visit_type_arms`].
pub(crate) fn visit_pack_arms(tps: &mut TypePackStringifier, cycle_tp: TypePackId) {
  // SAFETY: cycle_tp 指向类型 arena 中的 TypePackVar，仅读取 variant
  unsafe {
    match &(*cycle_tp).ty {
      TypePackVariant::Bound(b) => {
        let btv = BoundTypePack { bound_to: *b };
        tps.operator_call_4(cycle_tp, &btv)
      }
      TypePackVariant::Error(_) => {
        let etv = ErrorTypePack {
          index: 0,
          synthetic: None,
        };
        tps.operator_call_5(cycle_tp, &etv)
      }
      TypePackVariant::Free(ftv) => tps.operator_call(cycle_tp, ftv),
      TypePackVariant::Generic(gtv) => tps.operator_call_2(cycle_tp, gtv),
      TypePackVariant::TypePack(pack) => tps.operator_call_7(cycle_tp, pack),
      TypePackVariant::Variadic(vtp) => tps.operator_call_8(cycle_tp, vtp),
      TypePackVariant::Blocked(btp) => tps.operator_call_3(cycle_tp, btp),
      TypePackVariant::TypeFunctionInstance(tfitp) => tps.operator_call_6(cycle_tp, tfitp),
    }
  }
}

/// C++ `ToStringResult toStringDetailed(TypeId ty, ToStringOptions& opts)`.
pub fn to_string_detailed(ty: TypeId, opts: &mut ToStringOptions) -> ToStringResult {
  /*
   * 1. Walk the Type and track seen TypeIds.  When you reencounter a TypeId, add it to a set of seen cycles.
   * 2. Generate some names for each cycle.  For a starting point, we can just call them t0, t1 and so on.
   * 3. For each seen cycle, stringify it like we do now, but replace each known cycle with its name.
   * 4. Print out the root of the type using the same algorithm as step 3.
   */
  let ty = follow(ty);
  let mut result = ToStringResult::default();

  let mut state = unsafe {
    StringifierState::stringifier_state_stringifier_state(
      opts as *mut ToStringOptions,
      &mut result as *mut ToStringResult,
    )
  };

  let mut cycles: BTreeSet<TypeId> = BTreeSet::new();
  let mut cycle_tps: BTreeSet<TypePackId> = BTreeSet::new();

  find_cyclic_types(&mut cycles, &mut cycle_tps, ty, opts.exhaustive);

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

  if !opts.exhaustive {
    if state.ignore_synthetic_name {
      if let Some(ttv) = get::<TableType>(ty)
        && ttv.name.is_some()
      {
        let name = ttv.name.clone().unwrap();
        unsafe {
          table_type_to_string_detailed(
            ty,
            ttv as *const TableType,
            IgnoreSyntheticName::Yes,
            &mut result,
            &opts.scope,
            &name,
            &mut tvs,
          )
        };

        return result;
      }
    } else {
      if let Some(ttv) = get::<TableType>(ty)
        && (ttv.name.is_some() || ttv.synthetic_name.is_some())
      {
        let name = match &ttv.name {
          Some(name) => name.clone(),
          None => ttv.synthetic_name.clone().unwrap(),
        };
        unsafe {
          table_type_to_string_detailed(
            ty,
            ttv as *const TableType,
            IgnoreSyntheticName::No,
            &mut result,
            &opts.scope,
            &name,
            &mut tvs,
          )
        };

        return result;
      }

      if let Some(mtv) = get::<MetatableType>(ty)
        && mtv.synthetic_name.is_some()
      {
        result.invalid = true;
        result.name = mtv.synthetic_name.clone().unwrap();
        return result;
      }
    }
  }

  /* If the root itself is a cycle, we special case a little.
   * We go out of our way to print the following:
   *
   * t1 where t1 = the_whole_root_type
   */
  if let Some(p) = state.cycle_names.find(&ty) {
    let name = p.clone();
    state.emit(name.as_str());
  } else {
    unsafe { tvs.stringify_type_id(ty) };
  }

  if !state.cycle_names.empty() || !state.cycle_tp_names.empty() {
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
    result.truncated = true;

    result.name.push_str("... *TRUNCATED*");
  }

  result
}

// Pinned overload name advertised by the dependency cards.
pub use to_string_detailed as to_string_detailed_type_id_to_string_options;
