use core::ptr::NonNull;

use crate::{
  records::{substitution::Substitution, type_function_context::TypeFunctionContext},
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct RefineTypeScrubber {
  pub(crate) base: Substitution,
  pub(crate) ctx: NonNull<TypeFunctionContext>,
  pub(crate) needle: TypeId,
}
