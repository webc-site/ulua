use alloc::sync::Arc;

use ulua_analysis::{
  functions::{freeze::freeze, persist_type::persist, unfreeze::unfreeze},
  records::{
    extern_type::ExternType, property_type::Property, scope::Scope, table_type::TableType,
    type_fun::TypeFun,
  },
};

use crate::records::to_dot_class_fixture::ToDotClassFixture;

impl ToDotClassFixture {
  pub fn to_dot_class_fixture(&mut self) {
    let frontend = self.base.get_frontend();
    let number_type = unsafe { (*frontend.builtin_types).number_type };
    let string_type = unsafe { (*frontend.builtin_types).string_type };

    let globals = &mut frontend.globals;
    unfreeze(globals.global_types_mut());

    let base_class_meta_type = globals.global_types_mut().add_type(TableType::new());

    let mut base_class_instance = ExternType {
      name: String::from("BaseClass"),
      props: Default::default(),
      parent: None,
      metatable: Some(base_class_meta_type),
      tags: Default::default(),
      user_data: None,
      definition_module_name: String::from("Test"),
      definition_location: None,
      indexer: None,
      relation: None,
    };
    base_class_instance
      .props
      .insert(String::from("BaseField"), Property::rw_type_id(number_type));
    let base_class_instance_type = globals.global_types_mut().add_type(base_class_instance);

    let mut child_class_instance = ExternType {
      name: String::from("ChildClass"),
      props: Default::default(),
      parent: Some(base_class_instance_type),
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: String::from("Test"),
      definition_location: None,
      indexer: None,
      relation: None,
    };
    child_class_instance.props.insert(
      String::from("ChildField"),
      Property::rw_type_id(string_type),
    );
    let child_class_instance_type = globals.global_types_mut().add_type(child_class_instance);

    let global_scope = globals.global_scope();
    let global_scope_raw = Arc::as_ptr(&global_scope) as *mut Scope;
    unsafe {
      (*global_scope_raw).exported_type_bindings.insert(
        String::from("BaseClass"),
        TypeFun::type_fun_type_id(base_class_instance_type),
      );
      (*global_scope_raw).exported_type_bindings.insert(
        String::from("ChildClass"),
        TypeFun::type_fun_type_id(child_class_instance_type),
      );

      let exported_types: Vec<_> = (*global_scope_raw)
        .exported_type_bindings
        .values()
        .map(TypeFun::r#type)
        .collect();
      for ty in exported_types {
        persist(ty);
      }
    }

    freeze(globals.global_types_mut());
  }
}
