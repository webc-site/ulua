use core::{ffi::c_int, str::from_utf8};

use ulua_compiler::records::compile_options::CompileOptions;
use ulua_vm::{macros::lua_environindex::LUA_ENVIRONINDEX, records::lua_state::LuaState};

use crate::common::functions::safe_api::{load_source, setsafeenv, state_mut};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_loadstring(l: *mut LuaState) -> c_int {
  let source = state_mut(l).check_bytes(1);
  let chunkname = state_mut(l).opt_bytes(2, source);

  // 把加载环境切回非沙箱（loadstring 需读全局）。
  setsafeenv(l, LUA_ENVIRONINDEX, false);

  let chunkname_str = from_utf8(chunkname).unwrap_or("");
  // 编译产物由 `load_source` 内部生成（缺省编译选项与 cpp 的 null options 同形），
  // 加载结果非零不 panic（cpp 语义）。
  let result = load_source(l, chunkname_str, source, &mut CompileOptions::default());

  if result == 0 {
    return 1;
  }

  // 失败路径压 nil 并插到错误串之下，凑成 (nil, msg) 两个返回值。
  state_mut(l).push_nil();
  state_mut(l).insert(-2);
  2
}
