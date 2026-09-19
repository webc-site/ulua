use crate::records::parser::Parser;

impl Parser {
  pub fn restore_locals(&mut self, offset: u32) {
    let offset = offset as usize;

    // 逆序遍历 offset 之后的局部，恢复遮蔽绑定（cpp `for (idx = size; idx > offset; --idx)`
    // 的切片迭代形态，免 i-1 索引算术）
    for &l_ptr in self.local_stack[offset..].iter().rev() {
      // Safety: local_stack holds live arena pointers; `l` borrows the arena,
      // not `self`, so the local_map mutation below does not alias it.
      let l = unsafe { &*l_ptr };
      *self.local_map.get_or_insert(l.name) = l.shadow;
    }

    self.local_stack.truncate(offset);
  }
}
