use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::lua_gettop::lua_gettop,
  macros::{lua_isnil::lua_isnil, lua_pop::lua_pop, lua_tonumber::lua_tonumber},
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_more_results_continuation(
  l: *mut LuaState,
  _status: c_int,
) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    LUAU_ASSERT!(lua_gettop(l) == 13);

    for _ in 0..9 {
      LUAU_ASSERT!(lua_isnil!(l, -1));
      lua_pop(l, 1);
    }

    LUAU_ASSERT!(lua_tonumber!(l, -1) == 0.5);
    1
  }
}
