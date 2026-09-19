//! `static std::string runCode(lua_State* l, const std::string& source)`
//! (`CLI/src/Web.cpp:71-140`).
//!
//! Compiles `source`, loads it into `l`, runs it on a fresh thread, prints any
//! results, and returns "" on success or a formatted error (with source:line
//! prefix and stack backtrace) on failure.

use alloc::string::String;
use core::{mem, ptr::null_mut};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_debugtrace::lua_debugtrace, lua_getinfo::lua_getinfo, lua_gettop::lua_gettop,
    lua_insert::lua_insert, lua_l_checkstack::lua_l_checkstack, lua_newthread::lua_newthread,
    lua_pcall::lua_pcall, lua_pushvalue::lua_pushvalue, lua_remove::lua_remove,
    lua_resume::lua_resume, lua_tolstring::lua_tolstring, lua_xmove::lua_xmove,
    luau_load::luau_load,
  },
  macros::{lua_getglobal::lua_getglobal, lua_minstack::LUA_MINSTACK, lua_pop::lua_pop},
  records::lua_debug::LuaDebug,
  type_aliases::lua_state::lua_State,
};

use crate::util::{NOT_ENOUGH_MEMORY, PRINT_NAME, cstr_cow, lua_str_to_string};

/// # Safety
/// `l` must be a valid, non-null pointer to an initialized `lua_State`.
pub unsafe fn run_code(l: *mut lua_State, source: &str) -> String {
  let bytecode = compile(
    source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  unsafe {
    let result = luau_load(
      l,
      c"=stdin".as_ptr(),
      bytecode.as_ptr().cast(),
      bytecode.len(),
      0,
    );
    drop(bytecode);

    if result != 0 {
      // 加载失败：错误信息在栈顶（读取后弹出）。同 cpp `std::string(msg, len)`，
      // 按长度取全字节，内嵌 NUL 不截断。
      let mut len = 0;
      let error = lua_str_to_string(lua_tolstring(l, -1, &mut len), len);
      lua_pop(l, 1);
      return error;
    }

    // lua_State* T = lua_newthread(l);
    let t = lua_newthread(l);
    if t.is_null() {
      // 线程分配失败：cpp 会直接崩溃；此处弹出 luau_load 留下的 chunk 并回报错误，
      // 不解引用空线程。
      lua_pop(l, 1);
      return NOT_ENOUGH_MEMORY.to_string_lossy().into_owned();
    }

    // lua_pushvalue(l, -2);
    // lua_remove(l, -3);
    // lua_xmove(l, T, 1);
    lua_pushvalue(l, -2);
    lua_remove(l, -3);
    lua_xmove(l, t, 1);

    // int status = lua_resume(T, NULL, 0);
    let status = lua_resume(t, null_mut(), 0);

    if status == LuaStatus::Ok as i32 {
      let n = lua_gettop(t);

      if n != 0 {
        lua_l_checkstack(t, LUA_MINSTACK, "too many results to print");
        lua_getglobal(t, PRINT_NAME.as_ptr());
        lua_insert(t, 1);
        lua_pcall(t, n, 0, 0);
      }

      lua_pop(l, 1); // pop T
      String::new()
    } else {
      let mut error = String::new();

      // LuaDebug ar;
      // if (lua_getinfo(l, 0, "sln", &ar))
      // LuaDebug 是纯 POD（指针/整数/数组），全零即全 null/0，合法初值。
      let mut ar: LuaDebug = mem::zeroed();
      if lua_getinfo(l, 0, c"sln".as_ptr(), &mut ar) != 0 {
        error.push_str(&cstr_cow(ar.short_src));
        error.push(':');
        error.push_str(&ar.currentline.to_string());
        error.push_str(": ");
      }

      if status == LuaStatus::Yield as i32 {
        error.push_str("thread yielded unexpectedly");
      } else {
        // else if (const char* str = lua_tostring(T, -1))
        error.push_str(&cstr_cow(lua_tolstring(t, -1, null_mut())));
      }

      error.push_str("\nstack backtrace:\n");
      error.push_str(&cstr_cow(lua_debugtrace(t)));

      lua_pop(l, 1); // pop T
      error
    }
  }
}
