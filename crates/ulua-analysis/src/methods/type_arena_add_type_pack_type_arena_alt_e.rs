use crate::{
  functions::as_mutable_type_pack_alt_d::as_mutable_type_pack,
  records::{type_arena::TypeArena, type_pack_var::TypePackVar},
  type_aliases::type_pack_id::TypePackId,
};

impl TypeArena {
  pub fn add_type_pack_type_pack_var(&mut self, tp: TypePackVar) -> TypePackId {
    let allocated = self.type_packs.allocate(tp);
    unsafe {
      (*as_mutable_type_pack(allocated)).owning_arena = self as *mut TypeArena;
    }
    allocated
  }
}
