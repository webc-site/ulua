use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::type_cloner::TypeCloner,
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

impl TypeCloner {
  pub fn find_type_or_pack(&self, kind: TypeOrPack) -> Option<TypeOrPack> {
    if let Some(ty) = TypeOrPack::get_if::<TypeId>(&kind) {
      self.find_type_id(*ty).map(TypeOrPack::V0)
    } else if let Some(tp) = TypeOrPack::get_if::<TypePackId>(&kind) {
      self.find_type_pack_id(*tp).map(TypeOrPack::V1)
    } else {
      LUAU_ASSERT!(false);
      None
    }
  }
}
