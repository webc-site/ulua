use core::{
  ffi::{c_char, c_void},
  slice::from_raw_parts,
  str::from_utf8,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_native_userdata_remapper(
  _context: *mut c_void,
  name: *const c_char,
  name_length: usize,
) -> u8 {
  if name.is_null() || name_length == 0 {
    return 0xff;
  }
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`_context` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let name_bytes = unsafe { from_raw_parts(name as *const u8, name_length) };
  let name_str = from_utf8(name_bytes).unwrap_or("");
  match name_str {
    "extra" => 0,
    "color" => 1,
    "vec2" => 2,
    "mat3" => 3,
    "vertex" => 4,
    _ => 0xff,
  }
}
