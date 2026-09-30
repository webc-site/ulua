use ulua_analysis::records::magic_function_call_context::MagicFunctionCallContext;

use crate::records::magic_instance_is_a::MagicInstanceIsA;

impl MagicInstanceIsA {
  pub fn infer(&self, _context: &MagicFunctionCallContext) -> bool {
    false
  }
}
