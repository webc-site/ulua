//! Node: `cxx:Method:Luau.Analysis:Analysis/src/NonStrictTypeChecker.cpp:208:non_strict_type_checker_flatten_pack`
//! Source: `Analysis/src/NonStrictTypeChecker.cpp:208-231` (hand-ported)

use core::ptr::{null, null_mut};

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type_pack::as_mutable, emplace_type_pack::emplace_type_pack, finite::finite,
    first::first, follow_type_pack::follow_type_pack_id, fresh_index::fresh_index,
    get_type_pack::get, size_type_pack::size,
  },
  records::{
    free_type::FreeType, free_type_pack::FreeTypePack,
    non_strict_type_checker::NonStrictTypeChecker, type_level::TypeLevel, type_pack::TypePack,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `TypeId NonStrictTypeChecker::flattenPack(TypePackId pack)`.
  pub(crate) fn flatten_pack(&mut self, pack: TypePackId) -> TypeId {
    let pack = unsafe { follow_type_pack_id(pack) };

    if let Some(fst) = first(pack, /*ignoreHiddenVariadics*/ false) {
      return fst;
    }

    if let Some(ftp) = get::<FreeTypePack>(pack) {
      let scope = ftp.scope;
      let result = unsafe {
        (*self.arena).add_type(FreeType {
          index: fresh_index(),
          level: TypeLevel::default(),
          scope,
          forwarded_type_alias: false,
          lower_bound: null(),
          upper_bound: null(),
          polarity: Polarity::Unknown,
        })
      };

      let free_tail = unsafe {
        (*self.arena).add_type_pack_t(FreeTypePack {
          index: fresh_index(),
          level: TypeLevel::default(),
          scope,
          polarity: Polarity::Unknown,
        })
      };

      let result_pack = {
        unsafe {
          emplace_type_pack(
            as_mutable(pack),
            TypePackVariant::TypePack(TypePack {
              head: alloc::vec![result],
              tail: Some(free_tail),
            }),
          )
        }
      };
      let _ = result_pack;

      return result;
    }

    if get::<ErrorTypePack>(pack).is_some() {
      return unsafe { (*self.builtin_types).error_type };
    }

    if unsafe { finite(pack, null_mut()) } && unsafe { size(pack, null_mut()) } == 0 {
      return unsafe { (*self.builtin_types).nil_type };
    }

    unsafe {
      (*self.ice).ice_string("flattenPack got a weird pack!");
    }
    unsafe { (*self.builtin_types).error_type }
  }
}
