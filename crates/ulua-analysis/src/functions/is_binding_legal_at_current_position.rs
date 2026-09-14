use ulua_ast::records::{location::Location, position::Position};

use crate::records::{binding::Binding, symbol::Symbol};

pub fn is_binding_legal_at_current_position(
  symbol: &Symbol,
  binding: &Binding,
  pos: Position,
) -> bool {
  if !symbol.local.is_null() {
    return binding.location.end < pos;
  }

  binding.location == Location::default() || !binding.location.contains_closed(pos)
}
