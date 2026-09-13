use alloc::string::String;

use crate::{enums::value_context::ValueContext, type_aliases::type_id::TypeId};

#[derive(Debug, Clone)]
pub struct HasPropConstraint {
  pub result_type: TypeId,
  pub subject_type: TypeId,
  pub prop: String,
  pub context: ValueContext,
  pub in_conditional: bool,
  pub suppress_simplification: bool,
}
