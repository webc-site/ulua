use alloc::string::String;
use std::env::var;

use ulua_cli_lib::functions::join_paths_file_utils::join_paths_basic_string_ch_ch_ch;

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
pub fn load_history(name: &str) -> Option<String> {
  let mut path = String::new();

  if let Ok(home) = var("HOME") {
    join_paths_basic_string_ch_ch_ch(&mut path, &home, name);
  } else if let Ok(user_profile) = var("USERPROFILE") {
    join_paths_basic_string_ch_ch_ch(&mut path, &user_profile, name);
  }

  if path.is_empty() { None } else { Some(path) }
}

// isocline's default history entry count, selected by passing -1 to ic_set_history.
pub const DEFAULT_HISTORY_ENTRIES: usize = 200;
