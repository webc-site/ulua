use crate::records::magic_function::MagicFunction;

impl Drop for MagicFunction {
  fn drop(&mut self) {
    // virtual ~MagicFunction() {}
  }
}
