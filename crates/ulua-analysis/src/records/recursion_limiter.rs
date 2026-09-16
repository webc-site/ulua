use crate::records::{native_stack_guard::NativeStackGuard, recursion_counter::RecursionCounter};
#[derive(Debug)]
pub struct RecursionLimiter {
  pub(crate) base: RecursionCounter,
  pub(crate) native_stack_guard: NativeStackGuard,
}
