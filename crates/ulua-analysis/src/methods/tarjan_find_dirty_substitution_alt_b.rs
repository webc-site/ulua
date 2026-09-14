use crate::{
  enums::tarjan_result::TarjanResult, records::tarjan::Tarjan,
  type_aliases::type_pack_id::TypePackId,
};

impl Tarjan {
  pub fn find_dirty_type_pack_id(&mut self, tp: TypePackId) -> TarjanResult {
    self.visit_root_type_pack_id(tp)
  }
}
