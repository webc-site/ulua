use crate::records::function::Function;

impl Function {
  pub fn new() -> Self {
    Self {
      vararg: false,
      loop_depth: 0,
    }
  }
}

pub fn parser_function_function() -> Function {
  Function::new()
}
