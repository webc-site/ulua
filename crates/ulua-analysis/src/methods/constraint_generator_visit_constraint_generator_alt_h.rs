// ConstraintGenerator::visit(const ScopePtr&, AstStatFunction*) (ConstraintGenerator.cpp:1799-1959).
use alloc::{sync::Arc, vec::Vec};
use core::{ffi::c_char, ptr::null_mut};

use ulua_ast::{
  records::{
    ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
    ast_stat_function::AstStatFunction,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use super::constraint_generator_prototype_type_definitions::{make_binding, scope_insert_binding};
use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    add_all_as_dependencies::add_all_as_dependencies,
    add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns,
    add_all_as_reverse_dependencies::add_all_as_reverse_dependencies,
    as_mutable_type::as_mutable_type_id, checkpoint::checkpoint, follow_type::follow,
    for_each_constraint::for_each_constraint, get_mutable_type::get_mutable,
    get_type_alt_j::get as get_type,
    propagate_deprecated_attribute_to_constraint::propagate_deprecated_attribute_to_constraint,
  },
  records::{
    binding::Binding, blocked_type::BlockedType, checkpoint::Checkpoint, constraint::Constraint,
    constraint_generator::ConstraintGenerator, generalization_constraint::GeneralizationConstraint,
    push_function_type_constraint::PushFunctionTypeConstraint, scope::Scope, symbol::Symbol,
  },
  type_aliases::{
    constraint_v::ConstraintV, def_id_def::DefId, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_variant::TypeVariant,
  },
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_function(
    &mut self,
    scope: &ScopePtr,
    function: *mut AstStatFunction,
  ) -> ControlFlow {
    let function_ref = unsafe { &*function };
    let name_expr = function_ref.name;
    let name_node = name_expr as *mut AstNode;
    // SAFETY: name_expr 由解析器保证非空（函数语句必有名字节点）。
    let name_node_ref = unsafe { &*name_node };
    // SAFETY: function_ref.func 由解析器保证非空（函数体必存在）。
    let body_fn = unsafe { &*function_ref.func };
    // 从 Arc 指针（而非 &Scope 共享引用）派生可变指针：后续会经它写入 bindings，
    // 经共享引用转发写入属未定义行为；crate 惯用法（同 env_scope_raw）。
    let scope_raw = Arc::as_ptr(scope) as *mut Scope;

    let start = self.cg_checkpoint();
    let sig = unsafe {
      self.check_function_signature(
        scope,
        null_mut(),
        function_ref.func,
        None,
        Some(name_node_ref.location),
      )
    };
    let body_scope_raw = Arc::as_ptr(&sig.body_scope) as *mut Scope;

    let def = unsafe { (*self.dfg).get_def(name_expr as *const _) };

    let local_name = ast_node_try_as::<AstExprLocal>(name_node_ref);
    let global_name = ast_node_try_as::<AstExprGlobal>(name_node_ref);
    if let Some(local_name) = local_name {
      scope_bind(
        body_scope_raw,
        Symbol::from_local(local_name.local),
        make_binding(sig.signature, local_name.base.base.location),
        def,
        sig.signature,
      );
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    } else if let Some(global_name) = global_name {
      scope_bind(
        body_scope_raw,
        Symbol::from_global(global_name.name),
        make_binding(sig.signature, global_name.base.base.location),
        def,
        sig.signature,
      );
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    } else if ast_node_is::<AstExprIndexName>(name_node) {
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    }

    if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(name_node_ref) {
      let begin_prop = self.cg_checkpoint();
      let fn_ty = self.check_scope_ptr_ast_expr(scope, name_expr).ty;
      let end_prop = self.cg_checkpoint();
      let pftc: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
        &sig.signature_scope,
        body_fn.base.base.location,
        ConstraintV::PushFunctionType(PushFunctionTypeConstraint {
          expected_function_type: fn_ty,
          function_type: sig.signature,
          expr: function_ref.func,
          is_self: index_name.op == b':' as c_char,
        }),
      );

      if fflag::LuauConstraintGraph.get() {
        unsafe { add_all_as_dependencies(begin_prop, end_prop, self, pftc) };

        let begin_body = self.cg_checkpoint();
        self.check_function_body(&sig.body_scope, body_fn);
        let end_body = self.cg_checkpoint();

        unsafe { add_all_as_reverse_dependencies(begin_body, end_body, self, pftc) };
      } else {
        for_each_constraint(begin_prop, end_prop, self, |c: *mut Constraint| {
          constraint_push_dep(pftc, c);
        });
        let begin_body = self.cg_checkpoint();
        self.check_function_body(&sig.body_scope, body_fn);
        let end_body = self.cg_checkpoint();
        for_each_constraint(begin_body, end_body, self, |c: *mut Constraint| {
          constraint_push_dep(c, pftc);
        });
      }
    } else {
      self.check_function_body(&sig.body_scope, body_fn);
    }

    let end = self.cg_checkpoint();

    let mut generalized_type: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
    // constraintScope = sig.signatureScope ? sig.signatureScope : sig.bodyScope.
    let constraint_scope: &ScopePtr = &sig.signature_scope;

    let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
      constraint_scope,
      name_node_ref.location,
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type,
        source_type: sig.signature,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: false,
      }),
    );
    // generalized_type 刚由 add_type(BlockedType) 分配，必命中；对照 C++:2248
    // `getMutable<BlockedType>(generalizedType)->setOwner(gc)`
    let blocked = get_mutable::<BlockedType>(generalized_type).unwrap();
    blocked.set_owner(c as *const _);

    // SAFETY: c 由 add_constraint 返回，generator 持有，有效。
    unsafe { propagate_deprecated_attribute_to_constraint(&mut (*c).c, function_ref.func) };

    if fflag::LuauConstraintGraph.get() {
      unsafe { add_all_as_dependencies_and_chain_returns(start, end, self, c) };
    } else {
      let mut previous: *mut Constraint = null_mut();
      for_each_constraint(start, end, self, |constraint: *mut Constraint| {
        constraint_push_dep(c, constraint);
        if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c }
          && psc.returns
        {
          if !previous.is_null() {
            constraint_push_dep(constraint, previous);
          }
          previous = constraint;
        }
      });
    }

    let existing_function_ty: Option<TypeId> = self
      .lookup(scope, name_node_ref.location, def, false)
      .map(follow);

    if let Some(local_name) = ast_node_try_as::<AstExprLocal>(name_node_ref) {
      unsafe { self.visit_l_value_scope_ptr_ast_expr_type_id(scope, name_expr, generalized_type) };

      scope_bind(
        scope_raw,
        Symbol::from_local(local_name.local),
        make_binding(sig.signature, local_name.base.base.location),
        def,
        sig.signature,
      );
    } else if let Some(global_name) = ast_node_try_as::<AstExprGlobal>(name_node_ref) {
      if existing_function_ty.is_none() {
        // SAFETY: self.ice 由构造期持有，非空。
        unsafe {
          (*self.ice).ice_string_location(
            "prepopulateGlobalScope did not populate a global name",
            &global_name.base.base.location,
          );
        }
      }

      if let Some(existing) = existing_function_ty {
        let global_sym = global_name.name;
        // 对照 C++：`if (auto bt = get<BlockedType>(existing); bt && uninitializedGlobals.contains(...))`
        if let Some(bt) = get_type::<BlockedType>(existing)
          && self.uninitialized_globals.contains(&global_sym)
        {
          LUAU_ASSERT!(bt.get_owner().is_null());
          self.uninitialized_globals.erase(&global_sym);
          // SAFETY: existing 是 arena 内的类型句柄，与 C++ asMutable 同契约。
          unsafe {
            (*as_mutable_type_id(existing)).ty = TypeVariant::Bound(generalized_type);
          }
        }
      }

      scope_bind(
        scope_raw,
        Symbol::from_global(global_name.name),
        make_binding(sig.signature, global_name.base.base.location),
        def,
        sig.signature,
      );
    } else if ast_node_try_as::<AstExprIndexName>(name_node_ref).is_some() {
      unsafe { self.visit_l_value_scope_ptr_ast_expr_type_id(scope, name_expr, generalized_type) };
    } else if ast_node_is::<AstExprError>(name_node) {
      // SAFETY: builtin_types 由构造期持有，非空。
      generalized_type = unsafe { (*self.builtin_types).error_type };
    }

    if generalized_type.is_null() {
      // SAFETY: self.ice 由构造期持有，非空。
      unsafe {
        (*self.ice).ice_string_location(
          "generalizedType == nullptr",
          &function_ref.base.base.location,
        );
      }
    }

    self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, generalized_type);

    ControlFlow::None
  }

  /// `checkpoint(self)` 快照（visit 六处同款；unsafe 收口于此）。
  fn cg_checkpoint(&self) -> Checkpoint {
    // SAFETY: self 指针有效（正在调用的实例），constraints.len() 只读。
    unsafe { checkpoint(self as *const _) }
  }
}

/// 向 scope 写入绑定与 lvalue_types（visit 的 local/global 四处同款）。
/// scope_raw 须由 `Arc::as_ptr` 派生（Arc 存活、单线程独占写，crate 惯用法）。
fn scope_bind(scope_raw: *mut Scope, sym: Symbol, binding: Binding, def: DefId, ty: TypeId) {
  scope_insert_binding(scope_raw, sym, binding);
  // SAFETY: scope_raw 由 Arc::as_ptr 派生，指针非空且存活。
  unsafe {
    *(*scope_raw).lvalue_types.get_or_insert(def) = ty;
  }
}

/// 向 constraint 追加 deprecated 依赖（visit 四处同款）。
/// constraint/dep 均为 generator 持有的有效 Constraint 指针。
fn constraint_push_dep(constraint: *mut Constraint, dep: *mut Constraint) {
  // SAFETY: constraint 由 add_constraint/for_each_constraint 回调给出，有效。
  unsafe { (*constraint).deprecated_dependencies.push(dep) };
}
