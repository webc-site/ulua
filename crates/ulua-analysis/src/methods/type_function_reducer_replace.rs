//! `TypeFunctionReducer::replace<T>` (TypeFunction.cpp:324-344).
//!
//! C++ is a single template specialized on `TypeId`/`TypePackId`. The result
//! struct in this crate is monomorphized on `TypeId`, so the two
//! specializations are rendered as the concrete `replace_type_id` and
//! `replace_type_pack_id` methods. `asMutable(subject)->ty.emplace<Bound<T>>`
//! becomes assigning the `Bound` variant of the type/type-pack variant.

use alloc::string::String;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack},
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
  /// `subject`/`replacement` 须为会话 arena 内的存活句柄（C++ 模板版
  /// `TypeFunctionReducer::replace` 的入参契约，调用侧为刚归约出的结果类型）。
  pub(crate) fn replace_type_id(&mut self, subject: TypeId, replacement: TypeId) {
    // Safety: subject 是入队自本会话 arena 的 TypeId（非空 *const Type，bump 块
    // 地址不移动）；self.ctx 为 reducer 构造期以存活 `&mut` 借用接线的
    // `Handle<TypeFunctionContext>`（类型编码非空），其 arena 字段同为装配期
    // NonNull（存活至归约结束）。归属校验通过后 as_mutable_type_id 直译 C++ asMutable 的 const_cast：本 reducer
    // 串行推进，此刻该类型无其它活动可变借用，Bound 覆写为唯一写窗口。
    unsafe {
      if (*subject).owning_arena != self.ctx.get().arena_id() {
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
  /// `subject`/`replacement` 须为会话 arena 内的存活 pack 句柄（同 TypeId 版契约）。
  pub(crate) fn replace_type_pack_id(&mut self, subject: TypePackId, replacement: TypePackId) {
    // Safety: 与 TypeId 版同理——subject 指向会话 arena 存活的 TypePackVar；
    // ctx（Handle，类型编码非空）/arena 均为构造期接线；as_mutable_type_pack 直译
    // C++ asMutable，单线程串行下 Bound 覆写窗口唯一、无并存可变借用。
    unsafe {
      if (*subject).owning_arena != self.ctx.get().arena_id() {
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
      (*as_mutable_type_pack(subject)).ty = TypePackVariant::Bound(replacement);
    }

    self.result.reduced_packs.insert(subject);
  }
}
