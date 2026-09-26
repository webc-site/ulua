//! 调试期字节码校验链（`end_function → validate → {instructions, variadic, captures}`）
//! 的可达性回归。
//!
//! 本仓库的 `LUAU_ASSERT` 在处理器返回后无条件 `LUAU_DEBUGBREAK`，测试若直接让
//! 断言失败会 trap 掉进程；这里安装一个「计数 + panic」的 `extern "C-unwind"`
//! 处理器，把断言触发变成可捕获的 unwind——panic 仅用于**观测**断言确实执行，
//! 不掩盖任何失败（正常路径必须零触发才算通过）。
//!
//! 断言处理器是**进程级全局槽位**，这正是本文件必须是独立 test binary 的理由：
//! 放在 `src` 的单元测试里会和同进程内其它依赖该槽位的测试相互踩计数。

use core::{
  ffi::c_char,
  sync::atomic::{AtomicI32, Ordering},
};
use std::{
  panic::{AssertUnwindSafe, catch_unwind},
  sync::{Mutex, MutexGuard},
};

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  functions::assert_handler::{assert_handler, set_assert_handler},
  macros::luau_assertenabled::LUAU_ASSERTENABLED,
  type_aliases::assert_handler::AssertHandler,
};

static FIRED: AtomicI32 = AtomicI32::new(0);
/// 处理器是进程级全局槽位，串行化依赖它的测试，避免并行互相踩计数。
static HANDLER_LOCK: Mutex<()> = Mutex::new(());

extern "C-unwind" fn counting_handler(
  _expression: *const c_char,
  _file: *const c_char,
  _line: i32,
  _function: *const c_char,
) -> i32 {
  FIRED.fetch_add(1, Ordering::SeqCst);
  panic!("LUAU_ASSERT fired (captured by validate-chain test)");
}

/// 安装计数处理器并执行 `run`，返回断言触发次数。
/// release（`LUAU_ASSERTENABLED == false`）下 validate 短路，本套测试整体跳过。
fn fired_count(run: impl FnOnce()) -> i32 {
  if !LUAU_ASSERTENABLED {
    return 0;
  }
  let _guard: MutexGuard<'_, ()> = HANDLER_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let old: AssertHandler = assert_handler();
  FIRED.store(0, Ordering::SeqCst);
  set_assert_handler(Some(counting_handler));
  let _ = catch_unwind(AssertUnwindSafe(run));
  set_assert_handler(old);
  FIRED.load(Ordering::SeqCst)
}

/// 合法 IDIV/IDIVK 必须被指令校验接受：若算术分类表缺这两个 opcode，
/// `default => LUAU_ASSERT(false, "Unsupported opcode")` 会计数 +1。
#[test]
fn validate_accepts_idiv_and_idivk() {
  if !LUAU_ASSERTENABLED {
    return;
  }
  let fired = fired_count(|| {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.begin_function(0, false);
    bcb.add_constant_number(2.0);
    bcb.emit_ad(LuauOpcode::LOP_LOADNIL, 0, 0);
    bcb.emit_abc(LuauOpcode::LOP_IDIV, 0, 0, 0);
    bcb.emit_abc(LuauOpcode::LOP_IDIVK, 0, 0, 0);
    bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
    bcb.end_function(1, 0, 0, 0);
  });
  assert_eq!(fired, 0, "合法 IDIV/IDIVK 不得触发任何断言");
}

/// MOVE 引用越界寄存器必须触发 `validate_instructions` 的 VREG，
/// 证明 `end_function` 确实在编码前调用了 `validate`。
#[test]
fn validate_chain_fires_on_out_of_range_register() {
  if !LUAU_ASSERTENABLED {
    return;
  }
  let fired = fired_count(|| {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.begin_function(0, false);
    bcb.emit_abc(LuauOpcode::LOP_MOVE, 0, 99, 0);
    bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
    bcb.end_function(1, 0, 0, 0);
  });
  assert!(fired >= 1, "非法寄存器必须被 validate 链捕获");
}

/// 非变长函数直接 MULTRET RETURN 触发 `validate_variadic` 的配对检查。
#[test]
fn validate_chain_fires_on_unpaired_multret() {
  if !LUAU_ASSERTENABLED {
    return;
  }
  let fired = fired_count(|| {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.begin_function(0, false);
    bcb.emit_ad(LuauOpcode::LOP_LOADNIL, 0, 0);
    // B=0 → nresults=-1（MULTRET），但序列没有生产者开路
    bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 0, 0);
    bcb.end_function(1, 0, 0, 0);
  });
  assert!(fired >= 1, "孤立 MULTRET 必须被 validate_variadic 捕获");
}

/// CFG 版捕获检查：LCT_REF 后未 CLOSEUPVALS 就 RETURN 必须触发断言
/// （cpp `validateCaptures`：可达 RETURN 处 captured 必须全 false）。
#[test]
fn validate_captures_rejects_unclosed_ref() {
  if !LUAU_ASSERTENABLED {
    return;
  }
  let fired = fired_count(|| {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.begin_function(0, false);
    bcb.emit_ad(LuauOpcode::LOP_LOADNIL, 0, 0);
    let lct_ref = (LuauCaptureType::LCT_REF as u32) as u8;
    bcb.emit_abc(LuauOpcode::LOP_CAPTURE, lct_ref, 0, 0);
    bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
    bcb.end_function(1, 0, 0, 0);
  });
  assert!(
    fired >= 1,
    "未闭合的 CAPTURE REF 必须被 validate_captures 捕获"
  );
}

/// 合法闭合（CLOSEUPVALS >= 捕获寄存器）后 RETURN 不得触发任何断言。
#[test]
fn validate_captures_accepts_closed_ref() {
  if !LUAU_ASSERTENABLED {
    return;
  }
  let fired = fired_count(|| {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.begin_function(0, false);
    bcb.emit_ad(LuauOpcode::LOP_LOADNIL, 0, 0);
    let lct_ref = (LuauCaptureType::LCT_REF as u32) as u8;
    bcb.emit_abc(LuauOpcode::LOP_CAPTURE, lct_ref, 0, 0);
    bcb.emit_abc(LuauOpcode::LOP_CLOSEUPVALS, 0, 0, 0);
    bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
    bcb.end_function(1, 0, 0, 0);
  });
  assert_eq!(fired, 0, "闭合后的合法捕获序列不得触发断言");
}
