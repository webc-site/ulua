use alloc::{string::String, sync::Arc, vec::Vec};
use core::ffi::CStr;

use ulua_ast::records::ast_stat_declare_function::AstStatDeclareFunction;

use crate::{
  enums::control_flow::ControlFlow,
  records::{
    binding::Binding, function_argument::FunctionArgument, function_definition::FunctionDefinition,
    function_type::FunctionType, module::Module, scope::Scope, symbol::Symbol,
    type_checker::TypeChecker, type_pack::TypePack,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_declare_function(
    &mut self,
    scope: &ScopePtr,
    global: &AstStatDeclareFunction,
  ) -> ControlFlow {
    let fun_scope = self.child_function_scope(scope, &global.base.base.location, 0);

    let defs = self.create_generic_types(
      &fun_scope,
      None,
      &global.base.base,
      &global.generics,
      &global.generic_packs,
      false,
    );

    let generic_tys = defs.generic_types.iter().map(|def| def.ty).collect();
    let generic_tps = defs.generic_packs.iter().map(|def| def.tp).collect();

    let arg_pack =
      self.resolve_type_pack_scope_ptr_ast_type_list(fun_scope.clone(), &global.params);
    let ret_pack = if global.ret_types.is_null() {
      self.add_type_pack_type_pack(TypePack {
        head: Vec::new(),
        tail: None,
      })
    } else {
      self
        .resolve_type_pack_scope_ptr_ast_type_pack(fun_scope.clone(), unsafe { &*global.ret_types })
    };

    let module_raw =
      Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
    let defn = unsafe {
      FunctionDefinition {
        definition_module_name: Some((*module_raw).name.clone()),
        definition_location: global.base.base.location,
        vararg_location: if global.vararg {
          Some(global.vararg_location)
        } else {
          None
        },
        original_name_location: global.name_location,
      }
    };

    let mut ftv = FunctionType::function_type_new(arg_pack, ret_pack, Some(defn), false);
    ftv.level = fun_scope.level;
    ftv.generics = generic_tys;
    ftv.generic_packs = generic_tps;

    for (name, location) in global.param_names.iter() {
      if !name.value.is_null() {
        ftv.arg_names.push(Some(FunctionArgument {
          name: unsafe { CStr::from_ptr(name.value) }
            .to_string_lossy()
            .into_owned(),
          location: *location,
        }));
      } else {
        ftv.arg_names.push(None);
      }
    }

    let fn_ty = self.add_type(&ftv);
    let fn_name = unsafe { CStr::from_ptr(global.name.value) }
      .to_string_lossy()
      .into_owned();

    unsafe {
      (*module_raw).declared_globals.insert(fn_name, fn_ty);

      let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
      (*scope_raw).bindings.insert(
        Symbol::from_global(global.name),
        Binding {
          type_id: fn_ty,
          location: global.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    ControlFlow::None
  }
}
