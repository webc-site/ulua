use alloc::{string::String, sync::Arc};
use core::ffi::CStr;

use ulua_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  records::{
    binding::Binding, constraint_generator::ConstraintGenerator, module::Module, scope::Scope,
    symbol::Symbol,
  },
  type_aliases::{def_id_def::DefId, name_type::Name, type_id::TypeId},
};
impl ConstraintGenerator {
  // ConstraintGenerator::visit(const ScopePtr&, AstStatDeclareGlobal*)
  // (ConstraintGenerator.cpp:2273).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_declare_global(
    &mut self,
    scope: *mut Scope,
    global: *mut AstStatDeclareGlobal,
  ) -> ControlFlow {
    let global_ref = unsafe { &*global };
    LUAU_ASSERT!(!global_ref.type_.is_null());

    let global_ty: TypeId =
      self.resolve_type(scope, global_ref.type_, false, false, Polarity::Positive);
    let global_name: Name = unsafe {
      CStr::from_ptr(global_ref.name.value)
        .to_string_lossy()
        .into_owned()
    };

    // module->declaredGlobals[globalName] = globalTy;
    unsafe {
      let module_ptr = Arc::as_ptr(self.module.as_ref().unwrap()) as *mut Module;
      (*module_ptr)
        .declared_globals
        .insert(global_name, global_ty);
    }

    // rootScope->bindings[global->name] = Binding{globalTy, global->location};
    let binding = Binding {
      type_id: global_ty,
      location: global_ref.base.base.location,
      deprecated: false,
      deprecated_suggestion: String::new(),
      documentation_symbol: None,
    };
    unsafe {
      (*self.root_scope)
        .bindings
        .insert(Symbol::from_global(global_ref.name), binding);
    }

    let def: DefId = unsafe { (*self.dfg).get_def_declare_global(global) };
    unsafe {
      *(*self.root_scope).lvalue_types.get_or_insert(def) = global_ty;
    }
    unsafe {
      self.update_r_value_refinements_scope_def_id_type_id(self.root_scope, def, global_ty)
    };

    ControlFlow::None
  }
}
