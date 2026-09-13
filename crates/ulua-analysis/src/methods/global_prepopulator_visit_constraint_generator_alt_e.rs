use crate::records::global_prepopulator::GlobalPrepopulator;
impl GlobalPrepopulator {
  pub fn visit_ast_type_pack(&mut self) {
    // The C++ implementation always returns true and does not traverse children here.
  }
}
