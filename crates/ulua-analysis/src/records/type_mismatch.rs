use alloc::{string::String, sync::Arc};

use crate::{
  enums::context_error::Context, records::type_error::TypeError, type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone, PartialEq)]
pub struct TypeMismatch {
  pub wanted_type: TypeId,
  pub given_type: TypeId,
  pub reason: String,
  pub error: Option<Arc<TypeError>>,
  pub context: Context,
}
