//! cpp `ReplRequirer.cpp` 的 `static load`：在新线程上编译并运行被 require 的模块。

use core::ffi::{CStr, c_char, c_int, c_void};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_cli_lib::functions::read_file::read_file;
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_compiler::functions::compile::compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_gettop::lua_gettop, lua_isstring::lua_isstring, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_mainthread::lua_mainthread, lua_newthread::lua_newthread, lua_remove::lua_remove,
    lua_resume::lua_resume, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{lua_l_error::luaL_error, lua_tostring::lua_tostring},
  type_aliases::lua_state::lua_State,
};

use crate::records::repl_requirer::ReplRequirer;

/// # Safety
///
/// `l` 为有效的 `lua_State`；`ctx` 必须指向有效的 `ReplRequirer`；
/// `chunkname`/`loadname` 必须是有效 NUL 结尾串。
pub unsafe extern "C-unwind" fn load(
  l: *mut c_void,
  ctx: *mut c_void,
  _path: *const c_char,
  chunkname: *const c_char,
  loadname: *const c_char,
) -> c_int {
  let l = l as *mut lua_State;
  let req = unsafe { &*(ctx as *const ReplRequirer) };

  // module needs to run in a new thread, isolated from the rest
  // note: we create ML on main thread so that it doesn't inherit environment of l
  let gl = unsafe { lua_mainthread(l) };
  let ml = unsafe { lua_newthread(gl) };
  unsafe { lua_xmove(gl, l, 1) };

  // new thread needs to have the globals sandboxed
  unsafe { lua_l_sandboxthread(ml) };

  let loadname = unsafe { CStr::from_ptr(loadname) }.to_string_lossy();

  let contents = read_file(&loadname);
  let had_contents = contents.is_some();
  let mut status: c_int = LuaStatus::Ok as c_int;

  if let Some(ref source) = contents {
    // now we can compile & run module on the new thread
    // (req.copts 与 crate copts 来源不同, 保留独立编译调用)
    let options = (req.copts)();
    let parse_options = ParseOptions::default();
    let bytecode = compile(source, &options, &parse_options, NoopEncoder);
    status = unsafe {
      luau_load(
        ml,
        chunkname,
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
      )
    };
  }

  if !had_contents {
    unsafe { luaL_error!(l, "could not read file '{}'", loadname) };
  }

  if status == 0 {
    // cpp（ReplRequirer.cpp:170）此处为
    //   `if (FFlag::LuauCyclicRequireShortCircuit && lua_usesexport(ML, -1))
    //        luarequire_createplaceholder(L);`
    // 依赖的三个部件都未移植：FFlag LuauCyclicRequireShortCircuit、VM 的
    // lua_usesexport、Require 的 createPlaceholder（跨 crate 缺口）。缺它时循环
    // require + `export` 模块会按旧路径重入，而非拿到占位表；补齐全链路后再回填。
    if req.codegen_enable() {
      // The Rust codegen port exposes `luau_codegen_compile(l, idx)`; the
      // native CompilationOptions (CodeGenColdFunctions / recordCounters)
      // are not threaded through the public Rust API.
      unsafe { luau_codegen_compile(ml, -1) };
    }

    if req.coverage_active() {
      (req.coverage_track)(ml, -1);
    }

    if req.counters_active() {
      (req.counters_track)(ml, -1);
    }

    let status = unsafe { lua_resume(ml, l, 0) };

    if status == 0 {
      if unsafe { lua_gettop(ml) } != 1 {
        unsafe { luaL_error!(l, "module must return a single value") };
      }
    } else if status == LuaStatus::Yield as c_int {
      unsafe { luaL_error!(l, "module can not yield") };
    } else if unsafe { lua_isstring(ml, -1) } == 0 {
      unsafe { luaL_error!(l, "unknown error while running module") };
    } else {
      let msg = unsafe { lua_tostring!(ml, -1) };
      let msg = unsafe { CStr::from_ptr(msg) }.to_string_lossy();
      unsafe { luaL_error!(l, "error while running module: {}", msg) };
    }
  }

  // add ML result to l stack
  unsafe { lua_xmove(ml, l, 1) };

  // remove ML thread from l stack
  unsafe { lua_remove(l, -2) };

  // added one value to l stack: module result
  1
}
