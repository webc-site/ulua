use core::ffi::c_void;

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  records::shared_code_gen_context::SharedCodeGenContext,
};

impl SharedCodeGenContext {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn on_destroy_function(execdata: *mut c_void) {
    let header = unsafe { &*get_native_proto_exec_data_header(execdata as *const u32) };
    // execdata 的 native_module 必然存在（创建时填入）
    unsafe {
      header.native_module.as_ref().unwrap().release();
    }
  }
}
