use ulua_ast::{
  records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_misleading_and_or::LintMisleadingAndOr,
};
impl LintMisleadingAndOr {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
    if unsafe { (*node).op } != AstExprBinaryOp::Or {
      return true;
    }

    let left = unsafe { (*node).left };
    let and_ = unsafe { ast_node_as::<AstExprBinary>(left as *mut AstNode) };
    if and_.is_null() {
      return true;
    }

    if unsafe { (*and_).op } != AstExprBinaryOp::And {
      return true;
    }

    let mut alt: Option<&'static str> = None;

    let right = unsafe { (*and_).right };
    if !unsafe { ast_node_as::<AstExprConstantNil>(right as *mut AstNode) }.is_null() {
      alt = Some("nil");
    } else {
      let bool_node = unsafe { ast_node_as::<AstExprConstantBool>(right as *mut AstNode) };
      if !bool_node.is_null() && !unsafe { (*bool_node).value } {
        alt = Some("false");
      }
    }

    if let Some(alt_val) = alt {
      emit_warning(
        unsafe { &mut *self.context },
        Code::MisleadingAndOr,
        unsafe { (*node).base.base.location },
        format_args!(
          "The and-or expression always evaluates to the second alternative because the first alternative is {}; consider using if-then-else expression instead",
          alt_val
        ),
      );
    }

    true
  }
}
