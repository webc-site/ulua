#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeCorrectKind {
  None,
  Correct,
  CorrectFunctionResult,
}
