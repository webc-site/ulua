//! Port of `createSomeExternTypes` from `tests/Fixture.cpp`.
use alloc::string::String;

use ulua_analysis::{
  functions::{
    add_global_binding_builtin_definitions::add_global_binding_value, freeze::freeze,
    persist_type::persist, unfreeze::unfreeze,
  },
  records::{
    binding::Binding, frontend::Frontend, global_types::GlobalTypes, property_type::Property,
  },
  type_aliases::type_id::TypeId,
};
use ulua_ast::records::location::Location;

use crate::functions::extern_type_fixture::{
  export_type_binding, extern_prop, extern_type, method,
};

/// cpp Fixture 版全局绑定：`documentation_symbol` 为空（与夹具注册器的 `@test`
/// 版不同，保持上游差异）。
fn add_global_binding(globals: &mut GlobalTypes, name: &str, ty: TypeId) {
  add_global_binding_value(
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
  // 经 `Frontend::builtin_types_ref` chokepoint 取内建单例共享引用，随即拷出标量
  // TypeId，借用止于表达式，与随后 `&mut frontend.globals` 不重叠。
  let root_extern = frontend.builtin_types_ref().extern_type;
  let globals = &mut frontend.globals;

  unfreeze(globals.global_types_mut());

  let (parent_type, child_type, another_child_type, unrelated_type) = {
    let arena = globals.global_types_mut();

    let parent_type = arena.add_type(extern_type("Parent", Some(root_extern)));
    let method_type = method(arena, parent_type, &[], &[]);
    let virtual_method_type = method(arena, parent_type, &[], &[]);

    extern_prop(parent_type, "method", Property::rw_type_id(method_type));
    extern_prop(
      parent_type,
      "virtual_method",
      Property::rw_type_id(virtual_method_type),
    );

    let child_type = arena.add_type(extern_type("Child", Some(parent_type)));
    let another_child_type = arena.add_type(extern_type("AnotherChild", Some(parent_type)));
    let unrelated_type = arena.add_type(extern_type("Unrelated", Some(root_extern)));

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
