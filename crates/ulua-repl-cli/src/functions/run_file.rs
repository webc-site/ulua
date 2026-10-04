use ulua_cli_lib::functions::{
  normalize_path::normalize_path, read_file::read_file, report_open_error::report_open_error,
  setup_arguments::setup_arguments,
};
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    error_with_trace::error_with_trace, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_newthread::lua_newthread, luau_load::luau_load,
  },
  records::lua_state::LuaState,
};

use crate::functions::{
  compile_source::compile_source, counters_active::counters_active, counters_track::counters_track,
  coverage_active::coverage_active, coverage_track::coverage_track, get_file_path::get_file_path,
  repl_main::repl_codegen_enabled, run_repl_impl::run_repl_impl, state_ref::state,
};

// `luau_load`/`lua_resume` 的成功返回码（免散落字面 0）
const OK: i32 = LuaStatus::Ok as i32;

/// FFI 边界（ulua-vm c-API）：在 `gl` 上新建线程并即时沙箱化其全局——两步成对出现，
/// 新线程只有经 `lua_l_sandboxthread` 隔离全局后才能被脚本运行，单独看任一步都不成立
/// （沿用 w1e `load.rs` 的 `spawn_module_thread` 多步收口体例，此处为 runFile 专用的
/// 简化两步版）。
///
/// # Safety
///
/// `gl` 为存活主状态；返回的线程指针自此由 `gl` 的栈槽持有（调用方以 `gl.pop(1)` 配平），
/// 与 `gl` 在整个脚本运行窗口内共同存活。
unsafe fn sandboxed_thread(gl: &mut LuaState) -> *mut LuaState {
  // Safety: 两步同属本 fn 契约覆盖的存活主状态窗口。
  unsafe {
    let l = lua_newthread(gl);
    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(&mut *l);
    l
  }
}

/// 调用契约（本 fn 为 `pub(crate)` 安全 fn，crate 内唯一调用方 repl_main 保证）：`gl`
/// 必须指向存活、有效的 `LuaState` 主状态。
// DELIBERATE DEVIATION（review.md §9.3）：在新线程上编译并运行脚本的 VM 驱动
// （newthread/sandboxthread/luau_load/codegen/coverage/counters/resume 全为 ulua-vm
// c-API）；不可拆的 newthread+sandboxthread 两步收在私有 `# Safety` 封装
// `sandboxed_thread`，单步 c-API 调用以带 `// Safety:` 论证的最小 `unsafe` 块就地使用，
// `gl`/线程句柄的解引用关在 `state` 门面内，故本编排入口自身收编为安全 fn。主线程恢复
// 的 `from == NULL` 已由 `resume_main` 门面收口，本处不留裸 null。
// `repl` is used to indicate if a repl should be started after executing the file.
// `program_args` 是 `--program-args` 之后的原样参数（cpp 的 `program_argv/argc`）。
pub(crate) fn run_file(
  name: &str,
  gl: *mut LuaState,
  repl: bool,
  program_args: &[impl AsRef<str>],
) -> bool {
  // Safety: 调用契约保证 `gl` 非空、活跃，经 `state` 门面物化后全走安全方法
  // （单步 unsafe c-API 导出在各块内论证）。
  let gl = state(gl);
  // cpp `readFile(getFilePath(name))`：读文件走 getFilePath 的 .luau/.lua 回退，
  // 失败信息仍打印用户给定的原始名字（chunkname 同样基于它）。
  let Some(source) = get_file_path(name).and_then(|path| read_file(&path)) else {
    report_open_error(name);
    return false;
  };

  // module needs to run in a new thread, isolated from the rest
  // Safety: `sandboxed_thread` 前置即本入口的 gl 存活契约——返回线程由 gl 栈槽持有、
  // 末段 `gl.pop(1)` 配平，并经 `state` 门面物化后栈操作走安全方法。
  let l = state(unsafe { sandboxed_thread(gl) });

  // cpp Repl.cpp:604 `("@" + normalizePath(name)).c_str()`：源名直接以 String
  // 持有，内部 NUL 由 `luau_load` 按 cpp `strlen` 规则截断，无需预补终止符
  let chunkname = format!("@{}", normalize_path(name));
  let bytecode = compile_source(&source);

  // Safety: l 为上方新建且存活的线程；chunkname/bytecode 都是本帧持有的局部值，
  // luau_load 仅在本次调用窗口内借用；成功时 -1 即刚加载的模块函数。
  let status = if unsafe { luau_load(l, &chunkname, &bytecode, 0) } == OK {
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

    let nargs = program_args.len() as i32;
    // setup_arguments 收形为借用后为安全 fn：l 存活，其把 program_args 借用的字符串
    // 压成 VM 实参，nargs 与压入个数一致（其调用序契约）。
    setup_arguments(l, program_args);
    // FFI: c-API 要求 NULL —— 主线程恢复的 `from == NULL` 合法形态已由 ulua-vm
    // `LuaState::resume_main` 门面收口（内部单次落 null），调用侧不再书写裸 null 哨兵。
    l.resume_main(nargs)
  } else {
    LuaStatus::ErrSyntax as i32
  };

  if status != OK {
    // Safety: `error_with_trace` 为 unsafe 导出；l 存活、出错时 -1 为错误对象（其契约）。
    let error = unsafe { error_with_trace(l, status, "\nstacktrace:\n") };
    eprint!("{error}");
  }

  if repl {
    // run_repl_impl 现为 crate 内安全编排 fn；l 在交互循环期间有效且单线程驱动
    // （其文档契约）由本入口的存活前提保证。
    run_repl_impl(l);
  }
  // 与 lua_newthread 在 gl 上留下的线程槽配平。
  gl.pop(1);
  status == OK
}
