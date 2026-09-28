use ulua_vm::{functions::lua_callbacks::lua_callbacks, records::lua_state::LuaState};

use crate::common::functions::conformance_tag_method_error_debug_protected_error::conformance_tag_method_error_debug_protected_error;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tag_method_error_setup(l: *mut LuaState) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    (*lua_callbacks(l)).debugprotectederror =
      Some(conformance_tag_method_error_debug_protected_error);
  }
}
