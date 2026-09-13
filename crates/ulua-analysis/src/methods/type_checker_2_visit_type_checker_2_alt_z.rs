use core::ptr::null_mut;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
  ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
  ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
  ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
  ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
  ast_expr_if_else::AstExprIfElse, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_index_name::AstExprIndexName, ast_expr_instantiate::AstExprInstantiate,
  ast_expr_interp_string::AstExprInterpString, ast_expr_local::AstExprLocal,
  ast_expr_table::AstExprTable, ast_expr_type_assertion::AstExprTypeAssertion,
  ast_expr_unary::AstExprUnary, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};
impl TypeChecker2 {
  pub fn visit_ast_expr_value_context(&mut self, expr: *mut AstExpr, context: ValueContext) {
    let _pusher = self.push_stack(expr as *mut AstNode);

    unsafe {
      let node = expr as *mut AstNode;
      if (*node).is::<AstExprGroup>() {
        self.visit_ast_expr_group_value_context(expr as *mut AstExprGroup, context);
      } else if (*node).is::<AstExprConstantNil>() {
        self.visit_ast_expr_constant_nil(expr as *mut AstExprConstantNil);
      } else if (*node).is::<AstExprConstantBool>() {
        self.visit_ast_expr_constant_bool(expr as *mut AstExprConstantBool);
      } else if (*node).is::<AstExprConstantNumber>() {
        self.visit_ast_expr_constant_number(expr as *mut AstExprConstantNumber);
      } else if (*node).is::<AstExprConstantInteger>() {
        self.visit_ast_expr_constant_integer(expr as *mut AstExprConstantInteger);
      } else if (*node).is::<AstExprConstantString>() {
        self.visit_ast_expr_constant_string(expr as *mut AstExprConstantString);
      } else if (*node).is::<AstExprLocal>() {
        self.visit_ast_expr_local(expr as *mut AstExprLocal);
      } else if (*node).is::<AstExprGlobal>() {
        self.visit_ast_expr_global(expr as *mut AstExprGlobal);
      } else if (*node).is::<AstExprVarargs>() {
        self.visit_ast_expr_varargs(expr as *mut AstExprVarargs);
      } else if (*node).is::<AstExprCall>() {
        self.visit_ast_expr_call(expr as *mut AstExprCall);
      } else if (*node).is::<AstExprIndexName>() {
        self.visit_ast_expr_index_name_value_context(expr as *mut AstExprIndexName, context);
      } else if (*node).is::<AstExprIndexExpr>() {
        // SAFETY: is::<AstExprIndexExpr> 已判定动态类型；expr 指向 AST arena。
        self.visit_ast_expr_index_expr_value_context(&*(expr as *const AstExprIndexExpr), context);
      } else if (*node).is::<AstExprFunction>() {
        // SAFETY: is::<AstExprFunction> 已判定动态类型；expr 指向 AST arena。
        self.visit_ast_expr_function(&*(expr as *const AstExprFunction));
      } else if (*node).is::<AstExprTable>() {
        self.visit_ast_expr_table(expr as *mut AstExprTable);
      } else if (*node).is::<AstExprUnary>() {
        // SAFETY: is::<AstExprUnary> 已判定动态类型；expr 指向 AST arena。
        self.visit_ast_expr_unary(&*(expr as *const AstExprUnary));
      } else if (*node).is::<AstExprBinary>() {
        // SAFETY: is::<AstExprBinary> 已判定动态类型；expr 指向 AST arena。
        self.visit_ast_expr_binary_ast_node(&*(expr as *const AstExprBinary), null_mut());
      } else if (*node).is::<AstExprTypeAssertion>() {
        self.visit_ast_expr_type_assertion(expr as *mut AstExprTypeAssertion);
      } else if (*node).is::<AstExprIfElse>() {
        self.visit_ast_expr_if_else(expr as *mut AstExprIfElse);
      } else if (*node).is::<AstExprInstantiate>() {
        self.visit_ast_expr_instantiate(expr as *mut AstExprInstantiate);
      } else if (*node).is::<AstExprInterpString>() {
        self.visit_ast_expr_interp_string(expr as *mut AstExprInterpString);
      } else if (*node).is::<AstExprError>() {
        self.visit_ast_expr_error(expr as *mut AstExprError);
      } else {
        LUAU_ASSERT!(false);
        panic!("TypeChecker2 encountered an unknown expression type");
      }
    }
  }
}
