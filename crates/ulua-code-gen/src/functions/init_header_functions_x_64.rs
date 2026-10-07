//! x64 平台的 `HeaderEntryBuilder` 适配：gate 入口汇编骨架（
//! `functions::init_header_functions`）在 X64 上的真实分歧点——builder 构造、
//! 入口发射实现与天然字节流的 `Vec<u8>` 代码视图；骨架流程本身不再在本文件重复。

use crate::{
  enums::options::Arch,
  functions::build_entry_function_x_64::build_entry_function,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, label::Label, unwind_builder::UnwindBuilderImpl,
    vm_exit::EntryLocations,
  },
  traits::HeaderEntryBuilder,
};

impl HeaderEntryBuilder for AssemblyBuilderX64 {
  const ARCH: Arch = Arch::X64;

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
    &self.code
  }

  fn header_label_offset(&self, label: &Label) -> u32 {
    self.get_label_offset(label)
  }
}
