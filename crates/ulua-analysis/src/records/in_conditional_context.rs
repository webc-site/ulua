use crate::enums::type_context::TypeContext;

#[derive(Debug, Clone)]
pub struct InConditionalContext {
  pub(crate) type_context: *mut TypeContext,
  pub(crate) old_value: TypeContext,
}
