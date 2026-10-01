use core::ptr::NonNull;

use crate::records::{allocator::Allocator, ast_name_table::AstNameTable};

impl AstNameTable {
  /// Re-point the interning allocator to `allocator`. The fixture calls this
  /// before each parse so the name table uses the allocator at its *current*
  /// address after the owning struct has been moved.
  pub fn rebind_allocator(&mut self, allocator: *mut Allocator) {
    // cpp 引用语义的边界证明点：调用方传入的指针出自活的 `&mut Allocator`，
    // null 即调用方 bug，在唯一的 pub 写入口拦截，字段类型层不再携带可空性。
    self.allocator =
      NonNull::new(allocator).expect("rebind_allocator 的 arena 分配器指针恒非空（引用参数语义）");
  }
}
