use core::ptr::NonNull;

use crate::{
  functions::optional_node::opt_node,
  records::{ast_type::AstType, binding::Binding, name::Name, position::Position},
};

impl Binding {
  /// cpp `Parser::Binding::Binding(Name, AstType* annotation)`（`Parser.h:57`）。
  ///
  /// `None` 即 cpp 传 `nullptr` 的「未标注形参」形态（`parseBinding` 无 `:` 时的
  /// `parseOptionalType()`、`pushLocal(Binding(Name(nameSelf, start), nullptr))`）。
  pub fn new(
    name: Name,
    annotation: Option<NonNull<AstType>>,
    colon_position: Position,
    is_const: bool,
  ) -> Self {
    Self {
      name,
      annotation: opt_node(annotation),
      colon_position,
      is_const,
    }
  }
}
