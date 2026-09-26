use core::{ffi::c_int, mem::size_of};

use ulua_vm::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_newuserdatatagged::lua_newuserdatatagged},
  records::lua_state::LuaState,
};

use crate::common::{
  functions::direct_field_access_k_tag_vec_2::K_TAG_VEC2,
  records::vec_2_direct_field_access_test::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_create_vec_2(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let y = lua_l_checknumber(l, 2);

    let p = lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2) as *mut Vec2;
    (*p).x = x;
    (*p).y = y;

    1
  }
}
