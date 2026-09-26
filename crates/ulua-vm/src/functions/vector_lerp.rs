use core::array::from_fn;

use crate::{
  functions::{
    lua_l_checknumber::lua_l_checknumber,
    lua_l_checkvector::lua_l_checkvector,
    luai_lerpf::luai_lerpf,
    vector_shared::{vector_components, vector_push},
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn vector_lerp(l: *mut LuaState) -> i32 {
  // Safety: 契约保证索引 1/2 为 vector（分量窗口读 [0..=3]）、索引 3 为数值；压栈需 top 后 ≥1 空槽
  unsafe {
    let a = vector_components(lua_l_checkvector(l, 1));
    let b = vector_components(lua_l_checkvector(l, 2));
    let t = lua_l_checknumber(l, 3) as f32;

    // luai_lerpf 为纯算术：3 分量配置下第 4 位（两端皆 0.0）算出即弃
    vector_push(l, from_fn(|i| luai_lerpf(a[i], b[i], t)));

    1
  }
}

lua_lib_fn!(pub(crate) fn vector_lerp, vector_lerp_arm);
