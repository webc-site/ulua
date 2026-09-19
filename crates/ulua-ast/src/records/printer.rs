use crate::{
  records::{ast_node::AstNode, writer::Writer},
  rtti::{CstNodeClass, cst_node_as},
  type_aliases::cst_node_map::CstNodeMap,
};

/// `visualize_*` 系列入参归一：裸指针、`&*mut` 重引用与可变引用统一取
/// `*mut T`，镜像 C++ 按指针/引用重载传节点的调用形态。
pub trait IntoNodePtr<T> {
  fn into_node_ptr(self) -> *mut T;
}

impl<T> IntoNodePtr<T> for *mut T {
  fn into_node_ptr(self) -> *mut T {
    self
  }
}

impl<T> IntoNodePtr<T> for &mut T {
  fn into_node_ptr(self) -> *mut T {
    self
  }
}

impl<T> IntoNodePtr<T> for &*mut T {
  fn into_node_ptr(self) -> *mut T {
    *self
  }
}

/// `W: Writer` 泛型替代 `&mut dyn Writer`：Writer 的实现者全部在本 crate 内，
/// 静态分发消除 visualize 热路径上逐 token 的虚调用（仍镜像 C++ 虚接口）。
pub struct Printer<'a, W: Writer> {
  pub(crate) write_types: bool,
  pub(crate) writer: &'a mut W,
  pub(crate) cst_node_map: CstNodeMap,
}

impl<'a, W: Writer> Printer<'a, W> {
  /// 查 ast→cst 映射并下转为 `T`。unsafe 收口于此：映射值指向 arena 中存活的
  /// CST 节点，`cst_node_as` 经 class_index 命中后 repr(C) 布局保证下转有效；
  /// 调用侧拿到 `Option<&T>`，判空与字段访问全部走安全代码。
  pub(crate) fn lookup_cst_node<T: CstNodeClass>(&self, ast_node: *mut AstNode) -> Option<&'a T> {
    let cst_node = *self.cst_node_map.find(&ast_node)?;
    // SAFETY: 同函数级注释；引用生命周期取 Printer 的 'a，CST 节点存活期覆盖打印会话。
    unsafe { cst_node_as::<T>(cst_node).as_ref() }
  }
}
