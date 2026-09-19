use alloc::vec::Vec;

use crate::{
  records::{scope::Scope, type_level::TypeLevel, type_once_visitor::TypeOnceVisitor},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct Quantifier {
  pub base: TypeOnceVisitor,
  pub level: TypeLevel,
  pub generics: Vec<TypeId>,
  pub generic_packs: Vec<TypePackId>,
  pub scope: *mut Scope,
  pub seen_generic_type: bool,
  pub seen_mutable_type: bool,
}
