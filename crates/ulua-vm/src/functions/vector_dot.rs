use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector, lua_pushnumber::lua_pushnumber,
    vector_shared::vector_components,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn vector_dot(l: *mut LuaState) -> i32 {
  // Safety: 契约保证索引 1/2 为 vector，分量窗口只读 [0..=3]，不写栈
  unsafe {
    let a = vector_components(lua_l_checkvector(l, 1));
    let b = vector_components(lua_l_checkvector(l, 2));

    // 逐项乘积可为 -0.0，故 3 分量配置不并入第 4 项（`x + 0.0` 会把 -0.0 归一为 +0.0）
    let mut d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    if LUA_VECTOR_SIZE == 4 {
      d += a[3] * b[3];
    }

    lua_pushnumber(l, d as f64);
    1
  }
}
