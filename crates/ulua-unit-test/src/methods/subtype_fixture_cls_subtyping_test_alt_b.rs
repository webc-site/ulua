use ulua_analysis::{
  functions::get_mutable_type::get_mutable,
  records::extern_type::ExternType,
  type_aliases::{props_type::Props, type_id::TypeId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn cls_string_extern_type_props(&mut self, name: &str, props: Props) -> TypeId {
    let ty = self.cls_string_optional_type_id(name, None);
    if let Some(extern_ty) = get_mutable::<ExternType>(ty) {
      extern_ty.props = props;
    }
    ty
  }
}
