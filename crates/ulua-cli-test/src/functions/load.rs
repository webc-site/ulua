use alloc::borrow::Cow;
use core::ffi::{CStr, c_char, c_int, c_void};

use ulua_cli_lib::functions::read_file::read_file;
use ulua_code_gen::functions::luau_codegen_compile::luau_codegen_compile;
use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_gettop::lua_gettop, lua_isstring::lua_isstring, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_mainthread::lua_mainthread, lua_newthread::lua_newthread, lua_remove::lua_remove,
    lua_resume::lua_resume, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{lua_l_error::luaL_error, lua_tostring::lua_tostring},
  records::lua_state::lua_State,
};

use crate::records::repl_requirer::ReplRequirer;
unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

pub unsafe extern "C-unwind" fn load(
  l: *mut c_void,
  ctx: *mut c_void,
  _path: *const c_char,
  chunkname: *const c_char,
  loadname: *const c_char,
) -> c_int {
  unsafe {
    let l = l as *mut lua_State;
    let req = &*(ctx as *const ReplRequirer);

    // module needs to run in a new thread, isolated from the rest
    // note: we create ML on main thread so that it doesn't inherit environment of l
    let gl = lua_mainthread(l);
    let ml = lua_newthread(gl);
    lua_xmove(gl, l, 1);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(ml);

    let contents = read_file(&CStr::from_ptr(loadname).to_string_lossy());
    let had_contents = contents.is_some();
    let mut status: c_int = LuaStatus::Ok as c_int;

    if let Some(contents) = contents {
      // now we can compile & run module on the new thread
      let mut bytecode_size = 0usize;
      let compile_options = (req.copts)();
      let bytecode = luau_compile(
        contents.as_ptr() as *const c_char,
        contents.len(),
        compile_options as *mut _,
        &mut bytecode_size,
      );

      status = luau_load(ml, chunkname, bytecode, bytecode_size, 0);

      free(bytecode as *mut c_void);
    }

    if !had_contents {
      let loadname_str = CStr::from_ptr(loadname).to_string_lossy();
      luaL_error!(l, "could not read file '{}'", loadname_str);
    }

    if status == 0 {
      if req.codegen_enable() {
        luau_codegen_compile(ml, -1);
      }
      if req.coverage_active() {
        (req.coverage_track)(ml, -1);
      }
      if req.counters_active() {
        (req.counters_track)(ml, -1);
      }

      let status = lua_resume(ml, l, 0);

      if status == 0 {
        if lua_gettop(ml) != 1 {
          luaL_error!(l, "module must return a single value");
        }
      } else if status == LuaStatus::Yield as c_int {
        luaL_error!(l, "module can not yield");
      } else if lua_isstring(ml, -1) == 0 {
        luaL_error!(l, "unknown error while running module");
      } else {
        let msg = lua_tostring!(ml, -1);
        let msg_str = if msg.is_null() {
          Cow::Borrowed("")
        } else {
          CStr::from_ptr(msg).to_string_lossy()
        };
        luaL_error!(l, "error while running module: {}", msg_str);
      }
    }

    // add ML result to l stack
    lua_xmove(ml, l, 1);

    // remove ML thread from l stack
    lua_remove(l, -2);

    // added one value to l stack: module result
    1
  }
}
