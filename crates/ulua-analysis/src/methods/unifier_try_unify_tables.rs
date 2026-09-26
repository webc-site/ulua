//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyTables, L1829-2149)
//!
//! 裸指针安全说明：类型条目经 `TxnLog::txn_log_get_mutable` 以 `*mut TableType`
//! 暴露（对应 C++ TxnLog 的 pending 存储模型，条目可被日志变更重定位，故每次
//! 递归后须重新获取指针，这正是 C++ 原实现的固有形态）。所有 `(*ptr).field`
//! 解引用要么经 `is_null` 检查后进行，要么指针来自刚获取的合法条目；在不改动
//! `functions/` 层 API 的前提下无法替换为安全引用，函数级 `# Safety` 契约覆盖
//! 全部解引用。
use alloc::{string::String, vec::Vec};
use core::{
  mem::take,
  ptr::{from_mut, null},
};

use ulua_common::fflag;

use crate::{
  enums::{table_state::TableState, variance::Variance},
  functions::{
    get_mutable_txn_log::get_mutable_pending_type, is_optional::is_optional, is_prim::is_prim,
    maybe_string::maybe_string,
  },
  records::{
    instantiation::Instantiation,
    missing_properties::{Context as MissingPropertiesContext, MissingProperties},
    primitive_type::Type as PrimType,
    property_type::Property,
    table_indexer::TableIndexer,
    table_type::TableType,
    unification_too_complex::UnificationTooComplex,
    unifier::Unifier,
  },
  type_aliases::{
    collections::HashMap, literal_properties::LiteralProperties, name_type::Name,
    type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl Unifier {
  /// 吸收子 unifier 结果：上报属性错误、合并事务日志、累计失败标记，并恢复方差。
  /// 雷同逻辑抽自 `try_unify_tables` 的三处属性统一分支。
  fn absorb_child_unifier(
    &mut self,
    inner: Unifier,
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
  ///
  /// # Safety
  /// - `sub_ty`、`super_ty`：必须是经 `self.log.txn_log_get_mutable::<TableType>`
  ///   可解析为非空 pending 表项的表类型句柄。唯一调用方
  ///   （`unifier_try_unify_unifier.rs`）在入口处已用
  ///   `txn_log_get::<TableType>(…).is_some()` 双向验证；违约时函数头走
  ///   `ice_string` 分支但不会中止（C++ 对应 LUAU_ASSERT+throw），后续对
  ///   `sub_table`/`super_table` 的解引用即失去非空前提。
  /// - `is_intersection`：纯数据位（C++ bool 默认参数），无指针契约。
  /// - `literal_properties`：null（C++ 默认 `nullptr`）或指向在本调用完整
  ///   存续期内保持存活且不被改写的 `LiteralProperties`——函数仅经
  ///   `not_in_literal_properties` 只读 `find`，不跨调用保存该指针；非空侧
  ///   由调用方自 `Option<&LiteralProperties>` 借转裸，借用覆盖整个同步调用。
  pub unsafe fn unifier_try_unify_tables(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
    is_intersection: bool,
    literal_properties: *const LiteralProperties,
  ) {
    if is_prim(self.log.follow_type_id(sub_ty), PrimType::Table) {
      sub_ty = self.builtin_types_ref().empty_table_type;
    }

    if is_prim(self.log.follow_type_id(super_ty), PrimType::Table) {
      super_ty = self.builtin_types_ref().empty_table_type;
    }

    let mut active_sub_ty = sub_ty;
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
      // C++: `Instantiation instantiation{&log, types, builtinTypes, subTable->level, scope}`
      // 实例化成功时后续统一走实例化后的类型（activeSubTy / subTable 同步替换），
      // 仅 substitute 失败（过于复杂）才报 UnificationTooComplex。
      let mut instantiation = Instantiation::instantiation_new(
        &self.log as *const _,
        Some(self.types),
        self.builtin_types,
        // Safety: `sub_table` 是函数头 txn_log_get_mutable 取得的 pending 表项
        // 指针，按 fn 契约非空（上方 is_null 已入 ice 分支）；此处只读 Copy
        // `level`（C++ `subTable->level` 同形），Instantiation 仅在本分支同步
        // 使用它。
        unsafe { (*sub_table).level },
        self.scope.as_ptr(),
      );

      if let Some(instantiated) = instantiation.substitute_type_id(sub_ty) {
        active_sub_ty = instantiated;
        sub_table = self
          .log
          .txn_log_get_mutable::<TableType, TypeId>(active_sub_ty);

        if sub_table.is_null() {
          self
            .ice_string("instantiation made a table type into a non-table type in tryUnifyTables");
        }
      } else {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
        );
      }
    }

    // Optimization: First test that the property sets are compatible without doing any recursive unification
    if !has_indexer(sub_table) && table_state(sub_table) != TableState::Free {
      // Safety: `super_table` 是函数头 txn_log_get_mutable 返回的 pending 表项
      // 指针，按 fn 契约非空；循环体对该表仅经 props_contains/table_state 等
      // 只读访问器触碰，props 本身无任何写入路径，唯一写入目标是局部
      // `missing_properties`，故迭代器（携带指向 super_table.props 的引用）
      // 存活期内不会发生重叠可变访问。
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
      // Safety: 镜像上一处——`sub_table` 同为函数头契约保证非空的 pending 表
      // 项指针；本循环只读其 props（经 props_contains），只写局部
      // `extra_properties`，迭代器存活期内不存在对 sub_table 的可变访问。
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

        self.absorb_child_unifier(*inner_state, &name, super_ty, sub_ty, old_variance);
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

        self.absorb_child_unifier(*inner_state, &name, super_ty, sub_ty, old_variance);
      } else if table_state(sub_table) == TableState::Unsealed && is_optional(prop_ty) {
        // This is sound because unsealed table types are precise.
      } else if table_state(sub_table) == TableState::Free {
        // 不变量: name 取自 props_keys(super_table) 的键快照，且直至本分支尚无
        // 对 super_table.props 的写入（写入只发生在 sub 侧/queue 之后），get 必命中。
        let prop_clone = props_get_clone(super_table, &name)
          .expect("上方不变量注：name 取自 super 键快照且其间无 props 写入");
        // Safety: `active_sub_ty` 是存活 arena 句柄（初值 sub_ty 按 fn 契约为
        // 表类型，或被 substitute 结果替换）；queue_type_id 将其克隆进
        // log.type_var_changes 的 Box<PendingType>（Box 地址稳定），返回的项
        // 指针在本次 log commit/rollback 前持续有效，C++ `queueType(...)` 同形。
        let pending_sub = unsafe { self.log.queue_type_id(active_sub_ty) };
        // Safety: pending_sub 为上一行 queue_type_id 返回的非空指针；active_sub_ty
        // 按契约是表类型，克隆项变体即 TableType（原 LUAU_ASSERT 钉住，同 C++），
        // 故 `Some` 必命中，其内指针即有效变体字段。
        let ttv = from_mut(
          unsafe { get_mutable_pending_type::<TableType>(pending_sub) }
            .expect("LUAU_ASSERT 钉住：pending 变体必为 TableType"),
        );
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

        self.absorb_child_unifier(*inner_state, &name, super_ty, sub_ty, old_variance);
      } else if table_state(super_table) == TableState::Unsealed {
        let mut clone = prop.clone();
        let deep = self.unifier_deeply_optional(clone.type_deprecated(), &mut HashMap::new());
        clone.set_type(deep);

        // Safety: `super_ty` 按 fn 契约是表类型句柄；queue_type_id 克隆进
        // log.type_var_changes 的 Box<PendingType>，返回指针在 log 提交/回滚前
        // 有效（C++ `queueType(superTy)` 同形）。
        let pending_super = unsafe { self.log.queue_type_id(super_ty) };
        // Safety: pending_super 非空源自上一行；本分支 table_state(super_table)
        // ==Unsealed 已确证该项变体为 TableType，`Some` 必命中，字段指针有效，
        // props_insert 继承同一非空前提。
        let pending_super_ttv = from_mut(
          unsafe { get_mutable_pending_type::<TableType>(pending_super) }
            .expect("Unsealed 分支已确证 pending 变体为 TableType"),
        );
        props_insert(pending_super_ttv, name.clone(), clone);
        super_table = pending_super_ttv;
      } else if self.variance == Variance::Covariant {
        // nothing
      } else if table_state(super_table) == TableState::Free {
        // Safety: 与 Unsealed 分支同理——super_ty 为契约内表类型句柄，
        // queue_type_id 返回 log 内 Box<PendingType> 的稳定指针。
        let pending_super = unsafe { self.log.queue_type_id(super_ty) };
        // Safety: pending_super 非空（上一行），且进入本分支的前提
        // table_state(super_table)==Free 由同一 pending 表项读出，变体为
        // TableType，`Some` 必命中，字段指针有效。
        let pending_super_ttv = from_mut(
          unsafe { get_mutable_pending_type::<TableType>(pending_super) }
            .expect("Free 分支已确证 pending 变体为 TableType"),
        );
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
    let super_ty_f = self.log.follow_type_id(super_ty);
    let sub_ty_f = self.log.follow_type_id(active_sub_ty);
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
    let super_ty_new = self.log.follow_type_id(super_ty);
    let sub_ty_new = self.log.follow_type_id(active_sub_ty);

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
        // Safety: 走到此处说明 follow 后 sub_ty/super_ty 表身份未变（仍是表
        // 类型句柄，仅 txn log pending 重写导致指针换址），被调方
        // `unifier_try_unify_tables` 的表句柄契约成立；末参传 null 即 C++
        // 默认 nullptr，满足其「null 或调用期内存活」的字面量属性契约。
        unsafe { self.unifier_try_unify_tables(sub_ty, super_ty, is_intersection, null()) };
      }
      return true;
    }
    false
  }
}

/// 读取表状态（t 非空；裸指针契约同 C++ `getMutable`，只读）。
fn table_state(t: *const TableType) -> TableState {
  // Safety: 调用点的 t 均为 unifier_try_unify_tables 函数头/重启换址后的
  // pending 表项指针，按其 fn 契约非空；state 是 Copy 只读。
  unsafe { (*t).state }
}

/// t 是否有 indexer（t 非空，只读）。
fn has_indexer(t: *const TableType) -> bool {
  // Safety: 同上——非空 pending 表项指针；is_some 判读不触碰堆内存布局。
  unsafe { (*t).indexer.is_some() }
}

/// t 的 indexer 键类型是否为 string（t 非空，只读）。
fn has_string_indexer(t: *const TableType) -> bool {
  // Safety: t 为契约内非空表项指针；as_ref() 只在 Some 分支短暂借出，闭包
  // 仅读 Copy 句柄 index_type（maybe_string 走类型瞬时解引用）。
  unsafe {
    (*t)
      .indexer
      .as_ref()
      .is_some_and(|ix| maybe_string(ix.index_type))
  }
}

/// t 的 indexer 整体拷贝（TableIndexer 为 Copy；t 非空，只读）。
fn indexer_of(t: *const TableType) -> Option<TableIndexer> {
  // Safety: 调用点在 has_indexer 为真的分支（indexers 两侧同判），t 契约内
  // 非空；Copy 读出，不留引用。
  unsafe { (*t).indexer }
}

/// t 的 indexer 键类型（调用分支已保证 Some；t 非空，只读）。
fn index_key_of(t: *const TableType) -> TypeId {
  // Safety: 仅从 `super_has_indexer && sub_has_indexer` 分支以对应表指针调用，
  // indexer 为 Some 已判；t 非空按 fn 契约，unwrap 不触发。
  unsafe {
    (*t)
      .indexer
      .as_ref()
      .expect("双侧 Some 判定方进入本 fn（见 Safety 注）")
      .index_type
  }
}

/// t 的 indexer 值类型（调用分支已保证 Some；t 非空，只读）。
fn index_result_of(t: *const TableType) -> TypeId {
  // Safety: 与 index_key_of 同一批调用点（indexer 双侧 Some 或经
  // has_string_indexer 过滤），Some 前提成立；t 非空按 fn 契约。
  unsafe {
    (*t)
      .indexer
      .as_ref()
      .expect("与 index_key_of 同一批调用点，indexer 双侧 Some 已判（见 Safety 注）")
      .index_result_type
  }
}

/// t 是否已绑定到其他表（t 非空，只读）。
fn has_bound(t: *const TableType) -> bool {
  // Safety: 调用位于函数尾，前面 `super_table.is_null() || sub_table.is_null()`
  // 已早退兜底，此处 t 非空；bound_to 只读判 Option。
  unsafe { (*t).bound_to.is_some() }
}

/// props 是否含属性（t 非空，只读）。
fn props_contains(t: *const TableType, name: &Name) -> bool {
  // Safety: t 为 pending 表项指针（fn 契约非空）；contains_key 只读 map。
  unsafe { (*t).props.contains_key(name) }
}

/// props 中属性的类型句柄（t 非空，只读）。
fn props_type_of(t: *const TableType, name: &Name) -> Option<TypeId> {
  // Safety: t 非空按契约；get 的瞬时可引用只用于 Copy 出 TypeId 即弃。
  unsafe { (*t).props.get(name).map(|p| p.type_deprecated()) }
}

/// props 中属性的克隆（t 非空，只读）。
fn props_get_clone(t: *const TableType, name: &Name) -> Option<Property> {
  // Safety: t 非空按契约；cloned() 深拷贝后即释放对表内存的借用。
  unsafe { (*t).props.get(name).cloned() }
}

/// props 键快照（循环体内 props 可能被写入，须先取键再统一）。
fn props_keys(t: *const TableType) -> Vec<String> {
  // Safety: t 非空按契约；keys() 借用止于 collect 完成，快照后调用方不再
  // 依赖此指针的稳定性。
  unsafe { (*t).props.keys().cloned().collect() }
}

/// props 插入属性（t 非空；写入路径，契约同 C++ `ttv->props[name] = prop`）。
fn props_insert(t: *mut TableType, name: Name, prop: Property) {
  // Safety: 调用点传入的都是刚经 queue_type_id + get_mutable_pending_type
  // 取得的 pending 表项指针（Box 地址稳定、log 提交前有效），且处于该表项
  // 的独占改写窗口（C++ `ttv->props[name] = prop` 同形）。
  unsafe { (*t).props.insert(name, prop) };
}

/// `!literalProperties || !literalProperties->contains(name)`（C++ Resetter 分支同款）。
fn not_in_literal_properties(lp: *const LiteralProperties, name: &Name) -> bool {
  // Safety: 前置 `lp.is_null()` 短路后 lp 非空；目标由 unifier_try_unify_tables
  // 的 fn 契约保证在调用期存活且不被改写，find 只读。
  lp.is_null() || unsafe { (*lp).find(name).is_none() }
}
