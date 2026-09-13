//! Compile Luau source to bytecode (without running it).
//!
//!     cargo run -p ulua-example-compile-to-bytecode

use std::process::exit;
fn main() {
  let source = "local x = 41\nreturn x + 1";

  match ulua::compile(source) {
    Ok(bytecode) => {
      // `bytecode` is the same blob `luau_load` consumes. The first byte is
      // the bytecode version target.
      println!("compiled {} bytes of bytecode", bytecode.len());
      println!("version byte: {}", bytecode.first().copied().unwrap_or(0));
    }
    Err(message) => {
      // A parse/compile error is returned as the human-readable message.
      eprintln!("compile error: {message}");
      exit(1);
    }
  }

  // Syntax errors surface as `Err`, not a panic:
  match ulua::compile("local = = 3") {
    Ok(_) => unreachable!("that should not have compiled"),
    Err(message) => println!("(expected) syntax error: {message}"),
  }
}
