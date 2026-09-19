use crate::{
  records::{ast_array::AstArray, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatBlock {
  pub base: AstStat,
  pub body: AstArray<*mut AstStat>,
  pub has_end: bool,
}

impl AstNodeClass for AstStatBlock {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatBlock");
}
