use crate::{
  records::{contains_refinable_type::ContainsRefinableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl ContainsRefinableType {
  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _table: &TableType) -> bool {
    !self.found
  }
}
