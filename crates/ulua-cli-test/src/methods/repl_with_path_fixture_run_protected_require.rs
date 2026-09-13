use alloc::string::String;

use crate::{functions::run_code::run_code, records::repl_with_path_fixture::ReplWithPathFixture};

pub fn repl_with_path_fixture_run_protected_require(
  fixture: &ReplWithPathFixture,
  path: &str,
) -> String {
  let code = format!("return pcall(function() return require(\"{}\") end)", path);
  unsafe { run_code(fixture.lua_state as *mut _, &code) }
}
