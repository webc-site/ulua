use core::ffi::c_char;

use crate::records::ir_builder::IrBuilder;

pub type HostVectorNamecallHandler = Option<
  unsafe extern "C-unwind" fn(
    builder: *mut IrBuilder,
    member: *const c_char,
    member_length: usize,
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32,
  ) -> bool,
>;
