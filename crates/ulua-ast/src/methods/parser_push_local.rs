use crate::records::{
  ast_local::AstLocal, binding::Binding, node_handle::{Node, OptNode}, parser::Parser,
};

impl Parser {
  /// cpp `Parser::pushLocal`：产物出自 arena 分配（`alloc` 失败即中止），恒非空，
  /// 故返回 [`Node`]——符号表与 `local_stack` 的登记、以及全部调用点不再判空。
  pub fn push_local(&mut self, binding: &Binding) -> Node<AstLocal> {
    let name = binding.name;

    // C++: `AstLocal*& local = local_map[name.name];` — operator[] inserts a
    // null slot when absent, and the prior value becomes the new local's
    // shadow before the slot is reassigned to the freshly allocated local.
    // 「槽位缺席」即 cpp 的 nullptr，由 `OptNode` 在类型层承载，判空逻辑消失。
    let shadow = *self.local_map.get_or_insert(name.name);

    let function_depth = self.function_stack.len() - 1;
    let loop_depth = self
      .function_stack
      .last()
      .expect("Parser::new 构造期压入的顶层 chunk 底帧永不出栈，解析期间栈必非空")
      .loop_depth as usize;

    let new_local = AstLocal {
      name: name.name,
      location: name.location,
      // AstLocal::shadow 仍是 cpp 形态的裸槽（records 引用化的后续波次收口）。
      shadow: shadow.as_ptr(),
      function_depth,
      loop_depth,
      is_const: binding.is_const,
      is_exported: false,
      annotation: binding.annotation,
    };

    // arena 分配统一走 `Parser::alloc`（解引用收口在 `Parser::arena` 单点）。
    let local = Node::from_raw(self.alloc(new_local));

    *self.local_map.get_or_insert(name.name) = OptNode::from(local);
    self.local_stack.push(local);

    local
  }
}
