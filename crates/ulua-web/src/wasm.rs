//! Luau 游乐场的 `#[wasm_bindgen]` 浏览器 API。
//!
//! 本模块经 `wasm` feature 门控，把 crate 的执行/检查管线挂到 wasm-bindgen 导出
//! 的 `run`/`check` 两个入口上，与宿主侧入口
//! （[`execute_script`](crate::functions::execute_script::execute_script)、
//! [`check_script`](crate::functions::check_script::check_script)）共用
//! `run_in_sandbox`/`run_code`/`run_check`，向 JavaScript 暴露
//! `&str` -> `String` 的朴素接口。
//!
//! 导出两个函数：
//!
//! - [`run`] — 编译并在 VM 上执行源码，返回捕获的 `print` 输出（及错误文本）。
//! - [`check`] — 用分析器类型检查源码，返回换行拼接的诊断；无错误时为
//!   `"No errors."`。
//!
//! ## 捕获 `print`
//!
//! VM 默认 `print`（`lua_b_print`）经 `writestring` 写 `std::io::stdout()`；
//! `wasm32-unknown-unknown` 没有真实 stdout，输出会被静默丢弃。为让浏览器
//! 游乐场可用，[`run`] 在沙箱冻结全局表**之前**安装一个捕获版 `print` 全局：
//! 捕获函数追加到 thread-local 缓冲，脚本跑完后清空并回传 JavaScript。捕获
//! 行为与 `lua_b_print` 完全一致（制表符分隔参数、结尾换行、`luaL_tolstring`
//! 强转），可观察行为不变。

use core::{cell::RefCell, ffi::c_int, mem};
use std::{panic, string::String};

use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  functions::{
    install_lua_exception_panic_hook::install_lua_exception_panic_hook, lua_gettop::lua_gettop,
    lua_l_tolstring::lua_l_tolstring_ref, lua_pushcclosurek::lua_pushcclosurek,
  },
  macros::{lua_pop::lua_pop, lua_setglobal::lua_setglobal},
  records::{lua_exception::lua_exception, lua_state::LuaState},
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
  functions::{check_script::run_check, run_in_sandbox::run_in_sandbox},
  util::PRINT_NAME,
};

/// 无诊断时回给 JavaScript 的文案。
///
/// 这是跨语言契约而非展示文本：`website/src/lib/wasm.js` 按字面量识别它
/// （`raw === "No errors."` 即「零诊断」），改一个字符 playground 就把干净脚本
/// 显示成一条错误。cpp `Web.cpp:181` 的 C 入口用 nullptr 表达同一语义，JS 门面
/// 层负责换成该文案。
const NO_ERRORS: &str = "No errors.";

#[wasm_bindgen]
// Safety: 这是 wasm-bindgen 生成的 JS 互操作面（真 FFI 边界）。契约由
// `website/` 的 `app.js` 承担：`globalThis.__uluaOnRuntimeError` 必须在任何
// `run` 调用之前安装、且接受任意 `&str`；本 crate 只以 `&str` 传参（绑定层
// 负责把 UTF-8 拷进 JS 堆），不读取 JS 侧返回值，故无悬垂/别名风险。
unsafe extern "C" {
  // app.js installs `globalThis.__uluaOnRuntimeError` before any run. A Lua
  // runtime error is emulated with `panic_any` (lua_d_throw); on stable
  // wasm32-unknown-unknown (`panic = "abort"`) that traps the instance and the
  // message would otherwise be lost. The panic hook hands the recovered error
  // text to JS HERE, before the abort — so the playground reports a typed,
  // messaged runtime error (classified by the trap being a `lua_exception`),
  // never by scanning output text.
  #[wasm_bindgen(js_namespace = globalThis, js_name = __uluaOnRuntimeError)]
  fn on_runtime_error(message: &str);
}

/// Module start hook. A Lua runtime error reaches this panic hook as a
/// `lua_exception` payload (`lua_d_throw` -> `panic_any`). We recover its message
/// (`what()` reads the error object off the still-intact stack — `panic=abort`
/// does not unwind) and hand it to JS *before* the abort traps the instance, so
/// runtime errors surface with their text instead of an opaque `unreachable`
/// trap. Any other panic keeps the `console.error` diagnostic.
///
/// ## Hook ordering — why this is more than a single `set_hook`
///
/// The VM installs its OWN process-wide hook ([`install_lua_exception_panic_hook`],
/// fired lazily on the first `lua_d_rawrunprotected` during state setup) that
/// *silently swallows* every `lua_exception` payload: those panics are its
/// `longjmp` emulation, not crashes, and the CLI must not print "thread panicked"
/// for a normal `error()`. That same swallowing, however, also hides the error
/// *message* — the VM hook `take_hook()`s whatever we install and then `return`s
/// early for `lua_exception`, so a naive hook here would never be reached.
///
/// So we deliberately build the chain with OUR hook outermost (it runs first):
///
/// ```text
///   ours (lua_exception -> JS bridge)  ->  VM hook  ->  console_error_panic_hook
/// ```
///
/// Force `console_error_panic_hook` as the base, force the VM hook to install on
/// top of it *now* (so its captured `previous` is the console hook, not ours),
/// then wrap that with our hook. A `lua_exception` is intercepted here and its
/// message forwarded to JS; any real Rust bug falls through to the VM hook, which
/// delegates to `console_error_panic_hook` unchanged.
#[wasm_bindgen(start)]
pub fn wasm_start() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  install_lua_exception_panic_hook();
  let previous = panic::take_hook();
  panic::set_hook(Box::new(move |info| {
    if let Some(exc) = info.payload().downcast_ref::<lua_exception>() {
      // Safety: what() 返回的指针指向 VM 栈上仍存活的错误对象（panic=abort
      // 不做 unwind，栈保持完整）；null 时 cstr_cow 返回空串。
      let msg = unsafe { cstr_cow(exc.what()) };
      on_runtime_error(&msg);
    } else {
      previous(info);
    }
  }));
}

thread_local! {
    /// Accumulates `print` output for the current `run` call. Drained when the
    /// VM finishes so subsequent runs start clean.
    static PRINT_BUFFER: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Capturing replacement for the VM's `print`. Mirrors `lua_b_print`: each
/// argument is coerced with `luaL_tolstring`, arguments are tab-separated, and a
/// trailing newline is appended — but the bytes go to [`PRINT_BUFFER`] instead
/// of `stdout`.
unsafe extern "C-unwind" fn capturing_print(l: *mut LuaState) -> c_int {
  // Safety: 本函数只由 VM 经 `lua_pushcclosurek` 作为 Lua 闭包调用，故 `l` 必是
  // 调用它的那个活跃状态机；索引 `1..=lua_gettop(l)` 即本次调用的实参区，逐个
  // `luaL_tolstring` 会压入一个副本、随即由 `lua_pop` 弹出（栈高在每轮后回到
  // 原值，`i` 因此始终指向原实参）。`PRINT_BUFFER` 是线程局部的 `RefCell`，
  // 借用只在语句内生效，不与重入的 print 争借。

  // Safety: gettop 只读 `l` 的栈高。
  let n = unsafe { lua_gettop(l) };
  let mut line = String::new();
  for i in 1..=n {
    // Safety: `i` 在本帧实参区（见上契约）；tolstring 把参数的字符串表示压栈并
    // 以切片回报全字节（内嵌 NUL 不截断），在弹出前读出；`None`（旧 null 指针）
    // 与空切片同义，译成空串。
    let arg = unsafe { lua_l_tolstring_ref(l, i) }.unwrap_or_default();
    if i > 1 {
      line.push('\t');
    }
    line.push_str(&String::from_utf8_lossy(arg));
    // Safety: 弹出本轮 luaL_tolstring 压入的副本，栈高复原。
    unsafe { lua_pop(l, 1) };
  }
  line.push('\n');
  PRINT_BUFFER.with(|b| b.borrow_mut().push_str(&line));
  0
}

/// Structured result of a [`run`] call: the program's captured `print` output
/// and, *separately*, any error text (an empty string when the run succeeded).
///
/// Keeping the two apart — rather than concatenating them into one string the
/// caller then has to guess apart — is what lets the playground classify a run
/// correctly. With a single combined string the only signal available to
/// JavaScript was a content heuristic, which both *false-positived* (legitimate
/// output containing the word "error" — e.g. iterating `_G`, which has a global
/// literally named `error` — was painted as a failure) and *false-negatived* (a
/// compile error whose text lacked the magic words was reported as success).
/// `error` non-empty ⇔ the run failed; no scanning of `output` required.
#[wasm_bindgen]
pub struct RunResult {
  output: String,
  error: String,
}

#[wasm_bindgen]
impl RunResult {
  /// The script's captured `print` output (tab-separated arguments, one line
  /// per `print`, each terminated by a newline).
  #[wasm_bindgen(getter)]
  pub fn output(&self) -> String {
    self.output.clone()
  }

  /// Error text, or the empty string when the run succeeded. In the browser
  /// build this is a compile/load error message: a genuine *runtime* error
  /// traps the WebAssembly instance (`panic = "abort"` on
  /// `wasm32-unknown-unknown`) and is surfaced by the caller's trap handler,
  /// so it never reaches here.
  #[wasm_bindgen(getter)]
  pub fn error(&self) -> String {
    self.error.clone()
  }
}

/// Compile and execute `source` on a fresh sandboxed Luau VM, returning the
/// program's captured `print` output and any error text as separate fields of a
/// [`RunResult`].
///
/// This is the browser counterpart of the crate's
/// [`execute_script`](crate::functions::execute_script::execute_script):
/// 两者共用 [`run_in_sandbox`] 骨架，唯一差别是本入口在沙箱冻结全局表之前把
/// `print` 覆写成捕获版（cpp `Web.cpp:64-69` 的 `setupState` 没有这一步）。
#[wasm_bindgen]
pub fn run(source: &str) -> RunResult {
  // Reset the capture Buffer for this run.
  PRINT_BUFFER.with(|b| b.borrow_mut().clear());

  // Safety: 钩子只在 [`run_in_sandbox`] 契约给出的「openlibs 之后、luaL_sandbox
  // 冻结全局表之前」窗口内拿到该 `l`（此刻全局表仍可写），且指针不跨调用持有；
  // `capturing_print` 与 `lua_setglobal` 各自压/弹平衡，故交给 run_code 的栈
  // 与 cpp `executeScript` 一致。
  let error = run_in_sandbox(source, |l| unsafe {
    lua_pushcclosurek(
      l,
      Some(capturing_print),
      PRINT_NAME.as_ptr().cast(),
      0,
      None,
    );
    lua_setglobal(l, PRINT_NAME.as_ptr().cast());
  });

  let output = PRINT_BUFFER.with(|b| mem::take(&mut *b.borrow_mut()));
  RunResult { output, error }
}

/// Type-check `source` with the analyzer (old solver) and return the
/// newline-joined `line: message` diagnostics, or [`NO_ERRORS`] when clean.
#[wasm_bindgen]
pub fn check(source: &str) -> String {
  let result = run_check(source, false);
  if result.is_empty() {
    NO_ERRORS.to_owned()
  } else {
    result
  }
}
