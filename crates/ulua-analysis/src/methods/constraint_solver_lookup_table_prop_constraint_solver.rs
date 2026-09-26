use alloc::vec::Vec;

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
    any_type::AnyType, arena_handle::Handle, constraint::Constraint,
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
  ///
  /// # Safety
  /// - `constraint`：非空、对齐、整调用存活的 `Constraint`（C++
  ///   `NotNull<const Constraint>` 直传，通常是本次派发出的那条）；本包装不会
  ///   解引用它，仅按下方 unsafe 重载的逐参数契约原样转发。
  /// - `subject_type`：arena 驻留 TypeId。
  pub(crate) fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
    &mut self,
    constraint: *const Constraint,
    subject_type: TypeId,
    prop_name: &str,
    context: ValueContext,
    in_conditional: bool,
    suppress_simplification: bool,
  ) -> TablePropLookupResult {
    let mut seen = DenseHashSet::default();
    // Safety: 纯转发——`constraint` 按函数文档由调用方保证非空且整调用存活，
    // 本层原样传递不触碰其指向；`seen` 是刚 new 出的本地集，独占可变借用止于
    // 本次调用，unsafe 重载的逐参数前置全部成立。
    unsafe {
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
  }

  /// # Safety
  /// 对应 cpp `lookupTableProp(NotNull<const Constraint>, ..., Set<TypeId>&)`
  /// （`Analysis/src/ConstraintSolver.cpp:3425`）的调用契约，逐参数：
  /// - `constraint`：非空、对齐且整调用存活（C++ `NotNull` 直传，即正在派发
  ///   的那条约束，须活过包括全部递归分支在内的本次调用）。函数体只在
  ///   Free/Union/Intersection 分支 Copy 读其 `scope`/`location` 两字段，并把
  ///   指针值原样转发给 `constraint_solver_unify` 与递归调用作身份键；从不
  ///   写该对象、也不派生长生命周期引用。传悬垂/空指针即读已释放内存。
  /// - `subject_type`：arena 驻留 TypeId（入口 `follow_type_id` 后按变体匹配）。
  /// - `prop_name` / `context` / `in_conditional` / `suppress_simplification`：
  ///   纯值参数，遵循标准 Rust 引用契约即可，无额外前置。
  /// - `seen`：环保护集。入口查重命中即返回；随后在本参数的克隆上插入
  ///   `subject_type` 并沿递归传递该克隆（对应 C++ `ScopedSeenSet` 的进入
  ///   语义，不移除而是逐层克隆）。本函数不改动调用方集合，但调用方须保证
  ///   同一递归链共享同一集合来源，否则自引用类型会无限递归。
  pub unsafe fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
    &mut self,
    constraint: *const Constraint,
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
          // Safety: 给 Free 表的 `ttv.scope` fresh 属性类型；`self.arena.as_ptr()` 是
          // 构造期按 C++ `NotNull` 布线的 arena 指针，恒非空且随 solver 存活。
          // 独占借用止于本调用；`ttv` 是 arena 驻留节点的 'static 句柄，
          // `add_type` 只追加新节点、既有节点地址由 arena 固定分配保证不动。
          // （不用 `arena_mut()`+`builtin_types_ref()` 组合：二者对同一 `self`
          // 的借用互斥，借用检查器会拒绝同调用混用，故在此拆裸指针。）
          { self.arena.get_mut() },
          // Safety: `builtin_types` 同为构造期 NotNull 只读表，这里仅取
          // never/unknown 种子；与 arena 是两块不相交分配，并存借用无别名冲突。
          { self.builtin_types.get() },
          ttv.scope,
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
        // Safety: 对应 cpp:3519 的 LValue 尾递归（TODO __newindex 未及）：
        // `constraint` 原样转发正派发的 NotNull 约束；`mt.table` 是从
        // `&'static MetatableType` 共享读复制的 arena 句柄；`seen` 传入本层
        // 已插入 subject 的克隆，callee 只读取它再逐层克隆延伸环保护。
        return unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            mt.table,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };
      }

      // Safety: cpp:3523 RValue 的非尾递归（结果未命中还要续查元表
      // __index）：参数来源与 LValue 分支完全相同；本调用返回后对 `seen`
      // 的借用顺序结束，后续分支再可变重借同一克隆，无并存冲突。
      let result = unsafe {
        self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            mt.table,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
      };
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
            // Safety: `self.arena.get_mut()` 对应 cpp:3547 `extendTypePack(*arena,
            // ...)` 的引用解包——arena 由构造契约恒非空且随 solver 存活，独占
            // 借用止于本调用；`ft.ret_types` 是刚共享读出的 arena 驻留
            // TypePackId；`self.builtin_types.as_ptr()` 按指针值直传（C++ `BuiltinTypes&`
            // 形参同义），callee 内部只按 NotNull 契约读取。
            let rets = unsafe {
              extend_type_pack(
                self.arena.get_mut(),
                Handle::from_ptr(self.builtin_types.as_ptr()),
                ft.ret_types,
                1,
                Vec::new(),
              )
            };
            return TablePropLookupResult::found(if rets.head.len() == 1 {
              rets.head[0]
            } else {
              self.builtin_types_ref().nil_type
            });
          }

          // Safety: 元表 `__index` 指向非函数目标时的直传递归（cpp:3558）：
          // `index_type` 已 follow 且为 arena 句柄；约束按 NotNull 原样续传，
          // `seen` 是本层克隆（含当前 subject），环保护继续生效。
          return unsafe {
            self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                        constraint,
                        index_type,
                        prop_name,
                        context,
                        in_conditional,
                        suppress_simplification,
                        &mut seen,
                    )
          };
        }

        result
      } else if get_type::get::<MetatableType>(metatable).is_some() {
        // Safety: cpp:3561「元表的元表」递归：主体是上面 follow 过的
        // `metatable` arena 句柄；约束指针是入口 NotNull 的同一对象续传，
        // `seen` 借用在此顺序重借（上一递归已结束），无别名并存。
        unsafe {
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
        }
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
        // Safety: cpp:3608 Free 类型上界已成型为 Table/Primitive 时先行探测
        // 递归：`upper_bound` 是 follow 后句柄；未命中（prop_type 为空）则
        // 落入下方扩界逻辑，约束指针续传、`seen` 克隆继续环保护。
        let res = unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            upper_bound,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };

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

      track_interior_free_type(
        // Safety: `constraint` 为入口契约钉住的派发中约束（NotNull 直传），
        // 非空且活过本次调用的全部递归；此处仅 Copy 出其 `scope` 裸指针登记
        // 新上界（cpp:3619 `trackInteriorFreeType(constraint->scope, ...)`），
        // 不派生引用、无生命周期延长。
        unsafe { (*constraint).scope },
        new_upper_bound,
      );

      // C++ `LUAU_ASSERT(tt)`：刚 add_type 的 TableType 必然可变下转成功。
      let tt = get_mutable::<TableType>(new_upper_bound).expect("fresh table is TableType");

      let prop_type = fresh_type(
        // Safety: 上界表 `scope` 下 fresh 属性类型；arena 构造期 NotNull 恒非空，
        // 可变借用止于本调用。此刻持有的 `tt`/`ft` 均为 arena 驻留节点句柄，
        // `add_type` 只追加新 FreeType 节点，既有节点地址固定不被挪动。
        { self.arena.get_mut() },
        // Safety: builtin_types 是另一块构造期分配，只读 never/unknown 种子；
        // 与 arena 借用不相交，无别名冲突。
        { self.builtin_types.get() },
        scope,
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

      self.constraint_solver_unify(constraint, subject_type, new_upper_bound);

      TablePropLookupResult::found(prop_type)
    } else if let Some(utv) = get_type::get::<UnionType>(subject_type) {
      let mut blocked = Vec::new();
      let mut options = Vec::new();

      // C++ `for (TypeId ty : utv)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      for ty in begin_union_type(utv) {
        // Safety: cpp:3647 union 逐成员展开递归（UnionTypeIterator 已 follow）：
        // `ty` 为 arena 句柄；每个成员传入同一 `seen` 克隆（含当前 subject），
        // callee 内部再克隆故互不污染兄弟分支；约束指针全程存活原样续传。
        let result = unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            ty,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };

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
        return TablePropLookupResult::blocked(blocked);
      }

      if options.is_empty() {
        return TablePropLookupResult::empty();
      }

      if options.len() == 1 {
        return TablePropLookupResult::found(options[0]);
      }

      let prop_type = if options.len() == 2 && !suppress_simplification {
        // Safety: 派发中约束按入口契约非空存活；Copy 读 `scope` 传给
        // simplify（对应 cpp:3667/3669 内联解引用 `constraint->scope`），不留引用。
        let scope = unsafe { (*constraint).scope };
        // Safety: 同一存活约束对象再 Copy 读 `location`，仅值传参与报错定位。
        let location = unsafe { (*constraint).location };
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
      let mut blocked = Vec::new();
      let mut options = Vec::new();

      // C++ `for (TypeId ty : itv)`——IntersectionTypeIterator 同理展平。
      for ty in begin_intersection_type(itv) {
        // Safety: cpp:3684 intersection 逐部分展开递归：`ty` 来自展平迭代器
        // （已 follow 的 arena 成员）；`seen` 与 union 分支同款——本层克隆
        // 传入、callee 逐层再克隆，兄弟分支互不影响；约束 NotNull 续传。
        let result = unsafe {
          self
          .lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
            constraint,
            ty,
            prop_name,
            context,
            in_conditional,
            suppress_simplification,
            &mut seen,
          )
        };

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
        return TablePropLookupResult::blocked(blocked);
      }

      if options.is_empty() {
        return TablePropLookupResult::empty();
      }

      if options.len() == 1 {
        return TablePropLookupResult::found(options[0]);
      }

      let prop_type = if options.len() == 2 && !suppress_simplification {
        self.simplify_intersection_not_null_scope_location_type_id_type_id(
          // Safety: 同上 union 分支：`constraint` 按入口契约存活，这里只
          // Copy 读 `scope` 实参（cpp:3701 `constraint->scope`）。
          unsafe { (*constraint).scope },
          // Safety: 同一对象再 Copy 读 `location` 值传，不延长任何借用。
          unsafe { (*constraint).location },
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
            // Safety: cpp:3600 string 原始类型经元表 `__index` 属性值递归：
            // `index_type` 直接取 read_ty（callee 入口自会 follow），是
            // metatable 表的共享读句柄；约束与 `seen` 按入口契约续传。
            return unsafe {
              self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
                            constraint,
                            index_type,
                            prop_name,
                            context,
                            in_conditional,
                            suppress_simplification,
                            &mut seen,
                        )
            };
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
}
