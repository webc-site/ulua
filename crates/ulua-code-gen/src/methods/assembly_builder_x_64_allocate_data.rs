//! C++ `size_t AssemblyBuilderX64::allocateData(size_t size, size_t align)`
//! (CodeGen/src/AssemblyBuilderX64.cpp:1772-1789)。

use core::ptr::{copy_nonoverlapping, write_bytes};

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT, records::assembly_builder_x_64::AssemblyBuilderX64,
};

impl AssemblyBuilderX64 {
  /// 常量数据段自 `data` 末尾向前分配：剩余空间不足时把容量翻倍，并把已写入的
  /// 内容整体搬到新尾部（旧头部清零），`finalize` 据此按 `data_pos..len` 取段。
  pub fn allocate_data(&mut self, size: usize, align: usize) -> usize {
    CODEGEN_ASSERT!(align > 0 && align <= 16 && (align & (align - 1)) == 0);

    if self.data_pos < size {
      let old_size = self.data.len();
      self.data.resize(old_size * 2, 0);

      unsafe {
        copy_nonoverlapping(
          self.data.as_ptr(),
          self.data.as_mut_ptr().add(old_size),
          old_size,
        );
        write_bytes(self.data.as_mut_ptr(), 0, old_size);
      }

      self.data_pos += old_size;
    }

    self.data_pos = (self.data_pos - size) & !(align - 1);
    self.data_pos
  }
}
