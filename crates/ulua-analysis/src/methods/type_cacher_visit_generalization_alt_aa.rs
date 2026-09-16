use crate::{
  records::{
    type_cacher::TypeCacher, type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCacher {
  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.mark_uncacheable_type_pack_id(tp);
    false
  }
}
