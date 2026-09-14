//! Run a Luau script on a fresh VM — the simplest way to use ulua.
//!
//!     cargo run -p ulua-example-run-script

use std::process::exit;
fn main() {
  // `eval` compiles the source, opens a fresh VM with the standard library,
  // and runs the chunk (exactly like the `luau` CLI). It returns the Lua error
  // string on failure.
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

  match ulua::eval(source) {
    Ok(()) => {}
    Err(err) => {
      eprintln!("script error: {err}");
      exit(1);
    }
  }

  // Runtime errors come back as `Err` with the same message the CLI prints.
  if let Err(err) = ulua::eval("error('something went wrong')") {
    println!("(expected) caught runtime error: {err}");
  }
}
