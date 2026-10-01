//! `*mut LuaState` → `&mut LuaState` 的 crate 内门面：本 crate 各 C ABI 入口
//! （`lua_CFunction` 回调 / review.md §9.3 登记的 `pub unsafe fn` 边界）都需把
//! 调用方持有的 VM 句柄物化为引用后再走 ulua-vm 的安全方法，原先散落在各入口的
//! `unsafe { &mut *l }` / `(*l).x()` 收拢为本文件一处，入口只留一次物化。
//!
//! # Safety（模块级契约，全部调用点共同依赖）
//!
//! 1. `l` 来自 `lua_pushcclosure` 注册回调等 C ABI 入口，非空且在调用期内活跃
//!    （各入口的 `# Safety` 契约同一前提）；
//! 2. 单线程驱动：任一物化借用的存活期内不存在其它并存的可变别名；
//! 3. null 哨兵不进入本门面：入口指针由 VM 交出，恒非空。

use ulua_vm::records::lua_state::LuaState;

/// 物化 C ABI 入口持有的 VM 状态句柄；借用期刻意不受约束，与原裸指针解引用
/// 的借用检查行为同构，契约见模块头。
pub(crate) fn state(l: *mut LuaState) -> &'static mut LuaState {
  // Safety: 调用点均为 C ABI 入口，`l` 非空、活跃；借用期内单线程无并存可变别名。
  unsafe { &mut *l }
}
