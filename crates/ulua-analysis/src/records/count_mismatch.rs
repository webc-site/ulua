use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountMismatchContext {
  Arg,
  FunctionResult,
  ExprListResult,
  Return,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CountMismatch {
  pub(crate) expected: usize,
  pub(crate) maximum: Option<usize>,
  pub(crate) actual: usize,
  pub(crate) context: CountMismatchContext,
  pub(crate) is_variadic: bool,
  pub(crate) function: String,
}

impl Default for CountMismatch {
  fn default() -> Self {
    Self {
      expected: 0,
      maximum: None,
      actual: 0,
      context: CountMismatchContext::Arg,
      is_variadic: false,
      function: String::new(),
    }
  }
}

impl CountMismatch {
  pub const ARG: CountMismatchContext = CountMismatchContext::Arg;
  pub const FUNCTION_RESULT: CountMismatchContext = CountMismatchContext::FunctionResult;
  pub const EXPR_LIST_RESULT: CountMismatchContext = CountMismatchContext::ExprListResult;
  pub const RETURN: CountMismatchContext = CountMismatchContext::Return;

  pub fn expected(&self) -> usize {
    self.expected
  }

  pub fn actual(&self) -> usize {
    self.actual
  }

  pub fn context(&self) -> CountMismatchContext {
    self.context
  }

  pub fn is_variadic(&self) -> bool {
    self.is_variadic
  }
}
