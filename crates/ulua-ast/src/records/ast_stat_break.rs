use crate::{
  records::ast_stat::AstStat,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatBreak {
  pub base: AstStat,
}

impl AstNodeClass for AstStatBreak {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatBreak");
}
