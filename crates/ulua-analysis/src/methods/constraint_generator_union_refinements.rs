use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
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
  /// C++ `unionRefinements(const ScopePtr& scope, Location, RefinementContext& lhs,
  /// RefinementContext& rhs, RefinementContext* dest, std::vector<ConstraintV>*)`。
  /// 形参链已引用化（原 `# Safety` 契约由签名承担）：`dest` 为调用方独占存活的
  /// 输出对象（cpp `NotNull<RefinementContext*>`），不与 `lhs`/`rhs` 借用重叠；
  /// 单线程串行，循环体内先 insert 建槽再 get_mut 同键必命中。
  pub fn union_refinements(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    lhs: &RefinementContext,
    rhs: &RefinementContext,
    dest: &mut RefinementContext,
    _constraints: &mut Vec<ConstraintV>,
  ) {
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
        scope,
        location,
        left_discriminant_ty,
        right_discriminant_ty,
      );

      let should_append_nil =
        partition.should_append_nil_type || rhs_partition.should_append_nil_type;

      dest.insert(*def, RefinementPartition::default());
      // 紧邻上一行 insert 刚建槽，get_mut 同键必命中。
      let dest_partition = dest
        .get_mut(def)
        .expect("循环体内先 insert 建槽再 get_mut，同键必命中");
      dest_partition.discriminant_types.push(union_ty);
      dest_partition.should_append_nil_type |= should_append_nil;
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
        // arena 为构造时接线、指向本次 check 会话存活的类型 arena
        //（bump 块地址不移动），add_type 只追加节点。
        self.arena.get_mut().add_type(IntersectionType {
          parts: types.to_vec(),
        })
      }
    }
  }
}
