use alloc::string::String;
use core::{
  ffi::{CStr, c_char},
  ptr::null_mut,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
use ulua_cli_lib::functions::{normalize_path::normalize_path, read_file::read_file};
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_compiler::functions::compile::compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_debugtrace::lua_debugtrace, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_newthread::lua_newthread, lua_resume::lua_resume, luau_load::luau_load,
  },
  macros::{lua_pop::lua_pop, lua_tostring::lua_tostring},
  type_aliases::lua_state::lua_State,
};

use crate::functions::{
  copts::copts,
  counters_active::counters_active,
  counters_track::counters_track,
  coverage_active::coverage_active,
  coverage_track::coverage_track,
  repl_main::{program_args, repl_codegen_enabled},
  run_repl_impl::run_repl_impl,
  setup_arguments::setup_arguments,
};

/// # Safety
///
/// `gl` must be a valid, active pointer to a `lua_State`.
// `repl` is used to indicate if a repl should be started after executing the file.
pub unsafe fn run_file(name: &str, gl: *mut lua_State, repl: bool) -> bool {
  unsafe {
    let source = read_file(name);
    let source = match source {
      Some(s) => s,
      None => {
        eprintln!("Error opening {}", name);
        return false;
      }
    };

    // module needs to run in a new thread, isolated from the rest
    let l = lua_newthread(gl);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(l);

    let chunkname = String::from("@") + &normalize_path(name) + "\0";

    struct NoopEncoder;
    impl BytecodeEncoder for NoopEncoder {
      fn encode(&mut self, _data: &mut [u32]) {}
    }
    let options = copts();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let bytecode = compile(
      &source,
      &options,
      &parse_options,
      &mut encoder as *mut dyn BytecodeEncoder,
    );

    let status: i32 = if luau_load(
      l,
      chunkname.as_ptr() as *const c_char,
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    ) == 0
    {
      if repl_codegen_enabled() {
        // C++ sets CodeGenColdFunctions / recordCounters on the
        // CompilationOptions; the public Rust codegen API takes neither.
        luau_codegen_compile(l, -1);
      }

      if coverage_active() {
        coverage_track(l, -1);
      }

      if counters_active() {
        counters_track(l, -1);
      }

      let p_args = program_args();
      let p_args_len = p_args.len() as i32;
      setup_arguments(l, &p_args);
      lua_resume(l, null_mut(), p_args_len)
    } else {
      LuaStatus::ErrSyntax as i32
    };

    if status != 0 {
      let mut error: String;

      if status == LuaStatus::Yield as i32 {
        error = "thread yielded unexpectedly".into();
      } else {
        let str_ptr = lua_tostring!(l, -1);
        if !str_ptr.is_null() {
          error = CStr::from_ptr(str_ptr).to_string_lossy().into_owned();
        } else {
          error = String::new();
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
      run_repl_impl(l);
    }
    lua_pop(gl, 1);
    status == 0
  }
}
