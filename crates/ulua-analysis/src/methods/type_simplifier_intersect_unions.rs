use core::ptr::null_mut;

use ulua_common::{DFInt, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{add_intersection::add_intersection, get_type_alt_j::get_type_id},
  records::{type_simplifier::TypeSimplifier, union_builder::UnionBuilder, union_type::UnionType},
  type_aliases::type_id::TypeId,
};
impl TypeSimplifier {
  pub fn intersect_unions(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // C++ Simplify.cpp intersectUnions: 前置断言两侧均为 union
    let left_union = get_type_id::<UnionType>(left).expect("left is UnionType");
    let right_union = get_type_id::<UnionType>(right).expect("right is UnionType");

    let _new_parts: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());

    // Combinatorial blowup moment!!

    // combination size
    let option_size = left_union.options.len() * right_union.options.len();
    let max_size = DFInt::LuauSimplificationComplexityLimit.get() as usize;

    if option_size > max_size {
      return add_intersection(
        self.arena.cast_mut(),
        self.builtin_types as *mut _,
        &[left, right],
      );
    }

    let mut ub = UnionBuilder::new(self.arena.cast_mut(), self.builtin_types as *mut _);

    for &left_part in &left_union.options {
      for &right_part in &right_union.options {
        let simplified = self.intersect(left_part, right_part);
        ub.add(simplified);

        // Initial combination size check could not predict nested union iteration
        if ub.size() > max_size {
          return add_intersection(
            self.arena.cast_mut(),
            self.builtin_types as *mut _,
            &[left, right],
          );
        }
      }
    }

    ub.build()
  }
}
