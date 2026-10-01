//! Source: `Analysis/include/Luau/LValue.h` (LValue.h:17, hand-ported)

use crate::{
  macros::variant_member,
  records::{field::Field, symbol::Symbol},
};

// C++: using LValue = Variant<Symbol, Field>;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LValue {
  Symbol(Symbol),
  Field(Field),
}

variant_member! {
  /// `get_if<T>(&lvalue)` over the LValue variant.
  enum LValueMember: LValue {
    Symbol => Symbol,
    Field => Field,
  }
}
