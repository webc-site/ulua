// 协程 / yield 续跑用例
// 移植自 `cpp/tests/Conformance.test.cpp`。
//
// 已知缺件（对照 cpp `Conformance.test.cpp:1551-1580` 的 `TEST_CASE("CoroutineHost")`）：
// `coroutinehost.luau` fixture（出处 `cpp/tests/conformance/coroutinehost.luau`）未登记——
// 依赖三块 Rust VM 未实现的宿主面，实测该语料在当前 HEAD 于
// `coroutinehost.luau:8`（首次 `coroutine.finally` 调用）报
// `attempt to call a nil value`（临时探针验证，前端 parse/compile 均通过）：
// 1. `coroutine.finally` 库函数：cpp `lcorolib.cpp:456`（`cofinally`）注册于
//    cpp `lcorolib.cpp:488`（CO_FUNCS `{"finally", cofinally}`）；本端口
//    `ulua-vm/src/functions/luaopen_coroutine.rs` 的 CO_FUNCS 无 finally 项。
// 2. 宿主 finalizer API：cpp `lua.h:279-280` 声明、`lapi.cpp:1319`
//    （`lua_hasfinalizers`）与 `lapi.cpp:1346`（`lua_pushfinalizerfunction`，内部
//    `lua_pushcclosurek(runfinalizery/runfinalizercont)`），底层是 `lua_State` 的
//    `finalizers` 链表与 `luaD_preparefinalize`/`luaD_runfinalizers`；本端口
//    `ulua-vm` 全 crate 无对应符号（仅 `lua_callbacks.rs:40` 的 `userfinalizer`
//    GC 回调，语义不同）。cpp 的 `hostresume` setup 闭包
//    （`Conformance.test.cpp:1556-1579`）即按这套 API 写，照搬注册必失败。
// 3. 门控 FFlag `DebugLuauCoroutineFinally`：cpp `lapi.cpp:24` 定义，
//    `Conformance.test.cpp:1544/1553` 以 ScopedFastFlag 打开；本端口
//    `ulua-common/src/fflag.rs` 未定义该 flag。
// 补法：上述 1-3 落地后，连同 `TEST_CASE("CoroutineHost")` 的 hostresume setup
// 一并移植登记；连同本文件 `conformance_coroutine` 注释里裁掉的
// coroutine.luau finally 两段一起恢复对齐。

use core::ptr::null_mut;

#[test]
fn conformance_c_yield() {
  use ulua_common::fflag;

  use crate::common::{
    functions::{
      conformance_c_yield_setup::conformance_c_yield_setup, run_conformance::run_fixture_setup,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp `Conformance.test.cpp:1879` 的 CYield 用例不带作用域旗标；本端口
  // `resume_finish.rs` 按更新上游移植了 C 调用层级恢复修复（本地 cpp oracle 为修复
  // 前旧快照，见该文件 DELIBERATE DEVIATION 注），cyield.luau 的续体路径按旗开
  // 语义成立，故显式置真。
  let _luau_resume_restore_c_calls = ScopedFastFlag::new(&fflag::LuauResumeRestoreCcalls, true);

  run_fixture_setup("cyield.luau", conformance_c_yield_setup);
}

#[test]
fn conformance_coroutine() {
  use crate::common::functions::run_conformance::run_fixture;

  // fixture 相对 cpp `tests/conformance/coroutine.luau` 裁掉了尾部两块：
  // 411-618（coroutine.finally 前半：LIFO 顺序、错误传播、多返回值、dead/nil/wrapped
  // 注册约束、callback 可 yield）与 621-827（finally 后半：close 特殊值、close 回调出错、
  // 不重跑、3 万无上限、running coroutine、Promise 抽象），仅保留收尾 `end`/`return 'OK'`。
  // cpp `Conformance.test.cpp:1544-1548` 的 TEST_CASE("Coroutine") 以
  // `ScopedFastFlag{DebugLuauCoroutineFinally, true}` 显式验证这些段。根因：
  // Rust VM 未实现 `coroutine.finally`——FFlag 与 `cpp/VM/src/lcorolib.cpp:488`
  // 的 cofinally 均未移植（`ulua-vm/src/functions/luaopen_coroutine.rs` 的
  // CO_FUNCS 无 finally 项）。「coroutine.close errors with no error object」段
  // 不依赖 finally，已单独补回 fixture（实测通过）；finally 两段待 Rust 侧
  // `coroutine.finally` 落地后连同整块补回、保持与 cpp fixture 对齐。
  run_fixture("coroutine.luau");
}

#[test]
fn conformance_iter() {
  use ulua_common::fflag;

  use crate::common::{
    functions::{
      conformance_iter_setup::conformance_iter_setup, run_conformance::run_fixture_setup,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _luau_yield_iter = ScopedFastFlag::new(&fflag::LuauYieldIter2, true);

  run_fixture_setup("iter.luau", conformance_iter_setup);
}

#[test]
fn conformance_iter_fenv() {
  use crate::common::functions::run_conformance::run_fixture;

  run_fixture("iter_fenv.luau");
}

#[test]
fn conformance_p_call() {
  use ulua_common::fflag;
  use ulua_vm::functions::lua_newstate::lua_newstate;

  use crate::common::{
    functions::{
      conformance_p_call_setup::conformance_p_call_setup, limited_realloc::limited_realloc,
      run_conformance::run_conformance,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // 同 `conformance_c_yield`：resume 恢复 C 调用层级的修复按旗开语义跑
  // （cpp `Conformance.test.cpp:1604` 的 PCall 用例本身不带作用域旗标）。
  let _luau_resume_restore_c_calls = ScopedFastFlag::new(&fflag::LuauResumeRestoreCcalls, true);
  // FFI: c-API 要求 NULL
  let initial_lua_state = unsafe { lua_newstate(Some(limited_realloc), null_mut()) };

  run_conformance(
    "pcall.luau",
    Some(conformance_p_call_setup),
    None,
    Some(initial_lua_state),
    None,
    false,
    None,
  );
}
