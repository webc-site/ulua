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
  repl_main::repl_codegen_enabled, run_repl_impl::run_repl_impl,
};

// `luau_load`/`lua_resume` 的成功返回码（免散落字面 0）
const OK: i32 = LuaStatus::Ok as i32;

/// FFI 边界（ulua-vm c-API）：在 `gl` 上新建线程并即时沙箱化其全局——两步成对出现，
/// 新线程只有经 `lua_l_sandboxthread` 隔离全局后才能被脚本运行，单独看任一步都不成立
/// （沿用 w1e `load.rs` 的 `spawn_module_thread` 多步收口体例，此处为 runFile 专用的
/// 简化两步版）。
///
/// 调用序契约（正确性，非内存安全——`gl` 的存活/独占前提已由 `&mut` 接收者类型承载，
/// review.md §2 诚实降级为安全 `fn`）：`gl` 为存活主状态；返回的线程指针自此由 `gl`
/// 的栈槽持有（调用方以 `gl.pop(1)` 配平），与 `gl` 在整个脚本运行窗口内共同存活。
/// 体内两步 `lua_*` c-API 仍为 ulua-vm `unsafe` 导出，裸操作下沉为下方单个窄 `unsafe`
/// 块（`// SAFETY:` 就地论证）。
fn sandboxed_thread(gl: &mut LuaState) -> *mut LuaState {
  // SAFETY: 两步均为 ulua-vm c-API 导出，`lua_newthread` 交出的 `l` 与 `gl` 同属
  // 存活主状态窗口，`&mut *l` 重借窗止于当句。
  unsafe {
    let l = lua_newthread(gl);
    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(&mut *l);
    l
  }
}

/// 调用契约（本 fn 为 `pub(crate)` 安全 fn，crate 内唯一调用方 repl_main 保证）：`gl`
/// 为 repl_main 在其守卫句柄的唯一物化点交出的存活、有效 `LuaState` 主状态借用。
// review.md §2/§3 收形：`gl` 由裸 `*mut LuaState` 收编为借用 `&mut LuaState`，真实物化
// 点上移到 repl_main 入口一次。新线程上编译并运行脚本的 VM 驱动
// （newthread/sandboxthread/luau_load/codegen/coverage/counters/resume 全为 ulua-vm
// c-API）；不可拆的 newthread+sandboxthread 两步收在私有 `// SAFETY:` 封装
// `sandboxed_thread`，单步 c-API 调用以带 `// Safety:` 论证的最小 `unsafe` 块就地使用，
// 线程句柄只在下方一处 `unsafe { &mut *.. }` 物化点折成借用。主线程恢复的 `from == NULL`
// 已由 `resume_main` 门面收口，本处不留裸 null。
// `repl` is used to indicate if a repl should be started after executing the file.
// `program_args` 是 `--program-args` 之后的原样参数（cpp 的 `program_argv/argc`）。
pub(crate) fn run_file(
  name: &str,
  gl: &mut LuaState,
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
  // `sandboxed_thread` 收 `&mut` 借用为安全 fn，其 c-API 边界 unsafe 已下沉体内窄块；
  // 交出的新线程句柄在下方唯一的裸指针物化点折成带调用窗口生命周期的借用（本 port 内
  // 该句柄与 `gl` 是两个互不重叠的 LuaState 对象，故此处刻意不经 `gl` 再借用派生）。
  // Safety: `sandboxed_thread` 的调用序契约——句柄由 `gl` 栈槽持有、与 `gl` 共同存活至
  // 本函数末尾 `gl.pop(1)` 配平；REPL 单线程驱动，该借用存活期内无并存可变别名。
  let l = unsafe { &mut *sandboxed_thread(gl) };

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
