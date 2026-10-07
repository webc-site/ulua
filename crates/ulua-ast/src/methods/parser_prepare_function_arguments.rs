use alloc::vec::Vec;

use crate::records::{
  ast_local::AstLocal,
  binding::Binding,
  location::Location,
  name::Name,
  node_handle::{Node, OptNode},
  parser::Parser,
  position::Position,
  temp_vector::TempVector,
};

impl Parser {
  /// cpp `Parser::prepareFunctionArguments`（`Parser.cpp:2256`）。
  ///
  /// 返回的 `self` 在 cpp 里以 `AstLocal* self = nullptr` 起算、仅 `hasself` 时
  /// `pushLocal` 赋值（`Parser.cpp:2258-2261`）——即「非方法则无 self 绑定」，
  /// 用 [`OptNode`] 表达（`None` 即那个 `nullptr`）；`bindinglist`/普通形参恒有
  /// local，保持 [`Node`] 切片形态（cpp 的 `TempVector<AstLocal*> vars(scratchLocal)`
  /// 之 Rust 对偶，records 引用化收口前由调用方在构造边界折算）。
  pub fn prepare_function_arguments(
    &mut self,
    start: &Location,
    hasself: bool,
    args: &TempVector<'_, Binding>,
  ) -> (OptNode<AstLocal>, Vec<Node<AstLocal>>) {
    let self_local = if hasself {
      // C++: push_local(Binding(Name(name_self, start), nullptr));
      // `Parser::Name { name, location }` is the (AstName, Location) pair。
      // cpp 的 `nullptr` 标注即此处的 `None`：self 永远没有类型标注。
      let binding = Binding::new(
        Name {
          name: self.name_self,
          location: *start,
        },
        None,
        Position::default(),
        false,
      );
      OptNode::from(self.push_local(&binding))
    } else {
      OptNode::default()
    };

    // C++ uses a `TempVector<AstLocal*> vars(scratchLocal)` here; a local Vec
    // produces an identical `copy` result and avoids holding a borrow of
    // `self.scratch_local` across the `self.push_local` calls (the scratch
    // Buffer is only an allocation-reuse optimization, not observable).
    // 暂存面保持句柄形态，收口进 `AstExprFunction::args` 时由调用方单点折算。
    let mut vars: Vec<Node<AstLocal>> = Vec::new();
    for arg in args.iter() {
      vars.push(self.push_local(arg));
    }

    (self_local, vars)
  }
}
