use alloc::vec::Vec;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::union_type::UnionType,
  type_aliases::{type_id::TypeId, type_id_predicate::TypeIdPredicate},
};

pub fn filter_map(type_: TypeId, predicate: TypeIdPredicate) -> Vec<TypeId> {
  let type_ = follow_type_id(type_);

  if let Some(utv) = get_type_id::<UnionType>(type_) {
    // Dedupe while preserving the union's option order. The C++ original
    // uses `std::set<TypeId>` (ordered by pointer address — deterministic
    // within a run but ASLR-dependent across runs); a `HashSet<TypeId>`
    // iterates in per-instance RandomState order, making the resulting
    // union's option order — and every diagnostic derived from it —
    // nondeterministic even within a single process. Insertion-order
    // dedup is fully deterministic and keeps diagnostics stable.
    let mut options: Vec<TypeId> = Vec::new();

    for &option in utv.options.iter() {
      let followed_option = follow_type_id(option);
      if let Some(out) = predicate(followed_option)
        && !options.contains(&out)
      {
        options.push(out);
      }
    }

    options
  } else if let Some(out) = predicate(type_) {
    vec![out]
  } else {
    Vec::new()
  }
}
