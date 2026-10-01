use core::{
  ffi::{c_char, c_int},
  slice::from_raw_parts,
  str::from_utf8,
};

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::get_or_create_atom::get_or_create_atom;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_direct_access_useratom(
  _l: *mut LuaState,
  s: *const c_char,
  len: usize,
) -> i16 {
  if s.is_null() || len == 0 {
    return -1;
  }
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`_l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let bytes = unsafe { from_raw_parts(s as *const u8, len) };
  let name = from_utf8(bytes).unwrap_or("");
  get_or_create_atom(name) as c_int as i16
}
