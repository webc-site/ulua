use ulua_ast::{
  records::{
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_node::AstNode,
    ast_stat_for::AstStatFor,
    location::Location,
  },
  rtti::ast_node_as,
};
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_for_range::LintForRange};

impl LintForRange {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_stat_for_linter(&mut self, node: *mut AstStatFor) -> bool {
    unsafe {
      if (*node).step.is_null() {
        let fc = ast_node_as::<AstExprConstantNumber>((*node).from as *mut AstNode);
        let fu = ast_node_as::<AstExprUnary>((*node).from as *mut AstNode);
        let tc = ast_node_as::<AstExprConstantNumber>((*node).to as *mut AstNode);
        let tu = ast_node_as::<AstExprUnary>((*node).to as *mut AstNode);

        let range_location = Location::new(
          (*(*node).from).base.location.begin,
          (*(*node).to).base.location.end,
        );

        if (!fu.is_null() && (*fu).op == AstExprUnaryOp::Len && !tc.is_null() && (*tc).value == 1.0)
          || (!fc.is_null() && !tc.is_null() && (*fc).value > (*tc).value)
        {
          emit_warning(
            &mut *self.context,
            Code::ForRange,
            range_location,
            format_args!(
              "For loop should iterate backwards; did you forget to specify -1 as step?"
            ),
          );
        } else if !fc.is_null()
          && !tc.is_null()
          && self.get_loop_end((*fc).value, (*tc).value) != (*tc).value
        {
          emit_warning(
            &mut *self.context,
            Code::ForRange,
            range_location,
            format_args!(
              "For loop ends at {} instead of {}; did you forget to specify step?",
              self.get_loop_end((*fc).value, (*tc).value),
              (*tc).value
            ),
          );
        } else if !fc.is_null()
          && !tu.is_null()
          && (*fc).value == 0.0
          && (*tu).op == AstExprUnaryOp::Len
        {
          emit_warning(
            &mut *self.context,
            Code::ForRange,
            range_location,
            format_args!("For loop starts at 0, but arrays start at 1"),
          );
        } else if !fu.is_null()
          && (*fu).op == AstExprUnaryOp::Len
          && !tc.is_null()
          && (*tc).value == 0.0
        {
          emit_warning(
            &mut *self.context,
            Code::ForRange,
            range_location,
            format_args!(
              "For loop should iterate backwards; did you forget to specify -1 as step? Also consider changing 0 to 1 since arrays start at 1"
            ),
          );
        }
      }
    }

    true
  }
}
