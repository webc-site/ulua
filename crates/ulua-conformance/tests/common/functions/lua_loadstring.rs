use core::{ffi::c_int, slice::from_raw_parts};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{
  functions::compile::compile, records::compile_options::CompileOptions,
};
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_l_checklstring::lua_l_checklstring, lua_pushnil::lua_pushnil,
    lua_setsafeenv::lua_setsafeenv, luau_load::luau_load,
  },
  luaL_optstring,
  macros::lua_environindex::LUA_ENVIRONINDEX,
  records::lua_state::LuaState,
};

use crate::common::functions::cstr_text::cstr_text;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_loadstring(l: *mut LuaState) -> c_int {
  // Safety: `l` 为本用例存活的 LuaState；`lua_l_checklstring` 校验参数 1 为串并写出长度。
  let mut len = 0usize;
  let source = unsafe { lua_l_checklstring(l, 1, &mut len) };
  // Safety: `source` 是上一行返回的 VM 串缓冲；`luaL_optstring!` 在参数 2 缺省时复用
  // 它作为 chunk 名，返回 NUL 结尾指针。
  let chunkname = unsafe { luaL_optstring!(l, 2, source) };

  // Safety: `l` 存活；把加载环境切回非沙箱（loadstring 需读全局）。
  unsafe { lua_setsafeenv(l, LUA_ENVIRONINDEX, 0) };

  // Safety: `source`/`len` 是刚校验的合法字节区间。
  let source_bytes = unsafe { from_raw_parts(source.cast::<u8>(), len) };
  let bytecode = compile(
    source_bytes,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  // Safety: 上一段保证 `chunkname` 为 NUL 结尾串（lossy 只用于传给 luau_load 的名称）。
  let chunkname = unsafe { cstr_text(chunkname) };
  // Safety: `l` 存活；`bytecode` 为安全生成的字节码，加载结果非零不 panic（cpp 语义）。
  let result = unsafe { luau_load(l, &chunkname, &bytecode, 0) };

  if result == 0 {
    return 1;
  }

  // Safety: `l` 存活；失败路径压 nil 并插到错误串之下，凑成 (nil, msg) 两个返回值。
  unsafe {
    lua_pushnil(l);
    lua_insert(l, -2);
  };
  2
}
