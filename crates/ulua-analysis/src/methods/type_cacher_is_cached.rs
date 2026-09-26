use crate::{
  functions::follow_type, records::type_cacher::TypeCacher, type_aliases::type_id::TypeId,
};

impl TypeCacher {
  pub fn is_cached(&self, ty: TypeId) -> bool {
    unsafe { (*self.cached_types).contains(&follow_type::follow(ty)) }
  }
}
