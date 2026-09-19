use core::ffi::c_char;

use crate::records::ir_builder::IrBuilder;

pub type HostUserdataAccessHandler = Option<
  unsafe extern "C-unwind" fn(
    builder: *mut IrBuilder,
    r#type: u8,
    member: *const c_char,
    member_length: usize,
    result_reg: i32,
    source_reg: i32,
    pcpos: i32,
  ) -> bool,
>;
