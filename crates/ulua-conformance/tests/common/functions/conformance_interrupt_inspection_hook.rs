use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::common::functions::cstr::cstr;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_hook(
  l: *mut LuaState,
  ar: *mut LuaDebug,
) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`ar` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    assert_ne!(0, lua_getinfo(l, 0, cstr(b"nsl\0"), ar));
  }
}
