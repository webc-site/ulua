use core::ffi::c_int;

use ulua_vm::{
  functions::lua_touserdatatagged::lua_touserdatatagged, macros::lua_l_typeerror::luaL_typeerror,
  records::lua_state::LuaState,
};

use crate::common::records::{userdata_tags::K_TAG_VEC2, vec_2_conformance_ir_hooks::Vec2};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_vec_2_get(l: *mut LuaState, idx: i32) -> *mut Vec2 {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let a = lua_touserdatatagged(l, idx, K_TAG_VEC2 as c_int) as *mut Vec2;

    if !a.is_null() {
      return a;
    }

    luaL_typeerror!(l, idx, "vec2");
  }
}
