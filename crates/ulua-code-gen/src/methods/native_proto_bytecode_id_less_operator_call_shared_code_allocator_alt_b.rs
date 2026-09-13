use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  records::native_proto_bytecode_id_less::NativeProtoBytecodeIdLess,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

impl NativeProtoBytecodeIdLess {
  pub fn operator_call_2(&self, left: &NativeProtoExecDataPtr, right: u32) -> bool {
    unsafe {
      let header = get_native_proto_exec_data_header(left.as_ptr());
      (*header).bytecode_id < right
    }
  }
}
