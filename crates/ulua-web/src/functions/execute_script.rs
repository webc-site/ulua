//! `executeScript`（cpp `CLI/src/Web.cpp:184-208`）的 Rust 形态。
//!
//! cpp 用 `extern "C" const char*` + 函数内 `static std::string` 缓存把结果以裸指针
//! 交回宿主；本 crate 不是 C ABI 边界（见 [`crate::functions::check_script`] 模块头的
//! 同条理由），故结果直接以 `Option<String>` 表达：`None` 即 cpp 的 `nullptr`
//! （`Web.cpp:207` 的 `result.empty() ? nullptr : result.c_str()`），无需任何
//! 跨调用存活的可变缓存。跑码序列本身与 wasm `run` 共用 [`run_in_sandbox`]。

use crate::{functions::run_in_sandbox::run_in_sandbox, util::nullable};

/// 在新建的沙箱状态上执行 `source`，返回错误文本；执行成功（无错误）返回 `None`。
///
/// 「new state → setup state → sandbox thread → run code」整段与 wasm `run` 同构，
/// 已收进 [`run_in_sandbox`]；native 入口没有「沙箱冻结全局表前装捕获 print」
/// 这步，钩子为空闭包（cpp `Web.cpp:64-69` 的 setupState 即 openlibs + sandbox）。
pub fn execute_script(source: &str) -> Option<String> {
  nullable(run_in_sandbox(source, |_| {}))
}
