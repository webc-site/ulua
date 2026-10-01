use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  functions::optional_node::node_opt,
  records::{
    ast_array::AstArray, ast_local::AstLocal, binding::Binding, location::Location, name::Name,
    parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  /// cpp `Parser::prepareFunctionArguments`（`Parser.cpp:2256`）。
  ///
  /// 返回的 `self` 在 cpp 里以 `AstLocal* self = nullptr` 起算、仅 `hasself` 时
  /// `pushLocal` 赋值（`Parser.cpp:2258-2261`）——即「非方法则无 self 绑定」，
  /// 用 `Option<NonNull<AstLocal>>` 表达；`bindinglist`/普通形参恒有 local，保持切片形态。
  pub fn prepare_function_arguments(
    &mut self,
    start: &Location,
    hasself: bool,
    args: &TempVector<'_, Binding>,
  ) -> (Option<NonNull<AstLocal>>, AstArray<*mut AstLocal>) {
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
      node_opt(self.push_local(&binding))
    } else {
      None
    };

    // C++ uses a `TempVector<AstLocal*> vars(scratchLocal)` here; a local Vec
    // produces an identical `copy` result and avoids holding a borrow of
    // `self.scratch_local` across the `self.push_local` calls (the scratch
    // Buffer is only an allocation-reuse optimization, not observable).
    let mut vars: Vec<*mut AstLocal> = Vec::new();
    for arg in args.iter() {
      let local = self.push_local(arg);
      vars.push(local);
    }

    let copied = self.copy_initializer_list_t(&vars);
    (self_local, copied)
  }
}
