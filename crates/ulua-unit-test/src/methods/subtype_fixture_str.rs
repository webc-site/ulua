use alloc::string::ToString;

use ulua_analysis::{
  records::{singleton_type::SingletonType, string_singleton::StringSingleton},
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn str(&mut self, literal: &str) -> TypeId {
    self.arena.add_type(SingletonType {
      variant: SingletonVariant::V1(StringSingleton {
        value: literal.to_string(),
      }),
    })
  }
}
