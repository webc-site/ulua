use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    vector_shared::{sum_squares, vector_components, vector_push},
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn vector_normalize(l: *mut LuaState) -> i32 {
  // Safety: 契约保证索引 1 为 vector（分量窗口读 [0..=3]），压栈需 top 后 ≥1 空槽
  unsafe {
    let v = vector_components(lua_l_checkvector(l, 1));

    let inv_sqrt = 1.0f32 / sum_squares(v).sqrt();
    vector_push(l, v.map(|x| x * inv_sqrt));

    1
  }
}

lua_lib_fn!(pub(crate) fn vector_normalize, vector_normalize_arm);
