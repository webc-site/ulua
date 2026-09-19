use core::{ffi::c_char, ptr::null_mut};
use std::ffi::CString;

use ulua_cli_lib::functions::{
  normalize_path::normalize_path, read_file::read_file, setup_arguments::setup_arguments,
};
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_l_sandboxthread::lua_l_sandboxthread, lua_newthread::lua_newthread, lua_resume::lua_resume,
    luau_load::luau_load,
  },
  macros::lua_pop::lua_pop,
  type_aliases::lua_state::lua_State,
};

use crate::functions::{
  compile_source::compile_source, counters_active::counters_active, counters_track::counters_track,
  coverage_active::coverage_active, coverage_track::coverage_track,
  error_with_trace::error_with_trace, get_file_path::get_file_path,
  repl_main::repl_codegen_enabled, run_repl_impl::run_repl_impl,
};

/// # Safety
///
/// `gl` must be a valid, active pointer to a `lua_State`.
// `repl` is used to indicate if a repl should be started after executing the file.
// `program_args` 是 `--program-args` 之后的原样参数（cpp 的 `program_argv/argc`）。
pub unsafe fn run_file(
  name: &str,
  gl: *mut lua_State,
  repl: bool,
  program_args: &[impl AsRef<str>],
) -> bool {
  unsafe {
    // cpp `readFile(getFilePath(name))`：读文件走 getFilePath 的 .luau/.lua 回退，
    // 失败信息仍打印用户给定的原始名字（chunkname 同样基于它）。
    let Some(source) = get_file_path(name).and_then(|path| read_file(&path)) else {
      eprintln!("Error opening {}", name);
      return false;
    };

    // module needs to run in a new thread, isolated from the rest
    let l = lua_newthread(gl);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(l);

    // cpp Repl.cpp:604 `("@" + normalizePath(name)).c_str()`：由 CString 负责终止符
    let chunkname = CString::new(format!("@{}", normalize_path(name))).unwrap_or_default();

    let bytecode = compile_source(&source);

    let status: i32 = if luau_load(
      l,
      chunkname.as_ptr(),
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

      let p_args_len = program_args.len() as i32;
      setup_arguments(l, program_args);
      lua_resume(l, null_mut(), p_args_len)
    } else {
      LuaStatus::ErrSyntax as i32
    };

    if status != 0 {
      let error = error_with_trace(l, status, "\nstacktrace:\n");
      eprint!("{}", error);
    }

    if repl {
      run_repl_impl(l);
    }
    lua_pop(gl, 1);
    status == 0
  }
}
