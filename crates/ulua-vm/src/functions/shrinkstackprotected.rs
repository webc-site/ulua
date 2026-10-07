//! Source: `VM/src/lgc.cpp` (lgc.cpp:485-498, hand-ported)

use core::{ffi::c_void, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_rawrunprotected_ldo::lua_d_rawrunprotected_mut, shrinkstack::shrinkstack},
  records::lua_state::LuaState,
};

/// # Safety
/// 仅作 lua_d_rawrunprotected_mut 的受保护回调：`l` 为该受保护状态本体；shrinkstack 内 realloc
/// 失败抛 ErrMem 由保护帧吞掉，禁止在受保护帧之外直接调用。对应 cpp lgc.cpp:525 `CallContext::run`
// C++ uses a local `struct CallContext { static void run(...) }`; a local fn is
// the Rust equivalent of that protected-call trampoline.
//
// r16-v25 收形边界：本跳板系 `Pfunc` 受保护回调，为真 FFI/ABI 边界（panic 须能 unwind 穿过），
// 签名保持裸形不收；仅体内把已收形的 `shrinkstack` 以一次 `&mut *l` 就地重建传入
// （借用窗止于当句），`(*l)` 非空前提由跳板契约给出。
unsafe extern "C-unwind" fn run(l: *mut LuaState, _ud: *mut c_void) {
  // SAFETY: 契约保证 `l` 存活；缩栈在受保护回调中执行，失败仅保持原栈大小
  unsafe {
    shrinkstack(&mut *l);
  }
}

/// # Safety
/// 调用方须保证：CallInfo 数组尚余至少一层供 rawrunprotected 开保护帧；本函数只在 GC
/// propagate 阶段对已灰化/黑化的存活线程调用，返回后栈可能因 ErrMem 保持原大小，调用方
/// 不得假设 stacksize 已缩小。cpp lgc.cpp:522 `shrinkstackprotected`
///
/// r16-v25 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载，原散文「`l` 存活且」
/// 名实随形只留调用序/受保护帧前提（判例：commit `9b448ba4`）；对仍收裸形的
/// `lua_d_rawrunprotected_mut` 转调采用一次 `l.as_mut_ptr()` 就地重建（借用窗止于当句，
/// 未跨调用持有重建出的引用），`run` 跳板与 `ud` 载荷面按既有长注原样保留。
pub(crate) unsafe fn shrinkstackprotected(l: &mut LuaState) {
  unsafe {
    // the resize call can fail on exception, in which case we will continue with original size
    // # Safety/保留理由：`ud` 不是出参而是 cpp `luaD_rawrunprotected(L, f, ud)` 的 ABI
    // 载荷，`null_mut()` 在此表达「本回调无额外载荷」——`run` 所需状态全部由首参 `l`
    // 携带（体内只 `shrinkstack(&mut *l)`），与 oracle `lgc.cpp:525 CallContext::run` 逐位同形。
    // 闭包化不可行：`Pfunc` 是无捕获的 `unsafe extern "C-unwind" fn(l, ud)`（panic 须
    // unwind 穿过它到 catch 边界），故消掉 `ud` 只能改 `Pfunc`/`lua_d_rawrunprotected(_mut)`
    // 签名，牵连 `lua_d_pcall`、`lua_newstate`、`resume_finish`、`lua_checkstack`、
    // `luau_load`、`stringresizeprotected`、`tableresizeprotected`、ulua-capi 导出壳与
    // vm_protected_call 集成测试等本票 3 文件之外的消费方，按 review.md §1.3 退回最小方案。
    let status = lua_d_rawrunprotected_mut(l.as_mut_ptr(), Some(run), null_mut());
    LUAU_ASSERT!(status == LuaStatus::Ok as i32 || status == LuaStatus::ErrMem as i32);
  }
}
