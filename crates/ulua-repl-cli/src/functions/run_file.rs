use core::ptr::null_mut;

use ulua_cli_lib::functions::{
  normalize_path::normalize_path, read_file::read_file, report_open_error::report_open_error,
  setup_arguments::setup_arguments,
};
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_l_sandboxthread::lua_l_sandboxthread, lua_newthread::lua_newthread, lua_resume::lua_resume,
    luau_load::luau_load,
  },
  macros::lua_pop::lua_pop,
  records::lua_state::LuaState,
};

use crate::functions::{
  compile_source::compile_source, counters_active::counters_active, counters_track::counters_track,
  coverage_active::coverage_active, coverage_track::coverage_track,
  error_with_trace::error_with_trace, get_file_path::get_file_path,
  repl_main::repl_codegen_enabled, run_repl_impl::run_repl_impl,
};

/// # Safety
///
/// `gl` must be a valid, active pointer to a `LuaState`.
// `repl` is used to indicate if a repl should be started after executing the file.
// `program_args` 是 `--program-args` 之后的原样参数（cpp 的 `program_argv/argc`）。
pub(crate) unsafe fn run_file(
  name: &str,
  gl: *mut LuaState,
  repl: bool,
  program_args: &[impl AsRef<str>],
) -> bool {
  // cpp `readFile(getFilePath(name))`：读文件走 getFilePath 的 .luau/.lua 回退，
  // 失败信息仍打印用户给定的原始名字（chunkname 同样基于它）。
  let Some(source) = get_file_path(name).and_then(|path| read_file(&path)) else {
    report_open_error(name);
    return false;
  };

  // module needs to run in a new thread, isolated from the rest
  // Safety: gl 为 fn /// # Safety 保证的存活主状态；lua_newthread 返回的线程 l
  // 由 gl 栈槽持有（末段 lua_pop(gl,1) 统一配平），sandboxthread 只操作该新线程。
  let l = unsafe {
    let l = lua_newthread(gl);
    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(l);
    l
  };

  // cpp Repl.cpp:604 `("@" + normalizePath(name)).c_str()`：源名直接以 String
  // 持有，内部 NUL 由 `luau_load` 按 cpp `strlen` 规则截断，无需 CString 终止符
  let chunkname = format!("@{}", normalize_path(name));
  let bytecode = compile_source(&source);

  // Safety: l 为上方新建且存活的线程；chunkname/bytecode 都是本帧持有的局部值，
  // luau_load 仅在本次调用窗口内借用；成功时 -1 即刚加载的模块函数。
  let status: i32 = if unsafe { luau_load(l, &chunkname, &bytecode, 0) } == 0 {
    // Safety: l 存活且 -1 为已加载闭包；三个探针均就地改写该线程栈上原型或
    // 登记回调，不产生跨帧指针。
    if repl_codegen_enabled() {
      // C++ sets CodeGenColdFunctions / recordCounters on the
      // CompilationOptions; the public Rust codegen API takes neither.
      unsafe { luau_codegen_compile(l, -1) };
    }

    if coverage_active() {
      coverage_track(l, -1);
    }

    if counters_active() {
      counters_track(l, -1);
    }

    let p_args_len = program_args.len() as i32;
    // Safety: l 存活；setup_arguments 把 program_args 借用的字符串压成 VM 实参，
    // nargs 与压入个数一致；resume 的 from=null 是 C API 合法形态（主线程无父调用方）。
    unsafe {
      setup_arguments(l, program_args);
      lua_resume(l, null_mut(), p_args_len)
    }
  } else {
    LuaStatus::ErrSyntax as i32
  };

  if status != 0 {
    // Safety: l 存活、出错时 -1 为错误对象（error_with_trace 契约）。
    let error = unsafe { error_with_trace(l, status, "\nstacktrace:\n") };
    eprint!("{error}");
  }

  if repl {
    // Safety: l 在交互循环期间有效且单线程驱动（run_repl_impl 的 /// # Safety）。
    unsafe { run_repl_impl(l) };
  }
  // Safety: 与 lua_newthread 在 gl 上留下的线程槽配平。
  unsafe { lua_pop(gl, 1) };
  status == 0
}
