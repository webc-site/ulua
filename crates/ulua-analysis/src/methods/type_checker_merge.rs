use alloc::vec::Vec;

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type, merge::merge},
  records::{type_checker::TypeChecker, union_type::UnionType},
  type_aliases::{refinement_map::RefinementMap, type_id::TypeId},
};
impl TypeChecker {
  pub fn merge(&mut self, l: &mut RefinementMap, r: &RefinementMap) {
    // 上游 lambda 捕获 `this` 并调用 `addType`（TypeInfer.cpp:5460-5482）。
    // `Luau::merge` 对每个 key 至多调用一次回调（LValue.cpp:91-101），所以
    // `impl FnMut` 直接可变借用 self 就够了，无需伪造 `*mut TypeChecker` 别名。
    merge(l, r, |a, b| {
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

      fn flatten_into(push: &mut impl FnMut(TypeId), ty: TypeId) {
        match get_type::get::<UnionType>(follow_type::follow(ty)) {
          // C++ `set.insert(begin(utv), end(utv))`——UnionTypeIterator 展平
          // 嵌套 union 并 follow，裸遍历 options 会漏掉嵌套成员。
          Some(utv) => begin_union_type(utv).for_each(push),
          None => push(ty),
        }
      }
      flatten_into(&mut push, a);
      flatten_into(&mut push, b);

      if options.len() == 1 {
        options[0]
      } else {
        self.add_type(&UnionType { options })
      }
    });
  }
}
