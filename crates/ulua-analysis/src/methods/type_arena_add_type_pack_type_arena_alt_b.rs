use crate::{
  functions::as_mutable_type_pack_alt_d::as_mutable_type_pack,
  records::{type_arena::TypeArena, type_pack::TypePack, type_pack_var::TypePackVar},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeArena {
  pub fn add_type_pack_initializer_list_type_id(&mut self, types: &[TypeId]) -> TypePackId {
    let tp = TypePack {
      head: types.to_vec(),
      tail: None,
    };
    let allocated = self.type_packs.allocate(TypePackVar::from(tp));
    unsafe {
      (*as_mutable_type_pack(allocated)).owning_arena = self as *mut TypeArena;
    }
    allocated
  }
}
