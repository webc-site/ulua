//! Faithful port of `runReplImpl` from `CLI/src/Repl.cpp`, with the isocline
//! line editor replaced by `rustyline`.
//!
//! isocline → rustyline mapping:
//!   * `ic_set_default_completer(completeRepl, l)` + `ic_complete_word` become a
//!     rustyline `Helper` whose `Completer::complete` runs the faithful
//!     `complete_repl` port (Luau's global-table introspection).
//!   * The C++ multiline behavior — keep reading while `runCode` reports a
//!     parse error ending in "<eof>" — becomes `Validator::validate` returning
//!     `ValidationResult::Incomplete` for the same incomplete-input condition,
//!     so the editor keeps reading lines within a single `readline` call.
//!   * `ic_set_history(path, -1)` / isocline's auto-save become
//!     `Editor::load_history` + `set_max_history_size` + `save_history` on exit;
//!     `ic_history_add` becomes `Editor::add_history_entry`.

use alloc::{borrow::Cow, string::String};

use rustyline::{
  Context, Editor, Helper,
  completion::{Completer, Pair},
  config::Config,
  error::ReadlineError,
  highlight::Highlighter,
  hint::Hinter,
  history::FileHistory,
  validate::{ValidationContext, ValidationResult, Validator},
};
use ulua_vm::records::lua_state::LuaState;

use crate::functions::{
  compile_source::compile_source,
  complete_repl::complete_repl,
  load_history::{DEFAULT_HISTORY_ENTRIES, load_history},
  run_code::run_code,
};

// rustyline Helper bridging the REPL to the faithful completion / incomplete
// detection ports. It holds the raw `LuaState` so the completer can introspect
// the global table exactly as `getCompletions` did in C++.
struct ReplHelper {
  l: *mut LuaState,
}

impl Completer for ReplHelper {
  type Candidate = Pair;

  fn complete(
    &self,
    line: &str,
    pos: usize,
    _ctx: &Context<'_>,
  ) -> rustyline::Result<(usize, Vec<Pair>)> {
    // Faithful port of completeRepl / icGetCompletions / getCompletions.
    // Safety: self.l 与构造它的 run_repl_impl 参数 l 同一状态（fn /// # Safety：整个循环内有效），rustyline 补全回调在 REPL 同一线程同步触发，complete_repl 的存活前提由该串行驱动保证。
    let (start, completions) = unsafe { complete_repl(self.l, line, pos) };
    Ok((start, completions))
  }
}

// No hints: isocline's default completer offered completions, not inline hints.
impl Hinter for ReplHelper {
  type Hint = String;
}

// No syntax highlighting: matches the plain isocline REPL transport.
impl Highlighter for ReplHelper {
  fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
    Cow::Borrowed(line)
  }
}

impl Validator for ReplHelper {
  fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
    // Same incomplete-input detection as Repl.cpp's multiline loop: compile
    // the Buffer and treat a parse error that ends in "<eof>" as an
    // incomplete statement, so the editor keeps reading more lines.
    if is_incomplete_chunk(ctx.input()) {
      Ok(ValidationResult::Incomplete)
    } else {
      Ok(ValidationResult::Valid(None))
    }
  }
}

impl Helper for ReplHelper {}

// Compile `source` (without executing) and report whether the parse failed with
// the "<eof>" suffix that marks an incomplete statement. `compile` returns
// error bytecode of the form `\0<message>` on failure (mirroring
// BytecodeBuilder::getError), so we detect the leading NUL and inspect the
// trailing message exactly as the C++ loop inspected `runCode`'s error string.
fn is_incomplete_chunk(source: &str) -> bool {
  let bytecode = compile_source(source);

  // Successful bytecode begins with LBC_VERSION_TARGET (non-zero); error
  // bytecode begins with a NUL marker followed by the message.
  let message = bytecode.as_slice();
  if message.first() != Some(&0) {
    return false;
  }

  message[1..].ends_with(b"<eof>")
}

/// # Safety
///
/// `l` 必须指向存活、已 sandbox 的主线程 `LuaState`，并在整个交互式循环期间保持有效；
/// REPL 为单线程驱动，循环内经 `run_code`/`complete_repl` 直接解引用该指针（含作为补全
/// helper 的 `ReplHelper { l }` 长期持有者）。
pub(crate) unsafe fn run_repl_impl(l: *mut LuaState) {
  // isocline's `ic_set_history(path, -1)` capped history at its default of 200
  // entries; mirror that via the editor configuration.
  // rustyline 的初始化是可失败的（isocline 不是），失败原因必须可见，
  // 否则 REPL 静默退出、用户看不到任何提示。builder→Config→Editor 的
  // 两级失败收进同一个 match 臂，错误文案单点持有。
  let mut editor: Editor<ReplHelper, FileHistory> = match Config::builder()
    .max_history_size(DEFAULT_HISTORY_ENTRIES)
    .map(|b| b.build())
    .and_then(Editor::with_config)
  {
    Ok(e) => e,
    Err(e) => {
      eprintln!("Error initializing line editor: {e}");
      return;
    }
  };
  // ReplHelper 只拷贝裸地址（构造本身是安全的），其存活期被 editor（本帧局部）
  // 包住；补全/校验回调经 helper 解引用 l，前提由 fn /// # Safety（l 全程有效、
  // 单线程驱动）与调用方 run_repl 的守卫保证，editor 析构先于任何状态失效。
  editor.set_helper(Some(ReplHelper { l }));

  // Reset the locale to C — handled by the host environment in Rust.

  // Loads history from the given file; we also save it explicitly on exit
  // (isocline saved automatically on process exit).
  let history_path = load_history(".luau_history");
  if let Some(ref path) = history_path {
    let _ = editor.load_history(path);
  }

  loop {
    // C++ prompt: "" for a fresh statement, ">" for continuation lines.
    // rustyline reads the whole (possibly multiline) statement in one call,
    // so the initial prompt is the empty string.
    let prompt = "";

    match editor.readline(prompt) {
      Ok(line) => {
        // cpp（Repl.cpp:524）只在新语句首行（buffer 为空）尝试 `return <line>`；
        // rustyline 把多行续行累积成一条 line，含 '\n' 即处于 cpp 的续行态——
        // 续行只跑原文（`return 1 +\n2` 在 cpp 是语法错误，不得在此绕道编译）。
        if !line.contains('\n') {
          // Try the expression shorthand: `return <line>`.
          // 预分配一次成型，免 `from + &line` 的二次扩容
          let mut wrapped = String::with_capacity("return ".len() + line.len());
          wrapped.push_str("return ");
          wrapped.push_str(&line);
          // Safety: l 自本循环开始到返回全程有效且仅本线程驱动（fn /// # Safety）。
          if unsafe { run_code(l, &wrapped) }.is_none() {
            let _ = editor.add_history_entry(line.as_str());
            continue;
          }
        }

        // An "<eof>" error means an incomplete chunk slipped through;
        // skip printing and let the next read continue it. (With the
        // rustyline validator this is normally caught before accept.)
        // Safety: 同上，l 在本帧循环内有效且单线程驱动。
        if let Some(error) = unsafe { run_code(l, &line) } {
          if error.ends_with("<eof>") {
            continue;
          }

          if !error.is_empty() {
            println!("{error}");
          }
        }

        let _ = editor.add_history_entry(line.as_str());
      }
      // Ctrl-C / Ctrl-D 正常结束，不打印
      Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
      Err(e) => {
        eprintln!("Error reading input: {e}");
        break;
      }
    }
  }

  if let Some(ref path) = history_path {
    let _ = editor.save_history(path);
  }
}
