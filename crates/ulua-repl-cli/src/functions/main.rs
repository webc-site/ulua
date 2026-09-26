use std::process::exit;

use ulua_cli_lib::functions::{argv::argv, set_luau_flags_default::set_luau_flags_default};

use crate::functions::repl_main::repl_main;

pub fn main() {
  set_luau_flags_default();

  let exit_code = repl_main(&argv());
  exit(exit_code);
}
