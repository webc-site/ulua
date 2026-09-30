//! Source: `Analysis/include/Luau/Def.h:66` (hand-ported)
use ulua_common::records::variant::Variant2;

use crate::{
  macros::variant_member,
  records::{cell::Cell, phi::Phi},
};

// Def::V — the definition variant. (Previous content here was a copy of an
// unrelated Bound/Error/Value variant: wrong-content alias, fixed 06-12.)
pub type Variant = Variant2<Cell, Phi>;

variant_member! {
  /// `get_if<T>(&v)` over this variant, the Rust shape of C++ overload-on-T.
  position VariantMember: Variant {
    0 => Cell,
    1 => Phi,
  }
}
