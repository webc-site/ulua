use crate::{
  functions::follow_type::follow_type_id, records::type_cacher::TypeCacher,
  type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn is_cached(&self, ty: TypeId) -> bool {
    unsafe { (*self.cached_types).contains(&follow_type_id(ty)) }
  }
}
