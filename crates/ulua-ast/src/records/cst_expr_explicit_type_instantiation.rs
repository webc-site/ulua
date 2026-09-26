use crate::records::{cst_node::CstNode, cst_type_instantiation::CstTypeInstantiation};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprExplicitTypeInstantiation {
  pub base: CstNode,
  pub instantiation: CstTypeInstantiation,
}

impl_cst_node_class!(CstExprExplicitTypeInstantiation);
impl_cst_node_new!(CstExprExplicitTypeInstantiation, instantiation: CstTypeInstantiation);
