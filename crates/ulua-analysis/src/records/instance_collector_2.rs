use alloc::string::String;

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  records::{extern_type::ExternType, type_once_visitor::TypeOnceVisitor},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct InstanceCollector2 {
  pub base: TypeOnceVisitor,
  pub tys: VecDeque<TypeId>,
  pub tps: VecDeque<TypePackId>,
  pub cyclic_instance: DenseHashSet<TypeId>,
  pub instance_arguments: DenseHashSet<TypeId>,
}

impl InstanceCollector2 {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("InstanceCollector2"), true),
      tys: VecDeque::new(),
      tps: VecDeque::new(),
      cyclic_instance: DenseHashSet::default(),
      instance_arguments: DenseHashSet::default(),
    }
  }

  // `cycle` lives in its own method node file (methods/instance_collector_2_cycle.rs).

  pub fn visit_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }
}

impl Default for InstanceCollector2 {
  fn default() -> Self {
    Self::new()
  }
}
