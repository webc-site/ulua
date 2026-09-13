//! @interface-stub
use alloc::collections::VecDeque;
use core::ptr::null;

use ulua_common::records::{dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault};

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow, get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    constraint_generator::ConstraintGenerator, metatable_type::MetatableType,
    table_type::TableType, type_ids::TypeIds, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
impl DenseDefault for TypeIds {
  fn dense_default() -> Self {
    TypeIds::new()
  }
}

impl ConstraintGenerator {
  pub fn record_property_assignment(&mut self, ty: TypeId) -> bool {
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
    let mut queue = VecDeque::new();

    queue.push_back(ty);

    let mut incremented = false;

    while let Some(front) = queue.pop_front() {
      let t = follow(front);

      if seen.find(&t).is_some() {
        continue;
      }

      seen.insert(t);

      // 对照 C++ recordPropertyAssignment：`getMutable<TableType>(t); ttv && ...`
      if let Some(tt) = get_mutable_type_id::<TableType>(t)
        && tt.state == TableState::Unsealed
      {
        tt.remaining_props += 1;
        incremented = true;
        continue;
      }

      if let Some(mt) = get_type_id::<MetatableType>(t) {
        queue.push_back(mt.table);
        continue;
      }

      if let Some(local_domain) = self.local_types.find(&t) {
        for &domain_ty in &local_domain.order {
          queue.push_back(domain_ty);
        }
        continue;
      }

      if let Some(ut) = get_type_id::<UnionType>(t) {
        for &part in &ut.options {
          queue.push_back(part);
        }
      }
    }

    incremented
  }
}
