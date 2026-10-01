use crate::records::{
  magic_function::MagicFunction, magic_function_call_context::MagicFunctionCallContext,
};

#[derive(Debug, Clone)]
pub struct MagicFind {
  pub base: MagicFunction,
  pub infer: fn(&MagicFunctionCallContext) -> bool,
}
