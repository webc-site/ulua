use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak, ast_stat_continue::AstStatContinue, ast_stat_if::AstStatIf,
    ast_stat_return::AstStatReturn,
  },
  rtti::{ast_node_as, ast_node_is},
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{is_constant_false::is_constant_false, is_constant_true::is_constant_true},
  records::constant::Constant,
};

pub fn always_terminates(
  constants: &DenseHashMap<*mut AstExpr, Constant>,
  node: *mut AstStat,
) -> bool {
  unsafe {
    let stat_block = ast_node_as::<AstStatBlock>(node as *mut AstNode);
    if !stat_block.is_null() {
      let body = (*stat_block).body;
      for &item in body.iter() {
        if always_terminates(constants, item) {
          return true;
        }
      }
      return false;
    }

    if ast_node_is::<AstStatReturn>(&*(node as *mut AstNode)) {
      return true;
    }

    if ast_node_is::<AstStatBreak>(&*(node as *mut AstNode))
      || ast_node_is::<AstStatContinue>(&*(node as *mut AstNode))
    {
      return true;
    }

    let stat_if = ast_node_as::<AstStatIf>(node as *mut AstNode);
    if !stat_if.is_null() {
      let condition = (*stat_if).condition;
      let thenbody = (*stat_if).thenbody;
      let elsebody = (*stat_if).elsebody;

      if is_constant_true(constants, condition) {
        return always_terminates(constants, thenbody as *mut AstStat);
      }

      if is_constant_false(constants, condition) && !elsebody.is_null() {
        return always_terminates(constants, elsebody);
      }

      return !elsebody.is_null()
        && always_terminates(constants, thenbody as *mut AstStat)
        && always_terminates(constants, elsebody);
    }

    false
  }
}
