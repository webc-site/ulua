use alloc::string::String;
use core::{ffi::c_char, ptr::null_mut, slice::from_raw_parts};

use ulua_vm::{
  functions::{
    lua_checkstack::lua_checkstack, lua_gettop::lua_gettop, lua_insert::lua_insert,
    lua_l_checkstack::lua_l_checkstack, lua_newthread::lua_newthread, lua_pcall::lua_pcall,
    lua_pushvalue::lua_pushvalue, lua_remove::lua_remove, lua_resume::lua_resume,
    lua_tolstring::lua_tolstring, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{
    lua_getglobal::lua_getglobal, lua_isnil::lua_isnil, lua_minstack::LUA_MINSTACK,
    lua_pop::lua_pop,
  },
  type_aliases::lua_state::lua_State,
};

use crate::functions::{compile_source::compile_source, error_with_trace::error_with_trace};

/// 运行 `source`：成功返回 `None`，失败返回 `Some(错误文本)`。
///
/// cpp `Repl.cpp:239` 的 `runCode` 用空串当成功哨兵；这里用 `Option` 表达同一
/// 语义，避免调用方把「错误文本恰为空」误判成成功。
///
/// # Safety
///
/// `l` must be a valid, active pointer to a `lua_State`.
pub unsafe fn run_code(l: *mut lua_State, source: &str) -> Option<String> {
  unsafe {
    lua_checkstack(l, LUA_MINSTACK);

    let bytecode = compile_source(source);

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

      return Some(error);
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
      None
    } else {
      let error = error_with_trace(t, status, "\nstack backtrace:\n");

      lua_pop(l, 1);
      Some(error)
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
