//! Prove `install_afl_panic_hook` routes correctly:
//!   verify_hook compileerror  -> CompileError unwinds + is catchable (exit 0)
//!   verify_hook realpanic     -> any other panic aborts (SIGABRT, exit 134)
//! Run both and check exit codes — confirms real bugs still surface to AFL while
//! the compiler's intentional CompileError throw does not become a false crash.
use std::{env::args, panic::catch_unwind, process::exit};

use ulua_ast::records::location::Location;
use ulua_compiler::records::compile_error::CompileError;

fn main() {
  ulua_fuzz::install_afl_panic_hook();
  match args().nth(1).as_deref() {
    Some("compileerror") => {
      let r = catch_unwind(|| {
        // the real raise path: panic_any(CompileError)
        // (raise 已改为 CompileError 关联函数，接受 &Location)
        CompileError::raise(&Location::default(), format_args!("boom"));
      });
      // hook returned (no abort) -> the panic unwound and was caught here.
      eprintln!("compileerror: caught = {}", r.is_err());
      exit(0);
    }
    Some("realpanic") => {
      // hook should abort BEFORE this catch_unwind ever sees it.
      let _ = catch_unwind(|| panic!("genuine bug"));
      eprintln!("realpanic: NOT aborted (BUG — hook let a real panic through)");
      exit(0);
    }
    _ => {
      eprintln!("usage: verify_hook compileerror|realpanic");
      exit(2);
    }
  }
}
