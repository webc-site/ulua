//! Port of `createSomeExternTypes` from `tests/Fixture.cpp`.
use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_analysis::{
  functions::{
    add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
    freeze::freeze, get_mutable_type::get_mutable_type_id,
    make_function_builtin_definitions::make_function, persist_type::persist, unfreeze::unfreeze,
  },
  records::{
    binding::Binding, extern_type::ExternType, frontend::Frontend, global_types::GlobalTypes,
    property_type::Property, scope::Scope, type_fun::TypeFun,
  },
  type_aliases::type_id::TypeId,
};
use ulua_ast::records::location::Location;

fn make_extern_type(name: &str, parent: TypeId) -> ExternType {
  ExternType {
    name: String::from(name),
    props: Default::default(),
    parent: Some(parent),
    metatable: None,
    tags: Default::default(),
    user_data: None,
    definition_module_name: String::from("Test"),
    definition_location: None,
    indexer: None,
    relation: None,
  }
}

fn export_type_binding(globals: &mut GlobalTypes, name: &str, ty: TypeId) {
  let module_scope = globals.global_scope();
  let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

  unsafe {
    (*module_scope_ptr).exported_type_bindings.insert(
      String::from(name),
      TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
        Vec::new(),
        ty,
        None,
      ),
    );
  }
}

fn add_global_binding(globals: &mut GlobalTypes, name: &str, ty: TypeId) {
  add_global_binding_builtin_definitions_alt_b(
    globals,
    name,
    Binding {
      type_id: ty,
      location: Location::default(),
      deprecated: false,
      deprecated_suggestion: String::new(),
      documentation_symbol: None,
    },
  );
}

pub fn create_some_extern_types(frontend: &mut Frontend) {
  let extern_type = unsafe { (*frontend.builtin_types).extern_type };
  let globals = &mut frontend.globals;

  unfreeze(globals.global_types_mut());

  let (parent_type, child_type, another_child_type, unrelated_type) = {
    let arena = globals.global_types_mut();

    let parent_type = arena.add_type(make_extern_type("Parent", extern_type));
    let method_type = make_function(arena, Some(parent_type), Vec::new(), Vec::new(), false);
    let virtual_method_type =
      make_function(arena, Some(parent_type), Vec::new(), Vec::new(), false);

    let parent_extern_type =
      get_mutable_type_id::<ExternType>(parent_type).expect("expected Parent extern type");
    parent_extern_type
      .props
      .insert(String::from("method"), Property::rw_type_id(method_type));
    parent_extern_type.props.insert(
      String::from("virtual_method"),
      Property::rw_type_id(virtual_method_type),
    );

    let child_type = arena.add_type(make_extern_type("Child", parent_type));
    let another_child_type = arena.add_type(make_extern_type("AnotherChild", parent_type));
    let unrelated_type = arena.add_type(make_extern_type("Unrelated", extern_type));

    (parent_type, child_type, another_child_type, unrelated_type)
  };

  add_global_binding(globals, "Parent", parent_type);
  export_type_binding(globals, "Parent", parent_type);

  add_global_binding(globals, "Child", child_type);
  export_type_binding(globals, "Child", child_type);

  add_global_binding(globals, "AnotherChild", another_child_type);
  export_type_binding(globals, "AnotherChild", another_child_type);

  add_global_binding(globals, "Unrelated", unrelated_type);
  export_type_binding(globals, "Unrelated", unrelated_type);

  persist(parent_type);
  persist(child_type);
  persist(another_child_type);
  persist(unrelated_type);

  freeze(globals.global_types_mut());
}
