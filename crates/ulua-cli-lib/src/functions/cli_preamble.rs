//! 各 CLI `main`/`run` 的共享前奏（对应上游 `luau-*` CLI `main` 开头的同一段）：
//! 读 `argv[0]` → 注册断言处理器 → 开启默认 Luau 旗标 → 检测顶层 `--help`。
//!
//! 副作用时序与各 CLI 原实现逐字一致：`install_assertion_handler()` 在
//! `set_luau_flags_default()` 之前。`argv0()` 是纯函数（无副作用），故放在安装
//! 处理器之前或之后都等价，这里与原 analyze/ast 的「先 argv0」次序保持一致。
//!
//! `help_requested` 只做 `args[1] == "--help"` 的精确检测（analyze/ast 语义）。
//! 需要全参扫描或额外识别 `-h` 的 CLI（compile）忽略此标记、保留自身循环内检测；
//! `display_help` 因各 CLI 文案不同仍由调用方执行，本函数只返回早退标记。

use crate::functions::{
  argv::argv0, assertion_handler::install_assertion_handler,
  set_luau_flags_default::set_luau_flags_default,
};

/// [`cli_preamble`] 的产物：`argv[0]` 与顶层 `--help` 早退标记。
pub struct Preamble<'a> {
  /// `argv[0]`，空参数表时落回调用方给的默认程序名。
  pub argv0: &'a str,
  /// `args[1] == "--help"`：调用方据此决定是否 `display_help` 后按自身退出码返回。
  pub help_requested: bool,
}

/// 见模块文档。`default_argv0` 仅在 `args` 为空时使用。
pub fn cli_preamble<'a, S: AsRef<str>>(args: &'a [S], default_argv0: &'a str) -> Preamble<'a> {
  // cpp 的 `char** argv` 按字节使用；`env::args()` 遇非 UTF-8 会 panic，
  // 统一由调用方走 lossy argv()，故此处入参已是可比较的字符串切片。
  let argv0 = argv0(args, default_argv0);

  // Luau::assertHandler() = assertionHandler;
  install_assertion_handler();

  // setLuauFlagsDefault();
  set_luau_flags_default();

  // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
  let help_requested = args.len() >= 2 && args[1].as_ref() == "--help";

  Preamble {
    argv0,
    help_requested,
  }
}
