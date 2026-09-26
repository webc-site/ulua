use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::apply_type_function::ApplyTypeFunction,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyTypeFunction {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    // cpp `find` 未命中返回 null 由紧随 LUAU_ASSERT 拦截；Rust 以 None 表意
    // 同一未命中，expect 确定化呈现。
    let arg = self
      .type_arguments
      .find(&ty)
      .expect("type_arguments 未命中即 cpp 断言拦截的同位置 null");
    LUAU_ASSERT!(!arg.is_null());
    *arg
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let arg = self
      .type_pack_arguments
      .find(&tp)
      .expect("TypePackId not found in type_pack_arguments");
    LUAU_ASSERT!(!arg.is_null());
    *arg
  }
}
