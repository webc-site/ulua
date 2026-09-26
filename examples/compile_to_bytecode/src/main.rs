//! 把 Luau 源码编译为字节码（不运行）。
//!
//!     cargo run -p ulua-example-compile-to-bytecode

use ulua::Lua;

/// 被编译的源码：`return 41 + 1`。
const SOURCE: &str = "local x = 41\nreturn x + 1";
/// 语法非法的源码，用于演示「错误以 `Err` 返回」。
const BROKEN: &str = "local = = 3";
/// `SOURCE` 的执行结果，同时是断言的期望值。
const EXPECTED: i64 = 42;

/// 示例主流程用 `Result` 串联可失败步骤：任何非预期错误都由 Rust 打印并
/// 以非零码退出，不需要 `unwrap`/`expect` 或手写 `process::exit`。
fn main() -> ulua::Result<()> {
  let bytecode = ulua::compile(SOURCE)?;

  // `bytecode` 与 `luau_load` 消费的是同一个 blob。首字节是
  // 字节码版本目标。
  println!("compiled {} bytes of bytecode", bytecode.len());
  println!("version byte: {}", bytecode.first().copied().unwrap_or(0));

  // 1. 通过便捷函数直接执行预编译字节码
  ulua::eval_bytecode(&bytecode)?;

  // 2. 通过 Lua 宿主环境加载并获取返回值
  let res: i64 = Lua::new().load_bytecode(&bytecode).eval()?;
  println!("bytecode execution result: {res}");
  assert_eq!(res, EXPECTED);

  // 语法错误以 `Err` 出现，不是 panic：
  let err = ulua::compile(BROKEN).expect_err("BROKEN 不应编译成功");
  println!("(expected) syntax error: {err}");
  Ok(())
}
