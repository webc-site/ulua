use core::ptr;

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::records::{
  instance_collector_2::InstanceCollector2, type_once_visitor::TypeOnceVisitor,
};

impl InstanceCollector2 {
  pub fn instance_collector_2(&mut self) {
    self.base = TypeOnceVisitor::new("InstanceCollector2".to_string(), true);
    self.tys = VecDeque::new();
    self.tps = VecDeque::new();
    self.cyclic_instance = DenseHashSet::new(ptr::null_mut());
    self.instance_arguments = DenseHashSet::new(ptr::null_mut());
  }
}
