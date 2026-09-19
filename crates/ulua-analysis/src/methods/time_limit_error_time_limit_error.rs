use crate::records::{
  internal_compiler_error::InternalCompilerError, time_limit_error::TimeLimitError,
};

impl TimeLimitError {
  pub fn time_limit_error_time_limit_error(module_name: &str) -> Self {
    Self {
      base: InternalCompilerError::new(
        "Typeinfer failed to complete in allotted time".to_string(),
        Some(module_name.to_owned()),
        None,
      ),
    }
  }
}
