use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{free_type_pack::FreeTypePack, type_pack::TypePack, unifier::Unifier},
  type_aliases::{error_type_pack::ErrorTypePack, type_pack_id::TypePackId},
};

impl Unifier {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn occurs_check_dense_hash_set_type_pack_id_type_pack_id_type_pack_id(
    &mut self,
    seen: &mut DenseHashSet<TypePackId>,
    mut needle: TypePackId,
    mut haystack: TypePackId,
  ) -> bool {
    needle = unsafe { self.log.follow_type_pack_id(needle) };
    haystack = unsafe { self.log.follow_type_pack_id(haystack) };

    if seen.find(&haystack).is_some() {
      return false;
    }

    seen.insert(haystack);

    if get_type_pack_id::<ErrorTypePack>(needle).is_some() {
      return false;
    }

    if get_type_pack_id::<FreeTypePack>(needle).is_none() {
      self.ice_string("Expected needle pack to be free");
    }

    while get_type_pack_id::<ErrorTypePack>(haystack).is_none() {
      if needle == haystack {
        return true;
      }

      if let Some(pack) = get_type_pack_id::<TypePack>(haystack)
        && let Some(tail) = pack.tail
      {
        haystack = unsafe { self.log.follow_type_pack_id(tail) };
        continue;
      }

      break;
    }

    false
  }
}
