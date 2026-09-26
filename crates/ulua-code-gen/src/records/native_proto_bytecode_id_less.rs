use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NativeProtoBytecodeIdLess {
  pub(crate) _unused: [u8; 0],
}

impl NativeProtoBytecodeIdLess {
  pub fn operator_call(
    &self,
    left: &NativeProtoExecDataPtr,
    right: &NativeProtoExecDataPtr,
  ) -> bool {
    // Safety: left/right 为 &NativeProtoExecDataPtr(NonNull<u32>), as_ptr() 非空/对齐; get_native_proto_exec_data_header
    // 按契约从各自 exec data 反推出落在同一分配内、按 header 对齐的 *const Header, 只读 bytecode_id 比较, 无别名写。
    unsafe {
      let left_header = get_native_proto_exec_data_header(left.as_ptr());
      let right_header = get_native_proto_exec_data_header(right.as_ptr());
      (*left_header).bytecode_id < (*right_header).bytecode_id
    }
  }

  pub fn operator_call_2(&self, left: &NativeProtoExecDataPtr, right: u32) -> bool {
    // Safety: left.as_ptr() 来自 NonNull<u32> 故非空/对齐, header 反推依契约落在同一分配内, 只读 bytecode_id 与 u32 比较。
    unsafe {
      let header = get_native_proto_exec_data_header(left.as_ptr());
      (*header).bytecode_id < right
    }
  }

  pub fn operator_call_3(&self, left: u32, right: &NativeProtoExecDataPtr) -> bool {
    // Safety: right.as_ptr() 来自 NonNull<u32> 故非空/对齐, header 反推依契约落在同一分配内, 只读 bytecode_id 与 u32 比较。
    unsafe {
      let header = get_native_proto_exec_data_header(right.as_ptr());
      left < (*header).bytecode_id
    }
  }
}
