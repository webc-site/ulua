use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  records::type_once_visitor::TypeOnceVisitor,
  type_aliases::{
    type_id::TypeId, type_or_type_pack_id_set::TypeOrTypePackIdSet, type_pack_id::TypePackId,
  },
};
#[derive(Debug, Clone)]
pub struct InstanceCollector {
  pub base: TypeOnceVisitor,
  pub recorded_tys: DenseHashSet<TypeId>,
  pub tys: VecDeque<TypeId>,
  pub recorded_tps: DenseHashSet<TypePackId>,
  pub tps: VecDeque<TypePackId>,
  pub should_guess: TypeOrTypePackIdSet,
  pub type_function_instance_stack: Vec<*const c_void>,
  pub cyclic_instance: Vec<TypeId>,
}
