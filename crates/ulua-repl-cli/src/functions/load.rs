use alloc::string::String;
use core::ffi::{CStr, c_char, c_void};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
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

pub unsafe fn load(
  l: *mut lua_State,
  ctx: *mut c_void,
  _path: *const c_char,
  chunkname: *const c_char,
  loadname: *const c_char,
) -> i32 {
  unsafe {
    let req = &*(ctx as *const ReplRequirer);

    // module needs to run in a new thread, isolated from the rest
    // note: we create ML on main thread so that it doesn't inherit environment of l
    let gl = lua_mainthread(l);
    let ml = lua_newthread(gl);
    lua_xmove(gl, l, 1);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(ml);

    let loadname_str = CStr::from_ptr(loadname).to_string_lossy();

    let contents = read_file(&loadname_str);
    let had_contents = contents.is_some();
    let mut status: i32 = LuaStatus::Ok as i32;

    if let Some(ref source) = contents {
      // now we can compile & run module on the new thread
      struct NoopEncoder;
      impl BytecodeEncoder for NoopEncoder {
        fn encode(&mut self, _data: &mut [u32]) {}
      }
      let options = (req.copts.unwrap())();
      let parse_options = ParseOptions::default();
      let mut encoder = NoopEncoder;
      let source_owned: String = source.clone();
      let bytecode = compile(
        &source_owned,
        &options,
        &parse_options,
        &mut encoder as *mut dyn BytecodeEncoder,
      );
      status = luau_load(
        ml,
        chunkname,
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
      );
    }

    if !had_contents {
      luaL_error!(l, "could not read file '{}'", loadname_str);
    }

    if status == 0 {
      if (req.codegen_enable.unwrap())() {
        // The Rust codegen port exposes `luau_codegen_compile(l, idx)`; the
        // native CompilationOptions (CodeGenColdFunctions / recordCounters)
        // are not threaded through the public Rust API.
        luau_codegen_compile(ml, -1);
      }

      if (req.coverage_active.unwrap())() {
        (req.coverage_track.unwrap())(ml as *mut c_void, -1);
      }

      if (req.counters_active.unwrap())() {
        (req.counters_track.unwrap())(ml as *mut c_void, -1);
      }

      let status = lua_resume(ml, l, 0);

      if status == 0 {
        if lua_gettop(ml) != 1 {
          luaL_error!(l, "module must return a single value");
        }
      } else if status == LuaStatus::Yield as i32 {
        luaL_error!(l, "module can not yield");
      } else if lua_isstring(ml, -1) == 0 {
        luaL_error!(l, "unknown error while running module");
      } else {
        let msg = lua_tostring!(ml, -1);
        let msg = CStr::from_ptr(msg).to_string_lossy();
        luaL_error!(l, "error while running module: {}", msg);
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
