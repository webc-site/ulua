use alloc::boxed::Box;

use ulua_analysis::{
  records::{free_type_pack::FreeTypePack, type_level::TypeLevel, type_pack_var::TypePackVar},
  type_aliases::type_pack_id::TypePackId,
};

use crate::records::type_pack_fixture::TypePackFixture;

impl TypePackFixture {
  pub fn fresh_type_pack(&mut self) -> TypePackId {
    let type_pack = Box::new(TypePackVar::from(FreeTypePack::new(TypeLevel::default())));
    let type_pack_id = type_pack.as_ref() as *const TypePackVar;
    self.type_packs.push(type_pack);
    type_pack_id
  }
}
