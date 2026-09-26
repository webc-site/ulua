use ulua_analysis::{
  functions::{
    add_global_binding_builtin_definitions::add_global_binding_value, freeze::freeze,
    persist_type::persist, unfreeze::unfreeze,
  },
  records::{builtin_types::BuiltinTypes, global_types::GlobalTypes, property_type::Property},
};

use crate::{
  functions::extern_type_fixture::{
    binding, export_type_binding, extern_prop, extern_type, free_function, method,
  },
  records::ac_fixture_impl::AcFixtureImpl,
};

pub fn register_ac_extern_type_fixture_types(fixture: &mut AcFixtureImpl) {
  // (a) 类 `as *mut Frontend` + 缓存句柄 `&*fixture.base.builtin_types` 绕道
  // 已消除：经 `Frontend::builtin_types_ref` chokepoint 取只读借用（不绑定
  // frontend），随后两笔 `&mut globals*` 顺序登记（cpp registerGlobals 同款），
  // 全程安全借用。
  let frontend = fixture.get_frontend();
  let builtins = frontend.builtin_types_ref();
  register_globals(&mut frontend.globals, builtins);
  register_globals(&mut frontend.globals_for_autocomplete, builtins);
}

fn register_globals(globals: &mut GlobalTypes, builtins: &BuiltinTypes) {
  unfreeze(globals.global_types_mut());

  let (base_class_instance_type, base_class_type, child_class_instance_type, child_class_type) = {
    let arena = globals.global_types_mut();
    let number_type = builtins.number_type;
    let string_type = builtins.string_type;

    let base_class_instance_type = arena.add_type(extern_type("BaseClass", None));
    let base_method = method(arena, base_class_instance_type, &[number_type], &[]);
    extern_prop(
      base_class_instance_type,
      "BaseMethod",
      Property::readonly(base_method),
    );
    extern_prop(
      base_class_instance_type,
      "BaseField",
      Property::rw_type_id(number_type),
    );

    let base_class_type = arena.add_type(extern_type("BaseClass", None));
    let base_new = free_function(arena, &[], &[base_class_instance_type]);
    extern_prop(base_class_type, "New", Property::rw_type_id(base_new));

    let child_class_instance_type =
      arena.add_type(extern_type("ChildClass", Some(base_class_instance_type)));
    let child_method = method(arena, child_class_instance_type, &[], &[string_type]);
    extern_prop(
      child_class_instance_type,
      "Method",
      Property::rw_type_id(child_method),
    );

    let child_class_type = arena.add_type(extern_type("ChildClass", Some(base_class_type)));
    let child_new = free_function(arena, &[], &[child_class_instance_type]);
    extern_prop(child_class_type, "New", Property::rw_type_id(child_new));

    (
      base_class_instance_type,
      base_class_type,
      child_class_instance_type,
      child_class_type,
    )
  };

  export_type_binding(globals, "BaseClass", base_class_instance_type);
  export_type_binding(globals, "ChildClass", child_class_instance_type);

  add_global_binding_value(globals, "BaseClass", binding(base_class_type));
  add_global_binding_value(globals, "ChildClass", binding(child_class_type));

  persist(base_class_instance_type);
  persist(base_class_type);
  persist(child_class_instance_type);
  persist(child_class_type);

  freeze(globals.global_types_mut());
}
