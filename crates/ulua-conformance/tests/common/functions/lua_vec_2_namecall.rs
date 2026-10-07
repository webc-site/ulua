// 边界契约测试：null 系 c-API 合法实参（既有约定 review.md §2）
use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::{
  cstr_text::cstr_text,
  lua_vec_2_clone::lua_vec_2_clone,
  lua_vec_2_dot::lua_vec_2_dot,
  lua_vec_2_get::lua_vec_2_get,
  lua_vec_2_min::lua_vec_2_min,
  lua_vec_2_reenter::lua_vec_2_reenter,
  safe_api::{namecallatom, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_namecall(l: *mut LuaState) -> i32 {
  // `namecallatom` 返回 namecall 方法名的 NUL 结尾指针或 null（不取 arg 计数）。
  let str_ptr = namecallatom(l, None);

  if !str_ptr.is_null() {
    // Safety: 上面已排除 null，`namecallatom` 保证其 NUL 结尾。
    let str_slice = unsafe { cstr_text(str_ptr) };

    // 分派：`lua_vec_2_get` 校验 self 为 Vec2 userdata 并交回数据指针；下游
    // Dot/Min/Clone/Reenter 都是 safe 门面（自行压栈并给出返回计数）。
    if str_slice == "Dot" {
      let self_ptr = lua_vec_2_get(l, 1);
      return lua_vec_2_dot(l, self_ptr);
    }

    if str_slice == "Min" {
      let self_ptr = lua_vec_2_get(l, 1);
      return lua_vec_2_min(l, self_ptr);
    }

    if str_slice == "Clone" {
      let self_ptr = lua_vec_2_get(l, 1);
      return lua_vec_2_clone(l, self_ptr);
    }

    if str_slice == "Reenter" {
      let self_ptr = lua_vec_2_get(l, 1);
      return lua_vec_2_reenter(l, self_ptr);
    }
  }

  let arg1_str = state_mut(l).check_str(1);

  // 末分支按 cpp 抛 Lua 错误（宏内为 C ABI `luaL_error`，格式串为已校验的
  // `arg1_str`）；该调用不返回。
  // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
  unsafe { luaL_error!(&mut *l, "{} is not a valid method of vector", arg1_str) }
}
