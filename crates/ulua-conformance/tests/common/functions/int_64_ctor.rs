use core::ffi::c_int;

use ulua_vm::{functions::lua_l_checknumber::lua_l_checknumber, records::lua_state::LuaState};

use crate::common::functions::push_int_64::push_int_64;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_ctor(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let value = lua_l_checknumber(l, 1);
    push_int_64(l, value as i64);
    1
  }
}
