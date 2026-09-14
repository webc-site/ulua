use alloc::{string::String, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{type_function_context::TypeFunctionContext, type_once_visitor::TypeOnceVisitor},
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct FindUserTypeFunctionBlockers {
  pub base: TypeOnceVisitor,
  pub(crate) ctx: NonNull<TypeFunctionContext>,
  pub(crate) blocking_type_map: DenseHashSet<TypeId>,
  pub(crate) blocking_types: Vec<TypeId>,
}

impl FindUserTypeFunctionBlockers {
  pub fn new(ctx: NonNull<TypeFunctionContext>) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("FindUserTypeFunctionBlockers"), true),
      ctx,
      blocking_type_map: DenseHashSet::new(null_mut()),
      blocking_types: Vec::new(),
    }
  }
}
