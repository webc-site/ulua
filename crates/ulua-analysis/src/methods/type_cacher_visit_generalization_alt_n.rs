use crate::{
  records::{no_refine_type::NoRefineType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_no_refine_type(&mut self, ty: TypeId, _nrt: &NoRefineType) -> bool {
    self.cache(ty);
    false
  }
}
