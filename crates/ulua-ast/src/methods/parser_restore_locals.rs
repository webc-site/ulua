use crate::records::parser::Parser;

impl Parser {
  pub fn restore_locals(&mut self, offset: u32) {
    let offset = offset as usize;

    // 逆序遍历 offset 之后的局部，恢复遮蔽绑定（cpp `for (idx = size; idx > offset; --idx)`
    // 的切片迭代形态，免 i-1 索引算术）
    for &l_ptr in self.local_stack[offset..].iter().rev() {
      // Safety: local_stack 存的是 arena 存活节点指针；`l` 借用的是 arena 而非
      // `self`，下方对 local_map 的改写与之不构成别名冲突。
      let l = unsafe { &*l_ptr };
      *self.local_map.get_or_insert(l.name) = l.shadow;
    }

    self.local_stack.truncate(offset);
  }
}
