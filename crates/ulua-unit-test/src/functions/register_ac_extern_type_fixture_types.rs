use alloc::{string::String, sync::Arc};

use ulua_analysis::{
  functions::{
    add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
    freeze::freeze, get_mutable_type::get_mutable_type_id,
    make_function_builtin_definitions::make_function, persist_type::persist, unfreeze::unfreeze,
  },
  records::{
    binding::Binding, builtin_types::BuiltinTypes, extern_type::ExternType, frontend::Frontend,
    global_types::GlobalTypes, property_type::Property, scope::Scope, type_fun::TypeFun,
  },
  type_aliases::type_id::TypeId,
};
use ulua_ast::records::location::Location;

use crate::records::ac_fixture_impl::AcFixtureImpl;

pub fn register_ac_extern_type_fixture_types(fixture: &mut AcFixtureImpl) {
  let frontend = fixture.get_frontend() as *mut Frontend;
  let builtins = unsafe { &*fixture.base.builtin_types };

  unsafe {
    register_globals(&mut (*frontend).globals, builtins);
    register_globals(&mut (*frontend).globals_for_autocomplete, builtins);
  }
}

fn extern_type(name: &str, parent: Option<TypeId>) -> ExternType {
  ExternType {
    name: String::from(name),
    props: Default::default(),
    parent,
    metatable: None,
    tags: Default::default(),
    user_data: None,
    definition_module_name: String::from("Test"),
    definition_location: None,
    indexer: None,
    relation: None,
  }
}

fn binding(ty: TypeId) -> Binding {
  Binding {
    type_id: ty,
    location: Location::default(),
    deprecated: false,
    deprecated_suggestion: String::new(),
    documentation_symbol: Some(String::from("@test")),
  }
}

fn export_type_binding(globals: &mut GlobalTypes, name: &str, ty: TypeId) {
  let module_scope = globals.global_scope();
  let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

  unsafe {
    (*module_scope_ptr)
      .exported_type_bindings
      .insert(String::from(name), TypeFun::type_fun_type_id(ty));
  }
}

fn register_globals(globals: &mut GlobalTypes, builtins: &BuiltinTypes) {
  unfreeze(globals.global_types_mut());

  let (base_class_instance_type, base_class_type, child_class_instance_type, child_class_type) = {
    let arena = globals.global_types_mut();
    let number_type = builtins.number_type;
    let string_type = builtins.string_type;

    let base_class_instance_type = arena.add_type(extern_type("BaseClass", None));
    let base_method = make_function(
      arena,
      Some(base_class_instance_type),
      alloc::vec![number_type],
      alloc::vec![],
      false,
    );
    let base_class_instance = get_mutable_type_id::<ExternType>(base_class_instance_type)
      .expect("expected BaseClass instance extern type");
    base_class_instance
      .props
      .insert(String::from("BaseMethod"), Property::readonly(base_method));
    base_class_instance
      .props
      .insert(String::from("BaseField"), Property::rw_type_id(number_type));

    let base_class_type = arena.add_type(extern_type("BaseClass", None));
    let base_new = make_function(
      arena,
      None,
      alloc::vec![],
      alloc::vec![base_class_instance_type],
      false,
    );
    let base_class =
      get_mutable_type_id::<ExternType>(base_class_type).expect("expected BaseClass extern type");
    base_class
      .props
      .insert(String::from("New"), Property::rw_type_id(base_new));

    let child_class_instance_type =
      arena.add_type(extern_type("ChildClass", Some(base_class_instance_type)));
    let child_method = make_function(
      arena,
      Some(child_class_instance_type),
      alloc::vec![],
      alloc::vec![string_type],
      false,
    );
    let child_class_instance = get_mutable_type_id::<ExternType>(child_class_instance_type)
      .expect("expected ChildClass instance extern type");
    child_class_instance
      .props
      .insert(String::from("Method"), Property::rw_type_id(child_method));

    let child_class_type = arena.add_type(extern_type("ChildClass", Some(base_class_type)));
    let child_new = make_function(
      arena,
      None,
      alloc::vec![],
      alloc::vec![child_class_instance_type],
      false,
    );
    let child_class =
      get_mutable_type_id::<ExternType>(child_class_type).expect("expected ChildClass extern type");
    child_class
      .props
      .insert(String::from("New"), Property::rw_type_id(child_new));

    (
      base_class_instance_type,
      base_class_type,
      child_class_instance_type,
      child_class_type,
    )
  };

  export_type_binding(globals, "BaseClass", base_class_instance_type);
  export_type_binding(globals, "ChildClass", child_class_instance_type);

  add_global_binding_builtin_definitions_alt_b(globals, "BaseClass", binding(base_class_type));
  add_global_binding_builtin_definitions_alt_b(globals, "ChildClass", binding(child_class_type));

  persist(base_class_instance_type);
  persist(base_class_type);
  persist(child_class_instance_type);
  persist(child_class_type);

  freeze(globals.global_types_mut());
}
