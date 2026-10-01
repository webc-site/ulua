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
  // a 为本测试 Box 保活的存活、对齐节点，单线程只读（resolve 现为安全 API）。
  assert!(reg.resolve(1).is_some());
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
  assert_eq!(reg.resolve(id).map(|n| n.0), Some(9));
  // id 唯一、单线程、该 Node 仅由注册表持有且此前无并存 `&mut`。
  reg.resolve_mut(id).unwrap().0 = 10;
  assert_eq!(reg.resolve(id).map(|n| n.0), Some(10));
}

/// `register` 纯追加路径的 `resolve` 断节点**值**（不止既有 `is_some`）：句柄
/// 解析回来的正是当初登记的那块节点内容，且 `resolve_ptr` 抄回的地址即入参。
#[test]
fn register_resolve_returns_the_stored_node_value() {
  let mut reg: HandleRegistry<Node> = HandleRegistry::new();
  let node: *const Node = Box::into_raw(Box::new(Node(123)));
  let id = reg.register(node);
  assert_eq!(id, 1, "id 从 1 起");
  // 解析值而非仅存在性：句柄 1 还原出 Node(123)。
  assert_eq!(reg.resolve(id).map(|n| n.0), Some(123));
  // 地址视图：resolve_ptr 抄回原登记地址，且与值视图同址。
  assert_eq!(reg.resolve_ptr(id), Some(node));
  let _ = unsafe { Box::from_raw(node as *mut Node) };
}

/// 契约 1「同址 ⇔ 同句柄」在 `intern` 与 `intern_arc` 两个入口之间的互操作往返：
/// 同一地址经任一入口恒得同一首发句柄（反查表共享），且异址必发不同句柄。
#[test]
fn intern_and_intern_arc_agree_on_address_handle_bijection() {
  let mut reg: HandleRegistry<Node> = HandleRegistry::new();
  let arc = Arc::new(Node(77));
  let addr: *const Node = Arc::as_ptr(&arc);

  // 同址、跨入口：intern_arc 首发登记（Shared 槽），再 intern 同一地址经反查表
  // 回到同一句柄，绝不重复追加节点。
  let id_arc = reg.intern_arc(&arc);
  assert_eq!(id_arc, 1);
  assert_eq!(
    reg.intern(addr),
    id_arc,
    "intern 与 intern_arc 对同一地址必须互操作到同一句柄"
  );
  assert_eq!(
    reg.intern_arc(&arc),
    id_arc,
    "反向：再 intern_arc 仍首发句柄"
  );
  // 解析视图自该句柄还原出 Arc 保活的节点值与地址。
  assert_eq!(reg.resolve(id_arc).map(|n| n.0), Some(77));
  assert_eq!(reg.resolve_ptr(id_arc), Some(addr));

  // 异址 ⇔ 异句柄：另一独立对象经 intern 拿到新句柄，不与上面的坍缩。
  let other: *const Node = Box::into_raw(Box::new(Node(88)));
  let id_other = reg.intern(other);
  assert_ne!(id_other, id_arc, "异址必发不同句柄");
  assert_eq!(reg.resolve(id_other).map(|n| n.0), Some(88));
  assert_eq!(reg.resolve_ptr(id_other), Some(other));
  let _ = unsafe { Box::from_raw(other as *mut Node) };
}
