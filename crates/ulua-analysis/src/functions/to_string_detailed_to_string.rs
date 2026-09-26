use alloc::{collections::BTreeSet, string::String, vec::Vec};

use crate::{
  enums::ignore_synthetic_name::IgnoreSyntheticName,
  functions::{
    assign_cycle_names::assign_cycle_names,
    find_cyclic_types::{find_cyclic_types, find_cyclic_types_type_pack_id},
    follow_type::follow,
    get_type::{get, type_variant_of},
    get_type_pack::type_pack_variant_of,
    table_type_to_string_detailed::table_type_to_string_detailed,
  },
  records::{
    metatable_type::MetatableType, stringifier_state::StringifierState, table_type::TableType,
    to_string_options::ToStringOptions, to_string_result::ToStringResult,
    type_pack_stringifier::TypePackStringifier, type_stringifier::TypeStringifier,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack,
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
    type_variant::TypeVariant,
  },
};

pub(crate) fn visit_type_arms(tvs: &mut TypeStringifier, cycle_ty: TypeId) {
  // 变体读取收口在 `type_variant_of`（arena 节点有效性契约同 C++ get）。
  match type_variant_of(cycle_ty) {
    TypeVariant::Bound(b) => {
      let btv = BoundType { bound_to: *b };
      tvs.stringify_bound(cycle_ty, &btv)
    }
    TypeVariant::Error(etv) => tvs.stringify_error(cycle_ty, etv),
    TypeVariant::Free(ftv) => tvs.stringify_free(cycle_ty, ftv),
    TypeVariant::Generic(gtv) => tvs.stringify_generic(cycle_ty, gtv),
    TypeVariant::Primitive(ptv) => tvs.stringify_primitive(cycle_ty, ptv),
    TypeVariant::Singleton(stv) => tvs.stringify_singleton(cycle_ty, stv),
    TypeVariant::Blocked(btv) => tvs.stringify_blocked(cycle_ty, btv),
    TypeVariant::PendingExpansion(petv) => tvs.stringify_pending_expansion(cycle_ty, petv),
    TypeVariant::Function(ftv) => tvs.stringify_function(cycle_ty, ftv),
    TypeVariant::Table(ttv) => tvs.stringify_table(cycle_ty, ttv),
    TypeVariant::Metatable(mtv) => tvs.stringify_metatable(cycle_ty, mtv),
    TypeVariant::Extern(etv) => tvs.stringify_extern(cycle_ty, etv),
    TypeVariant::Any(atv) => tvs.stringify_any(cycle_ty, atv),
    TypeVariant::Union(utv) => tvs.stringify_union(cycle_ty, utv),
    TypeVariant::Intersection(itv) => tvs.stringify_intersection(cycle_ty, itv),
    TypeVariant::Lazy(ltv) => tvs.stringify_lazy(cycle_ty, ltv),
    TypeVariant::Unknown(utv) => tvs.stringify_unknown(cycle_ty, utv),
    TypeVariant::Never(ntv) => tvs.stringify_never(cycle_ty, ntv),
    TypeVariant::Negation(ntv) => tvs.stringify_negation(cycle_ty, ntv),
    TypeVariant::NoRefine(nrt) => tvs.stringify_no_refine(cycle_ty, nrt),
    TypeVariant::TypeFunctionInstance(tfitv) => {
      tvs.stringify_type_function_instance(cycle_ty, tfitv)
    }
  }
}

/// Pack analog of [`visit_type_arms`].
pub(crate) fn visit_pack_arms(tps: &mut TypePackStringifier, cycle_tp: TypePackId) {
  // 变体读取收口在 `type_pack_variant_of`（arena 节点有效性契约同 C++ get）。
  match type_pack_variant_of(cycle_tp) {
    TypePackVariant::Bound(b) => {
      let btv = BoundTypePack { bound_to: *b };
      tps.stringify_bound_pack(cycle_tp, &btv)
    }
    TypePackVariant::Error(_) => {
      let etv = ErrorTypePack {
        index: 0,
        synthetic: None,
      };
      tps.stringify_error_pack(cycle_tp, &etv)
    }
    TypePackVariant::Free(ftv) => tps.stringify_free_pack(cycle_tp, ftv),
    TypePackVariant::Generic(gtv) => tps.stringify_generic_pack(cycle_tp, gtv),
    TypePackVariant::TypePack(pack) => tps.stringify_type_pack(cycle_tp, pack),
    TypePackVariant::Variadic(vtp) => tps.stringify_variadic_pack(cycle_tp, vtp),
    TypePackVariant::Blocked(btp) => tps.stringify_blocked_pack(cycle_tp, btp),
    TypePackVariant::TypeFunctionInstance(tfitp) => {
      tps.stringify_type_function_instance_pack(cycle_tp, tfitp)
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

  // Safety: 构造期把 `opts`/`result` 的活借用裸化为指针存入 state（C++ 引用成员的
  // 直译）。`opts` 是入参 `&mut`、`result` 是本函数局部，二者均存活至 `state` 全部
  // 使用（含被 `tvs`/`tps` 借用）之后，函数返回 `result` 前不再有其他别名写借用。
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
        // Safety: let 链同判据 ttv.name.is_some() 蕴含 Some（cpp `*ttv->name` 前提）。
        let name = ttv
          .name
          .clone()
          .expect("let 链同判据 name.is_some() 蕴含 Some");
        // Safety: `ty` 为 arena 存活 TypeId、`ttv` 由 `get::<TableType>(ty)` 得到
        // （`&'static` 指向同一 arena 节点，非空）；`result`/`opts.scope`/`tvs` 均为
        // 本函数在借用的活对象，`tvs.state` 指向上面构造的存活 `state`。被调 unsafe fn
        // 只读取这些有效输入并写入 `result`，与 C++ 同契约。
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
          // Safety: or 判据 name.is_some() || synthetic_name.is_some()，name 侧
          // None 时 synthetic_name 必为 Some（cpp 同款析取）。
          None => ttv
            .synthetic_name
            .clone()
            .expect("or 判据蕴含：name 侧 None 时 synthetic_name 必为 Some"),
        };
        // Safety: 与上面 `IgnoreSyntheticName::Yes` 分支同一契约——`ty`/`ttv` 指向
        // 同一 arena 存活节点，`result`/`opts.scope`/`tvs` 为活借用，仅只读输入并写 result。
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
        // Safety: let 链同判据 mtv.synthetic_name.is_some() 蕴含 Some。
        result.name = mtv
          .synthetic_name
          .clone()
          .expect("let 链同判据 synthetic_name.is_some() 蕴含 Some");
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
    // stringify_type_id 已降 safe：state 存活与句柄前提在其内部窄块证成。
    tvs.stringify_type_id(ty);
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

/// C++ `ToStringResult toStringDetailed(TypePackId tp, ToStringOptions& opts)`.
pub fn to_string_detailed_type_pack_id_to_string_options(
  tp: TypePackId,
  opts: &mut ToStringOptions,
) -> ToStringResult {
  let mut result = ToStringResult::default();
  // Safety: 同上——构造期裸化 `opts`/`result` 借用存入 state，二者存活至本函数所有
  // 使用（含 `tvs`/`tps` 借用）之后，返回前无其它别名写。
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
