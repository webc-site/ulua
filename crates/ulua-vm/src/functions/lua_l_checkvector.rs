use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tovector::lua_tovector, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// # Safety
/// 调用方须保证：`l` 为存活调用帧、`narg` 为可读实参栈槽；非 vector 时 `tag_error` 经 `l` 抛错不返回。
/// 返回的 `*const f32` 指向该槽 TValue 内联 vector 数据，仅在该槽未被覆写、对象未被 GC 回收期间有效。
/// cpp laux.cpp:266 `luaL_checkvector`
pub unsafe fn lua_l_checkvector(l: *mut LuaState, narg: i32) -> *const f32 {
  // Safety: 契约保证 `l` 为存活调用帧且 narg 栈槽为可读 vector TValue，失配路径抛错不返回
  unsafe {
    let v = lua_tovector(l, narg);
    if v.is_null() {
      tag_error(l, narg, LuaType::Vector as i32);
    }
    v
  }
}
