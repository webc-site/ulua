//! `type_simplifier` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::mem::swap;

use ulua_common::{dfint, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{inhabited::Inhabited, relation::Relation},
  functions::{
    add_intersection::add_intersection, begin_type::begin_union_type, follow_type, get_type,
    intersect_one_with_intersection::intersect_one_with_intersection,
    relate_simplify::relate_type_id_type_id,
  },
  records::{
    intersection_builder::IntersectionBuilder, intersection_type::IntersectionType,
    negation_type::NegationType, never_type::NeverType, property_type::Property, type_ids::TypeIds,
    type_simplifier::TypeSimplifier, union_builder::UnionBuilder, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_from_parts(&mut self, parts: TypeIds) -> TypeId {
    let builtin_types = self.builtin_types.get();

    if parts.size() == 0 {
      return builtin_types.unknown_type;
    }

    if parts.size() == 1 {
      return parts.front();
    }

    let mut source = TypeIds::new();
    let mut dest = TypeIds::new();

    source.reserve(parts.size());
    dest.reserve(parts.size());

    for &part in &parts.order {
      if intersect_one_with_intersection(self, &mut source, &mut dest, part) == Inhabited::No {
        return builtin_types.never_type;
      }

      swap(&mut source, &mut dest);
      dest.clear_without_realloc();
    }

    let mut ib = IntersectionBuilder::new(self.arena, self.builtin_types);

    for &ty in &source.order {
      ib.add(ty);
    }

    ib.build()
  }
}

impl TypeSimplifier {
  pub fn intersect_negations(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // C++ Simplify.cpp intersectNegations: LUAU_ASSERT(leftNegation/rightNegation)
    let left_negation = get_type::get::<NegationType>(left).expect("left is NegationType");
    if get_type::get::<UnionType>(follow_type::follow(left_negation.ty)).is_some() {
      return self.intersect_negated_union(left, right);
    }
    let right_negation = get_type::get::<NegationType>(right).expect("right is NegationType");
    if get_type::get::<UnionType>(follow_type::follow(right_negation.ty)).is_some() {
      return self.intersect_negated_union(right, left);
    }
    match relate_type_id_type_id(left_negation.ty, right_negation.ty) {
      // ~true & ~true
      Relation::Coincident | Relation::Superset => left,
      // ~true & ~boolean
      Relation::Subset => right,
      // ~boolean & ~string
      _ => {
        let arena = self.arena.get_mut();
        arena.add_type(IntersectionType {
          parts: alloc::vec![left, right],
        })
      }
    }
  }
}

impl TypeSimplifier {
  pub fn intersect_one(&self, target: TypeId, discriminant: TypeId) -> Option<TypeId> {
    let builtin_types = self.builtin_types.get();
    match relate_type_id_type_id(target, discriminant) {
      Relation::Disjoint => Some(builtin_types.never_type),
      Relation::Subset | Relation::Coincident => Some(target),
      Relation::Superset => Some(discriminant),
      Relation::Intersects => None,
    }
  }
}

// Faithful port of `TypeSimplifier::intersectProperty` (Simplify.cpp:1790-1827).

impl TypeSimplifier {
  pub fn intersect_property(
    &self,
    target: &Property,
    discriminant: &Property,
    seen: &mut DenseHashSet<TypeId>,
  ) -> Option<Property> {
    // NOTE: I invite the reader to refactor the below code as a fun coding
    // exercise. It looks ugly to me, but I don't think we can make it
    // any cleaner.

    let mut prop = Property {
      deprecated: target.deprecated || discriminant.deprecated,
      ..Default::default()
    };

    // We're trying to follow the following rules for both read and write types:
    // * If the type is present on both properties, intersect it, and return
    //   `None` if we fail.
    // * If the type only exists on one property or the other, take that.

    match (target.read_ty, discriminant.read_ty) {
      (Some(l), Some(r)) => {
        prop.read_ty = self
          .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(l, r, seen);
        prop.read_ty?;
      }
      (Some(l), None) => prop.read_ty = Some(l),
      (None, Some(r)) => prop.read_ty = Some(r),
      (None, None) => {}
    }

    match (target.write_ty, discriminant.write_ty) {
      (Some(l), Some(r)) => {
        prop.write_ty = self
          .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(l, r, seen);
        prop.write_ty?;
      }
      (Some(l), None) => prop.write_ty = Some(l),
      (None, Some(r)) => prop.write_ty = Some(r),
      (None, None) => {}
    }

    Some(prop)
  }
}

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

impl TypeSimplifier {
  pub fn mk_negation(&self, ty: TypeId) -> TypeId {
    // 契约：self.builtin_types 在构造期接线为 `Handle<BuiltinTypes>`，指向比
    // self 长寿的内置类型表；仅重建共享借用读取其字段，不产生可变别名。
    let builtin_types = self.builtin_types.get();
    // 契约：self.arena 为构造期接线的 `Handle<TypeArena>`，指向比 self 长寿且本线程独占
    // 的类型 arena（bump arena 地址稳定）；&self 仅借用 simplifier 字段、不覆盖 arena，
    // 故重建 &mut 与上面的 builtin_types 借用指向不同对象，无别名冲突。
    let arena = self.arena.get_mut();
    if ty == builtin_types.truthy_type {
      builtin_types.falsy_type
    } else if ty == builtin_types.falsy_type {
      builtin_types.truthy_type
    } else if let Some(ntv) = get_type::get::<NegationType>(ty) {
      follow_type::follow(ntv.ty)
    } else {
      arena.add_type(NegationType { ty })
    }
  }
}

impl TypeSimplifier {
  pub fn subtract_one(&self, target: TypeId, discriminant: TypeId) -> Option<TypeId> {
    let builtin_types = self.builtin_types.get();
    let target = follow_type::follow(target);
    let discriminant = follow_type::follow(discriminant);
    if let Some(nt) = get_type::get::<NegationType>(discriminant) {
      return self.intersect_one(target, nt.ty);
    }
    match relate_type_id_type_id(target, discriminant) {
      Relation::Disjoint => Some(target),
      Relation::Subset | Relation::Coincident => Some(builtin_types.never_type),
      _ => None,
    }
  }
}
