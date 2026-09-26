use core::ffi::c_uint;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tounsignedx::lua_tounsignedx, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// # Safety
/// 调用方须保证：`l` 为存活调用帧、`narg` 为可读实参栈槽（`lua_tounsignedx`/`tag_error` 按索引读取该槽）；
/// 非数值路径经 `l` 抛错、不返回，故须在可捕获错误的受保护帧内调用。cpp laux.cpp:252 `luaL_checkunsigned`
pub unsafe fn lua_l_checkunsigned(l: *mut LuaState, narg: i32) -> c_uint {
  // Safety: 契约保证 `l` 为存活调用帧且 narg 栈槽可读；负数/非数值经 argerror 抛错、不返回
  unsafe {
    // 对应 cpp laux.cpp：lua_tounsignedx 失败（非数字）时 tag_error
    let Some(d) = lua_tounsignedx(l, narg) else {
      tag_error(l, narg, LuaType::Number as i32);
    };
    d
  }
}
