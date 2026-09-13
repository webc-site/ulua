use crate::{
  enums::tarjan_result::TarjanResult, records::tarjan::Tarjan, type_aliases::type_id::TypeId,
};

impl Tarjan {
  pub fn find_dirty_type_id(&mut self, ty: TypeId) -> TarjanResult {
    self.visit_root_type_id(ty)
  }
}
