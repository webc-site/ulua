//! 安全 RTTI 下转型辅助：把测试里成片的
//! `unsafe { rtti::ast_node_as::<T>(p) }` + `unsafe { &*p }` 收敛为单个安全调用。
//! SAFETY 前置条件集中封装于此，调用侧无需 unsafe。

use core::ptr::{NonNull, from_mut, from_ref};

use ulua_ast::{records::ast_node::AstNode, rtti::AstNodeClass};

/// `node->as<T>()` 的安全封装：裸指针 → `Option<&'a T>`（null 或类型不符得 None）。
///
/// # Safety（前置条件，调用侧免 unsafe）
/// `node` 必须为 null 或指向仍存活的 AST 节点（repr(C) 单继承，首字段链为
/// `AstNode`——即各 `Ast*` 节点结构）。测试场景成立：节点来自 fixture 的
/// arena 分配器，测试期内存活。
#[inline]
pub fn node_as<'a, T: AstNodeClass>(node: *mut AstNode) -> Option<&'a T> {
  let n = NonNull::new(node)?;
  // SAFETY: 见函数级前置条件；NonNull 非空且指向存活节点。
  let base = unsafe { n.as_ref() };
  // class_index 命中后 repr(C) 单继承布局保证 cast::<T> 指针与基址重合
  // （与 cpp `static_cast<T*>(this)` 同理）。
  if base.class_index == T::CLASS_INDEX {
    // SAFETY: 同上，命中分支内指针有效且布局匹配。
    Some(unsafe { &*(from_ref(base) as *const T) })
  } else {
    None
  }
}

/// [`node_as`] 的可变变体：裸指针 → `Option<&'a mut T>`。
///
/// # Safety（前置条件，调用侧免 unsafe）
/// 同 [`node_as`]，另需调用方保证同一节点的 `&mut` 生命周期互不重叠
/// （arena 独占所有权，逐个用完即弃，测试场景成立）。
#[inline]
pub fn node_as_mut<'a, T: AstNodeClass>(node: *mut AstNode) -> Option<&'a mut T> {
  let mut n = NonNull::new(node)?;
  // SAFETY: 见函数级前置条件；NonNull 非空且指向存活节点。
  let base = unsafe { n.as_mut() };
  if base.class_index == T::CLASS_INDEX {
    // SAFETY: 同上，命中分支内指针有效且布局匹配。
    Some(unsafe { &mut *(from_mut(base) as *mut T) })
  } else {
    None
  }
}
