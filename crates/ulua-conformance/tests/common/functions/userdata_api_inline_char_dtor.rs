use core::{ffi::c_void, sync::atomic::Ordering};

use crate::common::functions::userdata_api_dtor_hits::USERDATA_API_DTOR_HITS;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_api_inline_char_dtor(data: *mut c_void) {
  unsafe {
    USERDATA_API_DTOR_HITS.fetch_add(*(data as *const i8) as i32, Ordering::SeqCst);
  }
}
