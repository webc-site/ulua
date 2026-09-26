use crate::{
  records::{extern_type::ExternType, generic_counter::GenericCounter, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl GenericCounter {
  pub fn visit_type_id(&mut self) -> bool {
    self.check_limits();
    !self.hit_limits
  }

  pub fn visit_type_id_function_type(&mut self) -> bool {
    self.check_limits();
    false
  }

  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _tt: &TableType) -> bool {
    self.check_limits();
    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }

  pub fn visit_type_id_generic_type(&mut self) -> bool {
    // Mirrors the C++:
    // bool visit(TypeId ty, const GenericType&) override { ... }
    //
    // This Rust one-shot item only specifies the generic-type visit hook;
    // the TypeId is provided by the visitor dispatch into GenericCounter.
    self.visit_type_id()
  }

  pub fn visit_type_pack_id_generic_type_pack(&mut self) -> bool {
    false
  }
}
