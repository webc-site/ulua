use crate::records::{ast_array::AstArray, cst_node::CstNode};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprConstantInteger {
  pub base: CstNode,
  pub value: AstArray<u8>,
}

impl_cst_node_class!(CstExprConstantInteger);
impl_cst_node_new!(CstExprConstantInteger, value: AstArray<u8>);
