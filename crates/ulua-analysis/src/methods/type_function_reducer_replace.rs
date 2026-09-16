//! `TypeFunctionReducer::replace<T>` (TypeFunction.cpp:324-344).
//!
//! C++ is a single template specialized on `TypeId`/`TypePackId`. The result
//! struct in this crate is monomorphized on `TypeId`, so the two
//! specializations are rendered as the concrete `replace_type_id` and
//! `replace_type_pack_id` methods. `asMutable(subject)->ty.emplace<Bound<T>>`
//! becomes assigning the `Bound` variant of the type/type-pack variant.

use alloc::string::String;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack_id},
  records::{
    internal_error::InternalError, type_error::TypeError,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::{
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};
impl TypeFunctionReducer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub(crate) fn replace_type_id(&mut self, subject: TypeId, replacement: TypeId) {
    unsafe {
      if (*subject).owning_arena != (*self.ctx.as_ptr()).arena.as_ptr() {
        self
          .result
          .errors
          .push(TypeError::type_error_location_type_error_data(
            self.location,
            TypeErrorData::InternalError(InternalError::new(String::from(
              "Attempting to modify a type function instance from another arena",
            ))),
          ));
        return;
      }

      // asMutable(subject)->ty.emplace<Unifiable::Bound<TypeId>>(replacement);
      (*as_mutable_type_id(subject)).ty = TypeVariant::Bound(replacement);
    }

    self.result.reduced_types.insert(subject);
  }

  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn replace_type_pack_id(&mut self, subject: TypePackId, replacement: TypePackId) {
    unsafe {
      if (*subject).owning_arena != (*self.ctx.as_ptr()).arena.as_ptr() {
        self
          .result
          .errors
          .push(TypeError::type_error_location_type_error_data(
            self.location,
            TypeErrorData::InternalError(InternalError::new(String::from(
              "Attempting to modify a type function instance from another arena",
            ))),
          ));
        return;
      }

      // asMutable(subject)->ty.emplace<Unifiable::Bound<TypePackId>>(replacement);
      (*as_mutable_type_pack_id(subject)).ty = TypePackVariant::Bound(replacement);
    }

    self.result.reduced_packs.insert(subject);
  }
}
