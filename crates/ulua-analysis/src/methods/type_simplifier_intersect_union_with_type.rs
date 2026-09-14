use ulua_common::DFInt;

use crate::{
  functions::{
    add_intersection::add_intersection, begin_type::begin_union_type, end_type::end_union_type,
    get_type_alt_j::get_type_id,
  },
  records::{
    never_type::NeverType, type_simplifier::TypeSimplifier, union_builder::UnionBuilder,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_union_with_type(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // C++ Simplify.cpp intersectUnionWithType: 前置断言 left 为 union
    let left_union = get_type_id::<UnionType>(left).expect("left is UnionType");

    let mut changed = false;
    let max_size = DFInt::LuauSimplificationComplexityLimit.get() as usize;

    if left_union.options.len() > max_size {
      return add_intersection(
        self.arena.cast_mut(),
        self.builtin_types as *mut _,
        &[left, right],
      );
    }

    let mut ub = UnionBuilder::new(self.arena.cast_mut(), self.builtin_types as *mut _);
    ub.reserve(left_union.options.len());

    // `for (TypeId part : leftUnion)` uses the cycle-protecting
    // `UnionTypeIterator`, which follows and flattens nested unions and
    // skips ones it has already visited — without it, a cyclic union here
    // recurses forever.
    let mut iter = begin_union_type(left_union);
    let end = end_union_type(left_union);
    while iter.operator_ne(&end) {
      let part = iter.operator_deref();

      let simplified = self.intersect(right, part);
      changed |= simplified != part;

      if get_type_id::<NeverType>(simplified).is_some() {
        changed = true;
        iter.operator_inc();
        continue;
      }

      ub.add(simplified);

      // Initial combination size check could not predict nested union iteration
      if ub.size() > max_size {
        return add_intersection(
          self.arena.cast_mut(),
          self.builtin_types as *mut _,
          &[left, right],
        );
      }

      iter.operator_inc();
    }

    if !changed {
      return left;
    }

    ub.build()
  }
}
