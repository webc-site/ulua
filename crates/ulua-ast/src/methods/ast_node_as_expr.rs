use core::ptr::null_mut;

use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  },
  rtti::AstNodeClass,
};

impl AstNode {
  #[inline]
  pub fn as_expr(&self) -> *mut AstExpr {
    let is_expr = matches!(
      self.class_index,
      AstExprBinary::CLASS_INDEX
        | AstExprCall::CLASS_INDEX
        | AstExprConstantBool::CLASS_INDEX
        | AstExprConstantInteger::CLASS_INDEX
        | AstExprConstantNil::CLASS_INDEX
        | AstExprConstantNumber::CLASS_INDEX
        | AstExprConstantString::CLASS_INDEX
        | AstExprError::CLASS_INDEX
        | AstExprFunction::CLASS_INDEX
        | AstExprGlobal::CLASS_INDEX
        | AstExprGroup::CLASS_INDEX
        | AstExprIfElse::CLASS_INDEX
        | AstExprIndexExpr::CLASS_INDEX
        | AstExprIndexName::CLASS_INDEX
        | AstExprInstantiate::CLASS_INDEX
        | AstExprInterpString::CLASS_INDEX
        | AstExprLocal::CLASS_INDEX
        | AstExprTable::CLASS_INDEX
        | AstExprTypeAssertion::CLASS_INDEX
        | AstExprUnary::CLASS_INDEX
        | AstExprVarargs::CLASS_INDEX
    );

    if is_expr {
      self as *const AstNode as *mut AstExpr
    } else {
      null_mut()
    }
  }

  #[inline]
  pub fn as_expr_const(&self) -> *const AstExpr {
    self.as_expr() as *const AstExpr
  }
}
