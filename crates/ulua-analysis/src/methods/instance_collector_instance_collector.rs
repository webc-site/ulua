use alloc::vec::Vec;

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  records::{instance_collector::InstanceCollector, type_once_visitor::TypeOnceVisitor},
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};
impl InstanceCollector {
  pub fn instance_collector(&mut self) {
    self.base = TypeOnceVisitor::new("InstanceCollector".to_string(), true);
    self.recorded_tys = DenseHashSet::new(TypeId::default());
    self.tys = VecDeque::new();
    self.recorded_tps = DenseHashSet::new(TypePackId::default());
    self.tps = VecDeque::new();
    self.should_guess = TypeOrTypePackIdSet::default();
    self.type_function_instance_stack = Vec::new();
    self.cyclic_instance = Vec::new();
  }
}
