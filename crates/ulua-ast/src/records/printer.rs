use core::ptr::null_mut;

use crate::{
  records::{ast_node::AstNode, writer::Writer},
  rtti::{CstNodeClass, cst_node_as},
  type_aliases::cst_node_map::CstNodeMap,
};

pub struct Printer<'a> {
  pub(crate) write_types: bool,
  pub(crate) writer: &'a mut dyn Writer,
  pub(crate) cst_node_map: CstNodeMap,
}

impl<'a> Printer<'a> {
  pub(crate) fn lookup_cst_node<T: CstNodeClass>(&self, ast_node: *mut AstNode) -> *mut T {
    if let Some(&cst_node) = self.cst_node_map.find(&ast_node) {
      return unsafe { cst_node_as::<T>(cst_node) };
    }
    null_mut()
  }
}
