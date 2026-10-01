use ulua_vm::{macros::lua_vector_size::LUA_VECTOR_SIZE, records::lua_state::LuaState};

use crate::common::functions::safe_api::{l_checkvector, pushvector3, pushvector4};
pub(crate) fn lua_vector_cross(l: *mut LuaState) -> i32 {
  // 参数 1/2 各为 vector（失配按 cpp 抛 Lua 错误），取分量切片。
  let a = l_checkvector(l, 1);
  let b = l_checkvector(l, 2);

  let (x, y, z) = (
    a[1] * b[2] - a[2] * b[1],
    a[2] * b[0] - a[0] * b[2],
    a[0] * b[1] - a[1] * b[0],
  );

  // 按构建期分量布局压入叉积结果（四维形态第四分量置 0）。
  if LUA_VECTOR_SIZE == 4 {
    pushvector4(l, x, y, z, 0.0);
  } else {
    pushvector3(l, x, y, z);
  }

  1
}
