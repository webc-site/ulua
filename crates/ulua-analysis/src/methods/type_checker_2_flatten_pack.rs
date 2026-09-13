//! Node: `cxx:Method:Luau.Analysis:Analysis/src/TypeChecker2.cpp:2723:type_checker_2_flatten_pack`
//! Source: `Analysis/src/TypeChecker2.cpp:2723-2749` (hand-ported)

use alloc::string::String;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type_pack::as_mutable, finite::finite, first::first,
    follow_type_pack::follow_type_pack_id, fresh_index::fresh_index, get_type_pack::get,
    size_type_pack::size,
  },
  records::{
    free_type_pack::FreeTypePack, internal_error::InternalError, type_checker_2::TypeChecker2,
    type_level::TypeLevel, type_pack::TypePack, type_pack_var::TypePackVar,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_error_data::IntoTypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `TypeId TypeChecker2::flattenPack(TypePackId pack)`.
  pub(crate) fn flatten_pack(&mut self, pack: TypePackId) -> TypeId {
    let pack = unsafe { follow_type_pack_id(pack) };

    if let Some(fst) = first(pack, /*ignoreHiddenVariadics*/ false) {
      return fst;
    }

    if let Some(ftp) = get::<FreeTypePack>(pack) {
      let scope = ftp.scope;
      let result = unsafe {
        (*self.module)
          .internal_types
          .fresh_type_not_null_builtin_types_scope(&*self.builtin_types, scope)
      };
      let free_tail = unsafe {
        (*self.module)
          .internal_types
          .add_type_pack_type_pack_var(TypePackVar {
            ty: TypePackVariant::Free(FreeTypePack {
              index: fresh_index(),
              level: TypeLevel::default(),
              scope,
              polarity: Polarity::Unknown,
            }),
            persistent: false,
            owning_arena: null_mut(),
          })
      };

      let result_pack = unsafe { &mut *as_mutable(pack) };
      result_pack.ty = TypePackVariant::TypePack(TypePack {
        head: alloc::vec![result],
        tail: Some(free_tail),
      });

      return result;
    }

    if get::<ErrorTypePack>(pack).is_some() {
      return unsafe { (*self.builtin_types).error_type };
    }

    if unsafe { finite(pack, null_mut()) } && unsafe { size(pack, null_mut()) } == 0 {
      // `(f())` where `f()` returns no values is coerced into `nil`
      return unsafe { (*self.builtin_types).nil_type };
    }

    let err = InternalError::new(String::from("flattenPack got a weird pack!"));
    self.report_error_type_error_data_location(err.into_type_error_data(), &Location::default());
    unsafe { (*self.builtin_types).error_type }
  }
}
