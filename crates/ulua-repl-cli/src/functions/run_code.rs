use alloc::string::String;
use core::{
  ffi::{CStr, c_char},
  ptr::null_mut,
  slice::from_raw_parts,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
use ulua_compiler::functions::compile::compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_checkstack::lua_checkstack, lua_debugtrace::lua_debugtrace, lua_gettop::lua_gettop,
    lua_insert::lua_insert, lua_l_checkstack::lua_l_checkstack, lua_newthread::lua_newthread,
    lua_pcall::lua_pcall, lua_pushvalue::lua_pushvalue, lua_remove::lua_remove,
    lua_resume::lua_resume, lua_tolstring::lua_tolstring, lua_xmove::lua_xmove,
    luau_load::luau_load,
  },
  macros::{
    lua_getglobal::lua_getglobal, lua_isnil::lua_isnil, lua_minstack::LUA_MINSTACK,
    lua_pop::lua_pop, lua_tostring::lua_tostring,
  },
  type_aliases::lua_state::lua_State,
};

use crate::functions::copts::copts;

/// # Safety
///
/// `l` must be a valid, active pointer to a `lua_State`.
pub unsafe fn run_code(l: *mut lua_State, source: &str) -> String {
  unsafe {
    lua_checkstack(l, LUA_MINSTACK);

    struct NoopEncoder;
    impl BytecodeEncoder for NoopEncoder {
      fn encode(&mut self, _data: &mut [u32]) {}
    }
    let options = copts();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let source_owned: String = source.into();
    let bytecode = compile(
      &source_owned,
      &options,
      &parse_options,
      &mut encoder as *mut dyn BytecodeEncoder,
    );

    if luau_load(
      l,
      c"=stdin".as_ptr(),
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    ) != 0
    {
      let mut len: usize = 0;
      let msg = lua_tolstring(l, -1, &mut len as *mut usize);

      let error = lua_string_from(msg, len);
      lua_pop(l, 1);

      return error;
    }

    let t = lua_newthread(l);

    lua_pushvalue(l, -2);
    lua_remove(l, -3);
    lua_xmove(l, t, 1);

    let status = lua_resume(t, null_mut(), 0);

    if status == 0 {
      let n = lua_gettop(t);

      if n != 0 {
        lua_l_checkstack(t, LUA_MINSTACK, "too many results to print");
        lua_getglobal(t, c"_PRETTYPRINT".as_ptr());
        // If _PRETTYPRINT is nil, then use the standard print function instead
        if lua_isnil!(t, -1) {
          lua_pop(t, 1);
          lua_getglobal(t, c"print".as_ptr());
        }
        lua_insert(t, 1);
        lua_pcall(t, n, 0, 0);
      }

      lua_pop(l, 1);
      String::new()
    } else {
      let mut error: String;

      if status == LuaStatus::Yield as i32 {
        error = "thread yielded unexpectedly".into();
      } else {
        let str_ptr = lua_tostring!(t, -1);
        if !str_ptr.is_null() {
          error = CStr::from_ptr(str_ptr).to_string_lossy().into_owned();
        } else {
          error = String::new();
        }
      }

      error.push_str("\nstack backtrace:\n");
      let trace = lua_debugtrace(t);
      if !trace.is_null() {
        error.push_str(&CStr::from_ptr(trace).to_string_lossy());
      }

      lua_pop(l, 1);
      error
    }
  }
}

// Faithful port of `std::string error(msg, len)`: build a String from the raw
// (pointer, length) pair returned by lua_tolstring.
unsafe fn lua_string_from(msg: *const c_char, len: usize) -> String {
  unsafe {
    if msg.is_null() {
      return String::new();
    }
    let bytes = from_raw_parts(msg as *const u8, len);
    String::from_utf8_lossy(bytes).into_owned()
  }
}
