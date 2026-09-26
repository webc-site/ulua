//! require 配置 C 回调的 `ctx` 裸指针 → 引用重建公共收口。
//!
//! cpp 侧 11 个 `static` 回调各自手写 `&*(ctx as *const ReplRequirer)` /
//! `&mut *(ctx as *mut ReplRequirer)`；Rust 侧收敛为本模块两函数，生命周期
//! 契约（userdata 存活、单线程串行独占）只论证一次。

use core::ffi::c_void;

use crate::records::repl_requirer::ReplRequirer;

/// 由 `ctx` 重建 `ReplRequirer` 共享引用（只读回调：get_config / get_cache_key /
/// get_chunkname / get_loadname / get_config_status / is_module_present / load）。
///
/// # Safety
/// `ctx` 必须是 `create_cli_require_context` 经 `lua_newuserdatadtor` 分配、以
/// 地址为键登记进 registry 的 `ReplRequirer` userdata 指针（块地址稳定、与 state
/// 同生命周期）；本回调处于 require 导航单线程串行的同步调用窗口，对象存活且
/// 无其它借用者。
pub(crate) unsafe fn requirer<'a>(ctx: *mut c_void) -> &'a ReplRequirer {
  // Safety: 契约保证 ctx 指向存活 ReplRequirer 且无别名写者，只读重建成立。
  unsafe { &*ctx.cast::<ReplRequirer>() }
}

/// 由 `ctx` 重建 `ReplRequirer` 独占引用（会改 vfs 的导航回调：to_parent /
/// to_child / jump_to_alias / reset）。
///
/// # Safety
/// 同 [`requirer`]，且本调用窗口内无人持有该对象的任何存活借用（导航器阻塞
/// REPL 主体、串行独占调用），可变重建无别名冲突。
pub(crate) unsafe fn requirer_mut<'a>(ctx: *mut c_void) -> &'a mut ReplRequirer {
  // Safety: 契约保证 ctx 指向存活 ReplRequirer 且独占，&mut 重建成立。
  unsafe { &mut *ctx.cast::<ReplRequirer>() }
}
