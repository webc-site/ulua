//! Source: `Analysis/include/Luau/Def.h:66` (hand-ported)
use ulua_common::records::variant::Variant2;

use crate::records::{cell::Cell, phi::Phi};

// Def::V — the definition variant. (Previous content here was a copy of an
// unrelated Bound/Error/Value variant: wrong-content alias, fixed 06-12.)
pub type Variant = Variant2<Cell, Phi>;

/// `get_if<T>(&v)` over this variant, the Rust shape of C++ overload-on-T.
pub trait VariantMember: Sized {
  fn get_if(v: &Variant) -> Option<&Self>;
  fn get_if_mut(v: &mut Variant) -> Option<&mut Self>;
}

impl VariantMember for Cell {
  fn get_if(v: &Variant) -> Option<&Self> {
    v.get_if_0()
  }
  fn get_if_mut(v: &mut Variant) -> Option<&mut Self> {
    v.get_if_0_mut()
  }
}

impl VariantMember for Phi {
  fn get_if(v: &Variant) -> Option<&Self> {
    v.get_if_1()
  }
  fn get_if_mut(v: &mut Variant) -> Option<&mut Self> {
    v.get_if_1_mut()
  }
}
