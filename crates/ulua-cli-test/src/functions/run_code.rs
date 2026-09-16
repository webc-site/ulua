use alloc::string::String;
use core::{
  ffi::{c_char, c_void},
  ptr::null_mut,
  slice::from_raw_parts,
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
      let error = stack_error(l);
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
      let error = stack_error(thread);

      lua_pop(l, 1);
      error
    }
  }
}

/// 读取指定状态栈顶的错误字符串（对应 cpp `std::string error(msg, len)`）；
/// 错误消息可能回显源码中的任意字节，lossy 转换替代 UB
unsafe fn stack_error(state: *mut lua_State) -> String {
  let mut len = 0usize;
  let msg = unsafe { lua_tolstring(state, -1, &mut len) };
  if msg.is_null() {
    String::new()
  } else {
    // SAFETY: lua_tolstring 返回 len 字节缓冲
    String::from_utf8_lossy(unsafe { from_raw_parts(msg as *const u8, len) }).into_owned()
  }
}

unsafe extern "C" {
  fn free(ptr: *mut c_void);
}
