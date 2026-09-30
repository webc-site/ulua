use alloc::string::String;
use std::env::var;

use ulua_cli_lib::functions::join_paths_file_utils::join_paths;

// Faithful rustyline analog of Repl.cpp's `loadHistory`.
//
// C++ resolved the history file path from $HOME (falling back to $USERPROFILE)
// joined with `name`, then handed it to isocline via `ic_set_history(path, -1)`,
// where -1 selected isocline's default entry count (= 200). isocline then loaded
// the file and automatically saved history on process exit.
//
// rustyline manages history through the `Editor` rather than a global call, so
// this returns the resolved path; `run_repl_impl` loads it, caps it at the
// default 200 entries, and saves it when the loop ends.
pub(crate) fn load_history(name: &str) -> Option<String> {
  // cpp: $HOME 优先，未设置时回退 $USERPROFILE
  let base = var("HOME").ok().or_else(|| var("USERPROFILE").ok())?;
  // cpp Repl.cpp:488 两参 `const char*` 调用决议到 string_view 版重载：
  // rhs 无前导分隔符时才补 '/'（name 为固定历史文件名，不含前导分隔符）
  let path = join_paths(&base, name, false);

  (!path.is_empty()).then_some(path)
}

// isocline's default history entry count, selected by passing -1 to ic_set_history.
pub(crate) const DEFAULT_HISTORY_ENTRIES: usize = 200;
