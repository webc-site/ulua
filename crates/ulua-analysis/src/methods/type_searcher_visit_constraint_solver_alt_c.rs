use crate::{
  records::{extern_type::ExternType, type_searcher::TypeSearcher},
  type_aliases::type_id::TypeId,
};

impl TypeSearcher {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }
}
