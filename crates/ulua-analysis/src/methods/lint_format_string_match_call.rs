use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_constant_string::AstExprConstantString, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, is_string::is_string},
  records::lint_format_string::LintFormatString,
};

impl LintFormatString {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn match_call(&mut self, node: *mut AstExprCall) {
    let node = unsafe { &*node };
    let func = unsafe { rtti::ast_node_as::<AstExprIndexName>(node.func as *mut AstNode) };
    if func.is_null() {
      return;
    }

    if node.self_ {
      let group = unsafe { rtti::ast_node_as::<AstExprGroup>((*func).expr as *mut AstNode) };
      let self_expr: *mut AstExpr = if !group.is_null() {
        unsafe { (*group).expr }
      } else {
        unsafe { (*func).expr }
      };

      if rtti::ast_node_is::<AstExprConstantString>(self_expr as *mut AstNode) {
        self.match_string_call(unsafe { (*func).index }, self_expr, node.args);
      } else if let Some(type_id) = unsafe { (*self.context).get_type(self_expr) }
        && is_string(type_id)
      {
        self.match_string_call(unsafe { (*func).index }, self_expr, node.args);
      }
      return;
    }

    let lib = unsafe { rtti::ast_node_as::<AstExprGlobal>((*func).expr as *mut AstNode) };
    if lib.is_null() {
      return;
    }

    let lib_name = unsafe { (*lib).name };

    if lib_name == "string" {
      if let Some(&first_arg) = node.args.as_slice().first() {
        let rest = AstArray {
          data: unsafe { node.args.data.add(1) },
          size: node.args.size - 1,
        };
        self.match_string_call(unsafe { (*func).index }, first_arg, rest);
      }
    } else if lib_name == "os"
      && unsafe { (*func).index } == "date"
      && let Some(&arg0) = node.args.as_slice().first()
    {
      let fmt = unsafe { rtti::ast_node_as::<AstExprConstantString>(arg0 as *mut AstNode) };
      if !fmt.is_null() {
        let fmt = unsafe { &*fmt };
        let bytes = fmt.value.as_bytes();
        if let Some(error) = self.check_date_format(bytes) {
          emit_warning(
            unsafe { &mut *self.context },
            Code::FormatString,
            fmt.base.base.location,
            format_args!("Invalid date format: {}", error),
          );
        }
      }
    }
  }
}
