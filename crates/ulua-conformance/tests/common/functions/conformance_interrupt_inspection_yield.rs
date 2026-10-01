use core::mem::zeroed;

use ulua_vm::{
  functions::{lua_getinfo::lua_getinfo, luau_callhook::luau_callhook},
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::common::functions::{
  conformance_interrupt_inspection_hook::conformance_interrupt_inspection_hook, cstr::cstr,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_yield(l: *mut LuaState) -> bool {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let mut ar: LuaDebug = zeroed();
    assert_ne!(0, lua_getinfo(l, 0, cstr(b"nsl\0"), &mut ar));

    luau_callhook(l, Some(conformance_interrupt_inspection_hook), None);

    false
  }
}
