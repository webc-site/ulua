use crate::records::{ast_array::AstArray, cst_node::CstNode};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprConstantNumber {
  pub base: CstNode,
  pub value: AstArray<u8>,
}

impl_cst_node_class!(CstExprConstantNumber);
impl_cst_node_new!(CstExprConstantNumber, value: AstArray<u8>);
