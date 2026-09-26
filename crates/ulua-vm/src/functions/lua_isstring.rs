use crate::{
  enums::lua_type::LuaType, functions::lua_type::lua_type, records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_isstring(l: *mut LuaState, idx: i32) -> i32 {
  // Safety:lua_type 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let t = unsafe { lua_type(l, idx) };
  if t == LuaType::String as i32 || t == LuaType::Number as i32 {
    1
  } else {
    0
  }
}
