use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  records::native_proto_bytecode_id_less::NativeProtoBytecodeIdLess,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

impl NativeProtoBytecodeIdLess {
  pub fn operator_call_3(&self, left: u32, right: &NativeProtoExecDataPtr) -> bool {
    unsafe {
      let header = get_native_proto_exec_data_header(right.as_ptr());
      left < (*header).bytecode_id
    }
  }
}
