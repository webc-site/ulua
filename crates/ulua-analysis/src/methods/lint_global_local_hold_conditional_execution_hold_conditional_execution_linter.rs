use crate::records::lint_global_local::LintGlobalLocal;

impl LintGlobalLocal {
  pub fn hold_conditional_execution(&mut self) {
    if let Some(info) = self.function_stack.last_mut()
      && !info.conditional_execution
    {
      info.conditional_execution = true;
    }
  }

  pub(crate) fn set_conditional_execution(&mut self) -> bool {
    if let Some(info) = self.function_stack.last_mut()
      && !info.conditional_execution
    {
      info.conditional_execution = true;
      return true;
    }

    false
  }
}
