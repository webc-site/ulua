use alloc::string::ToString;
use core::ffi::c_int;

use ulua_vm::{functions::lua_pushlstring::lua_pushlstring, records::lua_state::LuaState};

use crate::common::functions::get_int_64::get_int_64;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_tostring(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let string = get_int_64(l, 1).to_string();
    lua_pushlstring(l, string.as_ptr().cast(), string.len());
    1
  }
}
