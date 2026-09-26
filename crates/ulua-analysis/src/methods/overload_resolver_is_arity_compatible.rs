use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type, is_optional_type::is_optional_type,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, overload_resolver::OverloadResolver,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl OverloadResolver<'_> {
  pub(crate) fn is_arity_compatible(
    &self,
    candidate: TypePackId,
    desired: TypePackId,
    builtin_types: Handle<BuiltinTypes>,
  ) -> bool {
    let (candidate_head, candidate_tail) = flatten_type_pack_id(candidate);
    let (desired_head, desired_tail) = flatten_type_pack_id(desired);

    if candidate_head.len() < desired_head.len() {
      if candidate_tail.is_some() {
        return true;
      }

      let all_unsatisfied_optional = desired_head[candidate_head.len()..]
        .iter()
        .all(|&d| unsafe {
          // Safety: builtin_types 源自 OverloadResolver 构造期布线的会话句柄（C++
          // NotNull<BuiltinTypes>），指向对象存活于 self 生命周期；is_optional_type 对其只读。
          is_optional_type(follow_type::follow(d), builtin_types)
        });
      if !all_unsatisfied_optional {
        return false;
      }
    }

    if candidate_head.len() > desired_head.len() {
      return desired_tail.is_some();
    }

    true
  }
}
