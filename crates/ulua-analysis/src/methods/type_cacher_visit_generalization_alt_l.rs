use crate::{
  records::{extern_type::ExternType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_extern_type(&mut self, ty: TypeId, _et: &ExternType) -> bool {
    self.cache(ty);
    false
  }
}
