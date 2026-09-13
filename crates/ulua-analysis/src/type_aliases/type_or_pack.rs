use ulua_common::records::variant::Variant2;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

pub type TypeOrPack = Variant2<TypeId, TypePackId>;

/// `tyOrTp.get_if<T>()` over TypeOrPack (TypeOrPack.h).
pub trait TypeOrPackMember: Sized {
  fn get_if(v: &TypeOrPack) -> Option<&Self>;
  fn get_if_mut(v: &mut TypeOrPack) -> Option<&mut Self>;
}

impl TypeOrPackMember for TypeId {
  fn get_if(v: &TypeOrPack) -> Option<&Self> {
    v.get_if_0()
  }
  fn get_if_mut(v: &mut TypeOrPack) -> Option<&mut Self> {
    v.get_if_0_mut()
  }
}

impl TypeOrPackMember for TypePackId {
  fn get_if(v: &TypeOrPack) -> Option<&Self> {
    v.get_if_1()
  }
  fn get_if_mut(v: &mut TypeOrPack) -> Option<&mut Self> {
    v.get_if_1_mut()
  }
}
