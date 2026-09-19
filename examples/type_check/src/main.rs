//! 用 ulua 对 Luau 脚本做类型检查 —— 补全 compile/eval/check 三件套。
//!
//!     cargo run -p ulua-example-type-check

fn main() {
  // `check` 对源码做类型检查（在已验证的旧求解器上）。源码类型检查干净时
  // 返回 `Ok(())`，否则 `Err` 携带每个类型错误一条 "line: message" 诊断
  // （行号 1-based）。
  let clean = r#"
        local function add(a: number, b: number): number
            return a + b
        end

        local total: number = add(2, 3)
        print(total)
    "#;

  match ulua::check(clean) {
    Ok(()) => println!("clean snippet: type-checks clean"),
    Err(errors) => {
      println!("clean snippet: unexpected diagnostics:");
      for err in &errors {
        println!("  {err}");
      }
    }
  }

  // 一个刻意类型错误的片段：把 string 赋给 `number`。
  let bad = r#"
        local x: number = "not a number"
        print(x)
    "#;

  match ulua::check(bad) {
    Ok(()) => println!("bad snippet: unexpectedly type-checked clean"),
    Err(errors) => {
      println!("bad snippet: {} type error(s):", errors.len());
      for err in &errors {
        println!("  {err}");
      }
    }
  }
}
