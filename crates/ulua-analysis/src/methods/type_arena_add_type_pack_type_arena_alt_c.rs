use crate::{
  functions::as_mutable_type_pack_alt_d::as_mutable_type_pack,
  records::{type_arena::TypeArena, type_pack::TypePack, type_pack_var::TypePackVar},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeArena {
  pub fn add_type_pack_vector_type_id_optional_type_pack_id(
    &mut self,
    types: Vec<TypeId>,
    tail: Option<TypePackId>,
  ) -> TypePackId {
    let tp = TypePack { head: types, tail };
    let allocated = self.type_packs.allocate(TypePackVar::from(tp));
    unsafe {
      (*as_mutable_type_pack(allocated)).owning_arena = self as *mut TypeArena;
    }
    allocated
  }
}
