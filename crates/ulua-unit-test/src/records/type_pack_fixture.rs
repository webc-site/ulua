use alloc::{boxed::Box, vec::Vec};

use ulua_analysis::{
  records::{r#type::Type, type_pack_var::TypePackVar},
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone, Default)]
pub struct TypePackFixture {
  pub type_packs: Vec<Box<TypePackVar>>,
  pub type_vars: Vec<Box<Type>>,
  pub types: Vec<TypeId>,
}
