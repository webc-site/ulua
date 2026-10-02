//! cpp `ReplRequirer.cpp` 的 `static load`：在新线程上编译并运行被 require 的模块。
//!
//! 审计结论（工单 r13-w1e）：装载驱动 `load` 本身为安全 fn；确需的 `unsafe` 全部
//! 落在 ulua-vm / ulua-require c-API 边界，收敛为本文件带 `# Safety` 契约的最小
//! 私有封装（`spawn_module_thread` / `throw` / `prepare` / `check_run`）与三个单步
//! 边界调用（`luau_load` / `resume` / `lua_xmove`）。文件读取走 `std::fs`
//! （`read_file`），字符串入参 `&[u8]`、出边界即转 owned `Cow<str>`，无 C 串管道
//! （review.md §2/§3/§10）。

use core::{fmt::Arguments, ptr::null};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_cli_lib::functions::read_file::read_file;
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_common::fflag::LuauCyclicRequireShortCircuit;
use ulua_compiler::functions::compile::compile;
use ulua_require::functions::cyclic_placeholder::luarequire_createplaceholder;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_isstring::lua_isstring, lua_l_error_l::lua_l_error_l,
    lua_l_sandboxthread::lua_l_sandboxthread, lua_mainthread::lua_mainthread,
    lua_newthread::lua_newthread, lua_usesexport::lua_usesexport, lua_xmove::lua_xmove,
    luau_load::luau_load,
  },
  records::lua_state::LuaState,
};

use crate::{
  functions::state_ref::state,
  records::repl_requirer::{ReplRequirer, host_str},
};

// `luau_load`/`lua_resume` 的返回码常量（免散落字面 0 与逐处 `as` 转型）
const OK: i32 = LuaStatus::Ok as i32;
const YIELD: i32 = LuaStatus::Yield as i32;

/// FFI 边界（ulua-vm c-API）：开模块线程的不可拆四步序列——沿 require 帧回溯主
/// 线程 gl → 在 gl 上新建线程（令其不继承 l 的环境）→ `lua_xmove` 把线程移到 l
/// 栈顶槽位持有 → 对新线程沙箱化全局。单独看任何一步都不成立：新线程只有在
/// xmove 之后才被 l 的栈槽持有，故契约合并为一条，免逐行重复论证。
///
/// # Safety
///
/// `l` 为 require 调用帧的活跃 `LuaState`（受保护帧、有栈余量）；返回的线程指针
/// 自此由 l 的栈槽持有，与 l 在整个后续装载窗口内共同存活。
unsafe fn spawn_module_thread(l: &mut LuaState) -> *mut LuaState {
  // Safety: 四步的同 VM、双状态存活前提由本 fn 契约共同覆盖。
  unsafe {
    let gl = lua_mainthread(l);
    let ml = lua_newthread(gl);
    lua_xmove(gl, l, 1);
    lua_l_sandboxthread(ml);
    ml
  }
}

/// FFI 边界（ulua-vm c-API）：在 `l` 上按格式化消息抛出 Lua 错误（发散，不返回）。
///
/// # Safety
///
/// `l` 为活跃且受保护的调用状态、栈上预留 ≥2 空槽（`lua_l_error_l` 的契约前置）；
/// `msg` 为纯 Rust 格式化串，在被调窗口内消费、无逃逸借用。
unsafe fn throw(l: *mut LuaState, msg: Arguments<'_>) -> ! {
  // FFI: c-API 要求 NULL —— `lua_l_error_l` 的 fmt 空指针是「已用 format_args!
  // 组装」的协议形态，本文件仅此一处经手，luaL_error! 宏的展开点随之消失。
  // Safety: 前置条件即本 fn 契约。
  unsafe { lua_l_error_l(l, null(), msg) }
}

/// FFI 边界（ulua-vm / ulua-require c-API）：`luau_load` 成功后、运行前的模块
/// 预处理——export 模块在循环 require 短路开关开启时先建占位表入缓存（循环
/// require 方经 `isCached` 拿到占位表而非重入本模块，cpp ReplRequirer.cpp:170）；
/// codegen 开启时就地编译闭包（改写该原型，不产生跨帧指针）。
///
/// # Safety
///
/// `l` 为 require 调用帧活跃状态且栈槽 2 为 `lua_requireinternal` 建立的 cacheKey
/// 串；`ml` 为存活模块线程、其 -1 槽为 `luau_load` 刚压入的闭包值。
unsafe fn prepare(l: *mut LuaState, ml: *mut LuaState, codegen: bool) {
  // Safety: 三处导出的栈布局前提由本 fn 契约逐条覆盖。
  unsafe {
    if LuauCyclicRequireShortCircuit.get() && lua_usesexport(ml, -1) != 0 {
      luarequire_createplaceholder(l);
    }
    if codegen {
      // The Rust codegen port exposes `luau_codegen_compile(l, idx)`; the
      // native CompilationOptions (CodeGenColdFunctions / recordCounters)
      // are not threaded through the public Rust API.
      luau_codegen_compile(ml, -1);
    }
  }
}

/// FFI 边界（ulua-vm c-API）：把模块闭包的运行结果映射为 Lua 错误——运行成功但
/// 返回值不唯一、模块 yield、或错误对象非串时抛对应错误（发散）；失败且错误为
/// 串时拼消息抛出；全部通过则安静返回。`get_top`/`lua_isstring`/`to_str` 均为
/// ulua-vm 安全方法，边界实质只在各 `throw` 抛出点。
///
/// # Safety
///
/// `l` 为 require 调用帧活跃状态（`throw` 前置）；`ml` 为 resume 已返回、仍由 l
/// 栈槽持有的存活模块线程，run_status 失败时其 -1 槽为错误对象。
unsafe fn check_run(l: *mut LuaState, ml: &mut LuaState, run_status: i32) {
  if run_status == OK {
    // ml 是 resume 已返回、仍由 l 栈槽持有的存活线程，-1 为模块返回值。
    if ml.get_top() != 1 {
      // Safety: l 活跃即 `throw` 契约全部前置（承本 fn 契约）。
      unsafe { throw(l, format_args!("module must return a single value")) };
    }
  } else if run_status == YIELD {
    // Safety: 同上。
    unsafe { throw(l, format_args!("module can not yield")) };
  } else if lua_isstring(ml, -1) == 0 {
    // Safety: 同上。
    unsafe { throw(l, format_args!("unknown error while running module")) };
  } else {
    let msg = ml.to_str(-1).unwrap_or_default();
    // Safety: msg 已转 Rust 借用，抛出不再依赖其它外部内存。
    unsafe { throw(l, format_args!("error while running module: {msg}")) };
  }
}

/// 对应 cpp `load` 回调体：宿主在 require 同步执行窗口内调用（由
/// `RequireHost::load` 的 `ReplRequirer` 实现转交）。
///
// DELIBERATE DEVIATION（review.md §9.3）：require 宿主的模块装载驱动全程在
// `*mut LuaState` 句柄上开线程/xmove/resume（ulua-vm c-API）；边界的 `unsafe`
// 已收敛为上列带 `# Safety` 契约的最小私有封装，`load` 自身为安全 fn，字节串
// 入参在本边界一次性转 owned，后续只见 Rust 类型。
pub(crate) fn load(
  req: &ReplRequirer,
  l: *mut LuaState,
  _path: &[u8],
  chunkname: &[u8],
  loadname: &[u8],
) -> i32 {
  // Safety: `state` 门面契约——`l` 为 ulua-require 在 require 同步窗口内交出的
  // 活跃句柄，单线程驱动、借用窗口内无并存可变别名。
  let l = state(l);
  // module needs to run in a new thread, isolated from the rest
  // note: we create ML on main thread so that it doesn't inherit environment of l
  // Safety: `spawn_module_thread` 前置即本入口的 l 活跃契约。
  let ml = state(unsafe { spawn_module_thread(l) });

  // Safety: loadname/chunkname 同源于 require 链路压栈的 VM 串字节（调用帧持有、
  // 本帧窗口内可读），此步按 cpp C 串消费规则（首 NUL 截断 + lossy）一次性转成
  // owned `Cow<str>`，后续逻辑只见 Rust 字符串。
  let (loadname, chunkname) = (host_str(loadname), host_str(chunkname));

  // cpp: `if (!contents) return luaL_error(L, "could not read file '%s'", loadName);`
  let Some(source) = read_file(&loadname) else {
    // Safety: l 是 require 调用帧的活跃状态，实参 loadname 已是 owned Cow，
    // 格式化路径不再触碰任何外部指针（`throw` 契约前置）。
    unsafe { throw(l, format_args!("could not read file '{}'", loadname)) }
  };

  // now we can compile & run module on the new thread
  // (req.copts 与 crate copts 来源不同, 保留独立编译调用)
  let options = (req.copts)();
  let bytecode = compile(&source, &options, &ParseOptions::default(), NoopEncoder);
  // Safety: `luau_load` 为 unsafe 导出；ml 为存活线程，chunkname 已在上一步转
  // owned 借用，bytecode 是本帧 Vec（长度自洽、本帧存活），仅在调用窗口内被
  // 完整借用。
  let load_status = unsafe { luau_load(ml, &chunkname, &bytecode, 0) };

  if load_status == OK {
    // Safety: `prepare` 契约成立——ml 的 -1 槽是刚压入的闭包，l 的栈槽 2 为
    // lua_requireinternal 建立的 cacheKey。
    unsafe { prepare(l, ml, req.codegen_enable()) };

    if req.coverage_active() {
      (req.coverage_track)(ml, -1);
    }

    if req.counters_active() {
      (req.counters_track)(ml, -1);
    }

    // Safety: `LuaState::resume` 为 unsafe 方法；ml 为持有唯一待执行闭包的存活
    // 线程，from=l 是其父线程（Lua/C API resume 配对），nargs=0 与栈中参数一致。
    let run_status = unsafe { ml.resume(l, 0) };
    // Safety: `check_run` 契约——ml 仍由 l 栈槽持有，run_status 为 resume 返回值。
    unsafe { check_run(l, ml, run_status) };
  }

  // add ML result to l stack, then remove the ML thread slot
  //
  // 与 `spawn_module_thread` 里 `lua_xmove(gl, l, 1)` 配平的收尾序列，必须整体看：
  // xmove 之后 l 栈顶是 [thread, result]，`remove(l, -2)` 弹走的正是该线程槽，
  // 只留 result。
  // Safety: ml 与 l 同属一个 VM 且都存活（xmove 前提成立）；-1 值移出后线程槽
  // 已无引用需求（值已移出），remove 只搬运栈槽、不读已失效内存。
  unsafe { lua_xmove(ml, l, 1) };
  // remove ML thread from l stack
  l.remove(-2);

  // added one value to l stack: module result
  1
}
