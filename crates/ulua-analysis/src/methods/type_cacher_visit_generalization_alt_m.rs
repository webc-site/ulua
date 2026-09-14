use crate::{
  records::{any_type::AnyType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_any_type(&mut self, ty: TypeId, _at: &AnyType) -> bool {
    self.cache(ty);
    false
  }
}
