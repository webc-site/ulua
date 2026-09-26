use ulua_common::dfint;

use crate::{
  functions::{add_intersection::add_intersection, begin_type::begin_union_type, get_type},
  records::{
    never_type::NeverType, type_simplifier::TypeSimplifier, union_builder::UnionBuilder,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_union_with_type(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // C++ Simplify.cpp intersectUnionWithType: 前置断言 left 为 union
    let left_union = get_type::get::<UnionType>(left).expect("left is UnionType");

    let mut changed = false;
    let max_size = dfint::LuauSimplificationComplexityLimit.get() as usize;

    if left_union.options.len() > max_size {
      return add_intersection(self.arena, self.builtin_types, &[left, right]);
    }

    let mut ub = UnionBuilder::new(self.arena, self.builtin_types);
    ub.reserve(left_union.options.len());

    // C++ 的 `for (TypeId part : leftUnion)` 走防环且展平嵌套 union 的
    // `TypeIterator`，裸遍历 options 会在环状 union 上无限递归。
    for part in begin_union_type(left_union) {
      let simplified = self.intersect(right, part);
      changed |= simplified != part;

      if get_type::get::<NeverType>(simplified).is_some() {
        changed = true;
        continue;
      }

      ub.add(simplified);

      // Initial combination size check could not predict nested union iteration
      if ub.size() > max_size {
        return add_intersection(self.arena, self.builtin_types, &[left, right]);
      }
    }

    if !changed {
      return left;
    }

    ub.build()
  }
}
