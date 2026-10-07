//! Source: `VM/src/ldebug.cpp:185`
//!
//! `lua_getinfo` — resolve a stack `level` (negative = relative to top, else a
//! call-info depth) to its closure, fill `ar` via `auxgetinfo`, and (when the
//! `f` option pushed the function) place it on the stack. Returns 1 if a
//! function was found at that level, else 0.

use core::ptr::null_mut;

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{auxgetinfo::auxgetinfo, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{incr_top::incr_top, setclvalue::setclvalue},
  records::{call_info::CallInfo, closure::Closure, lua_debug::LuaDebug, lua_state::LuaState},
};

/// # Safety
/// `l` 必须指向存活 `LuaState` 且其栈/调用帧深度覆盖 `level`（对 top/base_ci 做裸 offset 游走）；`what` 为
/// 选项模板字节切片（逐字节消费，无 NUL 语义，如 `b"sln"`）；`ar` 须为存活可写 LuaDebug（auxgetinfo 直填字段）；
/// `f` 选项命中时会向 `l` 压栈一个闭包，调用方须预留栈顶余量。对应 cpp ldebug.cpp:185。
pub unsafe fn lua_getinfo(l: *mut LuaState, level: i32, what: &[u8], ar: *mut LuaDebug) -> i32 {
  // SAFETY: 契约保证 `L` 调用栈深度覆盖 level 且 `ar` 可写；分支检查后 auxgetinfo 前置成立，出错路径不泄漏栈槽
  unsafe {
    // 既有约定（review.md §2）：VM c-API 边界签名折返——`f`/`ci` 为贯穿 auxgetinfo/getluaproto 调用链的局部裸指针哨兵，边界体内保留，勿改 Option
    let mut f: *mut Closure = null_mut();
    let mut ci: *mut CallInfo = null_mut();

    if level < 0 {
      // element has to be within stack
      // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
      // 即被替代式 `top.offset_from(base)` 的镜像（现读位点不变）。比较两侧同折 i32：
      // 槽距受 LUAI_MAXSTACK 约束、远小于 i32::MAX 无截差，`-level` 本为 i32 负数取反
      // （求值形不变），isize 式与 i32 式在现域逐值同真值、比较方向不变
      if (-level) > (*l).get_top() {
        return 0;
      }

      // 顶下 level 格槽地址经 `top_slot` 读数原语取得（上方 `-level <= top-base` 界检已过）
      let func = (*l).top_slot(level as isize);

      // and it has to be a function
      if !(*func).is_function() {
        return 0;
      }

      f = (*func).as_closure_ptr();
    } else if (level as u32) < (*l).ci.offset_from((*l).base_ci) as u32 {
      ci = (*l).ci.offset(-(level as isize));
      LUAU_ASSERT!((*(*ci).func).is_function());
      f = (*(*ci).func).as_closure_ptr();
    }

    if !f.is_null() {
      // auxgetinfo fills ar and optionally requests to put closure on stack
      let fcl = auxgetinfo(what, ar, f, ci);
      if !fcl.is_null() {
        lua_c_threadbarrier_lapi(l);
        setclvalue!(l, (*l).top, fcl);
        incr_top!(l);
      }
    }

    if f.is_null() { 0 } else { 1 }
  }
}
