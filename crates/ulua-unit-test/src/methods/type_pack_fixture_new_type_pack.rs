use alloc::{boxed::Box, vec::Vec};

use ulua_analysis::{
  records::{type_pack::TypePack, type_pack_var::TypePackVar},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

use crate::records::type_pack_fixture::TypePackFixture;
impl TypePackFixture {
  pub fn new_type_pack(&mut self, types: Vec<TypeId>, tail: Option<TypePackId>) -> TypePackId {
    let type_pack = Box::new(TypePackVar::from(TypePack::new(types, tail)));
    let type_pack_id = type_pack.as_ref() as *const TypePackVar;
    self.type_packs.push(type_pack);
    type_pack_id
  }
}
