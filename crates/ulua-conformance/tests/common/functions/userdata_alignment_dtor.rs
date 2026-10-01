use core::ffi::c_void;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_alignment_dtor(_data: *mut c_void) {}
