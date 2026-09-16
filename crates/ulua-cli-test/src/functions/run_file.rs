//! Node: `cxx:Function:Luau.CLI.Test:CLI/src/Repl.cpp:572:run_file`
//! Source: `CLI/src/Repl.cpp:572-647` (faithful port)
//!
//! `repl` indicates whether a REPL should be started after executing the file.
//! No CLI-test invocation passes `-i`, so the `runReplImpl` branch is never
//! reached from these tests.

use alloc::string::String;
use core::{
  ffi::{CStr, c_char, c_void},
  ptr::null_mut,
};

use ulua_cli_lib::functions::{normalize_path::normalize_path, read_file::read_file};
use ulua_compiler::{
  functions::luau_compile::luau_compile, records::lua_compile_options::LuaCompileOptions,
};
use ulua_vm::{
  functions::{
    lua_debugtrace::lua_debugtrace, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_newthread::lua_newthread, lua_resume::lua_resume, lua_tolstring::lua_tolstring,
    luau_load::luau_load,
  },
  macros::lua_pop::lua_pop,
  records::lua_state::lua_State,
};

use crate::functions::{copts::copts, repl_main::program_args, setup_arguments::setup_arguments};

// Status codes from VM/include/lua.h.
const LUA_YIELD: i32 = 1;
const LUA_ERRSYNTAX: i32 = 3;

unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

/// Faithful port of the C++ `runFile`.
pub fn run_file(name: &str, gl: *mut lua_State, repl: bool) -> bool {
  unsafe {
    let source = match read_file(name) {
      Some(s) => s,
      None => {
        eprintln!("Error opening {}", name);
        return false;
      }
    };

    // module needs to run in a new thread, isolated from the rest
    let l = lua_newthread(gl as *mut _);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(l);

    // chunkname = "@" + normalizePath(name)  (NUL-terminated for the C ABI)
    let chunkname = format!("@{}\0", normalize_path(name));

    // Luau::compile(*source, copts()) -> bytecode string.
    let mut options: LuaCompileOptions = copts();
    let mut bytecode_size: usize = 0;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      &mut options as *mut LuaCompileOptions,
      &mut bytecode_size,
    );

    let load_status = luau_load(
      l,
      chunkname.as_ptr() as *const c_char,
      bytecode as *const c_char,
      bytecode_size,
      0,
    );

    free(bytecode as *mut c_void);

    let status = if load_status == 0 {
      // codegen / coverage / counters are inactive in the CLI-test harness
      // (no --codegen flag, no coverageInit/countersInit), so the upstream
      // CodeGen::compile / coverage_track / counters_track branches are not
      // taken here.

      let p_args = program_args();
      setup_arguments(l as *mut _, &p_args);
      lua_resume(l, null_mut(), p_args.len() as i32)
    } else {
      LUA_ERRSYNTAX
    };

    if status != 0 {
      let mut error = String::new();

      if status == LUA_YIELD {
        error.push_str("thread yielded unexpectedly");
      } else {
        let str_ptr = lua_tolstring(l, -1, null_mut());
        if !str_ptr.is_null() {
          error.push_str(&CStr::from_ptr(str_ptr).to_string_lossy());
        }
      }

      error.push_str("\nstacktrace:\n");
      let trace = lua_debugtrace(l);
      if !trace.is_null() {
        error.push_str(&CStr::from_ptr(trace).to_string_lossy());
      }

      eprint!("{}", error);
    }

    if repl {
      // Upstream calls runReplImpl(l) here. No CLI-test invocation passes
      // -i, so this branch is unreachable on the tested path.
      panic!("interactive REPL after file execution is not supported in the CLI-test harness");
    }

    lua_pop(gl, 1);
    status == 0
  }
}
