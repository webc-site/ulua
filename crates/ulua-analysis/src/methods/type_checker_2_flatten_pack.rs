//! Source: `Analysis/src/TypeChecker2.cpp:2723-2749` (hand-ported)

use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, finite::finite, first::first, follow_type_pack,
    fresh_index::fresh_index, get_type_pack::get, size_type_pack::size,
  },
  records::{
    arena_handle::alias, arena_id::ArenaId, free_type_pack::FreeTypePack,
    internal_error::InternalError, type_checker_2::TypeChecker2, type_level::TypeLevel,
    type_pack::TypePack, type_pack_var::TypePackVar,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_error_data::IntoTypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};
impl TypeChecker2 {
  /// C++ `TypeId TypeChecker2::flattenPack(TypePackId pack)`.
  pub(crate) fn flatten_pack(&mut self, pack: TypePackId) -> TypeId {
    let pack = follow_type_pack::follow(pack);

    if let Some(fst) = first(pack, /*ignoreHiddenVariadics*/ false) {
      return fst;
    }

    if let Some(ftp) = get::<FreeTypePack>(pack) {
      let scope = ftp.scope;
      let result = alias(self.module)
        .internal_types
        .fresh_type_not_null_builtin_types_scope(self.builtin_types.get(), scope);
      let free_tail = alias(self.module)
        .internal_types
        .add_type_pack_type_pack_var(TypePackVar {
          ty: TypePackVariant::Free(FreeTypePack {
            index: fresh_index(),
            level: TypeLevel::default(),
            scope,
            polarity: Polarity::Unknown,
          }),
          persistent: false,
          owning_arena: ArenaId::NONE,
        });

      let result_pack = alias(as_mutable_type_pack(pack));
      result_pack.ty =
        TypePackVariant::TypePack(TypePack::new(alloc::vec![result], Some(free_tail)));

      return result;
    }

    if get::<ErrorTypePack>(pack).is_some() {
      return self.builtin_types_ref().error_type;
    }

    if finite(pack, None) && size(pack, None) == 0 {
      // `(f())` where `f()` returns no values is coerced into `nil`
      return self.builtin_types_ref().nil_type;
    }

    let err = InternalError::new(String::from("flattenPack got a weird pack!"));
    self.report_error_type_error_data_location(err.into_type_error_data(), &Location::default());
    self.builtin_types_ref().error_type
  }
}
