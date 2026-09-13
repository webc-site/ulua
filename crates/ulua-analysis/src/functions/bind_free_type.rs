//! @interface-stub
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, get_mutable_type::get_mutable_type_id,
    subsumes_scope::subsumes,
  },
  methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
  records::free_type::FreeType,
  type_aliases::type_id::TypeId,
};

pub fn bind_free_type(a: TypeId, b: TypeId) {
  let af = get_mutable_type_id::<FreeType>(a);
  let bf = get_mutable_type_id::<FreeType>(b);

  LUAU_ASSERT!(af.is_some() || bf.is_some());

  match (af, bf) {
    // C++: bf 为空 → b 不是 Free，直接绑 a 到 b
    (_, None) => bind(a, b),
    (None, Some(_)) => bind(b, a),
    (Some(af), Some(bf)) => {
      if subsumes(bf.scope, af.scope) {
        bind(a, b);
      } else if subsumes(af.scope, bf.scope) {
        bind(b, a);
      }
    }
  }
}

fn bind(ty: TypeId, mut bound_to: TypeId) {
  // SAFETY: ty/bound_to 为有效句柄，emplace 语义与 C++ ConstraintEmitter 一致
  unsafe {
    unifiable_bound_type_id_emplace_type_bound_type(&mut *as_mutable_type_id(ty), &mut bound_to);
  }
}
