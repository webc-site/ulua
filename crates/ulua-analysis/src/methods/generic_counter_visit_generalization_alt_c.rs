use crate::{
  records::{generic_counter::GenericCounter, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl GenericCounter {
  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _tt: &TableType) -> bool {
    self.check_limits();
    false
  }
}
