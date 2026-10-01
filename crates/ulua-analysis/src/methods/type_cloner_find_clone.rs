use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::follow_option::FollowOption,
  functions::{follow_type, follow_type_pack},
  records::{arena_handle::alias_ref, type_cloner::TypeCloner},
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

impl TypeCloner<'_> {
  pub(crate) fn find_type_id(&self, ty: TypeId) -> Option<TypeId> {
    let ty = follow_type::follow_with_option(ty, FollowOption::DisableLazyTypeThunks);

    if let Some(it) = self.types.get(&ty) {
      return Some(*it);
    } else if alias_ref(ty).persistent && self.force_ty != Some(ty) {
      return Some(ty);
    }

    None
  }

  pub(crate) fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypePackId> {
    let tp = follow_type_pack::follow(tp);

    if let Some(it) = self.packs.get(&tp) {
      return Some(*it);
    } else if alias_ref(tp).persistent && self.force_tp != Some(tp) {
      return Some(tp);
    }

    None
  }

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
