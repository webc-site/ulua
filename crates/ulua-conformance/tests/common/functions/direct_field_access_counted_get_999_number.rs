use core::ffi::c_void;

use ulua_vm::functions::lua_userdatadirectfield_setnumber::lua_userdatadirectfield_setnumber;

use crate::common::functions::direct_field_access_increment_handler_hit_count::direct_field_access_increment_handler_hit_count;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_counted_get_999_number(
  _ud: *mut c_void,
  result: *mut c_void,
) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`result` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    lua_userdatadirectfield_setnumber(result, 999.0);
    direct_field_access_increment_handler_hit_count();
  }
}
