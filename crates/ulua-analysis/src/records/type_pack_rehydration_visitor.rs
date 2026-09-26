use ulua_ast::records::allocator::Allocator;

use crate::{
  records::{arena_handle::alias, type_rehydration_visitor::TypeRehydrationVisitor},
  type_aliases::synthetic_names::SyntheticNames,
};

#[derive(Debug, Clone)]
pub struct TypePackRehydrationVisitor {
  pub(crate) allocator: *mut Allocator,
  pub(crate) synthetic_names: *mut SyntheticNames,
  pub(crate) type_visitor: *mut TypeRehydrationVisitor,
}

impl TypePackRehydrationVisitor {
  /// `self.allocator` 的统一读写入口（同 `TypeRehydrationVisitor::allocator_mut`）：
  /// 构造期存入、SourceModule 拥有的 arena 指针比本 visitor 长寿，`alias` 收口
  /// C++ 式独占借用（单线程串行、无并存别名），unsafe 不外渗；接收者 `&self` 与原
  /// `unsafe { &mut *self.allocator }` 写穿惯用法同构。
  #[inline]
  pub(crate) fn allocator_mut(&self) -> &'static mut Allocator {
    alias(self.allocator)
  }
}
