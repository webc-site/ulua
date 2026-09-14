#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprExplicitTypeInstantiation {
  pub base: CstNode,
  pub instantiation: CstTypeInstantiation,
}

impl CstNodeClass for CstExprExplicitTypeInstantiation {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprExplicitTypeInstantiation");
}
use crate::{
  records::{cst_node::CstNode, cst_type_instantiation::CstTypeInstantiation},
  rtti::{CstNodeClass, ast_rtti_index},
};
