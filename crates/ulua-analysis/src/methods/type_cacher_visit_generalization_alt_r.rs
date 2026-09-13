use crate::{
  records::{never_type::NeverType, type_cacher::TypeCacher},
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn visit_type_id_never_type(&mut self, ty: TypeId, _nt: &NeverType) -> bool {
    self.cache(ty);
    false
  }
}
