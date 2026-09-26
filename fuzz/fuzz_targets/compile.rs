// Port of Luau's `fuzz/compiler.cpp` (+ parser.cpp): feed arbitrary bytes as
// source and compile. The compiler must never panic/crash — only return
// `Ok(function)` or `Err(SyntaxError)`. AFL's coverage feedback explores the
// parser + bytecode compiler.

use std::str::from_utf8;

use ulua_rt::Lua;

#[cfg(not(feature = "afl-runtime"))]
include!("standalone.rs");

fn exercise_input(data: &[u8]) {
  if let Ok(src) = from_utf8(data) {
    let lua = Lua::new();
    let _ = lua.load(src).set_name("fuzz").into_function();
  }
}

ulua_fuzz::fuzz_main_hook!(exercise_input);
