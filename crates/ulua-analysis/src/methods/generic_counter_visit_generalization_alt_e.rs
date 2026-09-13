use crate::records::generic_counter::GenericCounter;

impl GenericCounter {
  pub fn visit_type_id_generic_type(&mut self) -> bool {
    // Mirrors the C++:
    // bool visit(TypeId ty, const GenericType&) override { ... }
    //
    // This Rust one-shot item only specifies the generic-type visit hook;
    // the TypeId is provided by the visitor dispatch into GenericCounter.
    self.visit_type_id()
  }
}
