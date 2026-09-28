//! 在全新 VM 上跑一个 Luau 脚本 —— 使用 ulua 最简单的方式。
//!
//!     cargo run -p ulua-example-run-script

/// 正常脚本：打印问候与 1..10 的和。
const SCRIPT: &str = r#"
    local function greet(name)
        return string.format("hello, %s!", name)
    end
    print(greet("ulua"))

    local sum = 0
    for i = 1, 10 do
        sum += i
    end
    print("sum 1..10 =", sum)
"#;

/// 刻意触发运行时错误的脚本。
const BROKEN: &str = "error('something went wrong')";

/// `fn main() -> Result` 是示例的惯用出口：失败时 Rust 打印 `Error: <消息>`
/// 并以非零码退出，无需手写 `process::exit`。
fn main() -> ulua::Result<()> {
  // `eval` 编译源码，开一个带标准库的全新 VM，并运行 chunk
  // （与 `luau` CLI 完全一致）。失败时返回 Lua 错误字符串。
  ulua::eval(SCRIPT)?;

  // 运行时错误以 `Err` 返回，消息与 CLI 打印的一致 —— 这里它正是期望结果。
  let err = ulua::eval(BROKEN).expect_err("BROKEN 应当以 Err 结束");
  println!("(expected) caught runtime error: {err}");
  Ok(())
}
