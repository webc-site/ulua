//! 用 ulua 对 Luau 脚本做类型检查 —— 补全 compile/eval/check 三件套。
//!
//!     cargo run -p ulua-example-type-check

use ulua::TypeDiagnostic;

/// 类型检查干净的片段。
const CLEAN: &str = r#"
    local function add(a: number, b: number): number
        return a + b
    end

    local total: number = add(2, 3)
    print(total)
"#;

/// 刻意类型错误的片段：把 string 赋给 `number`。
const BAD: &str = r#"
    local x: number = "not a number"
    print(x)
"#;

/// 逐条打印诊断（`TypeDiagnostic` 的 Display 即 "行号: 消息"，行号 1-based）。
fn report(label: &str, errors: &[TypeDiagnostic]) {
  println!("{label}: {} diagnostic(s):", errors.len());
  for err in errors {
    println!("  {err}");
  }
}

fn main() {
  // `check` 对源码做类型检查（在已验证的旧求解器上）。源码类型检查干净时
  // 返回 `Ok(())`，否则 `Err` 携带每个类型错误一条诊断。
  match ulua::check(CLEAN) {
    Ok(()) => println!("clean snippet: type-checks clean"),
    Err(errors) => report("clean snippet (unexpected)", &errors),
  }

  // 一个刻意类型错误的片段：把 string 赋给 `number`。
  match ulua::check(BAD) {
    Ok(()) => println!("bad snippet: unexpectedly type-checked clean"),
    Err(errors) => report("bad snippet", &errors),
  }
}
