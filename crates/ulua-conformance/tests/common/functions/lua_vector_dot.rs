use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_l_checkvector::lua_l_checkvector, lua_pushnumber::lua_pushnumber},
  records::lua_state::LuaState,
};

/// C-unwind ABI：作为 LuaCFunction 注册进 VM，避免 transmute。
pub(crate) extern "C-unwind" fn lua_vector_dot(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let a = lua_l_checkvector(l, 1);
    let b = lua_l_checkvector(l, 2);

    let result = (*a.add(0)) * (*b.add(0)) + (*a.add(1)) * (*b.add(1)) + (*a.add(2)) * (*b.add(2));
    lua_pushnumber(l, result as f64);
  }
  1
}
