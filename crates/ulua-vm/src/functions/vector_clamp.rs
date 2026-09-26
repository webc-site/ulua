use core::{
  array::from_fn,
  cmp::Ordering::{Equal, Less},
};

use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    luaui_clampf::luaui_clampf,
    vector_shared::{vector_components, vector_push},
  },
  macros::lua_l_argcheck::luaL_argcheck,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn vector_clamp(l: *mut LuaState) -> i32 {
  // Safety: 契约保证索引 1/2/3 为 vector（分量窗口读 [0..=3]）；压栈需 top 后 ≥1 空槽
  unsafe {
    let v = vector_components(lua_l_checkvector(l, 1));
    let min = vector_components(lua_l_checkvector(l, 2));
    let max = vector_components(lua_l_checkvector(l, 3));

    luaL_argcheck!(
      l,
      matches!(min[0].partial_cmp(&max[0]), Some(Less | Equal)),
      3,
      "max.x must be greater than or equal to min.x"
    );
    luaL_argcheck!(
      l,
      matches!(min[1].partial_cmp(&max[1]), Some(Less | Equal)),
      3,
      "max.y must be greater than or equal to min.y"
    );
    luaL_argcheck!(
      l,
      matches!(min[2].partial_cmp(&max[2]), Some(Less | Equal)),
      3,
      "max.z must be greater than or equal to min.z"
    );

    // luaui_clampf 为纯算术：3 分量配置下第 4 位（三端皆 0.0）算出即弃
    vector_push(l, from_fn(|i| luaui_clampf(v[i], min[i], max[i])));

    1
  }
}
