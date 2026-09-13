use crate::{
  records::ast_stat::AstStat,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatContinue {
  pub base: AstStat,
}

impl AstNodeClass for AstStatContinue {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatContinue");
}
