use core::mem::size_of;

use crate::records::native_proto_exec_data_header::NativeProtoExecDataHeader;

#[inline]
pub fn get_native_proto_exec_data_header(
  instruction_offsets: *const u32,
) -> *const NativeProtoExecDataHeader {
  let header_size = size_of::<NativeProtoExecDataHeader>();

  (unsafe { (instruction_offsets as *const u8).sub(header_size) }
    as *const NativeProtoExecDataHeader)
}
