use core::ptr::null_mut;

use crate::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak, ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
  },
  rtti::AstNodeClass,
};

impl AstNode {
  #[inline]
  pub fn as_stat(&mut self) -> *mut AstStat {
    let is_stat = matches!(
      self.class_index,
      AstStatAssign::CLASS_INDEX
        | AstStatBlock::CLASS_INDEX
        | AstStatBreak::CLASS_INDEX
        | AstStatClass::CLASS_INDEX
        | AstStatCompoundAssign::CLASS_INDEX
        | AstStatContinue::CLASS_INDEX
        | AstStatDeclareExternType::CLASS_INDEX
        | AstStatDeclareFunction::CLASS_INDEX
        | AstStatDeclareGlobal::CLASS_INDEX
        | AstStatError::CLASS_INDEX
        | AstStatExpr::CLASS_INDEX
        | AstStatFor::CLASS_INDEX
        | AstStatForIn::CLASS_INDEX
        | AstStatFunction::CLASS_INDEX
        | AstStatIf::CLASS_INDEX
        | AstStatLocal::CLASS_INDEX
        | AstStatLocalFunction::CLASS_INDEX
        | AstStatRepeat::CLASS_INDEX
        | AstStatReturn::CLASS_INDEX
        | AstStatTypeAlias::CLASS_INDEX
        | AstStatTypeFunction::CLASS_INDEX
        | AstStatWhile::CLASS_INDEX
    );

    if is_stat {
      self as *mut AstNode as *mut AstStat
    } else {
      null_mut()
    }
  }

  #[inline]
  pub fn as_stat_const(&self) -> *const AstStat {
    let node = self as *const AstNode as *mut AstNode;
    unsafe { (*node).as_stat() as *const AstStat }
  }
}
