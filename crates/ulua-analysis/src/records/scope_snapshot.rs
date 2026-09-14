use alloc::{string::String, vec::Vec};
use std::collections::HashMap;

use crate::records::{
  binding_snapshot::BindingSnapshot, type_binding_snapshot::TypeBindingSnapshot,
};

#[derive(Debug, Clone, Default)]
pub struct ScopeSnapshot {
  pub bindings: HashMap<String, BindingSnapshot>,
  pub type_bindings: HashMap<String, TypeBindingSnapshot>,
  pub type_pack_bindings: HashMap<String, TypeBindingSnapshot>,
  pub children: Vec<ScopeSnapshot>,
}
