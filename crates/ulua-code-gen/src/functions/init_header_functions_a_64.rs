//! a64 平台的 `HeaderEntryBuilder` 适配：gate 入口汇编骨架（
//! `functions::init_header_functions`）在 A64 上的真实分歧点——builder 构造、
//! 入口发射实现与 `Vec<u32>` 指令流的字节视图；骨架流程本身不再在本文件重复。

use core::{mem::size_of_val, slice::from_raw_parts};

use crate::{
  enums::options::Arch,
  functions::build_entry_function_a_64::build_entry_function,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, label::Label, unwind_builder::UnwindBuilderImpl,
    vm_exit::EntryLocations,
  },
  traits::HeaderEntryBuilder,
};

impl HeaderEntryBuilder for AssemblyBuilderA64 {
  const ARCH: Arch = Arch::A64;

  fn new_for_header() -> Self {
    Self::new(false, 0)
  }

  fn build_header_entry(&mut self, unwind: &mut UnwindBuilderImpl) -> EntryLocations {
    build_entry_function(self, unwind)
  }

  fn finalize_header(&mut self) {
    // 骨架与旧实现一致：不消费链接结果，后续以 label 偏移/分配判空把关。
    self.finalize();
  }

  fn data_bytes(&self) -> &[u8] {
    &self.data
  }

  fn code_bytes(&self) -> &[u8] {
    // Safety: 视图派生自本对象存活的 `Vec<u32>` 指令流缓冲区：元素 4 字节对齐、
    // 长度换算不超出分配域，u32 无填充字节；单次只读借用期间无并存 &mut。
    // `size_of_val` 与旧实现 `len * size_of::<u32>()` 逐值一致。
    unsafe {
      from_raw_parts(
        self.code.as_ptr().cast::<u8>(),
        size_of_val(self.code.as_slice()),
      )
    }
  }

  fn header_label_offset(&self, label: &Label) -> u32 {
    self.get_label_offset(label)
  }
}
