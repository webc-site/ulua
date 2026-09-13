use std::{env, process::exit};

use ulua_cli_lib::functions::set_luau_flags_default::set_luau_flags_default;

use crate::functions::repl_main::repl_main;

pub fn main() {
  set_luau_flags_default();

  let args: Vec<String> = env::args().collect();
  let exit_code = repl_main(&args);
  exit(exit_code);
}
