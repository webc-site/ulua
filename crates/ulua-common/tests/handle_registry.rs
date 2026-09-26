//! 迁移来源：`crates/ulua-common/src/records/handle_registry.rs` 的
//! `#[cfg(test)] mod tests`（句柄注册表 register/intern/intern_arc/resolve 语义）。
//!
//! 被测面全部为公开 API（`pub struct HandleRegistry` 与其 `pub` 方法），
//! 无需任何 `pub(crate)` 内部可见性，故整体迁出 src。

// 原 src 测试以 `alloc::` 路径导入（lib.rs 声明 `extern crate alloc`），
// 迁出后在 test crate 内显式声明，保持导入行逐字保真。
extern crate alloc;

use alloc::{boxed::Box, sync::Arc};

use ulua_common::records::handle_registry::HandleRegistry;

struct Node(u32);

#[test]
fn register_resolve_roundtrip_and_sentinel() {
  let mut reg = HandleRegistry::new();
  let a = Box::into_raw(Box::new(Node(7)));
  // 纯追加：id 从 1 起（0 为空哨兵，永不入库），单调不复用。
  assert_eq!(reg.register(a), 1);
  assert_eq!(reg.resolve_ptr(0), None);
  // SAFETY: a 为本测试 Box 保活的存活、对齐节点，单线程只读。
  assert!(unsafe { reg.resolve(1) }.is_some());
  assert_eq!(reg.resolve_ptr(2), None);
  let _ = unsafe { Box::from_raw(a) };
}

#[test]
fn intern_is_idempotent_per_address() {
  let mut reg = HandleRegistry::new();
  let a = Box::into_raw(Box::new(Node(1)));
  let b = Box::into_raw(Box::new(Node(2)));
  assert_eq!(reg.intern(a), 1);
  assert_eq!(reg.intern(a), 1);
  assert_eq!(reg.intern(b), 2);
  let _ = unsafe { Box::from_raw(a) };
  let _ = unsafe { Box::from_raw(b) };
}

#[test]
fn intern_arc_keeps_node_alive_after_host_drop() {
  let mut reg: HandleRegistry<Node> = HandleRegistry::new();
  let arc = Arc::new(Node(9));
  let id = reg.intern_arc(&arc);
  assert_eq!(id, 1);
  // 幂等：同一 Arc 堆块地址恒得首发句柄。
  assert_eq!(reg.intern_arc(&arc), 1);
  drop(arc);
  // 注册表持强引用保活：宿主句柄 drop 后仍可解析。
  assert_eq!(unsafe { reg.resolve(id) }.map(|n| n.0), Some(9));
  // SAFETY: id 唯一、单线程、该 Node 仅由注册表持有且此前无并存 `&mut`。
  unsafe { reg.resolve_mut(id) }.unwrap().0 = 10;
  assert_eq!(unsafe { reg.resolve(id) }.map(|n| n.0), Some(10));
}
