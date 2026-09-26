use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tointegerx::lua_tointegerx, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 必须是正在执行的 C 函数帧，`narg` 为其中合法栈索引（正/负/伪索引均可，但须落在有效范围）；
/// 非数值时经 `tag_error` 抛 Lua 错误且不会返回值。越界索引会按错槽取数或解引用悬垂 TValue。cpp laux.cpp:226。
pub unsafe fn lua_l_checkinteger(l: *mut LuaState, narg: i32) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧且 narg 栈槽可读；非数值路径经 typeerror 抛错、不返回
  unsafe {
    match lua_tointegerx(l, narg) {
      Some(d) => d,
      None => tag_error(l, narg, LuaType::Number as i32),
    }
  }
}

// lualib.h name
