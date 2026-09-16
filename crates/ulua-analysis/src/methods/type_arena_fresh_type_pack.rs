use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::as_mutable_type_pack_alt_d::as_mutable_type_pack,
  records::{
    free_type_pack::FreeTypePack, scope::Scope, type_arena::TypeArena, type_level::TypeLevel,
    type_pack_var::TypePackVar,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl TypeArena {
  pub fn fresh_type_pack(&mut self, scope: *mut Scope, polarity: Polarity) -> TypePackId {
    // FreeTypePack{scope, polarity}
    let mut free = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Polarity::None,
    };
    free.free_type_pack_scope_polarity(scope, polarity);

    let allocated = self.type_packs.allocate(TypePackVar::from(free));
    unsafe {
      (*as_mutable_type_pack(allocated)).owning_arena = self as *mut TypeArena;
    }
    allocated
  }
}
