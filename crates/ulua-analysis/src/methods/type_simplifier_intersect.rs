use crate::{
  enums::relation::Relation,
  functions::{
    add_intersection::add_intersection, get_type, is_type_variable::is_type_variable,
    relate_simplify::relate_type_id_type_id,
  },
  records::{
    any_type::AnyType, free_type::FreeType, intersection_type::IntersectionType,
    negation_type::NegationType, never_type::NeverType, recursion_limiter::RecursionLimiter,
    type_simplifier::TypeSimplifier, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};
impl TypeSimplifier {
  pub fn intersect(&mut self, mut left: TypeId, mut right: TypeId) -> TypeId {
    let _rl = RecursionLimiter::new("TypeSimplifier::intersect", &mut self.recursion_depth, 15);

    left = self.simplify_type_id(left);
    right = self.simplify_type_id(right);

    if left == right {
      return left;
    }

    // Safety: `self.builtin_types` 对应 C++ `const BuiltinTypes&`，构造时接线为
    // 只读内建类型表单例，非空、不可变且比持有者长寿；取共享借用仅读 `error_type`。
    let bt = self.builtin_types.get();
    if get_type::get::<AnyType>(left).is_some() && get_type::get::<ErrorType>(right).is_some() {
      return right;
    }
    if get_type::get::<AnyType>(right).is_some() && get_type::get::<ErrorType>(left).is_some() {
      return left;
    }
    if get_type::get::<UnknownType>(left).is_some() && get_type::get::<ErrorType>(right).is_none() {
      return right;
    }
    if get_type::get::<UnknownType>(right).is_some() && get_type::get::<ErrorType>(left).is_none() {
      return left;
    }
    if get_type::get::<AnyType>(left).is_some() && get_type::get::<UnionType>(right).is_some() {
      return self.union_(bt.error_type, right);
    }
    if get_type::get::<UnionType>(left).is_some() && get_type::get::<AnyType>(right).is_some() {
      return self.union_(bt.error_type, left);
    }
    if get_type::get::<AnyType>(left).is_some() {
      // Safety: `self.arena` 对应 C++ `NotNull<TypeArena>`，构造时接线、非空，块
      // 地址在遍历期不移动；本 simplify 调用单线程独占 arena，`get_mut` 物化的可变
      // 借用无别名冲突。
      let arena = self.arena.get_mut();
      return arena.add_type(UnionType {
        options: alloc::vec![right, bt.error_type],
      });
    }
    if get_type::get::<AnyType>(right).is_some() {
      // Safety: 同上——arena 非空、块地址稳定、单线程独占，重建可变借用无冲突。
      let arena = self.arena.get_mut();
      return arena.add_type(UnionType {
        options: alloc::vec![left, bt.error_type],
      });
    }
    if get_type::get::<UnknownType>(left).is_some() {
      return right;
    }
    if get_type::get::<UnknownType>(right).is_some() {
      return left;
    }
    if get_type::get::<NeverType>(left).is_some() {
      return left;
    }
    if get_type::get::<NeverType>(right).is_some() {
      return right;
    }

    if let Some(lf) = get_type::get::<FreeType>(left) {
      let relation = relate_type_id_type_id(lf.upper_bound, right);
      if matches!(relation, Relation::Subset | Relation::Coincident) {
        return left;
      }
    } else if let Some(rf) = get_type::get::<FreeType>(right) {
      let relation = relate_type_id_type_id(left, rf.upper_bound);
      if matches!(relation, Relation::Superset | Relation::Coincident) {
        return right;
      }
    }

    if is_type_variable(left) {
      self.blocked_types.insert(left);
      return add_intersection(self.arena, self.builtin_types, &[left, right]);
    }
    if is_type_variable(right) {
      self.blocked_types.insert(right);
      return add_intersection(self.arena, self.builtin_types, &[left, right]);
    }

    if get_type::get::<UnionType>(left).is_some() {
      return if get_type::get::<UnionType>(right).is_some() {
        self.intersect_unions(left, right)
      } else {
        self.intersect_union_with_type(left, right)
      };
    } else if get_type::get::<UnionType>(right).is_some() {
      return self.intersect_union_with_type(right, left);
    }

    if get_type::get::<IntersectionType>(left).is_some() {
      return self.intersect_intersection_with_type(left, right);
    } else if get_type::get::<IntersectionType>(right).is_some() {
      return self.intersect_intersection_with_type(right, left);
    }

    if get_type::get::<NegationType>(left).is_some() {
      return if get_type::get::<NegationType>(right).is_some() {
        self.intersect_negations(left, right)
      } else {
        self.intersect_type_with_negation(left, right)
      };
    } else if get_type::get::<NegationType>(right).is_some() {
      return self.intersect_type_with_negation(right, left);
    }

    if let Some(res) = self.basic_intersect(left, right) {
      res
    } else {
      // Safety: 同上——`self.arena` 非空、块地址稳定、单线程独占使用，
      // `get_mut` 物化的可变借用与任何只读借用不重叠。
      let arena = self.arena.get_mut();
      arena.add_type(IntersectionType {
        parts: alloc::vec![left, right],
      })
    }
  }
}
