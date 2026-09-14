use core::mem::size_of;

use crate::records::native_proto_exec_data_header::NativeProtoExecDataHeader;

#[inline]
pub fn compute_native_exec_data_size(
  bytecode_instruction_count: u32,
  extra_data_count: u32,
) -> usize {
  let header_size = size_of::<NativeProtoExecDataHeader>();
  let bytecode_size = (bytecode_instruction_count as usize) * size_of::<u32>();
  let extra_data_size = (extra_data_count as usize) * size_of::<u32>();
  header_size + bytecode_size + extra_data_size
}
