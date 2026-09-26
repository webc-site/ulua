//! `checkScript`（cpp `CLI/src/Web.cpp:142-182`）的 Rust 形态。
//!
//! cpp 把它做成 `extern "C" const char* checkScript(const char* source, int useNewSolver)`，
//! 并用函数内 `static std::string` 缓存返回裸指针。本 crate 不是 C ABI 边界：浏览器侧
//! 走 [`crate::wasm`] 的 `#[wasm_bindgen]` 门面（只吃 `&str`、只回 `String`），仓库内
//! 没有任何 `ccall`/`EXPORTED_FUNCTIONS` 引用这两个符号。故按 review.md §10 直接以
//! `&str -> Option<String>` 表达同一行为，空诊断即 `None`（cpp 的 `nullptr`），
//! C 串结果缓存随之消失。

use core::any::Any;
use std::{panic::catch_unwind, string::String};

use crate::{records::demo_frontend::DemoFrontend, util::nullable};

/// Web.cpp 里写死的单模块名（`Web.cpp:164`、`Web.cpp:166`）。
const MAIN_MODULE: &str = "main";

/// 纯 Rust 内部入口：建 demo `Frontend`、注册内置全局、检查单模块，返回换行
/// 拼接的 `line: message` 诊断（无错为空串）。resolver 布线与内置全局注册的
/// unsafe 收敛在 [`DemoFrontend`]，此处为纯安全代码。
///
/// 供 wasm 门面 [`crate::wasm::check`] 与 [`check_script`] 共用。
pub(crate) fn run_check(source: &str, use_new_solver: bool) -> String {
  DemoFrontend::new(use_new_solver).check_source(MAIN_MODULE, source)
}

/// 类型检查 `source`，返回换行拼接的诊断；零诊断返回 `None`
/// （对应 `Web.cpp:181` 的 `finalCheckResult.empty() ? nullptr : ...`）。
///
/// `use_new_solver` 即 cpp 的 `int useNewSolver`（`Web.cpp:154` 的
/// `useNewSolver ? SolverMode::New : SolverMode::Old`），C 边界的 0/非 0 归一
/// 已由类型（`bool`）在编译期完成。
pub fn check_script(source: &str, use_new_solver: bool) -> Option<String> {
  // try { ... } catch (const std::exception& e) { finalCheckResult = e.what(); }
  // （`Web.cpp:147`、`Web.cpp:176-179`）：分析器抛出的 panic 在这里收敛成
  // 诊断文本，与 cpp 的观察行为同向，而不是展开到调用方。
  let result = match catch_unwind(|| run_check(source, use_new_solver)) {
    Ok(diagnostics) => diagnostics,
    Err(payload) => panic_message(&*payload),
  };

  nullable(result)
}

/// panic 载荷既非 `&str` 也非 `String` 时的兜底文案。
const UNKNOWN_ERROR: &str = "unknown error";

/// 从捕获到的 panic 载荷取 `std::exception::what()` 等价文本。
///
/// 保留 `dyn Any`：形参类型即 `catch_unwind` 的错误形态（载荷类型集合
/// 运行期开放），无擦除替代。
fn panic_message(payload: &(dyn Any + Send)) -> String {
  payload
    .downcast_ref::<&str>()
    .copied()
    .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
    .unwrap_or(UNKNOWN_ERROR)
    .to_owned()
}
