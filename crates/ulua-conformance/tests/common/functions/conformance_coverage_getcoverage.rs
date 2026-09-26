use core::ffi::{c_int, c_void};

use ulua_vm::{
  functions::{lua_getcoverage::lua_getcoverage, lua_is_lfunction::lua_is_lfunction},
  macros::{lua_l_argexpected::luaL_argexpected, lua_newtable::lua_newtable},
  records::lua_state::LuaState,
};

use crate::common::functions::conformance_coverage_callback::conformance_coverage_callback;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_coverage_getcoverage(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    luaL_argexpected!(l, lua_is_lfunction(l, 1) != 0, 1, "function");

    lua_newtable(l);
    lua_getcoverage(l, 1, l as *mut c_void, Some(conformance_coverage_callback));

    1
  }
}
