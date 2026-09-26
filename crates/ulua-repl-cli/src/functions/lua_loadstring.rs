use alloc::string::String;
use core::{ffi::c_int, ptr::null_mut};

use ulua_common::functions::{c_slice::c_slice, c_str::cstr_cow};
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_l_checklstring::lua_l_checklstring,
    lua_l_optlstring::lua_l_optlstring, lua_pushnil::lua_pushnil, lua_setsafeenv::lua_setsafeenv,
    luau_load::luau_load,
  },
  macros::lua_environindex::LUA_ENVIRONINDEX,
  records::lua_state::LuaState,
};

use crate::functions::compile_source::compile_source;

/// `loadstring` 全局函数（cpp Repl.cpp 的 lua_loadstring），经 `setup_state`
/// 注册进 VM，由 VM 在 Lua 调用点回调。
///
/// # Safety
///
/// `l` 必须是 VM 在调用本 `lua_CFunction` 时传入的当前有效线程状态，且栈上
/// 参数布局符合 Lua/C API 调用约定（索引 1 为待检查的字符串参数）。
pub(crate) unsafe extern "C-unwind" fn lua_loadstring(l: *mut LuaState) -> i32 {
  // Safety: l 是 VM 调 loadstring 闭包时的当前有效状态（Lua/C API 回调约定），
  // 索引 1/2 为本次调用的实参槽位；lua_l_checklstring 非字符串即报错发散，返回的
  // s 非空且 len 个字节可读（出参指针 &mut len 本帧独占借用）；保留空指针：C 接口
  // luaL_optlstring(L, arg, def, szfl) 契约允许长度出参传 null（其实现判空）。
  let mut len: usize = 0;
  let s = unsafe { lua_l_checklstring(l, 1, &mut len as *mut usize) };
  let name_ptr = unsafe { lua_l_optlstring(l, 2, s, null_mut()) };

  // Safety: l 存活，仅改环境表的 safe 标志位。
  unsafe { lua_setsafeenv(l, LUA_ENVIRONINDEX, false as c_int) };

  // loadstring 参数可为任意字节串；Rust 编译管线要求 &str（UTF-8），
  // 非法序列 lossy 替换，替代 from_utf8_unchecked 的 UB
  // Safety: lua_l_checklstring 返回 s 非空且 len 字节可读，c_slice 仅在该长度内只读。
  let source: String =
    String::from_utf8_lossy(unsafe { c_slice(s as *const u8, len) }).into_owned();

  // 源名同样来自 VM 栈的 C 串：在此 FFI 边界转成 String（长度即 cpp 的
  // strlen，止于首个 NUL）；非 UTF-8 字节按 source 同款 lossy 规则替换
  // Safety: lua_l_optlstring 以非空的 checklstring 结果为默认值，name_ptr 非空且 NUL 结尾。
  let chunkname: String = unsafe { cstr_cow(name_ptr) }.into_owned();

  let bytecode = compile_source(&source);

  // Safety: l 存活；chunkname 为本帧 String（luau_load 按 cpp strlen 规则读取），
  // bytecode 是本帧 Vec，仅在本次调用窗口内借用。
  if unsafe { luau_load(l, &chunkname, &bytecode, 0) } == 0 {
    return 1;
  }

  // Safety: l 存活；报错路径把 nil 插到错误消息前作为多返回值（栈操作配平）。
  unsafe {
    lua_pushnil(l);
    lua_insert(l, -2); // put before error message
  }
  2 // return nil plus error message
}
