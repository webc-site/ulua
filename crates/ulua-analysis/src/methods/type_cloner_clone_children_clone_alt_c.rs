use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::type_cloner::TypeCloner,
  type_aliases::{
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
  },
};

impl TypeCloner {
  pub fn clone_children_type_or_pack(&mut self, kind: TypeOrPack) {
    if let Some(ty) = TypeId::get_if(&kind) {
      self.clone_children_type_id(*ty);
    } else if let Some(tp) = TypePackId::get_if(&kind) {
      self.clone_children_type_pack_id(*tp);
    } else {
      LUAU_ASSERT!(false);
    }
  }
}
