//! `lua_newstate` 的分配器跨界失败语义逐臂直测（对照 cpp `VM/src/lstate.cpp:184`）。
//!
//! 覆盖 review.md §2「所有权转手 / Drop 守卫」在状态构造入口的最早期边界：
//! - 分配器缺省（`None`）：cpp `LUAU_ASSERT`/返回 null 契约——不得触碰任何内存，直接回 null；
//! - 分配器首块即失败（`LG` 请求返回 null）：`lua_newstate` 立即回 null，无部分状态可回收，
//!   故不存在「半成品被误释放」或泄漏（其 Miri 无泄漏性由主代理测试保证）。
//!
//! 被测面为公开 `pub unsafe fn lua_newstate`（VM 侧唯一宿主分配器跨界点），无私有项依赖。
//! 中途 `f_luaopen` 抛 ErrMem 的部分回收臂需要按未知分配序列注入定点失败，非确定性，
//! 按评审「宁可少改不可改错」不在此强行构造。

use core::{
  ffi::c_void,
  ptr::{from_mut, null_mut},
};

use ulua_vm::functions::lua_newstate::lua_newstate;

/// 恒失败的宿主分配器：任何请求（含 `ptr == null` 的纯分配）一律回 null。
///
/// # Safety
/// 满足 cpp `lua_Alloc` 契约的退化实现：从不返回有效块，调用方据此判定分配失败。
unsafe extern "C-unwind" fn always_fail(
  _ud: *mut c_void,
  _ptr: *mut u8,
  _osize: usize,
  _nsize: usize,
) -> *mut u8 {
  null_mut()
}

/// 臂 1：分配器缺省（`None`）→ 立即回 null，`ud` 全程不触碰。
#[test]
fn missing_allocator_returns_null() {
  let mut ud = 0u8;
  // Safety: `f == None` 分支不解引用 ud，传入可写局部地址仅为证明其未被使用。
  let l = unsafe { lua_newstate(None, from_mut(&mut ud).cast::<c_void>()) };
  assert!(l.is_null(), "缺省分配器须直接回报状态创建失败");
}

/// 臂 2：首块 `LG` 分配即失败 → 回 null，无任何部分状态需要回收（无泄漏、无双重释放）。
#[test]
fn first_block_failure_returns_null() {
  // Safety: 契约允许 `ud` 为任意透传指针，本分配器不解引用它。
  let l = unsafe { lua_newstate(Some(always_fail), null_mut()) };
  assert!(l.is_null(), "宿主分配器首块返回 null 时状态创建必须失败");
}
