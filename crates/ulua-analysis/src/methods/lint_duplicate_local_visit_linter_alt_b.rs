use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_node::AstNode};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_duplicate_local::LintDuplicateLocal,
};

impl LintDuplicateLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    unsafe {
      let node_ref = &*node;

      if !node_ref.self_.is_null() {
        *self.locals.get_or_insert(node_ref.self_) = node as *mut AstNode;
      }

      for &arg in node_ref.args.as_slice() {
        *self.locals.get_or_insert(arg) = node as *mut AstNode;
      }

      for &local in node_ref.args.as_slice() {
        let local_ref = &*local;

        if !local_ref.shadow.is_null()
          && self.locals.find(&local_ref.shadow).copied() == Some(node as *mut AstNode)
          && !self.ignore_duplicate(local)
        {
          if local_ref.shadow == node_ref.self_ {
            emit_warning(
              &mut *self.context,
              Code::DuplicateLocal,
              local_ref.location,
              format_args!("Function parameter 'self' already defined implicitly"),
            );
          } else {
            let shadow = &*local_ref.shadow;

            if shadow.location.begin.line == local_ref.location.begin.line {
              emit_warning(
                &mut *self.context,
                Code::DuplicateLocal,
                local_ref.location,
                format_args!(
                  "Function parameter '{}' already defined on column {}",
                  local_ref.name,
                  shadow.location.begin.column + 1
                ),
              );
            } else {
              emit_warning(
                &mut *self.context,
                Code::DuplicateLocal,
                local_ref.location,
                format_args!(
                  "Function parameter '{}' already defined on line {}",
                  local_ref.name,
                  shadow.location.begin.line + 1
                ),
              );
            }
          }
        }
      }
    }

    true
  }
}
