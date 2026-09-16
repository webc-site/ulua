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
use ulua_common::{FFlag, FInt, LUAU_ASSERT};

use crate::{
  enums::value_context::ValueContext,
  records::{
    non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
    recursion_counter::RecursionCounter,
  },
  rtti::{ast_node_as, ast_rtti_index},
};
impl NonStrictTypeChecker {
  pub fn visit_ast_expr_value_context(
    &mut self,
    expr: *mut AstExpr,
    context: ValueContext,
  ) -> NonStrictContext {
    let mut _rc: Option<RecursionCounter> = None;
    if FFlag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
      _rc = Some(unsafe {
        RecursionCounter::recursion_counter_i32(&mut self.non_strict_recursion_count)
      });
      if FInt::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
        && self.non_strict_recursion_count >= FInt::LuauNonStrictTypeCheckerRecursionLimit.get()
      {
        return NonStrictContext::new();
      }
    }

    let _pusher = self.push_stack(expr as *mut AstNode);

    let expr_ptr = expr as *mut AstNode;
    let class_index = unsafe { (*expr_ptr).class_index };

    if class_index == ast_rtti_index("AstExprGroup") {
      unsafe { ast_node_as::<AstExprGroup>(expr_ptr) };
      let e = unsafe { &mut *(expr as *mut AstExprGroup) };
      unsafe { self.visit_ast_expr_group_value_context(e, context) }
    } else if class_index == ast_rtti_index("AstExprConstantNil") {
      let e = unsafe { &mut *(expr as *mut AstExprConstantNil) };
      self.visit_ast_expr_constant_nil(e)
    } else if class_index == ast_rtti_index("AstExprConstantBool") {
      let e = unsafe { &mut *(expr as *mut AstExprConstantBool) };
      self.visit_ast_expr_constant_bool(e)
    } else if class_index == ast_rtti_index("AstExprConstantNumber") {
      let e = unsafe { &mut *(expr as *mut AstExprConstantNumber) };
      self.visit_ast_expr_constant_number(e)
    } else if class_index == ast_rtti_index("AstExprConstantInteger") {
      let e = unsafe { &mut *(expr as *mut AstExprConstantInteger) };
      self.visit_ast_expr_constant_integer(e)
    } else if class_index == ast_rtti_index("AstExprConstantString") {
      let e = unsafe { &mut *(expr as *mut AstExprConstantString) };
      self.visit_ast_expr_constant_string(e)
    } else if class_index == ast_rtti_index("AstExprLocal") {
      let e = unsafe { &mut *(expr as *mut AstExprLocal) };
      self.visit_ast_expr_local_value_context(e, context)
    } else if class_index == ast_rtti_index("AstExprGlobal") {
      let e = unsafe { &mut *(expr as *mut AstExprGlobal) };
      unsafe { self.visit_ast_expr_global_value_context(e, context) }
    } else if class_index == ast_rtti_index("AstExprVarargs") {
      let e = unsafe { &mut *(expr as *mut AstExprVarargs) };
      self.visit_ast_expr_varargs(e)
    } else if class_index == ast_rtti_index("AstExprCall") {
      let e = unsafe { &mut *(expr as *mut AstExprCall) };
      self.visit_ast_expr_call(e)
    } else if class_index == ast_rtti_index("AstExprIndexName") {
      let e = unsafe { &mut *(expr as *mut AstExprIndexName) };
      unsafe { self.visit_ast_expr_index_name_value_context(e, context) }
    } else if class_index == ast_rtti_index("AstExprIndexExpr") {
      let e = unsafe { &mut *(expr as *mut AstExprIndexExpr) };
      unsafe { self.visit_ast_expr_index_expr_value_context(e, context) }
    } else if class_index == ast_rtti_index("AstExprFunction") {
      let e = unsafe { &mut *(expr as *mut AstExprFunction) };
      self.visit_ast_expr_function(e)
    } else if class_index == ast_rtti_index("AstExprTable") {
      let e = unsafe { &mut *(expr as *mut AstExprTable) };
      self.visit_ast_expr_table(e)
    } else if class_index == ast_rtti_index("AstExprUnary") {
      let e = unsafe { &mut *(expr as *mut AstExprUnary) };
      self.visit_ast_expr_unary(e)
    } else if class_index == ast_rtti_index("AstExprBinary") {
      let e = unsafe { &mut *(expr as *mut AstExprBinary) };
      self.visit_ast_expr_binary(e)
    } else if class_index == ast_rtti_index("AstExprTypeAssertion") {
      let e = unsafe { &mut *(expr as *mut AstExprTypeAssertion) };
      self.visit_ast_expr_type_assertion(e)
    } else if class_index == ast_rtti_index("AstExprIfElse") {
      let e = unsafe { &mut *(expr as *mut AstExprIfElse) };
      self.visit_ast_expr_if_else(e)
    } else if class_index == ast_rtti_index("AstExprInterpString") {
      let e = unsafe { &mut *(expr as *mut AstExprInterpString) };
      self.visit_ast_expr_interp_string(e)
    } else if class_index == ast_rtti_index("AstExprError") {
      let e = unsafe { &mut *(expr as *mut AstExprError) };
      self.visit_ast_expr_error(e)
    } else if class_index == ast_rtti_index("AstExprInstantiate") {
      let e = unsafe { &mut *(expr as *mut AstExprInstantiate) };
      unsafe { self.visit_ast_expr_instantiate(e) }
    } else {
      LUAU_ASSERT!(false);
      unsafe {
        (*self.ice).ice_string("NonStrictTypeChecker encountered an unknown expression type")
      };
      NonStrictContext::new()
    }
  }
}
