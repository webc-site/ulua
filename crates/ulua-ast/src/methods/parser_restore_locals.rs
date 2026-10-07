use crate::records::{node_handle::OptNode, parser::Parser};

impl Parser {
  pub fn restore_locals(&mut self, offset: u32) {
    let offset = offset as usize;

    // 逆序遍历 offset 之后的局部，恢复遮蔽绑定（cpp `for (idx = size; idx > offset; --idx)`
    // 的切片迭代形态，免 i-1 索引算术）。local_stack 的元素是 `push_local` 交回的
    // arena 句柄（类型层即恒非空，解引用收口在 `Node` 内部）；`&l` 模式把句柄按
    // `Copy` 拷出成本轮局部，随后对 `l.name`/`l.shadow` 的读取只借用该局部，与下方
    // 对 `local_map`（disjoint 字段）的改写不重叠。
    for &local in self.local_stack[offset..].iter().rev() {
      // AstLocal::shadow 仍是 cpp 形态的裸槽（遮蔽链尾为 nullptr），进符号表的
      // 值列时折算为 `OptNode`；records 引用化后续波次可撤掉这层折算。
      let shadow = OptNode::from_ptr(local.shadow);
      *self.local_map.get_or_insert(local.name) = shadow;
    }

    self.local_stack.truncate(offset);
  }
}
