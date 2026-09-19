use core::ptr::from_ref;

use crate::{
  records::{ast_node::AstNode, writer::Writer},
  rtti::{CstNodeClass, cst_node_as},
  type_aliases::cst_node_map::CstNodeMap,
};

/// `visualize_*` 系列入参归一：裸指针与共享引用统一取 `*const T`，镜像 C++
/// 按指针/引用重载传节点的调用形态。
///
/// 打印器对 AST 只读（写入只发生在 `Writer` 上），故这里只交出 const 指针：
/// 从共享引用造 `*mut`/`&mut` 再写穿节点正是本 crate 消除的别名 UB 形态。
pub trait IntoNodePtr<T> {
  fn into_node_ptr(self) -> *const T;
}

impl<T> IntoNodePtr<T> for *mut T {
  fn into_node_ptr(self) -> *const T {
    self.cast_const()
  }
}

impl<T> IntoNodePtr<T> for *const T {
  fn into_node_ptr(self) -> *const T {
    self
  }
}

impl<T> IntoNodePtr<T> for &T {
  fn into_node_ptr(self) -> *const T {
    self
  }
}

impl<T> IntoNodePtr<T> for &*mut T {
  fn into_node_ptr(self) -> *const T {
    (*self).cast_const()
  }
}

/// `W: Writer` 泛型替代 `&mut dyn Writer`：Writer 的实现者全部在本 crate 内，
/// 静态分发消除 visualize 热路径上逐 token 的虚调用（仍镜像 C++ 虚接口）。
pub struct Printer<'a, W: Writer> {
  pub(crate) write_types: bool,
  pub(crate) writer: &'a mut W,
  pub(crate) cst_node_map: &'a CstNodeMap,
}

impl<'a, W: Writer> Printer<'a, W> {
  /// 查 ast→cst 映射并下转为 `T`。unsafe 收口于此：映射值指向 arena 中存活的
  /// CST 节点，`cst_node_as` 经 class_index 命中后 repr(C) 布局保证下转有效。
  ///
  /// 生命周期取自入参借用 `'b`（不再借用 Printer 的 writer 生命周期 `'a`）：
  /// AST 与 CST 同处一个 arena，故「被查询的 AST 节点在 `'b` 内存活」即蕴含
  /// 其映射到的 CST 节点在 `'b` 内存活。键只按地址比较，从不解引用写回。
  pub(crate) fn lookup_cst_node<'b, T: CstNodeClass>(
    &self,
    ast_node: &'b AstNode,
  ) -> Option<&'b T> {
    // cpp 侧键型为 `AstNode*`；这里仅把地址重新拼回该形态用于查表。
    let key = from_ref(ast_node).cast_mut();
    let cst_node = *self.cst_node_map.find(&key)?;
    // SAFETY: 同函数级注释；CST 节点与 `ast_node` 同 arena，存活期覆盖 `'b`。
    unsafe { cst_node_as::<T>(cst_node).as_ref() }
  }
}
