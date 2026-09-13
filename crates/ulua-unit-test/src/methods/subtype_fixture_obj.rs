use alloc::string::String;

use ulua_analysis::{records::extern_type::ExternType, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn obj(&mut self, name: &str, parent: Option<TypeId>) -> TypeId {
    self.arena.add_type(ExternType {
      name: String::from(name),
      props: Default::default(),
      parent: Some(parent.unwrap_or(self.builtin_types.object_type)),
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: String::new(),
      definition_location: None,
      indexer: None,
      relation: None,
    })
  }
}
