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
use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
use ulua_compiler::functions::compile::compile;
use ulua_vm::type_aliases::lua_state::lua_State;

use crate::functions::{
  complete_repl::complete_repl,
  copts::copts,
  load_history::{DEFAULT_HISTORY_ENTRIES, load_history},
  run_code::run_code,
};

// rustyline Helper bridging the REPL to the faithful completion / incomplete
// detection ports. It holds the raw `lua_State` so the completer can introspect
// the global table exactly as `getCompletions` did in C++.
struct ReplHelper {
  l: *mut lua_State,
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
  struct NoopEncoder;
  impl BytecodeEncoder for NoopEncoder {
    fn encode(&mut self, _data: &mut [u32]) {}
  }
  let options = copts();
  let parse_options = ParseOptions::default();
  let mut encoder = NoopEncoder;
  let source_owned: String = source.into();
  let bytecode = compile(
    &source_owned,
    &options,
    &parse_options,
    &mut encoder as *mut dyn BytecodeEncoder,
  );

  // Successful bytecode begins with LBC_VERSION_TARGET (non-zero); error
  // bytecode begins with a NUL marker followed by the message.
  let bytes = bytecode.as_bytes();
  if bytes.first() != Some(&0) {
    return false;
  }

  let message = &bytecode[1..];
  message.ends_with("<eof>")
}

pub unsafe fn run_repl_impl(l: *mut lua_State) {
  unsafe {
    // isocline's `ic_set_history(path, -1)` capped history at its default of 200
    // entries; mirror that via the editor configuration.
    let config = match Config::builder().max_history_size(DEFAULT_HISTORY_ENTRIES) {
      Ok(b) => b.build(),
      Err(_) => return,
    };
    let mut editor: Editor<ReplHelper, FileHistory> = match Editor::with_config(config) {
      Ok(e) => e,
      Err(_) => return,
    };
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
          // First, try the expression shorthand: `return <line>`.
          if run_code(l, &(String::from("return ") + &line)).is_empty() {
            let _ = editor.add_history_entry(line.as_str());
            continue;
          }

          let error = run_code(l, &line);

          // An "<eof>" error means an incomplete chunk slipped through;
          // skip printing and let the next read continue it. (With the
          // rustyline validator this is normally caught before accept.)
          if error.len() >= 5 && error.ends_with("<eof>") {
            continue;
          }

          if !error.is_empty() {
            println!("{}", error);
          }

          let _ = editor.add_history_entry(line.as_str());
        }
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
        Err(_) => break,
      }
    }

    if let Some(ref path) = history_path {
      let _ = editor.save_history(path);
    }
  }
}
