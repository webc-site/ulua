use core::ptr::null;

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::constraint_graph::ConstraintGraph,
  type_aliases::{constraint_vertex::ConstraintVertex, type_pack_id::TypePackId},
};
impl ConstraintGraph {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn unblock_type_or_pack_type_pack_id(&mut self, vertex: TypePackId) {
    self.repair_type_references_type_pack_id(vertex);
    let vertex = unsafe { follow_type_pack_id(vertex) };
    let _vertex = vertex;

    self.clear_reverse_dependencies_of(ConstraintVertex::V2(null()));
  }
}
