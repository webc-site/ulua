use ulua_common::records::variant::Variant2;

use crate::records::{boolean_singleton::BooleanSingleton, string_singleton::StringSingleton};

pub type SingletonVariant = Variant2<BooleanSingleton, StringSingleton>;

/// `get_if<T>(&stv->variant)` over the singleton variant (Type.h).
pub trait SingletonVariantMember: Sized {
  fn get_if(v: &SingletonVariant) -> Option<&Self>;
  fn get_if_mut(v: &mut SingletonVariant) -> Option<&mut Self>;
}

impl SingletonVariantMember for BooleanSingleton {
  fn get_if(v: &SingletonVariant) -> Option<&Self> {
    v.get_if_0()
  }
  fn get_if_mut(v: &mut SingletonVariant) -> Option<&mut Self> {
    v.get_if_0_mut()
  }
}

impl SingletonVariantMember for StringSingleton {
  fn get_if(v: &SingletonVariant) -> Option<&Self> {
    v.get_if_1()
  }
  fn get_if_mut(v: &mut SingletonVariant) -> Option<&mut Self> {
    v.get_if_1_mut()
  }
}
