//! 调试期校验链（`BytecodeBuilder::validate`）行为测试。
//!
//! 对齐 cpp `BytecodeBuilder.cpp:248-250`：`endFunction` 在 `LUAU_ASSERTENABLED` 下
//! 必须调用 `validate()`，否则 `emit_*` 写坏寄存器/常量/跳跃/捕获在 debug 构建里无人发现。
//! 本文件用断言处理器把 `LUAU_ASSERT!` 失败计数化（返回 0 表示已接管，不再 debugbreak），
//! 从而能同时验证「合法字节码零误报」与「非法字节码必被抓到」。

use core::ffi::c_char;
use std::sync::atomic::{AtomicI32, Ordering};

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  functions::assert_handler::{assert_handler, set_assert_handler},
  macros::luau_assertenabled::LUAU_ASSERTENABLED,
  type_aliases::assert_handler::AssertHandler,
};

static TRIPPED: AtomicI32 = AtomicI32::new(0);

/// 记录断言并接管：返回 0 告诉 `LUAU_ASSERT!` 不要再触发断点。
unsafe extern "C-unwind" fn count_assert(
  _expression: *const c_char,
  _file: *const c_char,
  _line: i32,
  _function: *const c_char,
) -> i32 {
  TRIPPED.fetch_add(1, Ordering::SeqCst);
  0
}

/// 安装/复原全局断言处理器。校验失败是进程级全局状态，故所有阶段在同一测试内串行执行。
struct AssertCatcher {
  previous: AssertHandler,
}

impl AssertCatcher {
  fn new() -> Self {
    let previous = assert_handler();
    TRIPPED.store(0, Ordering::SeqCst);
    set_assert_handler(Some(count_assert));
    Self { previous }
  }

  /// 取走并清零当前断言计数。
  fn take_tripped(&self) -> i32 {
    TRIPPED.swap(0, Ordering::SeqCst)
  }
}

impl Drop for AssertCatcher {
  fn drop(&mut self) {
    set_assert_handler(self.previous);
  }
}

/// `local a = 2.0; local b = a // a; local c = a // 2.0; return a`
/// —— 覆盖 `LOP_IDIV` / `LOP_IDIVK`（cpp `validateInstructions:1744、1756`）。
fn build_arith(bcb: &mut BytecodeBuilder<'_>, maxstacksize: u8) {
  bcb.begin_function(0, false);

  let k = bcb.add_constant_number(2.0) as i16;
  bcb.emit_ad(LuauOpcode::LOP_LOADK, 0, k);
  bcb.emit_abc(LuauOpcode::LOP_IDIV, 1, 0, 0);
  bcb.emit_abc(LuauOpcode::LOP_IDIVK, 2, 0, k as u8);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);

  bcb.end_function(maxstacksize, 0, 0, 0);
}

/// 子函数体：`return upvalue0`，声明 1 个 upvalue。
fn build_child(bcb: &mut BytecodeBuilder<'_>) -> u32 {
  let child = bcb.begin_function(0, false);
  bcb.emit_abc(LuauOpcode::LOP_GETUPVAL, 0, 0, 0);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
  bcb.end_function(1, 1, 0, 0);
  child
}

/// 父函数：`NEWCLOSURE + CAPTURE REF`，捕获在 RETURN 前被 `CLOSEUPVALS` 关闭。
fn build_capture_closed(bcb: &mut BytecodeBuilder<'_>, child: u32) {
  bcb.begin_function(0, false);
  let pid = bcb.add_child_function(child);

  bcb.emit_ad(LuauOpcode::LOP_NEWCLOSURE, 0, pid);
  bcb.emit_abc(
    LuauOpcode::LOP_CAPTURE,
    LuauCaptureType::LCT_REF as u8,
    1,
    0,
  );
  bcb.emit_abc(LuauOpcode::LOP_CLOSEUPVALS, 1, 0, 0);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
  bcb.end_function(2, 0, 0, 0);
}

/// 捕获只发生在 fallthrough 分支上，跳转过去的那条 `RETURN` 路径没关闭——
/// 线性扫描（`validateInstructions` 的 `open_captures`）看不出问题，
/// 只有 cpp `validateCaptures` 的 CFG 数据流能抓到。
fn build_capture_leaks_on_jump(bcb: &mut BytecodeBuilder<'_>, child: u32) {
  bcb.begin_function(0, false);
  let pid = bcb.add_child_function(child);

  bcb.emit_ad(LuauOpcode::LOP_NEWCLOSURE, 0, pid);
  bcb.emit_abc(
    LuauOpcode::LOP_CAPTURE,
    LuauCaptureType::LCT_REF as u8,
    1,
    0,
  );

  let jump_label = bcb.emit_label();
  bcb.emit_ad(LuauOpcode::LOP_JUMP, 0, 0);

  bcb.emit_abc(LuauOpcode::LOP_CLOSEUPVALS, 1, 0, 0);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);

  let leak_label = bcb.emit_label();
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);

  // `JUMP` 目标即泄漏路径上的 RETURN
  assert!(
    bcb.patch_jump_d(jump_label, leak_label),
    "跳转应可直连，无需 trampoline"
  );
  bcb.end_function(2, 0, 0, 0);
}

/// 校验链必须真的挂在 `end_function` 上：合法字节码零误报，非法字节码必报。
#[test]
fn end_function_validates_in_debug_builds() {
  // 校验体整体只在 LUAU_ASSERTENABLED 下被调用（与 cpp `#ifdef` 等价），
  // release 构建里没有断言可数，直接跳过。
  if !LUAU_ASSERTENABLED {
    return;
  }

  let catcher = AssertCatcher::new();

  // 1) IDIV/IDIVK 必须在算术组里被认下，否则命中 "Unsupported opcode" 断言
  build_arith(&mut BytecodeBuilder::new(None), 3);
  assert_eq!(
    catcher.take_tripped(),
    0,
    "形状正确的算术函数不应触发任何校验断言"
  );

  // 2) 越界寄存器必须被 validateInstructions 抓到（r2 超出 maxstacksize=2）
  build_arith(&mut BytecodeBuilder::new(None), 2);
  assert!(
    catcher.take_tripped() > 0,
    "越界寄存器必须触发 VREG 断言，说明 validate 真的被调用了"
  );

  // 3) 捕获被关闭的函数不得误报（validateCaptures 的正向路径）
  let mut bcb = BytecodeBuilder::new(None);
  let child = build_child(&mut bcb);
  assert_eq!(catcher.take_tripped(), 0, "子函数体不应触发断言");
  build_capture_closed(&mut bcb, child);
  assert_eq!(
    catcher.take_tripped(),
    0,
    "CLOSEUPVALS 已关闭捕获，不应误报"
  );

  // 4) 泄漏路径上的 RETURN 必须被 validateCaptures 抓到
  let mut bcb = BytecodeBuilder::new(None);
  let child = build_child(&mut bcb);
  build_capture_leaks_on_jump(&mut bcb, child);
  assert!(
    catcher.take_tripped() > 0,
    "存在未关闭捕获就 RETURN 的可达路径，validateCaptures 必须报错"
  );
}
