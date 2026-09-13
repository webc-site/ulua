use core::ffi::CStr;

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_local::AstLocal, ast_name::AstName, ast_name_table::AstNameTable,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  functions::{
    get_builtin_function_id::get_builtin_function_id, get_global_state::get_global_state,
  },
  records::{
    builtin::Builtin, builtin_visitor::BuiltinVisitor, compile_options::CompileOptions,
    variable::Variable,
  },
};

impl BuiltinVisitor {
  pub fn new(
    result: &mut DenseHashMap<*mut AstExprCall, i32>,
    globals: &DenseHashMap<AstName, Global>,
    variables: &DenseHashMap<*mut AstLocal, Variable>,
    options: &CompileOptions,
    names: &AstNameTable,
  ) -> Self {
    let mut builtin_is_disabled = [false; 256];

    let disabled_builtins = options.disabled_builtins;
    if !disabled_builtins.is_null() {
      unsafe {
        let mut ptr = disabled_builtins;
        while !(*ptr).is_null() {
          let c_str = CStr::from_ptr(*ptr);
          let bytes = c_str.to_bytes();

          if let Some(dot) = bytes.iter().position(|&b| b == b'.') {
            let library = names.get_slice(&bytes[..dot]);
            let name = names.get_slice(&bytes[dot + 1..]);

            if !library.is_null()
              && !name.is_null()
              && get_global_state(globals, name) == Global::Default
            {
              let builtin = Builtin {
                object: library,
                method: name,
              };

              let bfid = get_builtin_function_id(&builtin, options);
              if bfid >= 0 && (bfid as usize) < 256 {
                builtin_is_disabled[bfid as usize] = true;
              }
            }
          } else {
            let name = names.get_slice(bytes);
            if !name.is_null() && get_global_state(globals, name) == Global::Default {
              let builtin = Builtin {
                object: AstName::new(),
                method: name,
              };

              let bfid = get_builtin_function_id(&builtin, options);
              if bfid >= 0 && (bfid as usize) < 256 {
                builtin_is_disabled[bfid as usize] = true;
              }
            }
          }
          ptr = ptr.add(1);
        }
      }
    }

    Self {
      result: result as *mut _,
      builtin_is_disabled,
      globals: globals as *const _,
      variables: variables as *const _,
      options: options as *const _,
    }
  }
}

pub fn builtin_visitor_builtin_visitor(
  result: &mut DenseHashMap<*mut AstExprCall, i32>,
  globals: &DenseHashMap<AstName, Global>,
  variables: &DenseHashMap<*mut AstLocal, Variable>,
  options: &CompileOptions,
  names: &AstNameTable,
) -> BuiltinVisitor {
  BuiltinVisitor::new(result, globals, variables, options, names)
}
