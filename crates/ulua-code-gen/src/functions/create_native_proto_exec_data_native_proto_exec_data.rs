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

/// execdata 分配块的对齐要求：header 内联于块首，块对齐必须同时满足 header（含裸指针/usize，
/// 8 对齐）与其后 u32 数组（4 对齐）的较大者。释放端按同一常量重建 layout，
/// 保证分配/释放 layout 逐位一致。
pub(crate) const NATIVE_EXEC_DATA_ALIGN: usize = {
  let u32_align = align_of::<u32>();
  let header_align = align_of::<NativeProtoExecDataHeader>();
  if u32_align > header_align {
    u32_align
  } else {
    header_align
  }
};

pub fn create_native_proto_exec_data_u32_u32(
  bytecode_instruction_count: u32,
  extra_data_count: u32,
) -> NativeProtoExecDataPtr {
  let total_size = compute_native_exec_data_size(bytecode_instruction_count, extra_data_count);
  let layout = Layout::from_size_align(total_size, NATIVE_EXEC_DATA_ALIGN)
    .expect("NativeProtoExecData 布局恒有效：size 恒为正（含 header）且 align 为 2 的幂");

  // Safety: `total_size` = header + 若干 u32，恒 > 0；`Layout::from_size_align` 已 `expect` 成功，
  // 故 `alloc(layout)` 参数合法。返回块至少按 layout 对齐；null 时立即 handle_alloc_error 中止。
  let bytes = unsafe { alloc(layout) };
  if bytes.is_null() {
    handle_alloc_error(layout);
  }

  let header = bytes as *mut NativeProtoExecDataHeader;
  // Safety: `bytes` 已证非空，且块按 `NATIVE_EXEC_DATA_ALIGN`（= align_of::<NativeProtoExecDataHeader>()）
  // 对齐、容量含整个 header，故 `header` 是指向已分配未初始化内存的对齐可写指针；`write` 以
  // Default 同值的字面量整体初始化 header，写入后 header 各字段（裸指针为 null、计数为入参）无悬垂。
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

  // Safety: `bytes` 已证非空，`size_of::<NativeProtoExecDataHeader>()` 为 4 的倍数（repr(C) 布局
  // 有 cpp 端 static_assert 把守），故 `bytes.add(..)` 仍是 4 对齐的合法指针，落于同一分配内紧随
  // header 的 u32 数组起始处。
  let data_ptr = unsafe { bytes.add(size_of::<NativeProtoExecDataHeader>()) as *mut u32 };
  // `data_ptr` 由上方证明非空（源 `bytes` 已判空），用 `NonNull::new(..).expect(..)` 作等价防御。
  NonNull::new(data_ptr)
    .expect("紧随 header 的 u32 数组指针必非空：bytes 已判非空且仅做同分配内偏移")
}
