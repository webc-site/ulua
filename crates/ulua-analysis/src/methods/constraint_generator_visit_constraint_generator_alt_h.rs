// ConstraintGenerator::visit(const ScopePtr&, AstStatFunction*) (ConstraintGenerator.cpp:1799-1959).
use alloc::{string::String, vec::Vec};
use core::{ffi::c_char, ptr::null_mut};

use ulua_ast::{
  records::{
    ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
    ast_stat_function::AstStatFunction,
  },
  rtti::{ast_node_as, ast_node_is},
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

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
    binding::Binding, blocked_type::BlockedType, constraint::Constraint,
    constraint_generator::ConstraintGenerator, generalization_constraint::GeneralizationConstraint,
    push_function_type_constraint::PushFunctionTypeConstraint, scope::Scope, symbol::Symbol,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId, type_variant::TypeVariant,
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
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

    let start = unsafe { checkpoint(self as *const _) };
    let sig = unsafe {
      self.check_function_signature(
        scope,
        null_mut(),
        function_ref.func,
        None,
        Some((*name_expr).base.location),
      )
    };
    let body_scope_raw = sig.body_scope.as_ref() as *const Scope as *mut Scope;

    let def = unsafe { (*self.dfg).get_def(name_expr as *const _) };

    let local_name = unsafe { ast_node_as::<AstExprLocal>(name_node) };
    let global_name = unsafe { ast_node_as::<AstExprGlobal>(name_node) };
    if !local_name.is_null() {
      unsafe {
        (*body_scope_raw).bindings.insert(
          Symbol::from_local((*local_name).local),
          Binding {
            type_id: sig.signature,
            location: (*local_name).base.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
        *(*body_scope_raw).lvalue_types.get_or_insert(def) = sig.signature;
      }
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    } else if !global_name.is_null() {
      unsafe {
        (*body_scope_raw).bindings.insert(
          Symbol::from_global((*global_name).name),
          Binding {
            type_id: sig.signature,
            location: (*global_name).base.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
        *(*body_scope_raw).lvalue_types.get_or_insert(def) = sig.signature;
      }
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    } else if ast_node_is::<AstExprIndexName>(name_node) {
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    }

    let index_name = unsafe { ast_node_as::<AstExprIndexName>(name_node) };
    if !index_name.is_null() {
      let begin_prop = unsafe { checkpoint(self as *const _) };
      let fn_ty = self.check_scope_ptr_ast_expr(scope, name_expr).ty;
      let end_prop = unsafe { checkpoint(self as *const _) };
      let pftc: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
        &sig.signature_scope,
        unsafe { (*function_ref.func).base.base.location },
        ConstraintV::PushFunctionType(PushFunctionTypeConstraint {
          expected_function_type: fn_ty,
          function_type: sig.signature,
          expr: function_ref.func,
          is_self: unsafe { (*index_name).op == b':' as c_char },
        }),
      );

      if FFlag::LuauConstraintGraph.get() {
        unsafe { add_all_as_dependencies(begin_prop, end_prop, self, pftc) };

        let begin_body = unsafe { checkpoint(self as *const _) };
        self.check_function_body(&sig.body_scope, unsafe { &*function_ref.func });
        let end_body = unsafe { checkpoint(self as *const _) };

        unsafe { add_all_as_reverse_dependencies(begin_body, end_body, self, pftc) };
      } else {
        for_each_constraint(begin_prop, end_prop, self, |c: *mut Constraint| {
          unsafe { (*pftc).deprecated_dependencies.push(c) };
        });
        let begin_body = unsafe { checkpoint(self as *const _) };
        self.check_function_body(&sig.body_scope, unsafe { &*function_ref.func });
        let end_body = unsafe { checkpoint(self as *const _) };
        for_each_constraint(begin_body, end_body, self, |c: *mut Constraint| {
          unsafe { (*c).deprecated_dependencies.push(pftc) };
        });
      }
    } else {
      self.check_function_body(&sig.body_scope, unsafe { &*function_ref.func });
    }

    let end = unsafe { checkpoint(self as *const _) };

    let mut generalized_type: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
    // constraintScope = sig.signatureScope ? sig.signatureScope : sig.bodyScope.
    let constraint_scope: &ScopePtr = &sig.signature_scope;

    let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
      constraint_scope,
      unsafe { (*name_expr).base.location },
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

    unsafe { propagate_deprecated_attribute_to_constraint(&mut (*c).c, function_ref.func) };

    if FFlag::LuauConstraintGraph.get() {
      unsafe { add_all_as_dependencies_and_chain_returns(start, end, self, c) };
    } else {
      let mut previous: *mut Constraint = null_mut();
      for_each_constraint(start, end, self, |constraint: *mut Constraint| {
        unsafe { (*c).deprecated_dependencies.push(constraint) };
        if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c }
          && psc.returns
        {
          if !previous.is_null() {
            unsafe { (*constraint).deprecated_dependencies.push(previous) };
          }
          previous = constraint;
        }
      });
    }

    let existing_function_ty: Option<TypeId> = self
      .lookup(scope, unsafe { (*name_expr).base.location }, def, false)
      .map(follow);

    let local_name = unsafe { ast_node_as::<AstExprLocal>(name_node) };
    let global_name = unsafe { ast_node_as::<AstExprGlobal>(name_node) };
    let index_name = unsafe { ast_node_as::<AstExprIndexName>(name_node) };
    if !local_name.is_null() {
      unsafe { self.visit_l_value_scope_ptr_ast_expr_type_id(scope, name_expr, generalized_type) };

      unsafe {
        (*scope_raw).bindings.insert(
          Symbol::from_local((*local_name).local),
          Binding {
            type_id: sig.signature,
            location: (*local_name).base.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
        *(*scope_raw).lvalue_types.get_or_insert(def) = sig.signature;
      }
    } else if !global_name.is_null() {
      if existing_function_ty.is_none() {
        unsafe {
          (*self.ice).ice_string_location(
            "prepopulateGlobalScope did not populate a global name",
            &(*global_name).base.base.location,
          );
        }
      }

      if let Some(existing) = existing_function_ty {
        let global_sym = unsafe { (*global_name).name };
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

      unsafe {
        (*scope_raw).bindings.insert(
          Symbol::from_global((*global_name).name),
          Binding {
            type_id: sig.signature,
            location: (*global_name).base.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
        *(*scope_raw).lvalue_types.get_or_insert(def) = sig.signature;
      }
    } else if !index_name.is_null() {
      unsafe { self.visit_l_value_scope_ptr_ast_expr_type_id(scope, name_expr, generalized_type) };
    } else if ast_node_is::<AstExprError>(name_node) {
      generalized_type = unsafe { (*self.builtin_types).error_type };
    }

    if generalized_type.is_null() {
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
}
