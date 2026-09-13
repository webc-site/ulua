use crate::records::block::Block;

impl Block {
  /// # Safety
  /// 调用方须保证 `target` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn add_successor(&mut self, target: *mut Block) {
    // C++: successors.emplace_back(target); target->predecessors.emplace_back(this);
    // BlockId = NotNull<Block> = *mut Block.
    self.successors.push(target);
    unsafe { &mut *target }
      .predecessors
      .push(self as *mut Block);
  }
}
