use alloc::string::String;
use core::{
  ffi::{c_char, c_void},
  ptr::null_mut,
  slice::from_raw_parts,
  str::{from_utf8, from_utf8_unchecked},
};

use ulua_compiler::functions::luau_compile::luau_compile;
/// Runs the specified Lua source code in state `l`.
///
use ulua_vm::enums::lua_type::LuaType;
use ulua_vm::{
  functions::{
    lua_checkstack::lua_checkstack, lua_gettop::lua_gettop, lua_insert::lua_insert,
    lua_l_checkstack::lua_l_checkstack, lua_newthread::lua_newthread, lua_pcall::lua_pcall,
    lua_pushvalue::lua_pushvalue, lua_remove::lua_remove, lua_resume::lua_resume,
    lua_tolstring::lua_tolstring, lua_type::lua_type, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{lua_getglobal::lua_getglobal, lua_minstack::LUA_MINSTACK, lua_pop::lua_pop},
  records::lua_state::lua_State,
};
/// # Safety
///
/// `l` must be a valid pointer to an initialized `lua_State`.
pub unsafe fn run_code(l: *mut lua_State, source: &str) -> String {
  unsafe {
    lua_checkstack(l, LUA_MINSTACK);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );

    let status = luau_load(l, c"=stdin".as_ptr(), bytecode, bytecode_size, 0);

    free(bytecode as *mut c_void);

    if status != 0 {
      let mut len = 0usize;
      let msg = lua_tolstring(l, -1, &mut len);
      let error = if msg.is_null() {
        String::new()
      } else {
        from_utf8(from_raw_parts(msg as *const u8, len))
          .unwrap_or("")
          .to_string()
      };
      lua_pop(l, 1);
      return error;
    }

    let thread = lua_newthread(l);

    lua_pushvalue(l, -2);
    lua_remove(l, -3);
    lua_xmove(l, thread, 1);

    let status = lua_resume(thread, null_mut(), 0);

    if status == 0 {
      let n = lua_gettop(thread);

      if n != 0 {
        lua_l_checkstack(thread, LUA_MINSTACK, "too many results to print");
        lua_getglobal(thread, c"_PRETTYPRINT".as_ptr());

        if lua_type(thread, -1) == LuaType::Nil as i32 {
          lua_pop(thread, 1);
          lua_getglobal(thread, c"print".as_ptr());
        }

        lua_insert(thread, 1);
        lua_pcall(thread, n, 0, 0);
      }

      lua_pop(l, 1);
      String::new()
    } else {
      let mut len = 0usize;
      let msg = lua_tolstring(thread, -1, &mut len);
      let error = from_utf8_unchecked(from_raw_parts(msg as *const u8, len)).to_string();

      lua_pop(l, 1);
      error
    }
  }
}

unsafe extern "C" {
  fn free(ptr: *mut c_void);
}
