//! JIT call inlining 第 2 阶段：CALL 站点观测插桩的 codegen 侧载体。
//!
//! 观测点两端形态（同一观测核，见 ulua-vm call_obs）：
//! - x64：`emit_inst_call` 无条件经 `call_prolog`，观测内嵌其中（incr_ci 前采集）；
//! - A64：CALL 在生成码内直通快路（守卫全过后 `br exectarget`，不经 call_prolog），
//!   快路守卫段与提交段之间插桩 `call_obs_site_hook`（NativeContext 槽间接调用）。
//!
//! 两处调用点契约一致：caller 帧（`l->ci` 未推进）+ 新帧未建——caller 闭包经
//! `l->ci->func` 可解，call pc 由 savedpc 反推（CALL 翻译的 SetSavedpc 先行于
//! 生成码），触发暖重编译的临时栈槽压入协议（见 trigger）在两侧同构。

use core::{
  ptr::copy_nonoverlapping,
  sync::atomic::{AtomicU32, Ordering},
};

use ulua_common::fflag;
use ulua_vm::{
  functions::call_obs::{call_obs_record_at, call_pc_of},
  records::{lua_state::LuaState, proto::Proto},
  type_aliases::t_value::TValue,
};

use crate::functions::{
  get_code_gen_context::get_code_gen_context, luau_codegen_compile::luau_codegen_warm_recompile,
};

/// 观测预算：每次观测恰扣一次的总次数上限。观测是热身期手段（站点恒定满阈值
/// 即 sealed、内联兑现后该站点 CALL 不再经过观测点），预算兜底「永不内联的
/// 站点」（如递归体）不无限付 blr——耗尽后置空 NativeContext.call_obs_hook，
/// 生成码侧回落为一次指针读 + 分支的常态成本。
const K_CALL_OBS_BUDGET: u32 = 1_000_000;

static CALL_OBS_BUDGET: AtomicU32 = AtomicU32::new(K_CALL_OBS_BUDGET);

/// A64 CALL 快路插桩入口（NativeContext.call_obs_hook 槽指向本函数）。
/// 快路守卫段已验函数 tag 与非 C 闭包，此处仅做 caller 帧形态防御。
///
/// # Safety
/// `l` 为存活 `LuaState` 且当前帧为 caller（快路提交段前）；`ra` 为快路守卫过的
/// 函数值栈槽。
pub unsafe extern "C-unwind" fn call_obs_site_hook(l: *mut LuaState, ra: *const TValue) {
  // Safety: 契约同上；观测核与触发协议内部仅读写活 state 的既有结构。
  unsafe {
    if !(*ra).is_function() {
      return;
    }
    let ci = (*l).ci;
    let savedpc = (*ci).savedpc;
    let func = (*ci).func;
    if savedpc.is_null() || !(*func).is_function() {
      return;
    }
    let fcl = (*func).as_closure_ptr();
    if (*fcl).is_c != 0 {
      return;
    }
    let caller = (*fcl).inner.l.p;
    if let Some(call_pc) = call_pc_of(caller, savedpc) {
      call_obs_record_maybe_recompile(l, caller, call_pc, ra);
    }
  }
}

/// x64 call_prolog 观测入口：caller/savedpc 已由调用点解出，直落观测核。
///
/// # Safety
/// 契约同 [`call_obs_site_hook`]；`caller`/`call_pc` 须与 `l->ci` 帧一致。
pub unsafe fn call_obs_record_maybe_recompile(
  l: *mut LuaState,
  caller: *mut Proto,
  call_pc: u32,
  ra: *const TValue,
) {
  if !fflag::LUAU_JIT_CALL_INLINE_OBS.get() {
    return;
  }
  // 预算：fetch_sub 到负即观测期结束；尽瞬间的这一次顺手把生成码可见的 hook
  // 槽置空（ecb.context 稳定），后续快路回落一次指针读 + 分支的常态短路。
  if CALL_OBS_BUDGET
    .try_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_sub(1))
    .is_err()
  {
    if CALL_OBS_BUDGET.load(Ordering::Relaxed) == 0 {
      if let Some(ctx) = unsafe { get_code_gen_context(l) } {
        ctx.context.call_obs_hook = None;
      }
    }
    return;
  }
  // Safety: 契约见模块注——调用点为 caller 帧；ra 为守卫过的函数值栈槽。
  unsafe {
    let ccl = (*ra).as_closure_ptr();
    if (*ccl).is_c != 0 {
      return;
    }
    if call_obs_record_at(caller, call_pc, ccl) {
      trigger_warm_recompile(l);
    }
  }
}

/// 满阈值的同步暖重编译：把 caller 闭包临时压到 caller 帧 `ci->top`（其上到
/// stack_last 是子帧空闲区；观测点的 `l->top` 是上一指令遗留值，不可作压栈
/// 基准），`l->top` 临时指向该槽供编译入口以 -1 索引读取（模仿 rt
/// `Function::warm_recompile` 的协议），编译后恢复，栈净变化为零。
///
/// 安全性：调用点栈已定形（call_prolog 在 checkstackfornewci 前、A64 快路在
/// 守卫全过后），且两处后续都不再消费旧栈指针；caller 在途 native 帧继续执行
/// 旧代码直至自然退出（旧 execdata 泄漏不释放，指针稳定），重入走新 exectarget；
/// 被测 callee 由 ra 槽锚定，编译窗口内 proto 必存活。
///
/// # Safety
/// `l` 存活且 `l->ci` 为 caller 帧（观测点契约：call_prolog 在 incr_ci 前、
/// A64 快路在提交段前）。
unsafe fn trigger_warm_recompile(l: *mut LuaState) {
  // Safety: 契约见上；临时槽写入落在 ci->top（界内），压入即被栈扫描 rooted；
  // ci->top == stack_last 的满栈场景放弃本次触发（性能损失，非正确性）。
  unsafe {
    let ci = (*l).ci;
    let func = (*ci).func;
    let slot = (*ci).top;
    if slot >= (*l).stack_last {
      return;
    }
    let saved_top = (*l).top;
    copy_nonoverlapping(func, slot, 1);
    (*l).top = slot.add(1);
    luau_codegen_warm_recompile(l, -1);
    (*l).top = saved_top;
  }
}
