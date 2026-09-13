use alloc::alloc::{alloc, handle_alloc_error};
use core::{
  alloc::Layout,
  mem::{align_of, size_of},
  ptr::{NonNull, null, null_mut},
};

use crate::{
  functions::compute_native_exec_data_size::compute_native_exec_data_size,
  records::native_proto_exec_data_header::NativeProtoExecDataHeader,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

pub fn create_native_proto_exec_data_u32_u32(
  bytecode_instruction_count: u32,
  extra_data_count: u32,
) -> NativeProtoExecDataPtr {
  let total_size = compute_native_exec_data_size(bytecode_instruction_count, extra_data_count);
  let layout = Layout::from_size_align(total_size, align_of::<u32>())
    .expect("Invalid layout for NativeProtoExecData");

  let bytes = unsafe { alloc(layout) };
  if bytes.is_null() {
    handle_alloc_error(layout);
  }

  let header = bytes as *mut NativeProtoExecDataHeader;
  unsafe {
    header.write(NativeProtoExecDataHeader {
      native_module: null_mut(),
      entry_offset_or_address: null(),
      bytecode_id: 0,
      bytecode_instruction_count,
      extra_data_count,
      native_code_size: 0,
    });
  }

  let data_ptr = unsafe { bytes.add(size_of::<NativeProtoExecDataHeader>()) as *mut u32 };
  unsafe { NonNull::new_unchecked(data_ptr) }
}
