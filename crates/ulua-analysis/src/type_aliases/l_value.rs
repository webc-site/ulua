//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/LValue.h:17:l_value`
//! Source: `Analysis/include/Luau/LValue.h` (LValue.h:17, hand-ported)

use crate::records::{field::Field, symbol::Symbol};

// C++: using LValue = Variant<Symbol, Field>;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LValue {
  Symbol(Symbol),
  Field(Field),
}

/// `get_if<T>(&lvalue)` over the LValue variant.
pub trait LValueMember: Sized {
  fn get_if(v: &LValue) -> Option<&Self>;
  fn get_if_mut(v: &mut LValue) -> Option<&mut Self>;
}

impl LValueMember for Symbol {
  fn get_if(v: &LValue) -> Option<&Self> {
    match v {
      LValue::Symbol(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut LValue) -> Option<&mut Self> {
    match v {
      LValue::Symbol(x) => Some(x),
      _ => None,
    }
  }
}

impl LValueMember for Field {
  fn get_if(v: &LValue) -> Option<&Self> {
    match v {
      LValue::Field(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut LValue) -> Option<&mut Self> {
    match v {
      LValue::Field(x) => Some(x),
      _ => None,
    }
  }
}
