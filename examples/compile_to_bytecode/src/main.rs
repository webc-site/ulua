//! 把 Luau 源码编译为字节码（不运行）。
//!
//!     cargo run -p ulua-example-compile-to-bytecode

use std::process::exit;

fn main() {
  let source = "local x = 41\nreturn x + 1";

  match ulua::compile(source) {
    Ok(bytecode) => {
      // `bytecode` 与 `luau_load` 消费的是同一个 blob。首字节是
      // 字节码版本目标。
      println!("compiled {} bytes of bytecode", bytecode.len());
      println!("version byte: {}", bytecode.first().copied().unwrap_or(0));
    }
    Err(message) => {
      // 解析/编译错误以人类可读消息返回。
      eprintln!("compile error: {message}");
      exit(1);
    }
  }

  // 语法错误以 `Err` 出现，不是 panic：
  match ulua::compile("local = = 3") {
    Ok(_) => unreachable!("that should not have compiled"),
    Err(message) => println!("(expected) syntax error: {message}"),
  }
}
