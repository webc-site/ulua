use core::ptr::{NonNull, null};

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  records::{constraint_graph::ConstraintGraph, constraint_list::ConstraintList, r#type::Type},
  type_aliases::constraint_vertex::ConstraintVertex,
};
impl ConstraintGraph {
  pub fn find_dependency_list(&mut self, vertex: ConstraintVertex) -> NonNull<ConstraintList> {
    if let Some(dep) = self.dependencies.find(&vertex) {
      return NonNull::new(*dep).unwrap();
    }

    let ptr = self.constraint_lists.push(ConstraintList {
      present: DenseHashMap::new(ConstraintVertex::V0(null::<Type>())),
      order: Vec::new(),
      entries: 0,
    });
    let newlist = NonNull::new(ptr).unwrap();

    let (_it, fresh) = self.dependencies.try_insert(vertex, newlist.as_ptr());
    LUAU_ASSERT!(fresh);
    newlist
  }
}
