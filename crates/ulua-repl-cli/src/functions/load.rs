//! cpp `ReplRequirer.cpp` 的 `static load`：在新线程上编译并运行被 require 的模块。

use core::ffi::{c_char, c_int, c_void};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_cli_lib::functions::read_file::read_file;
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_common::{fflag::LuauCyclicRequireShortCircuit, functions::c_str::cstr_cow};
use ulua_compiler::functions::compile::compile;
use ulua_require::functions::cyclic_placeholder::luarequire_createplaceholder;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_gettop::lua_gettop, lua_isstring::lua_isstring, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_mainthread::lua_mainthread, lua_newthread::lua_newthread, lua_remove::lua_remove,
    lua_resume::lua_resume, lua_usesexport::lua_usesexport, lua_xmove::lua_xmove,
    luau_load::luau_load,
  },
  macros::{lua_l_error::luaL_error, lua_tostring::lua_tostring},
  records::lua_state::LuaState,
};

use crate::functions::requirer_ref::requirer;

/// # Safety
///
/// `l` 为有效的 `LuaState`；`ctx` 必须指向有效的 `ReplRequirer`；
/// `chunkname`/`loadname` 必须是有效 NUL 结尾串。
pub(crate) unsafe extern "C-unwind" fn load(
  l: *mut c_void,
  ctx: *mut c_void,
  _path: *const c_char,
  chunkname: *const c_char,
  loadname: *const c_char,
) -> c_int {
  let l = l.cast::<LuaState>();
  // Safety: ctx 是 create_cli_require_context 经 lua_newuserdatadtor 分配、以地址
  // 为键登记进 registry 的 ReplRequirer userdata 指针，require 链路单线程调用本
  // 回调（requirer 契约），本帧只建共享引用。
  let req = unsafe { requirer(ctx) };

  // module needs to run in a new thread, isolated from the rest
  // note: we create ML on main thread so that it doesn't inherit environment of l
  //
  // 这四步（取主线程 → 在主线程上开新线程 → 移交给 l → 沙箱化）是一个不可拆的
  // 序列：单独看任何一步都不成立（新线程只有在 xmove 之后才被 l 的栈槽持有），
  // 故契约合并为一条，避免逐行重复论证。
  // Safety: l 由本 fn 的 /// # Safety 保证为活跃状态，lua_mainthread 仅沿该状态
  // 回溯主线程 gl（同 VM、同存活）；lua_newthread 在 gl 上创建的线程由紧随其后的
  // lua_xmove 移到 l 栈顶槽位持有（两侧同属一个 VM，栈上此刻恰有 1 个线程值），
  // 因此 gl/ml 在整个后续窗口内都随 l 的栈槽存活；lua_l_sandboxthread 只改写该
  // 活跃线程的环境表，不跨状态。
  let ml = unsafe {
    // note: we create ML on main thread so that it doesn't inherit environment of l
    let gl = lua_mainthread(&*l);
    let ml = lua_newthread(gl);
    lua_xmove(gl, l, 1);
    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(ml);
    ml
  };

  // Safety: loadname/chunkname 同源于 require 链路：lua_requireinternal 在调用 load
  // 前把两者作为 NUL 结尾 VM 串压栈（以 push_c_str / lua_tolstring 交出有效地址），
  // 串由调用帧持有、本帧窗口内可读；二者在此按 cpp `strlen` 规则一次性转成
  // `Cow<str>`（非 UTF-8 均 lossy 替换），后续逻辑只见 Rust 字符串。
  let (loadname, chunkname) = unsafe { (cstr_cow(loadname), cstr_cow(chunkname)) };

  // cpp: `if (!contents) return luaL_error(L, "could not read file '%s'", loadName);`
  let Some(source) = read_file(&loadname) else {
    // Safety: l 是 require 调用帧的活跃状态，栈余量由 VM 的 require 调用点预留
    // （lua_l_error_l 的 /// # Safety 前置：≥2 空槽 + 受保护帧）；实参 loadname 已是
    // owned Cow，格式化路径不再触碰任何外部指针。
    unsafe { luaL_error!(l, "could not read file '{}'", loadname) }
  };

  // now we can compile & run module on the new thread
  // (req.copts 与 crate copts 来源不同, 保留独立编译调用)
  let options = (req.copts)();
  let parse_options = ParseOptions::default();
  let bytecode = compile(&source, &options, &parse_options, NoopEncoder);
  // Safety: ml 为存活线程；chunkname 与 loadname 同理是 require 链路持有的 NUL 结尾 VM 串（已在本函数 FFI 边界转为借用）；bytecode 是本帧 Vec（非空、长度自洽、本帧存活），luau_load 在调用窗口内完整借用。
  let status = unsafe { luau_load(ml, &chunkname, &bytecode, 0) };

  if status == 0 {
    // cpp（ReplRequirer.cpp:170）：`export` 模块先建循环依赖占位表入缓存，
    // 循环 require 方经 `isCached` 拿到占位表而非重入本模块
    // Safety: ml 存活且 luau_load 成功后 -1 是其刚压入的闭包值。
    if LuauCyclicRequireShortCircuit.get() && unsafe { lua_usesexport(ml, -1) } != 0 {
      // Safety: l 存活且栈槽 2 为 lua_requireinternal 建立的 cacheKey 字符串，满足 luarequire_createplaceholder 的 /// # Safety 布局前提。
      unsafe { luarequire_createplaceholder(l) };
    }

    if req.codegen_enable() {
      // The Rust codegen port exposes `luau_codegen_compile(l, idx)`; the
      // native CompilationOptions (CodeGenColdFunctions / recordCounters)
      // are not threaded through the public Rust API.
      // Safety: ml 存活且 -1 为已加载闭包；codegen 编译就地改写该原型，不产生跨帧指针。
      unsafe { luau_codegen_compile(ml, -1) };
    }

    if req.coverage_active() {
      (req.coverage_track)(ml, -1);
    }

    if req.counters_active() {
      (req.counters_track)(ml, -1);
    }

    // Safety: ml 为持有唯一待执行函数的存活线程，from=l 是其父线程（Lua/C API resume 配对），nargs=0 与栈中参数一致。
    let status = unsafe { lua_resume(ml, l, 0) };

    if status == 0 {
      // Safety: ml 是 resume 已返回、仍由 l 栈槽持有的存活线程，-1 为模块返回值。
      if unsafe { lua_gettop(ml) } != 1 {
        // Safety: l 为 require 调用帧的活跃状态（栈余量与受保护帧前提同 luaL_error
        // 的 /// # Safety），本分支不触碰任何外部指针。
        unsafe { luaL_error!(l, "module must return a single value") };
      }
    } else if status == LuaStatus::Yield as c_int {
      // Safety: 同上，l 为活跃调用帧，错误宏按 C API 发散。
      unsafe { luaL_error!(l, "module can not yield") };
    // Safety: ml 存活且 resume 出错后 -1 为错误消息槽位。
    } else if unsafe { lua_isstring(ml, -1) } == 0 {
      // Safety: 同上，l 为活跃调用帧，错误宏按 C API 发散。
      unsafe { luaL_error!(l, "unknown error while running module") };
    } else {
      // Safety: ml 存活，且上一分支的 lua_isstring 已确认 -1 为字符串：
      // lua_tostring 对该槽位返回被栈槽持有的 NUL 结尾指针（非空），cstr_cow 只在
      // 本帧窗口内读取，随后 msg 即为自有的 Rust 文本。
      let msg = unsafe { cstr_cow(lua_tostring!(ml, -1)) };
      // Safety: l 为活跃调用帧；msg 已是 owned Cow，不再有指针依赖。
      unsafe { luaL_error!(l, "error while running module: {}", msg) };
    }
  }

  // add ML result to l stack, then remove the ML thread slot
  //
  // 与开头 `lua_xmove(gl, l, 1)` 配平的收尾序列，必须整体看：xmove 之后 l 栈顶是
  // [thread, result]，`remove(l, -2)` 弹走的正是该线程槽，只留 result。
  // Safety: ml 存活且 -1 为模块返回值，与 l 同属一个 VM（xmove 前提成立）；弹出后
  // 线程对象已无引用需求（值已移出），remove 只搬运栈槽、不读已失效内存。
  unsafe {
    lua_xmove(ml, l, 1);
    // remove ML thread from l stack
    lua_remove(l, -2);
  };

  // added one value to l stack: module result
  1
}
