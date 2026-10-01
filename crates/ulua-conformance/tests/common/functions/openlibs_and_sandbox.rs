//! cpp 手工用例的建库前置三连（openlibs → sandbox → sandboxthread）收口门面。
//! 本文件不再开口 `unsafe`：三个动作全部经由 `safe_api::openlibs_and_sandbox_all`，
//! 裸指针只在该门面内部出现一次。

use crate::common::functions::safe_api::{L, openlibs_and_sandbox_all};

/// cpp `tests/Conformance.test.cpp` 各手工用例里逐字重复的建库前置：
/// `luaL_openlibs(L); luaL_sandbox(L); luaL_sandboxthread(L);`（如
/// `Conformance.test.cpp:4452-4454`、`DirectFieldAccess` 与 huge-codegen 系列）。
///
/// 三步恒以同一顺序、同一 `LuaState` 连用，中间不夹杂其它调用，是「直接调 C API
/// 的用例」统一的库打开 + 沙箱化入口。收敛前本 crate 内有 8 份同形副本
/// （`bytecode.rs` 4 份、`codegen.rs` 3 份、`debugger.rs` 1 份），现全部切到本门面。
///
/// 前提与 `safe_api::openlibs_and_sandbox_all` 一致：`l` 为存活、尚未 openlibs 的
/// 状态（对同一状态重复调用会二次注册）。
pub fn openlibs_and_sandbox(l: L) {
  openlibs_and_sandbox_all(l);
}
