use core::ffi::c_void;

use ulua_common::FFlag;

use crate::{
  functions::{
    destroy_native_proto_exec_data::destroy_native_proto_exec_data,
    get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  },
  records::standalone_code_gen_context::StandaloneCodeGenContext,
};

impl StandaloneCodeGenContext {
  pub fn standalone_code_gen_context_on_destroy_function(execdata: *mut c_void) {
    unsafe {
      if FFlag::LuauCodegenFreeBlocks.get() {
        let header = get_native_proto_exec_data_header(execdata as *const u32);
        (*header).native_module.as_ref().unwrap().release();
      } else {
        destroy_native_proto_exec_data(execdata as *mut u32);
      }
    }
  }
}
