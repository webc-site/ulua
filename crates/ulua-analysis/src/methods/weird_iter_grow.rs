use core::ptr::{from_mut, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_type_pack::get_mutable_type_pack_id,
  records::{
    free_type_pack::FreeTypePack, type_pack::TypePack, type_pack_var::TypePackVar,
    weird_iter::WeirdIter,
  },
  type_aliases::{type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
impl WeirdIter {
  pub fn weird_iter_grow(&mut self, new_tail: TypePackId) {
    LUAU_ASSERT!(self.weird_iter_can_grow());
    LUAU_ASSERT!(!get_mutable_type_pack_id::<TypePack>(new_tail).is_none());

    let free_pack = get_mutable_type_pack_id::<FreeTypePack>(self.pack_id).unwrap();
    self.level = free_pack.level;
    if !free_pack.scope.is_null() {
      self.scope = free_pack.scope;
    }
    unsafe {
      (*self.log).replace_type_pack_id_type_pack_var(
        self.pack_id,
        TypePackVar {
          ty: TypePackVariant::Bound(new_tail),
          persistent: false,
          owning_arena: null_mut(),
        },
      );
    }
    self.pack_id = new_tail;
    self.pack = get_mutable_type_pack_id::<TypePack>(new_tail).map_or(null_mut(), from_mut);
    self.index = 0;
    self.growing = true;
  }
}
