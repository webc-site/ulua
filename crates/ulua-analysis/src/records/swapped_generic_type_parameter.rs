use alloc::string::String;

use crate::enums::kind::Kind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwappedGenericTypeParameter {
  pub name: String,
  pub kind: Kind,
}

impl SwappedGenericTypeParameter {
  pub const TYPE: Kind = Kind::Type;
  pub const PACK: Kind = Kind::Pack;
}
