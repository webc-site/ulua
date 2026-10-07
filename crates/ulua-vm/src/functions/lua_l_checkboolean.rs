use crate::{
  enums::lua_type::LuaType,
  functions::{lua_type::lua_type, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub fn lua_l_checkboolean(l: &mut LuaState, narg: i32) -> i32 {
  unsafe {
    // This checks specifically for boolean values, ignoring
    // all other truthy/falsy values. If the desired result
    // is true if value is present then lua_toboolean should
    // directly be used instead.

    let is_bool = lua_type(l, narg) == (LuaType::Boolean as i32);

    if !is_bool {
      tag_error(l, narg, LuaType::Boolean as i32);
    }

    l.to_boolean(narg) as i32
  }
}
