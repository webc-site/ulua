#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct UnifierOptions {
  pub is_function_call: bool,
}

unsafe impl Send for UnifierOptions {}
unsafe impl Sync for UnifierOptions {}
