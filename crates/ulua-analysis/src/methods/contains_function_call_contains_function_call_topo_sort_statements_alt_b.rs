use crate::records::contains_function_call::ContainsFunctionCall;

impl ContainsFunctionCall {
  pub fn contains_function_call_bool(&mut self, also_return: bool) {
    self.also_return = also_return;
    self.contains_function_call();
  }
}
