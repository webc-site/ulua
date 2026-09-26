use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_pushnumber::lua_pushnumber,
    vector_shared::{sum_squares, vector_components},
  },
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn vector_magnitude(l: *mut LuaState) -> i32 {
  // Safety: 契约保证索引 1 为 vector，其分量窗口只读 [0..=3]，不写栈
  unsafe {
    let v = vector_components(lua_l_checkvector(l, 1));

    lua_pushnumber(l, sum_squares(v).sqrt() as f64);
    1
  }
}
