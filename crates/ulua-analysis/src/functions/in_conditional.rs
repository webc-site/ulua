use crate::enums::type_context::TypeContext;

pub fn in_conditional(context: TypeContext) -> bool {
  context == TypeContext::Condition
}
