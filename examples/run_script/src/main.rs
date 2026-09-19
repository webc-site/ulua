//! 在全新 VM 上跑一个 Luau 脚本 —— 使用 ulua 最简单的方式。
//!
//!     cargo run -p ulua-example-run-script

use std::process::exit;

fn main() {
  // `eval` 编译源码，开一个带标准库的全新 VM，并运行 chunk
  // （与 `luau` CLI 完全一致）。失败时返回 Lua 错误字符串。
  let source = r#"
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

  if let Err(err) = ulua::eval(source) {
    eprintln!("script error: {err}");
    exit(1);
  }

  // 运行时错误以 `Err` 返回，消息与 CLI 打印的一致。
  if let Err(err) = ulua::eval("error('something went wrong')") {
    println!("(expected) caught runtime error: {err}");
  }
}
