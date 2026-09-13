use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{free_type_pack::FreeTypePack, replace_generics::ReplaceGenerics},
  type_aliases::type_pack_id::TypePackId,
};

impl ReplaceGenerics {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(self.is_dirty_type_pack_id(tp));
    let mut pack = FreeTypePack::new(self.level);
    pack.scope = self.scope;
    self.base.add_type_pack(pack)
  }
}
