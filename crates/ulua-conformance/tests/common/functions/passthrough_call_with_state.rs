use core::ffi::c_int;

use ulua_vm::{
  functions::lua_callyieldable_impl::lua_callyieldable, macros::lua_multret::LUA_MULTRET,
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_with_state(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    (*l).check_any(1);
    let args = (*l).get_top() - 1;

    (*l).push_number(42.0);
    (*l).insert(1);

    lua_callyieldable(l, args, LUA_MULTRET)
  }
}
