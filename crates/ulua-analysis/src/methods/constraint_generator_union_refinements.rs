use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{
    constraint_generator::ConstraintGenerator, intersection_type::IntersectionType,
    refinement_partition::RefinementPartition,
  },
  type_aliases::{
    constraint_v::ConstraintV, refinement_context::RefinementContext, scope_ptr_type::ScopePtr,
    type_id::TypeId,
  },
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn union_refinements(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    lhs: &RefinementContext,
    rhs: &RefinementContext,
    dest: *mut RefinementContext,
    _constraints: *mut Vec<ConstraintV>,
  ) {
    let scope_raw = arc_as_mut(scope);

    for (def, partition) in lhs.iter() {
      let rhs_partition = match rhs.get(def) {
        Some(p) => p,
        None => continue,
      };

      LUAU_ASSERT!(!partition.discriminant_types.is_empty());
      LUAU_ASSERT!(!rhs_partition.discriminant_types.is_empty());

      let left_discriminant_ty =
        self.intersect_discriminants(scope, location, &partition.discriminant_types);
      let right_discriminant_ty =
        self.intersect_discriminants(scope, location, &rhs_partition.discriminant_types);

      let union_ty = self.make_union_scope_ptr_location_type_id_type_id(
        scope_raw,
        location,
        left_discriminant_ty,
        right_discriminant_ty,
      );

      let should_append_nil =
        partition.should_append_nil_type || rhs_partition.should_append_nil_type;

      // Safety: `dest` 是调用方按 C++ `NotNull<RefinementContext*>` 传出的输出对象，
      // 非空且指向本次调用期间存活、独立于 `self`/`scope` 的 RefinementContext；本次为
      // 单线程串行独占写入，循环体内先 insert 建槽再 get_mut(def).unwrap() 取回该槽可变
      // 引用（key 刚写入必存在），无并发借用，故对 `*dest` 的两次解引用不产生别名冲突。
      unsafe {
        (*dest).insert(*def, RefinementPartition::default());
        // Safety: 紧邻上一行 insert 刚建槽，get_mut 同键必命中。
        let dest_partition = (*dest)
          .get_mut(def)
          .expect("循环体内先 insert 建槽再 get_mut，同键必命中");
        dest_partition.discriminant_types.push(union_ty);
        dest_partition.should_append_nil_type |= should_append_nil;
      }
    }
  }

  /// C++ `intersect` lambda (cpp ConstraintGenerator.cpp:633-641)：
  /// 1 → 唯一类型，2 → makeIntersect，更多 → IntersectionType。
  fn intersect_discriminants(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    types: &[TypeId],
  ) -> TypeId {
    match types {
      [only] => *only,
      [a, b] => self.make_intersect(scope, location, *a, *b),
      _ => {
        // Safety: `self.arena.as_ptr()` 在 ConstraintGenerator 构造时接线为非空裸指针，指向
        // 本次 check 会话存活的类型 arena（bump 块地址不移动），add_type 只追加节点。
        self.arena.get_mut().add_type(IntersectionType {
          parts: types.to_vec(),
        })
      }
    }
  }
}
