use crate::{
  records::{extern_type::ExternType, unscoped_generic_finder::UnscopedGenericFinder},
  type_aliases::type_id::TypeId,
};

impl UnscopedGenericFinder {
  pub fn visit_type_id_extern_type(&mut self, ty: TypeId, extern_type: &ExternType) -> bool {
    let _ty = ty;
    let _extern_type = extern_type;

    false
  }
}
