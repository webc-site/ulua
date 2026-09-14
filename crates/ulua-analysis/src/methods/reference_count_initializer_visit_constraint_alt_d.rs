use crate::{
  enums::table_state::TableState,
  records::{reference_count_initializer::ReferenceCountInitializer, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl ReferenceCountInitializer {
  pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    if tt.state == TableState::Unsealed || tt.state == TableState::Free {
      unsafe {
        (*self.mutated_types).order.push(ty);
      }
    }

    true
  }
}
