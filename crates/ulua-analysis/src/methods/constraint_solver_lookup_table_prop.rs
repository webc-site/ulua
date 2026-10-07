use alloc::vec::Vec;
use core::ptr::from_ref;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{polarity::Polarity, table_state::TableState, value_context::ValueContext},
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    extend_type_pack::extend_type_pack,
    fast_is_subtype::fast_is_subtype,
    follow_type,
    fresh_type::fresh_type,
    get_mutable_type::get_mutable,
    get_table_type::get_table_type,
    get_type,
    lookup_extern_type_prop::lookup_extern_type_prop,
    track_interior_free_type::track_interior_free_type,
  },
  records::{
    any_type::AnyType, arena_handle::alias_ref, constraint::Constraint,
    constraint_solver::ConstraintSolver, extern_type::ExternType, free_type::FreeType,
    function_type::FunctionType, intersection_type::IntersectionType,
    metatable_type::MetatableType, never_type::NeverType, primitive_type::PrimitiveType,
    property_type::Property, singleton_type::SingletonType, string_singleton::StringSingleton,
    table_prop_lookup_result::TablePropLookupResult, table_type::TableType, type_level::TypeLevel,
    union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl ConstraintSolver {
  /// 对应 C++ 不带 `seen` 集的 `lookupTableProp` 重载
  /// （`Analysis/src/ConstraintSolver.cpp:3412`）：新建空环保护集后转发。
  pub(crate) fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
    &mut self,
    constraint: &Constraint,
    subject_type: TypeId,
    prop_name: &str,
    context: ValueContext,
    in_conditional: bool,
    suppress_simplification: bool,
  ) -> TablePropLookupResult {
    let mut seen = DenseHashSet::default();
    self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
      constraint,
      subject_type,
      prop_name,
      context,
      in_conditional,
      suppress_simplification,
      &mut seen,
    )
  }

  /// 对应 cpp `lookupTableProp(NotNull<const Constraint>, ..., Set<TypeId>&)`
  /// （`Analysis/src/ConstraintSolver.cpp:3425`）。
  ///
  /// 参数契约（由 Rust 引用/借用语义直接承载，无需 unsafe 表达）：
  /// - `constraint`：正在派发的约束（C++ `NotNull` 直传）。函数体只读其
  ///   `scope`/`location` 两字段并按身份原样转发，从不写该对象；引用保证
  ///   其活过包括全部递归分支在内的本次调用。
  /// - `subject_type`：arena 驻留 TypeId（入口 `follow_type_id` 后按变体匹配）。
  /// - `seen`：环保护集。入口查重命中即返回；随后在本参数的克隆上插入
  ///   `subject_type` 并沿递归传递该克隆（对应 C++ `ScopedSeenSet` 的进入
  ///   语义，不移除而是逐层克隆）。调用方须保证同一递归链共享同一集合
  ///   来源，否则自引用类型会无限递归。
  pub(crate) fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
    &mut self,
    constraint: &Constraint,
    subject_type: TypeId,
    prop_name: &str,
    context: ValueContext,
    in_conditional: bool,
    suppress_simplification: bool,
    seen: &mut DenseHashSet<TypeId>,
  ) -> TablePropLookupResult {
    if seen.contains(&subject_type) {
      return TablePropLookupResult::empty();
    }

    let mut seen = seen.clone();
    seen.insert(subject_type);

    let subject_type = follow_type::follow(subject_type);

    if self.is_blocked_type_id(subject_type) {
      TablePropLookupResult::blocked_on(subject_type)
    } else if get_type::get::<AnyType>(subject_type).is_some()
      || get_type::get::<NeverType>(subject_type).is_some()
    {
      TablePropLookupResult::found(subject_type)
    } else if let Some(ttv) = get_mutable::<TableType>(subject_type) {
      if let Some(prop) = ttv.props.get(prop_name) {
        match context {
          ValueContext::RValue => {
            if let Some(read_ty) = prop.read_ty {
              return TablePropLookupResult::found(read_ty);
            }
          }
          ValueContext::LValue => {
            if let Some(write_ty) = prop.write_ty {
              return TablePropLookupResult::found(write_ty);
            }
          }
        }
      }

      if let Some(indexer) = ttv.indexer {
        if self.is_blocked_type_id(indexer.index_type) {
          return TablePropLookupResult::blocked_index(indexer.index_type);
        }

        // CLI-169235: build a faux string-singleton literal from the prop
        // name and reuse subtyping (same logic as `index<_, _>`), so a named
        // access hits the indexer when the name is a subtype of the index
        // key (e.g. "Val1" against a `"Val1"|"Val2"|"Val3"` key), not just
        // when the key is a plain string.
        let faux_literal = self
          .arena_mut()
          .add_type(SingletonType::new(SingletonVariant::V1(
            StringSingleton::new(prop_name.to_string()),
          )));
        if fast_is_subtype(faux_literal, indexer.index_type) {
          return TablePropLookupResult::index(indexer.index_result_type);
        }
      }

      if ttv.state == TableState::Free {
        let result = fresh_type(
          // Handle::get_mut/get 均以 `&self` 物化借用，两字段并存借用无需 unsafe。
          self.arena.get_mut(),
          self.builtin_types.get(),
          Some(alias_ref(ttv.scope)),
          Polarity::Mixed,
        );
        track_interior_free_type(ttv.scope, result);

        match context {
          ValueContext::RValue => {
            ttv
              .props
              .insert(prop_name.to_string(), Property::readonly(result));
          }
          ValueContext::LValue => {
            if let Some(prop) = ttv.props.get_mut(prop_name)
              && prop.is_read_only()
            {
              prop.write_ty = prop.read_ty;
              return TablePropLookupResult::found_opt(prop.read_ty);
            }

            ttv
              .props
              .insert(prop_name.to_string(), Property::rw_type_id(result));
          }
        }

        return TablePropLookupResult::found(result);
      }

      if in_conditional {
        return TablePropLookupResult::found(self.builtin_types_ref().unknown_type);
      }

      TablePropLookupResult::empty()
    } else if let Some(mt) = get_type::get::<MetatableType>(subject_type) {
      if context == ValueContext::LValue {
        // 对应 cpp:3519 的 LValue 尾递归（TODO __newindex 未及）：
        // `constraint` 原样转发正在派发的约束；`mt.table` 是从共享读的
        // `MetatableType` 复制的 arena 句柄；`seen` 传入本层已插入 subject
        // 的克隆，callee 只读取它再逐层克隆延伸环保护。
        return self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            mt.table,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          );
      }

      // cpp:3523 RValue 的非尾递归（结果未命中还要续查元表 __index）：
      // 参数来源与 LValue 分支完全相同；本调用返回后对 `seen` 的借用结束，
      // 后续分支再顺序重借同一克隆，无并存冲突。
      let result = self
        .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
          constraint,
          mt.table,
          prop_name,
          context,
          in_conditional,
          suppress_simplification,
          &mut seen,
        );
      if !result.blocked_types.is_empty() || result.prop_type.is_some() {
        return result;
      }

      let metatable = follow_type::follow(mt.metatable);
      if self.is_blocked_type_id(metatable) {
        return TablePropLookupResult::blocked_on(metatable);
      }

      if let Some(mtt) = get_table_type(metatable) {
        let Some(index_prop) = mtt.props.get("__index") else {
          return result;
        };

        if index_prop.is_write_only() {
          return TablePropLookupResult::found(self.builtin_types_ref().error_type);
        }

        if let Some(index_type) = index_prop.read_ty {
          let index_type = follow_type::follow(index_type);
          if let Some(ft) = get_type::get::<FunctionType>(index_type) {
            // 对应 cpp:3547 `extendTypePack(*arena, builtinTypes, ft->retTypes, 1, {})`。
            let rets = extend_type_pack(
              self.arena.get_mut(),
              self.builtin_types,
              ft.ret_types,
              1,
              Vec::new(),
            );
            return TablePropLookupResult::found(if rets.head.len() == 1 {
              rets.head[0]
            } else {
              self.builtin_types_ref().nil_type
            });
          }

          // 元表 `__index` 指向非函数目标时的直传递归（cpp:3558）：
          // `index_type` 已 follow 且为 arena 句柄；约束按身份原样续传，
          // `seen` 是本层克隆（含当前 subject），环保护继续生效。
          return self
            .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
              constraint,
              index_type,
              prop_name,
              context,
              in_conditional,
              suppress_simplification,
              &mut seen,
            );
        }

        result
      } else if get_type::get::<MetatableType>(metatable).is_some() {
        // cpp:3561「元表的元表」递归：主体是上面 follow 过的 `metatable`
        // arena 句柄；约束是入口同一对象续传，`seen` 借用在此顺序重借
        // （上一递归已结束），无别名并存。
        self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            metatable,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
      } else {
        result
      }
    } else if let Some(cls) = get_type::get::<ExternType>(subject_type) {
      if let Some(prop) = lookup_extern_type_prop(cls, prop_name) {
        return TablePropLookupResult::found_opt(if context == ValueContext::RValue {
          prop.read_ty
        } else {
          prop.write_ty
        });
      }

      if let Some(indexer) = &cls.indexer {
        return TablePropLookupResult::index(indexer.index_result_type);
      }

      TablePropLookupResult::empty()
    } else if let Some(ft) = get_type::get::<FreeType>(subject_type) {
      let upper_bound = follow_type::follow(ft.upper_bound);

      if get_type::get::<TableType>(upper_bound).is_some()
        || get_type::get::<PrimitiveType>(upper_bound).is_some()
      {
        // cpp:3608 Free 类型上界已成型为 Table/Primitive 时先行探测递归：
        // `upper_bound` 是 follow 后句柄；未命中（prop_type 为空）则落入
        // 下方扩界逻辑，约束按身份续传、`seen` 克隆继续环保护。
        let res = self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            upper_bound,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          );

        if res.prop_type.is_some() {
          return res;
        }
      }

      let scope = ft.scope;
      let new_upper_bound =
        self
          .arena_mut()
          .add_type(TableType::table_type_table_state_type_level_scope(
            TableState::Free,
            TypeLevel::default(),
            scope,
          ));

      // cpp:3619 `trackInteriorFreeType(constraint->scope, ...)`：约束登记的
      // `scope` 裸指针按原样值传（不派生引用）给收口函数。
      track_interior_free_type(constraint.scope, new_upper_bound);

      // C++ `LUAU_ASSERT(tt)`：刚 add_type 的 TableType 必然可变下转成功。
      let tt = get_mutable::<TableType>(new_upper_bound).expect("fresh table is TableType");

      let prop_type = fresh_type(
        self.arena.get_mut(),
        self.builtin_types.get(),
        Some(alias_ref(scope)),
        Polarity::Mixed,
      );
      track_interior_free_type(scope, prop_type);

      match context {
        ValueContext::RValue => {
          tt.props
            .insert(prop_name.to_string(), Property::readonly(prop_type));
        }
        ValueContext::LValue => {
          tt.props
            .insert(prop_name.to_string(), Property::rw_type_id(prop_type));
        }
      }

      // `constraint_solver_unify` 仍以 `*const Constraint` 作派发约束的身份键
      // （其文件尚未 Rust 化）；此处从整调用存活的引用收口取指针，仅本次
      // 调用内有效，与原 `NotNull` 直传语义同构。
      self.constraint_solver_unify(from_ref(constraint), subject_type, new_upper_bound);

      TablePropLookupResult::found(prop_type)
    } else if let Some(utv) = get_type::get::<UnionType>(subject_type) {
      // C++ `for (TypeId ty : utv)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      let options = match self.lookup_table_prop_over_members(
        constraint,
        begin_union_type(utv),
        prop_name,
        context,
        in_conditional,
        suppress_simplification,
        &mut seen,
      ) {
        Ok(early) => return early,
        Err(options) => options,
      };

      let prop_type = if options.len() == 2 && !suppress_simplification {
        // 对应 cpp:3667/3669 内联读取：scope 经 alias_ref 物化为共享引用，
        // location 值复制，借用止于下方 simplify 调用。
        let scope = alias_ref(constraint.scope);
        let location = constraint.location;
        if context == ValueContext::LValue {
          self.simplify_intersection_not_null_scope_location_type_id_type_id(
            scope, location, options[0], options[1],
          )
        } else {
          self.simplify_union(scope, location, options[0], options[1])
        }
      } else if context == ValueContext::LValue {
        self
          .arena_mut()
          .add_type(IntersectionType { parts: options })
      } else {
        self.arena_mut().add_type(UnionType { options })
      };

      TablePropLookupResult::found(prop_type)
    } else if let Some(itv) = get_type::get::<IntersectionType>(subject_type) {
      // C++ `for (TypeId ty : itv)`——IntersectionTypeIterator 同理展平。
      let options = match self.lookup_table_prop_over_members(
        constraint,
        begin_intersection_type(itv),
        prop_name,
        context,
        in_conditional,
        suppress_simplification,
        &mut seen,
      ) {
        Ok(early) => return early,
        Err(options) => options,
      };

      let prop_type = if options.len() == 2 && !suppress_simplification {
        // 同上 union 分支：scope 物化共享引用、location 值复制（cpp:3701）。
        self.simplify_intersection_not_null_scope_location_type_id_type_id(
          alias_ref(constraint.scope),
          constraint.location,
          options[0],
          options[1],
        )
      } else {
        self
          .arena_mut()
          .add_type(IntersectionType { parts: options })
      };

      TablePropLookupResult::found(prop_type)
    } else if let Some(pt) = get_type::get::<PrimitiveType>(subject_type) {
      if pt.r#type == PrimitiveType::STRING
        && let Some(metatable_id) = pt.metatable
      {
        let metatable = follow_type::follow(metatable_id);
        // C++ `LUAU_ASSERT(metatableTable)`：string 元表必然是 TableType。
        let metatable_table = get_type::get::<TableType>(metatable).expect("metatable is table");
        if let Some(index_prop) = metatable_table.props.get("__index") {
          if index_prop.is_write_only() {
            return TablePropLookupResult::found(self.builtin_types_ref().error_type);
          }

          if let Some(index_type) = index_prop.read_ty {
            // cpp:3600 string 原始类型经元表 `__index` 属性值递归：
            // `index_type` 直接取 read_ty（callee 入口自会 follow），是
            // metatable 表的共享读句柄；约束与 `seen` 按入口契约续传。
            return self
              .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                constraint,
                index_type,
                prop_name,
                context,
                in_conditional,
                suppress_simplification,
                &mut seen,
              );
          }
        }
      }

      if in_conditional && pt.r#type == PrimitiveType::TABLE {
        return TablePropLookupResult::found(self.builtin_types_ref().unknown_type);
      }

      TablePropLookupResult::empty()
    } else {
      TablePropLookupResult::empty()
    }
  }

  /// union/intersection 两分支的公共体（cpp:3646-3661 / 3683-3697 同构段）：
  /// 逐成员递归 `lookupTableProp` 并去重归并 blocked/options，处理 blocked、
  /// 空、单一 三类早退；多选项时返回 `Err(options)`，由调用方按变体语义
  /// （union 按 context 分派 simplify/构造，intersection 恒 intersection）
  /// 合并——该分歧点保留在各自分支。
  ///
  /// `constraint` 契约同主重载：正在派发的约束，只按身份原样转发给递归
  /// 调用，本函数从不读其字段。
  fn lookup_table_prop_over_members(
    &mut self,
    constraint: &Constraint,
    members: impl Iterator<Item = TypeId>,
    prop_name: &str,
    context: ValueContext,
    in_conditional: bool,
    suppress_simplification: bool,
    seen: &mut DenseHashSet<TypeId>,
  ) -> Result<TablePropLookupResult, Vec<TypeId>> {
    let mut blocked = Vec::new();
    let mut options = Vec::new();

    for ty in members {
      // cpp:3647/3684 逐成员展开递归：`ty` 为展平迭代器（已 follow）
      // 的 arena 句柄；每个成员传入同一 `seen` 克隆（含当前 subject），
      // callee 内部再克隆故互不污染兄弟分支；约束引用全程存活原样续传。
      let result = self
        .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
          constraint,
          ty,
          prop_name,
          context,
          in_conditional,
          suppress_simplification,
          seen,
        );

      for blocked_ty in result.blocked_types {
        if !blocked.contains(&blocked_ty) {
          blocked.push(blocked_ty);
        }
      }

      if let Some(prop_type) = result.prop_type
        && !options.contains(&prop_type)
      {
        options.push(prop_type);
      }
    }

    if !blocked.is_empty() {
      return Ok(TablePropLookupResult::blocked(blocked));
    }

    if options.is_empty() {
      return Ok(TablePropLookupResult::empty());
    }

    if options.len() == 1 {
      return Ok(TablePropLookupResult::found(options[0]));
    }

    Err(options)
  }
}
