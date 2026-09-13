use crate::{
  functions::as_mutable_type::as_mutable_type_id,
  records::{r#type::Type, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

impl TypeArena {
  pub fn add_tv(&mut self, tv: Type) -> TypeId {
    let allocated = self.types.allocate(tv);
    unsafe {
      (*as_mutable_type_id(allocated)).owning_arena = self as *mut TypeArena;
    }
    allocated
  }
}
