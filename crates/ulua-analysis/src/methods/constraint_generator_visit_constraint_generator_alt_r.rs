use alloc::{string::String, sync::Arc, vec::Vec};
use core::ffi::CStr;

use ulua_ast::records::{
  ast_attr::AstAttrType, ast_node::AstNode, ast_stat_declare_function::AstStatDeclareFunction,
};

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  records::{
    binding::Binding, constraint_generator::ConstraintGenerator,
    function_argument::FunctionArgument, function_definition::FunctionDefinition,
    function_type::FunctionType, module::Module, scope::Scope, symbol::Symbol,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_declare_function(
    &mut self,
    scope: &ScopePtr,
    global: *mut AstStatDeclareFunction,
  ) -> ControlFlow {
    let global_ref = unsafe { &*global };

    let generics = self.create_generics(scope, global_ref.generics, false, true);
    let generic_packs = self.create_generic_packs(scope, global_ref.generic_packs, false, true);

    let generic_tys = generics.iter().map(|(_, generic)| generic.ty).collect();
    let generic_tps = generic_packs
      .iter()
      .map(|(_, generic_pack)| generic_pack.tp)
      .collect();

    let fun_scope = if !generics.is_empty() || !generic_packs.is_empty() {
      unsafe { self.child_scope(global as *mut AstNode, scope) }
    } else {
      scope.clone()
    };
    let fun_scope_raw = fun_scope.as_ref() as *const Scope as *mut Scope;

    let param_pack = self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool_polarity(
      fun_scope_raw,
      &global_ref.params,
      false,
      false,
      Polarity::Negative,
    );
    let ret_pack = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
      fun_scope_raw,
      global_ref.ret_types,
      false,
      false,
      Polarity::Positive,
    );

    let module_name = self.module.as_ref().unwrap().name.clone();
    let defn = FunctionDefinition {
      definition_module_name: Some(module_name),
      definition_location: global_ref.base.base.location,
      vararg_location: if global_ref.vararg {
        Some(global_ref.vararg_location)
      } else {
        None
      },
      original_name_location: global_ref.name_location,
    };

    let mut function_type =
      FunctionType::function_type_new(param_pack, ret_pack, Some(defn), false);
    function_type.generics = generic_tys;
    function_type.generic_packs = generic_tps;
    function_type.is_checked_function = global_ref.is_checked_function();

    let deprecated_attr = global_ref.get_attribute(AstAttrType::Deprecated);
    if !deprecated_attr.is_null() {
      function_type.is_deprecated_function = true;
      function_type.deprecated_info =
        Some(Arc::new(unsafe { (*deprecated_attr).deprecated_info() }));
    }

    function_type.arg_names = Vec::with_capacity(global_ref.param_names.size);
    for &(name, location) in &global_ref.param_names {
      function_type.arg_names.push(Some(FunctionArgument {
        name: unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() },
        location,
      }));
    }

    let fn_type = unsafe { (*self.arena).add_type(function_type) };
    let fn_name: Name = unsafe {
      CStr::from_ptr(global_ref.name.value)
        .to_string_lossy()
        .into_owned()
    };

    unsafe {
      let module_ptr = Arc::as_ptr(self.module.as_ref().unwrap()) as *mut Module;
      (*module_ptr).declared_globals.insert(fn_name, fn_type);

      let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
      (*scope_raw).bindings.insert(
        Symbol::from_global(global_ref.name),
        Binding {
          type_id: fn_type,
          location: global_ref.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );

      let def = (*self.dfg).get_def_for_declare_function(global);
      *(*self.root_scope).lvalue_types.get_or_insert(def) = fn_type;
      self.update_r_value_refinements_scope_def_id_type_id(self.root_scope, def, fn_type);
    }

    ControlFlow::None
  }
}
