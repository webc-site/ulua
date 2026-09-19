//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyTables, L1829-2149)
//!
//! 裸指针安全说明：类型条目经 `TxnLog::txn_log_get_mutable` 以 `*mut TableType`
//! 暴露（对应 C++ TxnLog 的 pending 存储模型，条目可被日志变更重定位，故每次
//! 递归后须重新获取指针，这正是 C++ 原实现的固有形态）。所有 `(*ptr).field`
//! 解引用要么经 `is_null` 检查后进行，要么指针来自刚获取的合法条目；在不改动
//! `functions/` 层 API 的前提下无法替换为安全引用，函数级 `# Safety` 契约覆盖
//! 全部解引用。
use alloc::{string::String, vec::Vec};
use core::{mem::take, ptr::null};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{table_state::TableState, variance::Variance},
  functions::{
    get_mutable_txn_log::get_mutable_pending_type, is_optional::is_optional, is_prim::is_prim,
    maybe_string::maybe_string,
  },
  records::{
    missing_properties::{Context as MissingPropertiesContext, MissingProperties},
    primitive_type::Type as PrimType,
    property_type::Property,
    table_indexer::TableIndexer,
    table_type::TableType,
    unification_too_complex::UnificationTooComplex,
    unifier::Unifier,
  },
  type_aliases::{
    collections::{HashMap, HashMapExt},
    literal_properties::LiteralProperties,
    name_type::Name,
    type_error_data::TypeErrorData,
    type_id::TypeId,
  },
};
impl Unifier {
  /// 吸收子 unifier 结果：上报属性错误、合并事务日志、累计失败标记，并恢复方差。
  /// 雷同逻辑抽自 `try_unify_tables` 的三处属性统一分支。
  fn absorb_child_unifier(
    &mut self,
    inner: Box<Unifier>,
    name: &str,
    super_ty: TypeId,
    sub_ty: TypeId,
    old_variance: Variance,
  ) {
    self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
      &inner.errors,
      name,
      super_ty,
      sub_ty,
    );

    if inner.errors.is_empty() {
      self.log.concat(inner.log);
    }
    self.failure |= inner.failure;
    self.variance = old_variance;
  }

  /// `void Unifier::tryUnifyTables(TypeId sub_ty, TypeId super_ty, bool isIntersection, const LiteralProperties* literalProperties)`
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn unifier_try_unify_tables(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
    is_intersection: bool,
    literal_properties: *const LiteralProperties,
  ) {
    if is_prim(unsafe { self.log.follow_type_id(sub_ty) }, PrimType::Table) {
      sub_ty = unsafe { (*self.builtin_types).empty_table_type };
    }

    if is_prim(
      unsafe { self.log.follow_type_id(super_ty) },
      PrimType::Table,
    ) {
      super_ty = unsafe { (*self.builtin_types).empty_table_type };
    }

    let active_sub_ty = sub_ty;
    let mut super_table = self.log.txn_log_get_mutable::<TableType, TypeId>(super_ty);
    let mut sub_table = self.log.txn_log_get_mutable::<TableType, TypeId>(sub_ty);

    if super_table.is_null() || sub_table.is_null() {
      self.ice_string("passed non-table types to unifyTables");
    }

    let mut missing_properties: Vec<String> = Vec::new();
    let mut extra_properties: Vec<String> = Vec::new();

    if fflag::LuauInstantiateInSubtyping.get()
      && self.variance == Variance::Covariant
      && table_state(sub_table) == TableState::Generic
      && table_state(super_table) != TableState::Generic
    {
      // The Instantiation machinery is translated elsewhere in this crate;
      // keep this branch structurally present but avoid inventing
      // construction APIs (mirrors Unifier::tryUnifyFunctions). The C++
      // failure path here reports UnificationTooComplex.
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
      );
    }

    // Optimization: First test that the property sets are compatible without doing any recursive unification
    if !has_indexer(sub_table) && table_state(sub_table) != TableState::Free {
      // SAFETY: 循环体只读 props、只写 missing_properties，迭代器存活期内无写入。
      for (prop_name, super_prop) in unsafe { (*super_table).props.iter() } {
        if !props_contains(sub_table, prop_name)
          && table_state(sub_table) == TableState::Unsealed
          && !is_optional(super_prop.type_deprecated())
        {
          missing_properties.push(prop_name.clone());
        }
      }

      if !missing_properties.is_empty() {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::MissingProperties(MissingProperties {
            super_type: super_ty,
            sub_type: sub_ty,
            properties: take(&mut missing_properties),
            context: MissingPropertiesContext::Missing,
          }),
        );
        return;
      }
    }

    // And vice versa if we're invariant
    if self.variance == Variance::Invariant
      && !has_indexer(super_table)
      && table_state(super_table) != TableState::Unsealed
      && table_state(super_table) != TableState::Free
    {
      // SAFETY: 循环体只读 props、只写 extra_properties，迭代器存活期内无写入。
      for (prop_name, _sub_prop) in unsafe { (*sub_table).props.iter() } {
        if !props_contains(super_table, prop_name) {
          extra_properties.push(prop_name.clone());
        }
      }

      if !extra_properties.is_empty() {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::MissingProperties(MissingProperties {
            super_type: super_ty,
            sub_type: sub_ty,
            properties: take(&mut extra_properties),
            context: MissingPropertiesContext::Extra,
          }),
        );
        return;
      }
    }

    // Width subtyping: any property in the supertype must be in the subtype,
    // and the types must agree.
    // 键快照：循环体内 props 可能被写入（pending 表插入），须先取键再统一。
    let super_props = props_keys(super_table);
    for name in super_props {
      let Some(prop_ty) = props_type_of(super_table, &name) else {
        continue;
      };
      // 仅取 read_ty（TypeId 为 Copy），避免整属性 clone 的分配开销
      let sub_prop_ty = props_type_of(sub_table, &name);

      if let Some(sub_prop_ty) = sub_prop_ty {
        // TODO: read-only properties don't need invariance
        let old_variance = self.variance;
        if not_in_literal_properties(literal_properties, &name) {
          self.variance = Variance::Invariant;
        }

        let mut inner_state = self.unifier_make_child_unifier();
        inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
          sub_prop_ty,
          prop_ty,
          false,
          false,
          None,
        );

        self.absorb_child_unifier(inner_state, &name, super_ty, sub_ty, old_variance);
      } else if has_string_indexer(sub_table) {
        // TODO: read-only indexers don't need invariance
        let old_variance = self.variance;
        if not_in_literal_properties(literal_properties, &name) {
          self.variance = Variance::Invariant;
        }

        let index_result = index_result_of(sub_table);
        let mut inner_state = self.unifier_make_child_unifier();
        inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
          index_result,
          prop_ty,
          false,
          false,
          None,
        );

        self.absorb_child_unifier(inner_state, &name, super_ty, sub_ty, old_variance);
      } else if table_state(sub_table) == TableState::Unsealed && is_optional(prop_ty) {
        // This is sound because unsealed table types are precise.
      } else if table_state(sub_table) == TableState::Free {
        // SAFETY: name 是 super_table.props 的键且本迭代内 props 未被改写，get 必命中。
        let prop_clone = props_get_clone(super_table, &name).unwrap();
        // SAFETY: pending_sub 由 queue_type_id 返回，变体为 TableType（断言同 C++）。
        let pending_sub = unsafe { self.log.queue_type_id(active_sub_ty) };
        let ttv = unsafe { get_mutable_pending_type::<TableType>(pending_sub) };
        LUAU_ASSERT!(!ttv.is_null());
        props_insert(ttv, name.clone(), prop_clone);
        sub_table = ttv;
      } else {
        missing_properties.push(name.clone());
      }

      // 递归统一可能改写 txn log 使旧表指针失效；检测到则按 C++ 重启。
      if self.try_unify_tables_restart(
        sub_ty,
        super_ty,
        active_sub_ty,
        is_intersection,
        super_table,
        sub_table,
      ) {
        return;
      }
    }

    let sub_props = props_keys(sub_table);
    for name in sub_props {
      let Some(prop) = props_get_clone(sub_table, &name) else {
        continue;
      };

      if props_contains(super_table, &name) {
        // already unified above
      } else if has_string_indexer(super_table) {
        let old_variance = self.variance;
        if not_in_literal_properties(literal_properties, &name) {
          self.variance = Variance::Invariant;
        }

        let super_index_result = index_result_of(super_table);
        let mut inner_state = self.unifier_make_child_unifier();
        if fflag::LuauFixIndexerSubtypingOrdering.get() {
          inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
            prop.type_deprecated(),
            super_index_result,
            false,
            false,
            None,
          );
        } else {
          // Incredibly, the old solver depends on this bug somehow.
          inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
            super_index_result,
            prop.type_deprecated(),
            false,
            false,
            None,
          );
        }

        self.absorb_child_unifier(inner_state, &name, super_ty, sub_ty, old_variance);
      } else if table_state(super_table) == TableState::Unsealed {
        let mut clone = prop.clone();
        let deep = self.unifier_deeply_optional(clone.type_deprecated(), &mut HashMap::new());
        clone.set_type(deep);

        // SAFETY: pending_super 由 queue_type_id 返回，变体为 TableType（断言同 C++）。
        let pending_super = unsafe { self.log.queue_type_id(super_ty) };
        let pending_super_ttv = unsafe { get_mutable_pending_type::<TableType>(pending_super) };
        props_insert(pending_super_ttv, name.clone(), clone);
        super_table = pending_super_ttv;
      } else if self.variance == Variance::Covariant {
        // nothing
      } else if table_state(super_table) == TableState::Free {
        // SAFETY: pending_super 由 queue_type_id 返回，变体为 TableType（断言同 C++）。
        let pending_super = unsafe { self.log.queue_type_id(super_ty) };
        let pending_super_ttv = unsafe { get_mutable_pending_type::<TableType>(pending_super) };
        props_insert(pending_super_ttv, name.clone(), prop.clone());
        super_table = pending_super_ttv;
      } else {
        extra_properties.push(name.clone());
      }

      // 递归统一可能改写 txn log 使旧表指针失效；检测到则按 C++ 重启。
      if self.try_unify_tables_restart(
        sub_ty,
        super_ty,
        active_sub_ty,
        is_intersection,
        super_table,
        sub_table,
      ) {
        return;
      }
    }

    // Unify indexers
    let super_has_indexer = has_indexer(super_table);
    let sub_has_indexer = has_indexer(sub_table);

    if super_has_indexer && sub_has_indexer {
      let old_variance = self.variance;
      self.variance = Variance::Invariant;

      let sub_index_type = index_key_of(sub_table);
      let super_index_type = index_key_of(super_table);
      let sub_index_result = index_result_of(sub_table);
      let super_index_result = index_result_of(super_table);

      let mut inner_state = self.unifier_make_child_unifier();

      inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_index_type,
        super_index_type,
        false,
        false,
        None,
      );

      let reported = !inner_state.errors.is_empty();

      self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
        &inner_state.errors,
        "[indexer key]",
        super_ty,
        sub_ty,
      );

      inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_index_result,
        super_index_result,
        false,
        false,
        None,
      );

      if !reported {
        self.check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
          &inner_state.errors,
          "[indexer value]",
          super_ty,
          sub_ty,
        );
      }

      if inner_state.errors.is_empty() {
        self.log.concat(inner_state.log);
      }
      self.failure |= inner_state.failure;
      self.variance = old_variance;
    } else if super_has_indexer {
      if table_state(sub_table) == TableState::Unsealed
        || table_state(sub_table) == TableState::Free
      {
        let indexer = indexer_of(super_table);
        self.log.change_indexer(sub_ty, indexer);
      }
    } else if sub_has_indexer && self.variance == Variance::Invariant {
      // Symmetric if we are invariant
      if table_state(super_table) == TableState::Unsealed
        || table_state(super_table) == TableState::Free
      {
        let indexer = indexer_of(sub_table);
        self.log.change_indexer(super_ty, indexer);
      }
    }

    // Changing the indexer can invalidate the table pointers.
    let super_ty_f = unsafe { self.log.follow_type_id(super_ty) };
    let sub_ty_f = unsafe { self.log.follow_type_id(active_sub_ty) };
    super_table = self
      .log
      .txn_log_get_mutable::<TableType, TypeId>(super_ty_f);
    sub_table = self.log.txn_log_get_mutable::<TableType, TypeId>(sub_ty_f);

    if super_table.is_null() || sub_table.is_null() {
      return;
    }

    if !missing_properties.is_empty() {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::MissingProperties(MissingProperties {
          super_type: super_ty,
          sub_type: sub_ty,
          properties: take(&mut missing_properties),
          context: MissingPropertiesContext::Missing,
        }),
      );
      return;
    }

    if !extra_properties.is_empty() {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::MissingProperties(MissingProperties {
          super_type: super_ty,
          sub_type: sub_ty,
          properties: take(&mut extra_properties),
          context: MissingPropertiesContext::Extra,
        }),
      );
      return;
    }

    // Types are commonly cyclic; unifying a property may change the table itself.
    if has_bound(super_table) || has_bound(sub_table) {
      return self.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_ty, super_ty, false, false, None,
      );
    }

    if table_state(super_table) == TableState::Free {
      self.log.bind_table(super_ty, Some(sub_ty));
    } else if table_state(sub_table) == TableState::Free {
      self.log.bind_table(sub_ty, Some(super_ty));
    }
  }

  /// 递归统一可能改写 txn log 使旧表指针失效；若检测到已发生，按 C++ 重启。
  /// 返回 true 表示已触发重启，调用方须立即 return。
  ///
  /// 内部契约：首个重启路径（类型不再相同）先于指针比对返回，故走到比对时
  /// sub_ty/super_ty 的表身份未变，递归 `unifier_try_unify_tables` 的调用契约
  /// 由本函数自身成立，无需调用方再保证。
  fn try_unify_tables_restart(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    active_sub_ty: TypeId,
    is_intersection: bool,
    super_table: *mut TableType,
    sub_table: *mut TableType,
  ) -> bool {
    let super_ty_new = unsafe { self.log.follow_type_id(super_ty) };
    let sub_ty_new = unsafe { self.log.follow_type_id(active_sub_ty) };

    if (super_ty != super_ty_new || active_sub_ty != sub_ty_new) && self.errors.is_empty() {
      self.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_ty,
        super_ty,
        false,
        is_intersection,
        None,
      );
      return true;
    }

    let new_super_table = self
      .log
      .txn_log_get_mutable::<TableType, TypeId>(super_ty_new);
    let new_sub_table = self
      .log
      .txn_log_get_mutable::<TableType, TypeId>(sub_ty_new);

    if super_table != new_super_table || sub_table != new_sub_table {
      if self.errors.is_empty() {
        // SAFETY: 上面已确认 sub_ty/super_ty 仍为表（类型身份未变），契约成立。
        unsafe { self.unifier_try_unify_tables(sub_ty, super_ty, is_intersection, null()) };
      }
      return true;
    }
    false
  }
}

/// 读取表状态（t 非空；裸指针契约同 C++ `getMutable`，只读）。
fn table_state(t: *const TableType) -> TableState {
  unsafe { (*t).state }
}

/// t 是否有 indexer（t 非空，只读）。
fn has_indexer(t: *const TableType) -> bool {
  unsafe { (*t).indexer.is_some() }
}

/// t 的 indexer 键类型是否为 string（t 非空，只读）。
fn has_string_indexer(t: *const TableType) -> bool {
  unsafe {
    (*t)
      .indexer
      .as_ref()
      .is_some_and(|ix| maybe_string(ix.index_type))
  }
}

/// t 的 indexer 整体拷贝（TableIndexer 为 Copy；t 非空，只读）。
fn indexer_of(t: *const TableType) -> Option<TableIndexer> {
  unsafe { (*t).indexer }
}

/// t 的 indexer 键类型（调用分支已保证 Some；t 非空，只读）。
fn index_key_of(t: *const TableType) -> TypeId {
  unsafe { (*t).indexer.as_ref().unwrap().index_type }
}

/// t 的 indexer 值类型（调用分支已保证 Some；t 非空，只读）。
fn index_result_of(t: *const TableType) -> TypeId {
  unsafe { (*t).indexer.as_ref().unwrap().index_result_type }
}

/// t 是否已绑定到其他表（t 非空，只读）。
fn has_bound(t: *const TableType) -> bool {
  unsafe { (*t).bound_to.is_some() }
}

/// props 是否含属性（t 非空，只读）。
fn props_contains(t: *const TableType, name: &Name) -> bool {
  unsafe { (*t).props.contains_key(name) }
}

/// props 中属性的类型句柄（t 非空，只读）。
fn props_type_of(t: *const TableType, name: &Name) -> Option<TypeId> {
  unsafe { (*t).props.get(name).map(|p| p.type_deprecated()) }
}

/// props 中属性的克隆（t 非空，只读）。
fn props_get_clone(t: *const TableType, name: &Name) -> Option<Property> {
  unsafe { (*t).props.get(name).cloned() }
}

/// props 键快照（循环体内 props 可能被写入，须先取键再统一）。
fn props_keys(t: *const TableType) -> Vec<String> {
  unsafe { (*t).props.keys().cloned().collect() }
}

/// props 插入属性（t 非空；写入路径，契约同 C++ `ttv->props[name] = prop`）。
fn props_insert(t: *mut TableType, name: Name, prop: Property) {
  unsafe { (*t).props.insert(name, prop) };
}

/// `!literalProperties || !literalProperties->contains(name)`（C++ Resetter 分支同款）。
fn not_in_literal_properties(lp: *const LiteralProperties, name: &Name) -> bool {
  lp.is_null() || unsafe { (*lp).find(name).is_none() }
}
