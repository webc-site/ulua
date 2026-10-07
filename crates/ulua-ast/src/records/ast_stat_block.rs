use crate::records::{ast_stat::AstStat, node_handle::Nodes};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatBlock {
  pub base: AstStat,
  pub body: Nodes<AstStat>,
  pub has_end: bool,
}
