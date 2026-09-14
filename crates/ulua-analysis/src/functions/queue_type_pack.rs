use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::get_mutable_type_pack::get_mutable_type_pack_id,
  records::{
    free_type_pack::FreeTypePack, type_pack::TypePack, type_pack_var::TypePackVar, unifier::Unifier,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
pub(crate) fn queue_type_pack(
  queue: &mut Vec<TypeId>,
  seen_type_packs: &mut DenseHashSet<TypePackId>,
  state: &mut Unifier,
  mut a: TypePackId,
  any_type_pack: TypePackId,
) {
  loop {
    a = unsafe { state.log.follow_type_pack_id(a) };

    if seen_type_packs.find(&a).is_some() {
      break;
    }
    seen_type_packs.insert(a);

    if get_mutable_type_pack_id::<FreeTypePack>(a).is_some() {
      state.log.replace_type_pack_id_type_pack_var(
        a,
        TypePackVar {
          ty: TypePackVariant::Bound(any_type_pack),
          persistent: false,
          owning_arena: null_mut(),
        },
      );
    } else if let Some(tp) = get_mutable_type_pack_id::<TypePack>(a) {
      queue.extend(tp.head.iter().copied());

      if let Some(tail) = tp.tail {
        a = tail;
      } else {
        break;
      }
    } else {
      break;
    }
  }
}
