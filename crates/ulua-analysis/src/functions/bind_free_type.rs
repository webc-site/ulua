use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, get_mutable_type, subsumes_scope::subsumes},
  methods::unifiable::unifiable_bound_type_id_emplace_type_bound_type,
  records::{arena_handle::alias_opt, free_type::FreeType},
  type_aliases::type_id::TypeId,
};

pub fn bind_free_type(a: TypeId, b: TypeId) {
  let af = get_mutable_type::get_mutable::<FreeType>(a);
  let bf = get_mutable_type::get_mutable::<FreeType>(b);

  LUAU_ASSERT!(af.is_some() || bf.is_some());

  match (af, bf) {
    // C++: bf 为空 → b 不是 Free，直接绑 a 到 b
    (_, None) => bind(a, b),
    (None, Some(_)) => bind(b, a),
    (Some(af), Some(bf)) => {
      // FreeType::scope 按 cpp NotNull<Scope> 构造即非空；字段仍存裸指针（布局不改），
      // 比较经 `alias_opt` 收口为共享借用。
      if subsumes(alias_opt(bf.scope), alias_opt(af.scope)) {
        bind(a, b);
      } else if subsumes(alias_opt(af.scope), alias_opt(bf.scope)) {
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
