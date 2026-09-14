use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    is_optional_type::is_optional_type,
  },
  records::{builtin_types::BuiltinTypes, overload_resolver::OverloadResolver},
  type_aliases::type_pack_id::TypePackId,
};

impl OverloadResolver {
  /// # Safety
  /// 调用方须保证 `builtin_types` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn is_arity_compatible(
    &self,
    candidate: TypePackId,
    desired: TypePackId,
    builtin_types: *mut BuiltinTypes,
  ) -> bool {
    let (candidate_head, candidate_tail) = flatten_type_pack_id(candidate);
    let (desired_head, desired_tail) = flatten_type_pack_id(desired);

    if candidate_head.len() < desired_head.len() {
      if candidate_tail.is_some() {
        return true;
      }

      let all_unsatisfied_optional = desired_head[candidate_head.len()..]
        .iter()
        .all(|&d| unsafe { is_optional_type(follow_type_id(d), builtin_types) });
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
