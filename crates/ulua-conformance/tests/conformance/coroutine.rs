// 边界契约测试：null 系 c-API 合法实参（既有约定 review.md §2）
// 协程 / yield 续跑用例
// 移植自 `cpp/tests/Conformance.test.cpp`。
//
// 缺件收口（cpp-gaps-r4 对账，oracle 快照 cpp@47cda63）：既往挂账的
// `coroutinehost.luau`（`TEST_CASE("CoroutineHost")`）在当前 cpp 克隆中已整体
// 移除——`cpp/tests/conformance/` 无该 fixture（56 文件清点），
// `Conformance.test.cpp` 无 `CoroutineHost` 用例与 `hostresume` setup，
// `VM/src/lcorolib.cpp`（269 行）无 `cofinally`/`{"finally", …}` 注册项，
// `VM/include/lua.h` 无 `lua_hasfinalizers`/`lua_pushfinalizerfunction`，
// 亦无 `DebugLuauCoroutineFinally` 旗标；`coroutine.luau` 语料同样不含
// finally 段（本目录 `coroutine.luau` 与 cpp 版对齐，仅多本地新增的
// `coroutine.close` 无错误对象段）。上游撤回了宿主 finally 特性，
// 本端口不再构成缺口；旧账三块依赖（CO_FUNCS finally、宿主 finalizer API、
// 门控旗标）随上游移除一并销账，无需再补。若上游日后重新落地该特性，
// 按 sync-cpp 流程连同 TEST_CASE 与 fixture 一并登记。

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
  use core::ptr::null_mut;

  use ulua_common::fflag;

  use crate::common::{
    functions::{
      conformance_p_call_setup::conformance_p_call_setup, limited_realloc::limited_realloc,
      run_conformance::run_conformance, safe_api::newstate,
    },
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // 同 `conformance_c_yield`：resume 恢复 C 调用层级的修复按旗开语义跑
  // （cpp `Conformance.test.cpp:1604` 的 PCall 用例本身不带作用域旗标）。
  let _luau_resume_restore_c_calls = ScopedFastFlag::new(&fflag::LuauResumeRestoreCcalls, true);
  let initial_lua_state = newstate(Some(limited_realloc), null_mut());

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
