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
use ulua_common::LUAU_ASSERT;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  pub fn visit_ast_stat(&mut self, stat: *mut AstStat) {
    unsafe {
      let node = stat as *mut AstNode;
      let _pusher = self.type_checker_2_push_stack(node);
      if (*node).is::<AstStatBlock>() {
        self.visit_ast_stat_block(stat as *mut AstStatBlock);
      } else if (*node).is::<AstStatIf>() {
        self.visit_ast_stat_if(stat as *mut AstStatIf);
      } else if (*node).is::<AstStatWhile>() {
        self.visit_ast_stat_while(stat as *mut AstStatWhile);
      } else if (*node).is::<AstStatRepeat>() {
        self.visit_ast_stat_repeat(stat as *mut AstStatRepeat);
      } else if (*node).is::<AstStatBreak>() {
        self.visit_ast_stat_break(stat as *mut AstStatBreak);
      } else if (*node).is::<AstStatContinue>() {
        self.visit_ast_stat_continue(stat as *mut AstStatContinue);
      } else if (*node).is::<AstStatReturn>() {
        self.visit_ast_stat_return(stat as *mut AstStatReturn);
      } else if (*node).is::<AstStatExpr>() {
        self.visit_ast_stat_expr(stat as *mut AstStatExpr);
      } else if (*node).is::<AstStatLocal>() {
        self.visit_ast_stat_local(stat as *mut AstStatLocal);
      } else if (*node).is::<AstStatFor>() {
        self.visit_ast_stat_for(stat as *mut AstStatFor);
      } else if (*node).is::<AstStatForIn>() {
        // SAFETY: is::<AstStatForIn> 已判定动态类型；stat 指向 AST arena。
        self.visit_ast_stat_for_in(&*(stat as *const AstStatForIn));
      } else if (*node).is::<AstStatAssign>() {
        // SAFETY: is::<AstStatAssign> 已判定动态类型，cast 有效；stat 指向 AST arena。
        self.visit_ast_stat_assign(&*(stat as *const AstStatAssign));
      } else if (*node).is::<AstStatCompoundAssign>() {
        self.visit_ast_stat_compound_assign(stat as *mut AstStatCompoundAssign);
      } else if (*node).is::<AstStatFunction>() {
        self.visit_ast_stat_function(stat as *mut AstStatFunction);
      } else if (*node).is::<AstStatLocalFunction>() {
        self.visit_ast_stat_local_function(stat as *mut AstStatLocalFunction);
      } else if (*node).is::<AstStatTypeAlias>() {
        self.visit_ast_stat_type_alias(stat as *mut AstStatTypeAlias);
      } else if (*node).is::<AstStatTypeFunction>() {
        self.visit_ast_stat_type_function(stat as *mut AstStatTypeFunction);
      } else if (*node).is::<AstStatDeclareFunction>() {
        self.visit_ast_stat_declare_function(stat as *mut AstStatDeclareFunction);
      } else if (*node).is::<AstStatDeclareGlobal>() {
        self.visit_ast_stat_declare_global(stat as *mut AstStatDeclareGlobal);
      } else if (*node).is::<AstStatDeclareExternType>() {
        self.visit_ast_stat_declare_extern_type(stat as *mut AstStatDeclareExternType);
      } else if (*node).is::<AstStatClass>() {
        self.visit_ast_stat_class(stat as *mut AstStatClass);
      } else if (*node).is::<AstStatError>() {
        self.visit_ast_stat_error(stat as *mut AstStatError);
      } else {
        LUAU_ASSERT!(false, "TypeChecker2 encountered an unknown node type");
      }
    }
  }
}
