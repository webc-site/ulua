use core::ffi::{c_char, c_void};

use crate::common::records::conformance_gc_dump_enum_context::ConformanceGcDumpEnumContext;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_gc_dump_edge(
  context: *mut c_void,
  from: *mut c_void,
  to: *mut c_void,
  _name: *const c_char,
) {
  unsafe {
    let context = &mut *(context as *mut ConformanceGcDumpEnumContext);
    context.edges.insert(from as usize, to as usize);
  }
}
