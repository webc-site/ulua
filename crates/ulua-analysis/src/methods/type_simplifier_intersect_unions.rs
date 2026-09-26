use ulua_common::dfint;

use crate::{
  functions::{add_intersection::add_intersection, begin_type::begin_union_type, get_type},
  records::{type_simplifier::TypeSimplifier, union_builder::UnionBuilder, union_type::UnionType},
  type_aliases::type_id::TypeId,
};
impl TypeSimplifier {
  pub fn intersect_unions(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // C++ Simplify.cpp intersectUnions: 前置断言两侧均为 union
    let left_union = get_type::get::<UnionType>(left).expect("left is UnionType");
    let right_union = get_type::get::<UnionType>(right).expect("right is UnionType");

    // Combinatorial blowup moment!!

    // combination size
    let option_size = left_union.options.len() * right_union.options.len();
    let max_size = dfint::LuauSimplificationComplexityLimit.get() as usize;

    if option_size > max_size {
      return add_intersection(self.arena, self.builtin_types, &[left, right]);
    }

    let mut ub = UnionBuilder::new(self.arena, self.builtin_types);

    // C++ 的 `for (TypeId part : leftUnion)` 走防环且会展平嵌套 union 的
    // `TypeIterator`；裸遍历 options 会在环状 union 上无限递归。
    for left_part in begin_union_type(left_union) {
      for right_part in begin_union_type(right_union) {
        let simplified = self.intersect(left_part, right_part);
        ub.add(simplified);

        // Initial combination size check could not predict nested union iteration
        if ub.size() > max_size {
          return add_intersection(self.arena, self.builtin_types, &[left, right]);
        }
      }
    }

    ub.build()
  }
}
