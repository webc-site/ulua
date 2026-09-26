use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    vector_shared::{vector_components, vector_push},
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn vector_cross(l: *mut LuaState) -> i32 {
  // Safety: 契约保证索引 1/2 为 vector，分量窗口只读 [0..=3]；压栈需 top 后 ≥1 空槽
  unsafe {
    let a = vector_components(lua_l_checkvector(l, 1));
    let b = vector_components(lua_l_checkvector(l, 2));

    // 叉积只落在 x/y/z 上，w 恒 0.0（cpp 四分量重载同形）
    vector_push(
      l,
      [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
        0.0f32,
      ],
    );

    1
  }
}

lua_lib_fn!(pub(crate) fn vector_cross, vector_cross_arm);
