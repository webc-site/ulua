//! `VisitKey` —— 分析层 visitor 已访问集合（seen-set）的身份键。
//!
//! 背景：C++ `GenericTypeVisitor<S>` 的 `seen` 集合以类型/类型包节点的
//! 指针地址为去重键（`hasSeen`/`unsee` 只比较地址，从不解引用），移植时
//! 落成散落的 `HashSet<*mut ()>` / `DenseHashSet<*mut ()>` 与各处
//! `x as *mut ()` 裸指针转换。本 newtype 把“指针地址即哈希/相等语义”这一
//! 既有契约收敛到单一受审计点，调用点退化为 `DenseHashSet<VisitKey>`，
//! 不再手写指针转换。
//!
//! 语义与原 `*mut ()` 逐字节等价：`Hash`/`PartialEq` 派生直接转发到内部
//! `*mut ()`，与 `HashSet<*mut ()>` 的元素哈希/比较完全一致，故遍历顺序、
//! 环检测与报错消息均不变。键仅携带地址身份，永不解引用。

use core::ptr::null_mut;

use ulua_common::records::dense_hash_table::DenseDefault;

/// 以节点指针地址为身份的 seen-set 键。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VisitKey(*mut ());

impl VisitKey {
  /// 由任意（只读）节点指针建立身份键。语义等同原 `ptr as *mut ()`：仅保留
  /// 地址身份，调用方持有的 arena 生命周期即本键的有效性前提（C++ 同契约）。
  #[inline]
  pub const fn from_ptr<T>(ptr: *const T) -> Self {
    Self(ptr as *mut ())
  }
}

impl DenseDefault for VisitKey {
  /// 空槽占位与 `*mut T` 的 [`DenseDefault`] 逐位一致（null）；位图判定下该
  /// 占位不参与命中比较，故 null 键同样可正常 insert/find/erase。
  fn dense_default() -> Self {
    Self(null_mut())
  }
}
