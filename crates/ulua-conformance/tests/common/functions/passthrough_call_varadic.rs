use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_callyieldable_impl::lua_callyieldable, lua_gettop::lua_gettop,
    lua_l_checkany::lua_l_checkany,
  },
  macros::lua_multret::LUA_MULTRET,
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_varadic(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    lua_l_checkany(l, 1);
    let nargs = lua_gettop(l) - 1;
    lua_callyieldable(l, nargs, LUA_MULTRET)
  }
}
