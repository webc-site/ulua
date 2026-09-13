use core::{ffi::c_char, ptr::copy_nonoverlapping};

use ulua_ast::records::allocator::Allocator;
pub fn allocate_string_luau_allocator_string_view(
  allocator: &mut Allocator,
  contents: &str,
) -> *mut c_char {
  let size = contents.len();
  let result = allocator.allocate(size + 1) as *mut c_char;

  unsafe {
    copy_nonoverlapping(contents.as_ptr() as *const c_char, result, size);
    *result.add(size) = 0;
  }

  result
}
