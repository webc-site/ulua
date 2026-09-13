extern crate alloc;

use ulua_vm::records::Proto::Proto;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssertInlinerData {
  pub proto: *mut Proto,
  pub target: *mut Proto,
  pub pc: u32,
  pub called: bool,
}
