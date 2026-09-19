use core::fmt::Arguments;

use ulua_ast::{
  records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion,
    location::Location,
  },
  rtti::ast_node_try_as,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, follow_type::follow_type_id},
  records::{lint_context::LintContext, lint_table_operations::LintTableOperations},
};
impl LintTableOperations {
  /// # Safety
  /// 调用方须保证 `node、`func` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn check_table_call(&mut self, node: *mut AstExprCall, func: *mut AstExprIndexName) {
    // SAFETY: node 由调用方契约保证有效；本函数不修改 AST，共享引用存活期内无写入。
    let node_ref = unsafe { &*node };
    let args = node_ref.args.as_slice();
    // SAFETY: func 同上；index 只读，存活期内无写入。
    let index = unsafe { &(*func).index };

    if *index == "insert" && args.len() == 2 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };

      if let Some(tail) = ast_node_try_as::<AstExprCall>(&arg1.base)
        && let Some(funty) = unsafe { (*self.context).get_type(tail.func) }
      {
        let ret = self.get_return_count(follow_type_id(funty));

        if ret > 1 {
          warn(
            self.context,
            tail.base.base.location,
            format_args!(
              "table.insert may change behavior if the call returns more than one result; consider adding parentheses around second argument"
            ),
          );
        }
      }
    }

    if *index == "insert" && args.len() >= 3 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };

      if self.is_constant(args[1], 0.0) {
        warn(
          self.context,
          arg1.base.location,
          format_args!("table.insert uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }

      if self.is_length(args[1], args[0]) {
        warn(
          self.context,
          arg1.base.location,
          format_args!(
            "table.insert will insert the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
          ),
        );
      }

      if let Some(add) = ast_node_try_as::<AstExprBinary>(&arg1.base)
        && add.op == AstExprBinaryOp::Add
        && self.is_length(add.left, args[0])
        && self.is_constant(add.right, 1.0)
      {
        warn(
          self.context,
          arg1.base.location,
          format_args!(
            "table.insert will append the value to the table; consider removing the second argument for efficiency"
          ),
        );
      }
    }

    if *index == "remove" && args.len() >= 2 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };

      if self.is_constant(args[1], 0.0) {
        warn(
          self.context,
          arg1.base.location,
          format_args!("table.remove uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }

      if let Some(sub) = ast_node_try_as::<AstExprBinary>(&arg1.base)
        && sub.op == AstExprBinaryOp::Sub
        && self.is_length(sub.left, args[0])
        && self.is_constant(sub.right, 1.0)
      {
        warn(
          self.context,
          arg1.base.location,
          format_args!(
            "table.remove will remove the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
          ),
        );
      }
    }

    if *index == "move" && args.len() >= 4 {
      if self.is_constant(args[1], 0.0) {
        // SAFETY: args 元素由解析器保证非空。
        let arg1 = unsafe { &*args[1] };
        warn(
          self.context,
          arg1.base.location,
          format_args!("table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      } else if self.is_constant(args[3], 0.0) {
        // SAFETY: args 元素由解析器保证非空。
        let arg3 = unsafe { &*args[3] };
        warn(
          self.context,
          arg3.base.location,
          format_args!("table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }
    }

    if *index == "create" && args.len() == 2 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };

      if ast_node_try_as::<AstExprTable>(&arg1.base).is_some() {
        warn(
          self.context,
          arg1.base.location,
          format_args!(
            "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
          ),
        );
      }

      if let Some(assertion) = ast_node_try_as::<AstExprTypeAssertion>(&arg1.base) {
        // SAFETY: assertion.expr 由解析器保证非空（类型断言必有内层表达式）。
        let inner = unsafe { &*assertion.expr };
        if ast_node_try_as::<AstExprTable>(&inner.base).is_some() {
          warn(
            self.context,
            inner.base.location,
            format_args!(
              "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
            ),
          );
        }
      }
    }
  }
}

/// 统一告警入口（Code 固定为 TableOperations，雷同调用收敛于此）。
/// # Safety
/// context 由 LintTableOperations 持有，有效性由调用方契约保证。
fn warn(context: *mut LintContext, location: Location, args: Arguments<'_>) {
  // SAFETY: context 非空（调用方契约），仅在此处解引用一次。
  emit_warning(
    unsafe { &mut *context },
    Code::TableOperations,
    location,
    args,
  );
}
