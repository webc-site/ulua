use ulua_common::records::variant::Variant2;

use crate::{
  macros::variant_member,
  records::{boolean_singleton::BooleanSingleton, string_singleton::StringSingleton},
};

pub type SingletonVariant = Variant2<BooleanSingleton, StringSingleton>;

variant_member! {
  /// `get_if<T>(&stv->variant)` over the singleton variant (Type.h).
  position SingletonVariantMember: SingletonVariant {
    0 => BooleanSingleton,
    1 => StringSingleton,
  }
}
