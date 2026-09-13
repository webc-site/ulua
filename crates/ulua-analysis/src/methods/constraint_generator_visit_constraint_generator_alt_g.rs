// ConstraintGenerator::visit(const ScopePtr&, AstStatLocalFunction*) (ConstraintGenerator.cpp:1733-1797).
use alloc::{string::String, sync::Arc, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_stat_local_function::AstStatLocalFunction};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns,
    checkpoint::checkpoint, for_each_constraint::for_each_constraint,
    get_mutable_type::get_mutable,
    propagate_deprecated_attribute_to_constraint::propagate_deprecated_attribute_to_constraint,
  },
  records::{
    binding::Binding, blocked_type::BlockedType, constraint::Constraint,
    constraint_generator::ConstraintGenerator, generalization_constraint::GeneralizationConstraint,
    module::Module, scope::Scope, symbol::Symbol,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_local_function(
    &mut self,
    scope: &ScopePtr,
    function: *mut AstStatLocalFunction,
  ) -> ControlFlow {
    let function_ref = unsafe { &*function };
    let name_local = function_ref.name;
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

    // The parser ensures that every local function has a distinct Symbol for its name.
    let ty = unsafe { (*scope_raw).lookup_symbol(Symbol::from_local(name_local)) };
    LUAU_ASSERT!(ty.is_none());

    let function_type: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
    unsafe {
      (*scope_raw).bindings.insert(
        Symbol::from_local(name_local),
        Binding {
          type_id: function_type,
          location: (*name_local).location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    let sig = unsafe {
      self.check_function_signature(
        scope,
        null_mut(),
        function_ref.func,
        None,
        Some((*name_local).location),
      )
    };
    let body_scope_raw = sig.body_scope.as_ref() as *const Scope as *mut Scope;
    unsafe {
      (*body_scope_raw).bindings.insert(
        Symbol::from_local(name_local),
        Binding {
          type_id: sig.signature,
          location: (*name_local).location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    let def = unsafe { (*self.dfg).get_def_local(name_local) };
    unsafe {
      *(*scope_raw).lvalue_types.get_or_insert(def) = function_type;
    }
    self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, function_type);
    unsafe {
      *(*body_scope_raw).lvalue_types.get_or_insert(def) = sig.signature;
    }
    self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);

    let start = unsafe { checkpoint(self as *const _) };
    self.check_function_body(&sig.body_scope, unsafe { &*function_ref.func });
    let end = unsafe { checkpoint(self as *const _) };

    // constraintScope = sig.signatureScope ? sig.signatureScope : sig.bodyScope.
    let constraint_scope: &ScopePtr = &sig.signature_scope;

    let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
      constraint_scope,
      unsafe { (*name_local).location },
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type: function_type,
        source_type: sig.signature,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: false,
      }),
    );

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

    // function_type 刚由 add_type(BlockedType) 分配，必命中；对照 C++:2625
    // `getMutable<BlockedType>(functionType)->setOwner(genConstraint)`
    let blocked = get_mutable::<BlockedType>(function_type).unwrap();
    blocked.set_owner(c as *const _);

    // module->ast_types[function->func] = function_type;
    let module = self.module.as_ref().unwrap();
    let module_ptr = Arc::as_ptr(module) as *mut Module;
    unsafe {
      *(*module_ptr)
        .ast_types
        .get_or_insert(function_ref.func as *const _ as *const AstExpr) = function_type;
    }

    ControlFlow::None
  }
}
