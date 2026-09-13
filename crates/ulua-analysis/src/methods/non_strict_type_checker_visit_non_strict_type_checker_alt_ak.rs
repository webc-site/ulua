use alloc::string::{String, ToString};
use core::ffi::CStr;

use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_node::AstNode};

use crate::{
  records::{
    non_strict_context::NonStrictContext,
    non_strict_function_definition_error::NonStrictFunctionDefinitionError,
    non_strict_type_checker::NonStrictTypeChecker, scope::Scope,
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_expr_function(
    &mut self,
    expr_fn: *mut AstExprFunction,
  ) -> NonStrictContext {
    // TODO: should a function being used as an expression generate a context without the arguments?
    let pusher = self.push_stack(expr_fn as *mut AstNode);
    let mut remainder = unsafe { self.visit_ast_stat_block((*expr_fn).body) };

    let scope: *mut Scope = match &pusher {
      Some(p) => p.scope,
      None => unsafe { (*self.module).get_module_scope().as_ref() as *const Scope as *mut Scope },
    };

    unsafe {
      let args = &(*expr_fn).args;
      for &local in args.as_slice() {
        if let Some(ty) = self.will_run_time_error_function_definition(local, scope, &remainder) {
          let debugname_ptr = (*expr_fn).debugname.value;
          let debugname: String = if debugname_ptr.is_null() {
            String::new()
          } else {
            CStr::from_ptr(debugname_ptr).to_string_lossy().to_string()
          };
          let arg_name: String = if (*local).name.value.is_null() {
            String::new()
          } else {
            CStr::from_ptr((*local).name.value)
              .to_string_lossy()
              .to_string()
          };
          let loc = (*local).location;
          self.report_error(
            TypeErrorData::NonStrictFunctionDefinitionError(NonStrictFunctionDefinitionError::new(
              debugname, arg_name, ty,
            )),
            &loc,
          );
        }

        let def = (*self.dfg).get_def_for_local(local);
        remainder.remove(&def);

        self.visit_ast_type((*local).annotation);
      }

      self.visit_generics((*expr_fn).generics, (*expr_fn).generic_packs);

      self.visit_ast_type_pack((*expr_fn).return_annotation);

      if !(*expr_fn).vararg_annotation.is_null() {
        self.visit_ast_type_pack((*expr_fn).vararg_annotation);
      }
    }

    remainder
  }
}
