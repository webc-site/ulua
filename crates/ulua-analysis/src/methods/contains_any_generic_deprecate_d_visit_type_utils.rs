use crate::{
  records::{
    contains_any_generic_deprecated::ContainsAnyGenericDeprecated, extern_type::ExternType,
  },
  type_aliases::type_id::TypeId,
};

impl ContainsAnyGenericDeprecated {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _ext: &ExternType) -> bool {
    false
  }
}
