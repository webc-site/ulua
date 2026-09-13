use core::{ffi::c_void, ptr::null};

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

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, def::Def,
  symbol::Symbol,
};
impl DataFlowGraphBuilder {
  pub fn visit_expr_ast_expr(&mut self, e: *mut AstExpr) -> DataFlowResult {
    unsafe {
      if let Some(def) = self.graph.ast_defs.find(&(e as *const AstExpr)) {
        let key = self.graph.ast_refinement_keys.find(&(e as *const AstExpr));
        return DataFlowResult {
          def: *def as *const c_void,
          parent: if let Some(k) = key { *k } else { null() },
        };
      }

      let result = {
        let node = e as *mut AstNode;
        if (*node).is::<AstExprGroup>() {
          self.visit_expr_ast_expr_group(e as *mut AstExprGroup)
        } else if (*node).is::<AstExprConstantNil>()
          || (*node).is::<AstExprConstantBool>()
          || (*node).is::<AstExprConstantNumber>()
          || (*node).is::<AstExprConstantInteger>()
          || (*node).is::<AstExprConstantString>()
          || (*node).is::<AstExprVarargs>()
        {
          DataFlowResult {
            def: (*self.def_arena).fresh_cell(Symbol::default(), (*node).location, false)
              as *const c_void,
            parent: null(),
          }
        } else if (*node).is::<AstExprLocal>() {
          self.visit_expr_ast_expr_local(e as *mut AstExprLocal)
        } else if (*node).is::<AstExprGlobal>() {
          self.visit_expr_ast_expr_global(e as *mut AstExprGlobal)
        } else if (*node).is::<AstExprCall>() {
          self.visit_expr_ast_expr_call(e as *mut AstExprCall)
        } else if (*node).is::<AstExprIndexName>() {
          self.visit_expr_ast_expr_index_name(e as *mut AstExprIndexName)
        } else if (*node).is::<AstExprIndexExpr>() {
          self.visit_expr_ast_expr_index_expr(e as *mut AstExprIndexExpr)
        } else if (*node).is::<AstExprFunction>() {
          self.visit_expr_ast_expr_function(e as *mut AstExprFunction)
        } else if (*node).is::<AstExprTable>() {
          self.visit_expr_ast_expr_table(e as *mut AstExprTable)
        } else if (*node).is::<AstExprUnary>() {
          self.visit_expr_ast_expr_unary(e as *mut AstExprUnary)
        } else if (*node).is::<AstExprBinary>() {
          self.visit_expr_ast_expr_binary(e as *mut AstExprBinary)
        } else if (*node).is::<AstExprTypeAssertion>() {
          self.visit_expr_ast_expr_type_assertion(e as *mut AstExprTypeAssertion)
        } else if (*node).is::<AstExprIfElse>() {
          self.visit_expr_ast_expr_if_else(e as *mut AstExprIfElse)
        } else if (*node).is::<AstExprInterpString>() {
          self.visit_expr_ast_expr_interp_string(e as *mut AstExprInterpString)
        } else if (*node).is::<AstExprInstantiate>() {
          self.visit_expr_ast_expr_instantiate(e as *mut AstExprInstantiate)
        } else if (*node).is::<AstExprError>() {
          self.visit_expr_ast_expr_error(e as *mut AstExprError)
        } else {
          (*self.handle).ice_string("Unknown AstExpr in DataFlowGraphBuilder::visitExpr");
          DataFlowResult::default()
        }
      };

      *self.graph.ast_defs.get_or_insert(e as *const AstExpr) = result.def as *const Def;
      if !result.parent.is_null() {
        *self
          .graph
          .ast_refinement_keys
          .get_or_insert(e as *const AstExpr) = result.parent;
      }

      result
    }
  }
}
