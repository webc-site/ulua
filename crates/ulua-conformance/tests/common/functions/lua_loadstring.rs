use core::{ffi::c_int, ptr::null_mut, slice::from_raw_parts};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_l_checklstring::lua_l_checklstring, lua_pushnil::lua_pushnil,
    lua_setsafeenv::lua_setsafeenv, luau_load::luau_load,
  },
  luaL_optstring,
  macros::lua_environindex::LUA_ENVIRONINDEX,
  records::lua_state::LuaState,
};

use crate::common::functions::{c_alloc::c_free, cstr_text::cstr_text};

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

  // Safety: `source`/`len` 是刚校验的合法字节区间；`luau_compile` 返回非空产物（失败时
  // 亦返回可读取的缓冲）并写出 `bytecode_size`，选项传 null 表示取编译端缺省。
  let mut bytecode_size = 0usize;
  // FFI: c-API 要求 NULL
  let bytecode = unsafe { luau_compile(source, len, null_mut(), &mut bytecode_size) };

  // C 边界转换：`chunkname` 是栈上 VM 串（cpp `luau_load` 亦按 strlen 取用，
  // 内部 NUL 截断语义一致）；`bytecode` 为 luau_compile 的裸缓冲 + 长度出参
  // （cpp `std::string` data()/size() 同形态），至本处 c_free 前保持存活。
  // Safety: 上一段保证 `chunkname` 为 NUL 结尾串（lossy 只用于传给 luau_load 的名称）。
  let chunkname = unsafe { cstr_text(chunkname) };
  // Safety: `bytecode` 起 `bytecode_size` 字节连续可读，至本处 c_free 前存活。
  let bytes = unsafe { from_raw_parts(bytecode.cast::<u8>(), bytecode_size) };
  // Safety: `l` 存活；`bytes` 为上一步物化的合法切片，加载结果非零不 panic（cpp 语义）。
  let result = unsafe { luau_load(l, &chunkname, bytes, 0) };
  // Safety: `bytecode` 是 luau_compile 交回、尚未释放的裸缓冲。
  unsafe { c_free(bytecode.cast()) };

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
