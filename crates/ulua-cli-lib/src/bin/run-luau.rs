//! Full end-to-end Rust Luau runner.
//!
//! `run_luau <script.luau>` compiles the source with the Rust compiler and
//! executes the resulting bytecode on the Rust VM, in a single process — a
//! working Luau interpreter built entirely from the C++→Rust translation.
//! Unlike the differential-oracle `luau_run` (which loads a precompiled `.bc`
//! and dumps `results: N`), this prints the program's own output and reports
//! compile/runtime errors with a stack traceback, like the real `luau` CLI.

use core::{ffi::c_char, ptr::null_mut, slice::from_raw_parts};
use std::{
  env::args,
  ffi::{CStr, CString},
  fs::File,
  io::Read,
  panic::set_hook,
  process::exit,
};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  functions::{
    lua_debugtrace::lua_debugtrace, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_newthread::lua_newthread, lua_resume::lua_resume, lua_tolstring::lua_tolstring,
    luau_load::luau_load,
  },
  type_aliases::lua_state::lua_State,
};

unsafe fn stack_string(t: *mut lua_State) -> String {
  let mut len = 0usize;
  let s = unsafe { lua_tolstring(t, -1, &mut len) };
  if s.is_null() {
    return "<non-string error>".to_string();
  }
  let bytes = unsafe { from_raw_parts(s as *const u8, len) };
  String::from_utf8_lossy(bytes).into_owned()
}

fn main() {
  // not-yet-perfect VM paths panic; they surface as Lua errors via the VM's
  // protected-call machinery, so keep the raw panic output quiet.
  set_hook(Box::new(|_| {}));

  let path = args().nth(1).unwrap_or_else(|| {
    eprintln!("usage: run_luau <script.luau>");
    exit(2);
  });

  let mut src = Vec::new();
  File::open(&path)
    .unwrap_or_else(|e| {
      eprintln!("run_luau: cannot open {path}: {e}");
      exit(2);
    })
    .read_to_end(&mut src)
    .unwrap_or_else(|e| {
      eprintln!("run_luau: cannot read {path}: {e}");
      exit(2);
    });

  ulua_common::set_all_flags(true);

  // '@' prefix → luaO_chunkid shows the chunk as a plain filename in errors.
  let chunkname = CString::new(format!("@{path}")).expect("path has NUL");

  unsafe {
    // 1) compile with the Rust compiler (null options → -O1/-g1, like the CLI)
    let mut outsize: usize = 0;
    let bc = luau_compile(
      src.as_ptr() as *const c_char,
      src.len(),
      null_mut(),
      &mut outsize,
    );
    if bc.is_null() || outsize == 0 {
      eprintln!("run_luau: compilation produced no bytecode");
      exit(1);
    }

    // 2) VM + a thread to run on (mirrors CLI/src/Repl.cpp)
    let l = lua_l_newstate();
    assert!(!l.is_null(), "lua_l_newstate returned null");
    lua_l_openlibs(l);
    let t = lua_newthread(l);
    assert!(!t.is_null(), "lua_newthread returned null");

    // 3) load — luau_compile encodes compile errors as version-0 "error
    // bytecode", which luau_load reports as a load failure with the message.
    let rc = luau_load(t, chunkname.as_ptr(), bc as *const c_char, outsize, 0);
    if rc != 0 {
      eprintln!("{}", stack_string(t));
      exit(1);
    }

    // 4) run; the program's own print() output goes straight to stdout.
    let status = lua_resume(t, null_mut(), 0);
    if status != 0 {
      eprintln!("{}", stack_string(t));
      let tb = lua_debugtrace(t);
      if !tb.is_null() {
        let trace = CStr::from_ptr(tb).to_string_lossy();
        if !trace.trim().is_empty() {
          eprintln!("stack traceback:\n{}", trace);
        }
      }
      exit(1);
    }
  }
}
