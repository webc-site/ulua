use crate::{
  functions::follow_type::follow_type_id, records::type_cacher::TypeCacher,
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn is_uncacheable_type_id(&self, ty: TypeId) -> bool {
    self.uncacheable.contains(&follow_type_id(ty))
  }
}
