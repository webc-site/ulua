use ulua_common::records::variant::Variant2;

use crate::{
  macros::variant_member,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub type TypeOrPack = Variant2<TypeId, TypePackId>;

variant_member! {
  /// `tyOrTp.get_if<T>()` over TypeOrPack (TypeOrPack.h).
  position TypeOrPackMember: TypeOrPack {
    0 => TypeId,
    1 => TypePackId,
  }
}
