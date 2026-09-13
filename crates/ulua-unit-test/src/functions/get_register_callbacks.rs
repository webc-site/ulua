use alloc::collections::BTreeSet;
use core::ptr::null_mut;

use crate::type_aliases::register_callback::RegisterCallback;
pub fn get_register_callbacks() -> &'static mut BTreeSet<RegisterCallback> {
  static mut CBS: *mut BTreeSet<RegisterCallback> = null_mut();

  unsafe {
    if CBS.is_null() {
      let cbs = Box::new(BTreeSet::<RegisterCallback>::new());
      CBS = Box::into_raw(cbs);
    }
    &mut *CBS
  }
}
