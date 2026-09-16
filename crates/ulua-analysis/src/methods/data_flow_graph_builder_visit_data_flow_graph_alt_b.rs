use ulua_ast::records::{
  ast_node::AstNode, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
  ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak, ast_stat_class::AstStatClass,
  ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_continue::AstStatContinue,
  ast_stat_declare_extern_type::AstStatDeclareExternType,
  ast_stat_declare_function::AstStatDeclareFunction, ast_stat_declare_global::AstStatDeclareGlobal,
  ast_stat_error::AstStatError, ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor,
  ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
  ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
  ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
  ast_stat_type_alias::AstStatTypeAlias, ast_stat_type_function::AstStatTypeFunction,
  ast_stat_while::AstStatWhile,
};
use ulua_common::{FFlag, LUAU_ASSERT};

use crate::{
  enums::control_flow::ControlFlow, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  pub fn visit_ast_stat(&mut self, s: *mut AstStat) -> ControlFlow {
    unsafe {
      let node = s as *mut AstNode;
      if (*node).is::<AstStatBlock>() {
        self.visit_ast_stat_block(s as *mut AstStatBlock)
      } else if (*node).is::<AstStatIf>() {
        self.visit_ast_stat_if(s as *mut AstStatIf)
      } else if (*node).is::<AstStatWhile>() {
        self.visit_ast_stat_while(s as *mut AstStatWhile)
      } else if (*node).is::<AstStatRepeat>() {
        self.visit_ast_stat_repeat(s as *mut AstStatRepeat)
      } else if (*node).is::<AstStatBreak>() {
        self.visit_ast_stat_break(s as *mut AstStatBreak)
      } else if (*node).is::<AstStatContinue>() {
        self.visit_ast_stat_continue(s as *mut AstStatContinue)
      } else if (*node).is::<AstStatReturn>() {
        self.visit_ast_stat_return(s as *mut AstStatReturn)
      } else if (*node).is::<AstStatExpr>() {
        self.visit_ast_stat_expr(s as *mut AstStatExpr)
      } else if (*node).is::<AstStatLocal>() {
        self.visit_ast_stat_local(s as *mut AstStatLocal)
      } else if (*node).is::<AstStatFor>() {
        self.visit_ast_stat_for(s as *mut AstStatFor)
      } else if (*node).is::<AstStatForIn>() {
        self.visit_ast_stat_for_in(s as *mut AstStatForIn)
      } else if (*node).is::<AstStatAssign>() {
        self.visit_ast_stat_assign(s as *mut AstStatAssign)
      } else if (*node).is::<AstStatCompoundAssign>() {
        self.visit_ast_stat_compound_assign(s as *mut AstStatCompoundAssign)
      } else if (*node).is::<AstStatFunction>() {
        self.visit_ast_stat_function(s as *mut AstStatFunction)
      } else if (*node).is::<AstStatLocalFunction>() {
        self.visit_ast_stat_local_function(s as *mut AstStatLocalFunction)
      } else if (*node).is::<AstStatTypeAlias>() {
        self.visit_ast_stat_type_alias(s as *mut AstStatTypeAlias)
      } else if (*node).is::<AstStatTypeFunction>() {
        self.visit_ast_stat_type_function(s as *mut AstStatTypeFunction)
      } else if (*node).is::<AstStatDeclareGlobal>() {
        self.visit_ast_stat_declare_global(s as *mut AstStatDeclareGlobal)
      } else if (*node).is::<AstStatDeclareFunction>() {
        self.visit_ast_stat_declare_function(s as *mut AstStatDeclareFunction)
      } else if (*node).is::<AstStatDeclareExternType>() {
        self.visit_ast_stat_declare_extern_type(s as *mut AstStatDeclareExternType)
      } else if (*node).is::<AstStatClass>() {
        LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());
        self.visit_ast_stat_class(s as *mut AstStatClass)
      } else if (*node).is::<AstStatError>() {
        self.visit_ast_stat_error(s as *mut AstStatError)
      } else {
        // InternalErrorReporter::ice is not yet translated; use panic as a fallback
        panic!("Unknown AstStat in DataFlowGraphBuilder::visit");
      }
    }
  }
}
