//! 可空 arena 子节点槽位的读写门面（`review.md` §2「可空指针 → `Option`」的收口点）。
//!
//! cpp 侧 AST 的可空子节点（`AstTypeList::tailType`、`AstStatIf::elseBody`、
//! `AstLocal::annotation` 等）在头文件里就是裸指针 + `nullptr` 语义，且消费点散布
//! `ulua-analysis`（仅 `AstTypeList::tail_type` 一项就有 60 余处读写），故字段本体保留
//! `*mut T`；但**业务逻辑不再直接判空**：读侧用 [`node_opt`] 折成 `Option<NonNull<T>>`，
//! 写侧用 [`opt_node`] 把 `None` 落回 `nullptr`。两个函数都无 `unsafe`（`NonNull::new`
//! 自带判空、`NonNull::as_ptr` 只读地址），因此 null 哨兵在本 crate 里只剩
//! 「类型定义处 + 本门面」两处，不再渗透到 parser 分支逻辑。
//!
//! 之所以用 `NonNull` 而非 `&'static T`：这些槽位写出去之后 parser 仍会以裸指针改写节点
//! 字段（如 `AstStatBlock::hasEnd`），从 `&` 派生 `*mut` 会踩 noalias 证明，
//! `NonNull` 只承诺「非空 + 地址稳定」，与 bump arena 的实际契约一致。

use core::ptr::{NonNull, from_ref, null_mut};

/// 可空子节点字段 → `Option<NonNull<T>>`：cpp 读到 `nullptr` 的形态即 `None`。
///
/// 无需 `unsafe`：`NonNull::new` 只是「非空性」的运行期检查，不解引用。
#[inline]
pub fn node_opt<T>(node: *mut T) -> Option<NonNull<T>> {
  NonNull::new(node)
}

/// `Option<NonNull<T>>` → 可空子节点字段值：`None` 落为 cpp 的 `nullptr`。
///
/// 与 [`opt_ptr`] 同为 null 哨兵的写出点：[`opt_node`] 落**字段**，[`opt_ptr`] 落
/// **查询返回值**。`NonNull::as_ptr` 不构造引用，故与后续经由裸指针的节点改写不产生
/// 别名冲突。
#[inline]
pub fn opt_node<T>(node: Option<NonNull<T>>) -> *mut T {
  node.map_or(null_mut(), NonNull::as_ptr)
}

/// `Option<NonNull<T>>` 字段 → 只读共享引用：`None` 折叠为 `None`。
///
/// 契约（本模块唯一解引用点）：槽位里的 `NonNull` 恒出自 bump arena——页常驻
/// 直到解析会话结束、地址不移动，故读取到的节点在消费者生命周期内存活且只读
/// （遍历写穿请走裸指针句柄的 `as_mut` 路径，不经此门面）。
#[inline]
pub fn node_ref<T>(node: Option<NonNull<T>>) -> Option<&'static T> {
  // Safety: 前置条件即上方契约，由字段的全部写入点（arena 分配返回值）兑现。
  unsafe { node.map(|n| n.as_ref()) }
}

/// 可空槽位（`*mut`/`*const`）一步读成共享引用：null 折叠为 `None`（[`node_opt`] +
/// [`node_ref`] 的复合，业务侧不再出现 `unsafe { ptr.as_ref() }`）。
#[inline]
pub fn slot_opt<T>(node: *const T) -> Option<&'static T> {
  node_ref(NonNull::new(node.cast_mut()))
}

/// 非空槽位读成共享引用：槽位由 parser 以 arena 分配结果写入、恒非空
/// （cpp 同处直接解引用 null 即 UB，不可触发），违例时以断言语义 panic。
#[inline]
pub fn slot_ref<T>(node: *const T) -> &'static T {
  slot_opt(node).expect("parser 写入的 arena 子节点槽位恒非空（分配失败即中止解析）")
}

/// 只读查找结果 → cpp 边界的裸指针返回值：`None` 落为 `nullptr`。
///
/// 用于 `getAttribute` 家族：内部一律以 `Option<&T>` 表达命中与否，只在 pub 出口折回
/// cpp `AstAttr*` 形态（消费方散布 ulua-analysis / ulua-compiler）。由 `&` 派生 `*mut`
/// 仅透传地址：属性节点在 arena 内只读，写入方不存在，故不构成别名冲突。
#[inline]
pub fn opt_ptr<T>(node: Option<&T>) -> *mut T {
  node.map_or(null_mut(), |n| from_ref(n).cast_mut())
}
