use alloc::boxed::Box;
use core::ffi::c_int;

use crate::records::map_fixture::MapFixture;
impl MapFixture {
  pub fn make_ptr(&mut self) -> *mut c_int {
    self.ptrs.push(Box::new(0));
    let last = self.ptrs.last_mut().expect("vector is not empty");
    last.as_mut() as *mut c_int
  }
}
