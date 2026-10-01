use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::{
  cstr_text::cstr_text, lua_vector_cross::lua_vector_cross, lua_vector_dot::lua_vector_dot,
  safe_api::{namecallatom, state_mut},
};

/// # Safety
///
/// `l` 必须是指向有效 `LuaState` 的指针。
pub unsafe extern "C-unwind" fn lua_vector_namecall(l: *mut LuaState) -> i32 {
  let mut atom: i32 = 0;
  let str_ptr = namecallatom(l, Some(&mut atom));

  if !str_ptr.is_null() {
    // Safety: 上一步已排除 null，`namecallatom` 保证其 NUL 结尾。
    let str_slice = unsafe { cstr_text(str_ptr) };

    if str_slice == "Dot" {
      return lua_vector_dot(l);
    }

    if str_slice == "Cross" {
      return lua_vector_cross(l);
    }
  }

  let arg1_str = state_mut(l).check_str(1);

  // `luaL_error!` 恒发散，作为尾表达式即可。
  // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
  unsafe { luaL_error!(l, "{} is not a valid method of vector", arg1_str) }
}
