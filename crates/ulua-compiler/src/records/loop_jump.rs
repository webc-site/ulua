use crate::enums::type_compiler::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LoopJump {
  pub(crate) r#type: Type,
  pub(crate) label: usize,
}
