// 边界契约测试：null 系 c-API 合法实参（既有约定 review.md §2）
use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::{
  cstr_text::cstr_text,
  lua_vertex_clone::lua_vertex_clone,
  lua_vertex_get::lua_vertex_get,
  safe_api::{namecallatom, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_namecall(l: *mut LuaState) -> i32 {
  // `namecallatom` 返回 namecall 方法名的 NUL 结尾指针或 null（不取 arg 计数）。
  let str_ptr = namecallatom(l, None);

  if !str_ptr.is_null() {
    // `lua_vertex_get` 是 safe 门面：校验参数 1 为 Vertex userdata 并返回其数据指针。
    let self_ptr = lua_vertex_get(l, 1);
    // Safety: 上面已排除 null，`namecallatom` 保证其 NUL 结尾。
    let str_slice = unsafe { cstr_text(str_ptr) };

    // Clone 是 safe 门面（自行压栈并给出返回计数）。
    if str_slice == "Clone" {
      return lua_vertex_clone(l, self_ptr);
    }
  }

  let arg1_str = state_mut(l).check_str(1);

  // 末分支按 cpp 抛「非方法」Lua 错误（宏内为 C ABI `luaL_error`，格式串为已校验的
  // `arg1_str`）；该调用不返回。
  // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
  unsafe { luaL_error!(l, "{} is not a valid method of vertex", arg1_str) }
}
