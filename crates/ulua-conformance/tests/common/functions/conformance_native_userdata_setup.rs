use core::ptr::null_mut;

use ulua_code_gen::functions::set_userdata_remapper::set_userdata_remapper;
use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_native_userdata_remapper::conformance_native_userdata_remapper,
  setup_userdata_helpers::setup_userdata_helpers, setup_vector_helpers::setup_vector_helpers,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_native_userdata_setup(l: *mut LuaState) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    // FFI: c-API 要求 NULL
    set_userdata_remapper(l, null_mut(), Some(conformance_native_userdata_remapper));

    setup_vector_helpers(l);
    setup_userdata_helpers(l);
  }
}
