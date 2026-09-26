use crate::records::{ast_local::AstLocal, binding::Binding, parser::Parser};

impl Parser {
  pub fn push_local(&mut self, binding: &Binding) -> *mut AstLocal {
    let name = binding.name;

    // C++: `AstLocal*& local = local_map[name.name];` — operator[] inserts a
    // null slot when absent, and the prior value becomes the new local's
    // shadow before the slot is reassigned to the freshly allocated local.
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
      shadow,
      function_depth,
      loop_depth,
      is_const: binding.is_const,
      is_exported: false,
      annotation: binding.annotation,
    };

    // Safety: self.allocator 是指向 arena Allocator 的裸指针，由 Parser 构造期布线、
    // 存活期覆盖整个解析；alloc 仅经该指针在 arena 上追加分配。
    let local = self.alloc(new_local);

    *self.local_map.get_or_insert(name.name) = local;
    self.local_stack.push(local);

    local
  }
}
