use alloc::string::String;

use ulua_analysis::{
  functions::to_string_to_string::{
    to_string_type_id_to_string_options, to_string_type_pack_id_to_string_options,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

use crate::records::unifier_2_fixture::Unifier2Fixture;

impl Unifier2Fixture {
  pub fn to_string_type_id(&mut self, ty: TypeId) -> String {
    to_string_type_id_to_string_options(ty, &mut self.opts)
  }
}

impl Unifier2Fixture {
  pub fn to_string_type_pack_id(&mut self, ty: TypePackId) -> String {
    to_string_type_pack_id_to_string_options(ty, &mut self.opts)
  }
}
