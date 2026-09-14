use core::ptr::null_mut;

use crate::{
  records::{ast_node::AstNode, writer::Writer},
  rtti::{CstNodeClass, cst_node_as},
  type_aliases::cst_node_map::CstNodeMap,
};

/// `W: Writer` 泛型替代 `&mut dyn Writer`：Writer 的实现者全部在本 crate 内，
/// 静态分发消除 visualize 热路径上逐 token 的虚调用（仍镜像 C++ 虚接口）。
pub struct Printer<'a, W: Writer> {
  pub(crate) write_types: bool,
  pub(crate) writer: &'a mut W,
  pub(crate) cst_node_map: CstNodeMap,
}

impl<'a, W: Writer> Printer<'a, W> {
  pub(crate) fn lookup_cst_node<T: CstNodeClass>(&self, ast_node: *mut AstNode) -> *mut T {
    if let Some(&cst_node) = self.cst_node_map.find(&ast_node) {
      return unsafe { cst_node_as::<T>(cst_node) };
    }
    null_mut()
  }
}
