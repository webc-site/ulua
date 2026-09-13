use core::ffi::c_void;
use std::collections::BTreeMap;

use ulua_ast::records::allocator::Allocator;

use crate::{
  records::type_rehydration_options::TypeRehydrationOptions,
  type_aliases::synthetic_names::SyntheticNames,
};

#[derive(Debug)]
pub struct TypeRehydrationVisitor {
  pub(crate) seen: BTreeMap<*mut c_void, i32>,
  pub(crate) count: i32,
  pub(crate) allocator: *mut Allocator,
  pub(crate) synthetic_names: *mut SyntheticNames,
  pub(crate) options: TypeRehydrationOptions,
}
