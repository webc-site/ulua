use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  records::native_proto_bytecode_id_equal::NativeProtoBytecodeIdEqual,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

impl NativeProtoBytecodeIdEqual {
  pub fn operator_call(
    &self,
    left: &NativeProtoExecDataPtr,
    right: &NativeProtoExecDataPtr,
  ) -> bool {
    unsafe {
      let left_header = get_native_proto_exec_data_header(left.as_ptr());
      let right_header = get_native_proto_exec_data_header(right.as_ptr());
      (*left_header).bytecode_id == (*right_header).bytecode_id
    }
  }
}
