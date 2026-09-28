use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;
use ulua_common::fflag;

use crate::{
  blocked_early_exit,
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry, follow_type,
    fresh_type::fresh_type, get_type, instantiate::instantiate,
    track_interior_free_type::track_interior_free_type,
  },
  records::{
    any_type::AnyType, arena_handle::Handle, constraint::Constraint,
    constraint_solver::ConstraintSolver, free_type::FreeType, function_type::FunctionType,
    iterable_constraint::IterableConstraint, metatable_type::MetatableType, never_type::NeverType,
    primitive_type::PrimitiveType, reduce_constraint::ReduceConstraint,
    table_indexer::TableIndexer, table_type::TableType, type_level::TypeLevel,
    unification_too_complex::UnificationTooComplex,
  },
  type_aliases::{
    constraint_v::ConstraintV, props_type::Props, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl ConstraintSolver {
  pub fn try_dispatch_iterable_table(
    &mut self,
    iterator_ty: TypeId,
    c: &IterableConstraint,
    constraint: &Constraint,
    force: bool,
  ) -> bool {
    // bind/unify/block 仍以 `*const Constraint` 为键，边界处转 raw。
    let constraint_key = constraint as *const Constraint;
    let iterator_ty = follow_type::follow(iterator_ty);

    if get_type::get::<FreeType>(iterator_ty).is_some() {
      // `scope` 裸指针仅供 track_interior_free_type / 记录字段存值；逻辑链路走 scope_ref()。
      let scope = constraint.scope;
      let key_ty = fresh_type(
        self.arena.get_mut(),
        self.builtin_types.get(),
        Some(constraint.scope_ref()),
        Polarity::Mixed,
      );
      let value_ty = fresh_type(
        self.arena.get_mut(),
        self.builtin_types.get(),
        Some(constraint.scope_ref()),
        Polarity::Mixed,
      );
      track_interior_free_type(scope, key_ty);
      track_interior_free_type(scope, value_ty);

      let props = Props::default();
      let table_ty = self.arena_mut().add_type(
        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
          &props,
          Some(TableIndexer {
            index_type: key_ty,
            index_result_type: value_ty,
            is_read_only: false,
          }),
          TypeLevel::default(),
          scope,
          TableState::Sealed,
        ),
      );

      self.constraint_solver_unify(constraint_key, iterator_ty, table_ty);

      let mut it = c.variables.iter();
      if let Some(ty) = it.next() {
        // Safety: `bind_not_null_*` 要求约束指针非空且整调用存活——`constraint_key`
        // 由入口 `&Constraint` 借用指针化，正指向本次派发的 Iterable 约束；首变量
        // `*ty` 是该约束名下的迭代变量（BlockedType 以本约束为 owner），满足
        // can_mutate 前置，对应 cpp `bind(constraint, *it, keyTy)`。
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint_key, *ty, key_ty) };
      }
      if let Some(ty) = it.next() {
        // Safety: 第二迭代变量（value 位）的 bind，与上一处同一存活约束键；
        // `value_ty` 是刚在本分支 fresh 进 arena 的节点，三个 TypeId 均驻留 arena。
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint_key, *ty, value_ty) };
      }

      return true;
    }

    if get_type::get::<AnyType>(iterator_ty).is_some() {
      self.unpack_iterable_variables(constraint, c, self.builtin_types_ref().any_type);
      return true;
    }

    if get_type::get::<NeverType>(iterator_ty).is_some() {
      self.unpack_iterable_variables(constraint, c, self.builtin_types_ref().never_type);
      return true;
    }

    // Irksome: I don't think we have any way to guarantee that this table
    // type never has a metatable.

    if let Some(iterator_table) = get_type::get::<TableType>(iterator_ty) {
      if iterator_table.state == TableState::Free && !force {
        return self.block_type_id_not_null_constraint(iterator_ty, constraint_key);
      }

      if let Some(indexer) = iterator_table.indexer {
        let value_type = if fflag::LuauRefineNilFromTableIndexerResultType.get() {
          // Safety: `add_type_function_*` 本身是安全方法，unsafe 仅因同一调用内需
          // 并存 arena 独占借用与 builtin_types 共享读（访问器对会被借用检查器互斥
          // 拒绝）。两指针都是构造期写入的不相交分配且恒非空；本次只向 arena 追加
          // ApplyTypeFunction 节点（分页 arena 不迁移既有节点，`indexer` 是从 arena
          // 的 'static 视图拷出的 TableIndexer，值类型句柄保持有效），只读
          // `type_functions.intersect_func`（构造期装入 Box 后不再改写）与
          // `not_nil_type`（Copy TypeId 种子），不回写 builtin_types。
          let intersection_with_not_nil = {
            self
              .arena
              .get_mut()
              .add_type_function_type_function_initializer_list_type_id(
                &self.builtin_types.get().type_functions.intersect_func,
                &[
                  indexer.index_result_type,
                  self.builtin_types.get().not_nil_type,
                ],
              )
          };

          self.push_constraint(
            // Safety: Constraint.scope 按 cpp NotNull<Scope> 登记，恒非空。
            NonNull::new(constraint.scope).expect("Constraint.scope 为 NotNull 登记值，恒非空"),
            constraint.location,
            ConstraintV::Reduce(ReduceConstraint {
              ty: intersection_with_not_nil,
            }),
          );

          intersection_with_not_nil
        } else {
          indexer.index_result_type
        };

        let mut expected_variables = alloc::vec![indexer.index_type, value_type];
        // 不足的槽位以 error_type 补齐（Vec::resize 与 while-push 同义）
        expected_variables.resize(c.variables.len(), self.builtin_types_ref().error_type);

        for (variable, expected) in c.variables.iter().zip(expected_variables.iter()) {
          self.constraint_solver_unify(constraint_key, *variable, *expected);
          // Safety: 与 cpp 逐字对应的 unify+bind 成对调用：`constraint_key` 指向
          // 仍在派发中的存活约束（借用期内非空稳定），`*variable` 以其为 owner
          // 可原地 writeback；`unify` 的借用已随调用结束，bind 不与之重叠。
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(constraint_key, *variable, *expected)
          };
        }
      } else {
        self.unpack_iterable_variables(constraint, c, self.builtin_types_ref().error_type);
      }

      return true;
    }

    // else if (std::optional<TypeId> iterFn = findMetatableEntry(builtinTypes, errors, iteratorTy, "__iter", Location{}))
    let iter_fn = find_metatable_entry(
      Handle::from_ptr(self.builtin_types.as_ptr()),
      &mut self.errors,
      iterator_ty,
      "__iter",
      Location::default(),
    );
    if let Some(iter_fn) = iter_fn {
      blocked_early_exit!(self, type iter_fn, constraint_key);

      let instantiated_iter_fn = instantiate(
        self.builtin_types.get(),
        self.arena.get_mut(),
        &self.limits,
        constraint.scope_ref(),
        iter_fn,
      );

      if let Some(instantiated_iter_fn) = instantiated_iter_fn {
        if let Some(iter_ftv) = get_type::get::<FunctionType>(instantiated_iter_fn) {
          let expected_iter_args = self
            .arena_mut()
            .add_type_pack_initializer_list_type_id(&[iterator_ty]);
          self.constraint_solver_unify(constraint_key, iter_ftv.arg_types, expected_iter_args);

          // extend_type_pack 已 safe 化；对应 cpp
          // `extendTypePack(*arena, builtinTypes, iterFtv->retTypes, 2)`。
          let iter_rets = extend_type_pack(
            self.arena.get_mut(),
            Handle::from_ptr(self.builtin_types.as_ptr()),
            iter_ftv.ret_types,
            2,
            Vec::new(),
          );

          if iter_rets.head.is_empty() {
            // We've done what we can; this will get reported as an
            // error by the type checker.
            return true;
          }

          let next_fn_ty = iter_rets.head[0];

          let instantiated_next_fn = instantiate(
            self.builtin_types.get(),
            self.arena.get_mut(),
            &self.limits,
            constraint.scope_ref(),
            next_fn_ty,
          );

          if let Some(instantiated_next_fn) = instantiated_next_fn {
            // If nextFn is nullptr, then the iterator function has an improper signature.
            if let Some(next_fn) = get_type::get::<FunctionType>(instantiated_next_fn) {
              let ret_types = next_fn.ret_types;
              self.unpack_and_assign(c.variables.clone(), ret_types, NonNull::from(constraint));
            }

            return true;
          } else {
            let location = constraint.location;
            self.report_error_type_error_data_location(
              TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
              &location,
            );
          }
        } else {
          // TODO: Support __call and function overloads (what does an overload even mean for this?)
        }
      } else {
        let location = constraint.location;
        self.report_error_type_error_data_location(
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
          &location,
        );
      }

      return true;
    }

    // else if (auto iteratorMetatable = get<MetatableType>(iteratorTy))
    if let Some(iterator_metatable) = get_type::get::<MetatableType>(iterator_ty) {
      // If the metatable does not contain a `__iter` metamethod, then we iterate over the table part of the metatable.
      return self.try_dispatch_iterable_table(iterator_metatable.table, c, constraint, force);
    }

    if let Some(primitive_ty) = get_type::get::<PrimitiveType>(iterator_ty)
      && primitive_ty.r#type == PrimitiveType::TABLE
    {
      self.unpack_iterable_variables(constraint, c, self.builtin_types_ref().unknown_type);
      return true;
    }

    self.unpack_iterable_variables(constraint, c, self.builtin_types_ref().error_type);

    true
  }

  fn unpack_iterable_variables(
    &mut self,
    constraint: &Constraint,
    c: &IterableConstraint,
    ty: TypeId,
  ) {
    for var_ty in &c.variables {
      // Safety: 入参 `&Constraint` 隐式 coerce 为 *const 裸指针，函数体内借用
      // 持续存活（非空、稳定）；每个 `var_ty` 是该 Iterable 约束名下的迭代变量
      // （cpp unpack lambda 以 get<BlockedType>(varTy) 断言过 owner 归属），
      // 故 bind 的 BlockedType writeback 前置成立，兜底类型 `ty` 为内建 Copy 句柄。
      unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, *var_ty, ty) };
    }
  }
}
