//! @interface-stub
use alloc::vec::Vec;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, merge::merge},
  records::{type_checker::TypeChecker, union_type::UnionType},
  type_aliases::{refinement_map::RefinementMap, type_id::TypeId},
};
impl TypeChecker {
  pub fn merge(&mut self, l: &mut RefinementMap, r: &RefinementMap) {
    // merge 的回调要求 Fn（可能多次调用），&mut 捕获不可行，按 C++ 捕 this 处理。
    let this = self as *mut TypeChecker;

    merge(l, r, &|a, b| {
      // Insertion-order dedup (not HashSet<TypeId>): a pointer-keyed HashSet
      // iterates in per-instance randomized order, which would make the merged
      // union's option order — and every diagnostic derived from it —
      // nondeterministic across runs. Preserving a-then-b order is stable.
      let mut options: Vec<TypeId> = Vec::new();
      let mut push = |t: TypeId| {
        if !options.contains(&t) {
          options.push(t);
        }
      };

      let flatten_into = |push: &mut dyn FnMut(TypeId), ty: TypeId| match get_type_id::<UnionType>(
        follow_type_id(ty),
      ) {
        Some(utv) => utv.options.iter().copied().for_each(push),
        None => push(ty),
      };
      flatten_into(&mut push, a);
      flatten_into(&mut push, b);

      if options.len() == 1 {
        options[0]
      } else {
        // SAFETY: this 指向自身；l/r 与 self 无别名，单线程顺序调用。
        unsafe { (*this).add_type(&UnionType { options }) }
      }
    });
  }
}
