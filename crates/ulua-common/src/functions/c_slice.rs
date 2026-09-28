//! C「指针 + 计数」惯用法的切片封装：`ulua-vm` 原有私有 helper 提升为跨
//! crate 共享，收敛各处的 `is_null() || len == 0` 手工守卫。
use core::slice::{from_raw_parts, from_raw_parts_mut};

/// # Safety
/// 调用方保证 `p` 对 `len` 个 `T` 有效可读（正确对齐、不越界），且数据在返回值
/// 生命周期 `'a` 内持续有效；C 允许 NULL 配 0 长度（循环体不执行），此约定下
/// 返回空切片。
pub unsafe fn c_slice<'a, T>(p: *const T, len: usize) -> &'a [T] {
  if p.is_null() || len == 0 {
    &[]
  } else {
    // Safety: 非空分支，函数前置条件保证 `p` 对 `len` 个 `T` 有效可读、对齐且存活，
    // 且 `p` 非空、`len`>0，满足 from_raw_parts 的非空/对齐/不越界要求。
    unsafe { from_raw_parts(p, len) }
  }
}

/// 同 [`c_slice`]，可写版本。
/// # Safety
/// 调用方保证 `p` 对 `len` 个 `T` 有效可读写（正确对齐、不越界），在返回值生命周期
/// `'a` 内无其它别名且数据持续有效；NULL 配 0 长度同上，返回空切片。
pub unsafe fn c_slice_mut<'a, T>(p: *mut T, len: usize) -> &'a mut [T] {
  if p.is_null() || len == 0 {
    &mut []
  } else {
    // Safety: 非空分支，函数前置条件保证 `p` 对 `len` 个 `T` 有效可读写、对齐且存活，且无其它别名，
    // 且 `p` 非空、`len`>0，满足 from_raw_parts_mut 的非空/对齐/不越界与独占可变借用要求。
    unsafe { from_raw_parts_mut(p, len) }
  }
}
