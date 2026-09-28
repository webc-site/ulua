use core::{ffi::c_int, str::from_utf8};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  functions::{lua_setsafeenv::lua_setsafeenv, luau_load::luau_load},
  macros::lua_environindex::LUA_ENVIRONINDEX,
  records::lua_state::LuaState,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_loadstring(l: *mut LuaState) -> c_int {
  let source = unsafe { (*l).check_bytes(1) };
  let chunkname = unsafe { (*l).opt_bytes(2, source) };

  // Safety: `l` 存活；把加载环境切回非沙箱（loadstring 需读全局）。
  unsafe { lua_setsafeenv(l, LUA_ENVIRONINDEX, 0) };

  let bytecode = compile(
    source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  let chunkname_str = from_utf8(chunkname).unwrap_or("");
  // Safety: `l` 存活；`bytecode` 为本帧拥有的合法切片，加载结果非零不 panic（cpp 语义）。
  let result = unsafe { luau_load(l, chunkname_str, &bytecode, 0) };

  if result == 0 {
    return 1;
  }

  // Safety: `l` 存活；失败路径压 nil 并插到错误串之下，凑成 (nil, msg) 两个返回值。
  unsafe {
    (*l).push_nil();
    (*l).insert(-2);
  };
  2
}
