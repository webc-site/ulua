use std::collections::HashSet;

use crate::common::type_aliases::register_callback::RegisterCallback;

pub fn get_register_callbacks() -> &'static mut HashSet<RegisterCallback> {
  static mut INSTANCE: Option<HashSet<RegisterCallback>> = None;

  unsafe {
    let ptr = &raw mut INSTANCE;
    if (*ptr).is_none() {
      *ptr = Some(HashSet::new());
    }
    (*ptr).as_mut().unwrap()
  }
}
