use alloc::string::String;

use ulua_analysis::{
  functions::get_mutable_type::get_mutable,
  records::extern_type::ExternType,
  type_aliases::{module_name_type::ModuleName, props_type::Props, type_id::TypeId},
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn cls_string_optional_type_id(&mut self, name: &str, parent: Option<TypeId>) -> TypeId {
    self.arena.add_type(ExternType {
      name: String::from(name),
      props: Default::default(),
      parent: Some(parent.unwrap_or(self.builtin_types.extern_type)),
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: ModuleName::new(),
      definition_location: None,
      indexer: None,
      relation: None,
    })
  }
}

impl SubtypeFixture {
  pub fn cls_string_extern_type_props(&mut self, name: &str, props: Props) -> TypeId {
    let ty = self.cls_string_optional_type_id(name, None);
    if let Some(extern_ty) = get_mutable::<ExternType>(ty) {
      extern_ty.props = props;
    }
    ty
  }
}
