use core::mem::size_of;

use crate::records::native_proto_exec_data_header::NativeProtoExecDataHeader;

#[inline]
pub fn get_native_proto_exec_data_header_mut(
  instruction_offsets: *mut u32,
) -> *mut NativeProtoExecDataHeader {
  let header_size = size_of::<NativeProtoExecDataHeader>();

  // Safety: NativeProtoExecDataHeader 在 execdata 分配中紧接其 instruction_offsets 数组之前
  // 布局, 故由 instruction_offsets 反向偏移恰好 header_size 字节仍落在同一分配内, 指针算术
  // 合法; 该分配按 header 对齐起始, 反推地址同样满足 NativeProtoExecDataHeader 的对齐。
  (unsafe { (instruction_offsets as *mut u8).offset(-(header_size as isize)) }
    as *mut NativeProtoExecDataHeader)
}

#[inline]
pub fn get_native_proto_exec_data_header(
  instruction_offsets: *const u32,
) -> *const NativeProtoExecDataHeader {
  let header_size = size_of::<NativeProtoExecDataHeader>();

  // Safety: 与 mut 版同理——header 紧接 instruction_offsets 之前布局, 从数组首地址减去
  // header_size 仍在同一分配内且满足 header 对齐; 此处只做地址计算与只读转换, 不产生别名。
  (unsafe { (instruction_offsets as *const u8).sub(header_size) }
    as *const NativeProtoExecDataHeader)
}
