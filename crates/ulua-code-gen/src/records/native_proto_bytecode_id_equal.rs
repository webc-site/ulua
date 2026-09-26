use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NativeProtoBytecodeIdEqual {
  pub(crate) _unused: [u8; 0],
}

impl NativeProtoBytecodeIdEqual {
  pub fn operator_call(
    &self,
    left: &NativeProtoExecDataPtr,
    right: &NativeProtoExecDataPtr,
  ) -> bool {
    // Safety: left/right 为 NativeProtoExecDataPtr(NonNull), 其 as_ptr() 得到指向存活 exec-data 缓冲的
    // 非空、对齐指针; get_native_proto_exec_data_header 依 NativeProtoExecData 的"头部先于数组"布局
    // 返回位于其前的头部指针, 对有效输入同样非空且对齐, 故 (*header).bytecode_id 为头部内的原地字段读取。
    // 左右各为一次独立不可变读取, 无别名冲突。
    unsafe {
      let left_header = get_native_proto_exec_data_header(left.as_ptr());
      let right_header = get_native_proto_exec_data_header(right.as_ptr());
      (*left_header).bytecode_id == (*right_header).bytecode_id
    }
  }
}
