//! `*mut LuaState` → `&mut LuaState` 的 crate 内门面：本 crate 各 VM 边界入口
//! （`lua_CFunction` / interrupt / coverage 回调，及 require 宿主、REPL 驱动等
//! review.md §9.3 登记的 `pub unsafe fn` 边界）都需把 VM 交出的句柄物化为引用
//! 后再走 ulua-vm 的安全方法，原先散落各处的 `unsafe { &mut *l }` / `(*l).x()`
//! 收拢为本文件一处，入口只留一次物化。
//!
//! # Safety（模块级契约，全部调用点共同依赖）
//!
//! 1. `l` 为 VM 交出或刚创建的句柄（C ABI 回调形参、`lua_newthread` 新线程、
//!    守卫持有的主状态等），恒非空且在调用/采样窗口内活跃；
//! 2. REPL/CLI 为单线程驱动（采样线程只触原子面），物化借用的存活期内不存在
//!    其它并存的可变别名；
//! 3. null 哨兵不进入本门面：可空处一律在调用点先经判空/`Option` 收口。

use ulua_vm::records::lua_state::LuaState;

/// 物化 VM 边界入口持有的状态句柄；借用期刻意不受约束，与原裸指针解引用的
/// 借用检查行为同构，契约见模块头。
pub(crate) fn state(l: *mut LuaState) -> &'static mut LuaState {
  // Safety: 调用点均为 VM 边界入口，`l` 非空、窗口内活跃；借用期内单线程无并存可变别名。
  unsafe { &mut *l }
}
