use crate::{
  functions::follow_type::follow_type_id, records::type_cacher::TypeCacher,
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn cache(&self, ty: TypeId) {
    unsafe { (*self.cached_types).insert(follow_type_id(ty)) };
  }
}
