use alloc::vec::Vec;

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{type_checker::TypeChecker, union_type::UnionType},
  type_aliases::{type_id::TypeId, type_id_predicate::TypeIdPredicate},
};

/// C++ `std::vector<TypeId> filterMap(TypeId type, TypeIdPredicate predicate)`
/// (`Analysis/src/Type.cpp:1121-1138`)。
/// `tc` 只为回调提供上游 lambda 捕获的 `this`，本函数自身不碰它。
pub fn filter_map<P: TypeIdPredicate>(
  type_: TypeId,
  tc: &mut TypeChecker,
  predicate: &mut P,
) -> Vec<TypeId> {
  let type_ = follow_type::follow(type_);

  if let Some(utv) = get_type::get::<UnionType>(type_) {
    // Dedupe while preserving the union's option order. The C++ original
    // uses `std::set<TypeId>` (ordered by pointer address — deterministic
    // within a run but ASLR-dependent across runs); a `HashSet<TypeId>`
    // iterates in per-instance RandomState order, making the resulting
    // union's option order — and every diagnostic derived from it —
    // nondeterministic even within a single process. Insertion-order
    // dedup is fully deterministic and keeps diagnostics stable.
    let mut options: Vec<TypeId> = Vec::new();

    // C++ `for (TypeId option : utv)` — UnionTypeIterator 防环展平并 follow。
    for option in begin_union_type(utv) {
      let followed_option = follow_type::follow(option);
      if let Some(out) = predicate(tc, followed_option)
        && !options.contains(&out)
      {
        options.push(out);
      }
    }

    options
  } else if let Some(out) = predicate(tc, type_) {
    vec![out]
  } else {
    Vec::new()
  }
}
