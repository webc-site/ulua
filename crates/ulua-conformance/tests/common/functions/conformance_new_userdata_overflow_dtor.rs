use std::ffi::c_void;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_new_userdata_overflow_dtor(_data: *mut c_void) {}
