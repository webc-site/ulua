use ulua_ast::{
  records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, follow_type::follow_type_id},
  records::lint_table_operations::LintTableOperations,
};
impl LintTableOperations {
  /// # Safety
  /// 调用方须保证 `node、`func` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn check_table_call(&mut self, node: *mut AstExprCall, func: *mut AstExprIndexName) {
    let node_ref = unsafe { &*node };
    let args = node_ref.args.as_slice();

    if unsafe { (*func).index } == "insert" && args.len() == 2 {
      let tail = unsafe { ast_node_as::<AstExprCall>(args[1] as *mut AstNode) };

      if !tail.is_null()
        && let Some(funty) = unsafe { (*self.context).get_type((*tail).func) }
      {
        let ret = self.get_return_count(follow_type_id(funty));

        if ret > 1 {
          emit_warning(
            unsafe { &mut *self.context },
            Code::TableOperations,
            unsafe { (*tail).base.base.location },
            format_args!(
              "table.insert may change behavior if the call returns more than one result; consider adding parentheses around second argument"
            ),
          );
        }
      }
    }

    if unsafe { (*func).index } == "insert" && args.len() >= 3 {
      if self.is_constant(args[1], 0.0) {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[1]).base.location },
          format_args!("table.insert uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }

      if self.is_length(args[1], args[0]) {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[1]).base.location },
          format_args!(
            "table.insert will insert the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
          ),
        );
      }

      let add = unsafe { ast_node_as::<AstExprBinary>(args[1] as *mut AstNode) };
      if !add.is_null()
        && unsafe { (*add).op == AstExprBinaryOp::Add }
        && self.is_length(unsafe { (*add).left }, args[0])
        && self.is_constant(unsafe { (*add).right }, 1.0)
      {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[1]).base.location },
          format_args!(
            "table.insert will append the value to the table; consider removing the second argument for efficiency"
          ),
        );
      }
    }

    if unsafe { (*func).index } == "remove" && args.len() >= 2 {
      if self.is_constant(args[1], 0.0) {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[1]).base.location },
          format_args!("table.remove uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }

      let sub = unsafe { ast_node_as::<AstExprBinary>(args[1] as *mut AstNode) };
      if !sub.is_null()
        && unsafe { (*sub).op == AstExprBinaryOp::Sub }
        && self.is_length(unsafe { (*sub).left }, args[0])
        && self.is_constant(unsafe { (*sub).right }, 1.0)
      {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[1]).base.location },
          format_args!(
            "table.remove will remove the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
          ),
        );
      }
    }

    if unsafe { (*func).index } == "move" && args.len() >= 4 {
      if self.is_constant(args[1], 0.0) {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[1]).base.location },
          format_args!("table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      } else if self.is_constant(args[3], 0.0) {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[3]).base.location },
          format_args!("table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }
    }

    if unsafe { (*func).index } == "create" && args.len() == 2 {
      if !unsafe { ast_node_as::<AstExprTable>(args[1] as *mut AstNode) }.is_null() {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*args[1]).base.location },
          format_args!(
            "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
          ),
        );
      }

      let assertion = unsafe { ast_node_as::<AstExprTypeAssertion>(args[1] as *mut AstNode) };
      if !assertion.is_null()
        && !unsafe { ast_node_as::<AstExprTable>((*assertion).expr as *mut AstNode) }.is_null()
      {
        emit_warning(
          unsafe { &mut *self.context },
          Code::TableOperations,
          unsafe { (*(*assertion).expr).base.location },
          format_args!(
            "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
          ),
        );
      }
    }
  }
}
