use crate::{
  records::{singleton_type::SingletonType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_singleton_type(&mut self, ty: TypeId, _st: &SingletonType) -> bool {
    self.cache(ty);
    false
  }
}
