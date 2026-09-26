use alloc::string::String;

use ulua_analysis::{
  enums::polarity::Polarity,
  functions::{
    add_global_binding_builtin_definitions::add_global_binding_value, freeze::freeze,
    persist_type::persist, unfreeze::unfreeze,
  },
  records::{
    builtin_types::BuiltinTypes, frontend::Frontend, function_type::FunctionType,
    generic_type::GenericType, global_types::GlobalTypes, intersection_type::IntersectionType,
    property_type::Property, table_indexer::TableIndexer, table_type::TableType,
    type_pack::TypePack, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

use crate::functions::extern_type_fixture::{
  binding, export_type_binding, extern_prop, extern_type, extern_type_full, free_function, method,
  table_prop,
};

pub fn register_extern_type_fixture_types(frontend: &mut Frontend) -> (TypeId, TypeId) {
  // 经 `Frontend::builtin_types_ref` chokepoint 取内建单例共享引用，借用不绑定
  // `frontend`，与随后 `&mut frontend.globals` 不重叠。
  let builtins = frontend.builtin_types_ref();
  register_globals(&mut frontend.globals, builtins)
}

fn register_globals(globals: &mut GlobalTypes, builtins: &BuiltinTypes) -> (TypeId, TypeId) {
  unfreeze(globals.global_types_mut());

  let (
    base_class_instance_type,
    base_class_type,
    child_class_instance_type,
    child_class_type,
    grand_child_instance_type,
    another_child_instance_type,
    unrelated_class_instance_type,
    unrelated_class_type,
    vector2_type,
    vector2_instance_type,
    callable_class_type,
    indexable_class_type,
    indexable_numeric_key_class_type,
    duplicate_base_class_instance_type,
    class_with_generic_method_type,
  ) = {
    let arena = globals.global_types_mut();
    let number_type = builtins.number_type;
    let string_type = builtins.string_type;

    // Connection { Connect: (self, (BaseClass) -> ()) -> Connection }
    let connection_type = arena.add_type(extern_type("Connection", None));

    // BaseClass 实例侧 { BaseMethod, BaseField, Touched: Connection }
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
    extern_prop(
      base_class_instance_type,
      "Touched",
      Property::readonly(connection_type),
    );

    let connect_callback = free_function(arena, &[base_class_instance_type], &[]);
    let connect = method(arena, connection_type, &[connect_callback], &[]);
    extern_prop(connection_type, "Connect", Property::rw_type_id(connect));

    let base_class_type = arena.add_type(extern_type("BaseClass", None));
    let static_method = free_function(arena, &[], &[number_type]);
    let clone_method = free_function(
      arena,
      &[base_class_instance_type],
      &[base_class_instance_type],
    );
    let base_new = free_function(arena, &[], &[base_class_instance_type]);
    extern_prop(
      base_class_type,
      "StaticMethod",
      Property::rw_type_id(static_method),
    );
    extern_prop(base_class_type, "Clone", Property::rw_type_id(clone_method));
    extern_prop(base_class_type, "New", Property::rw_type_id(base_new));

    // ChildClass : BaseClass { Method: (self) -> string }, 静态侧 { New }
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

    // GrandChild : ChildClass { Method: (self) -> string }
    let grand_child_instance_type =
      arena.add_type(extern_type("GrandChild", Some(child_class_instance_type)));
    let grand_child_method = method(arena, grand_child_instance_type, &[], &[string_type]);
    extern_prop(
      grand_child_instance_type,
      "Method",
      Property::rw_type_id(grand_child_method),
    );

    // AnotherChild : BaseClass { Method: (self) -> string }
    let another_child_instance_type =
      arena.add_type(extern_type("AnotherChild", Some(base_class_instance_type)));
    let another_child_method = method(arena, another_child_instance_type, &[], &[string_type]);
    extern_prop(
      another_child_instance_type,
      "Method",
      Property::rw_type_id(another_child_method),
    );

    // UnrelatedClass { New: () -> UnrelatedClass }
    let unrelated_class_instance_type = arena.add_type(extern_type("UnrelatedClass", None));
    let unrelated_class_type = arena.add_type(extern_type("UnrelatedClass", None));
    let unrelated_new = free_function(arena, &[], &[unrelated_class_instance_type]);
    extern_prop(
      unrelated_class_type,
      "New",
      Property::rw_type_id(unrelated_new),
    );

    // Vector2 { X: number, Y: number }，元表含 __add / __mul（交集重载）；静态侧 { New }
    let vector2_meta_type = arena.add_type(TableType::new());
    let vector2_instance_type = arena.add_type(extern_type_full(
      "Vector2",
      None,
      Some(vector2_meta_type),
      None,
    ));
    extern_prop(
      vector2_instance_type,
      "X",
      Property::rw_type_id(number_type),
    );
    extern_prop(
      vector2_instance_type,
      "Y",
      Property::rw_type_id(number_type),
    );

    let vector2_type = arena.add_type(extern_type("Vector2", None));
    let vector2_new = free_function(arena, &[number_type, number_type], &[vector2_instance_type]);
    extern_prop(vector2_type, "New", Property::rw_type_id(vector2_new));

    let vector2_add = free_function(
      arena,
      &[vector2_instance_type, vector2_instance_type],
      &[vector2_instance_type],
    );
    let vector2_mul_vector = method(
      arena,
      vector2_instance_type,
      &[vector2_instance_type],
      &[vector2_instance_type],
    );
    let vector2_mul_number = method(
      arena,
      vector2_instance_type,
      &[number_type],
      &[vector2_instance_type],
    );
    let vector2_mul = arena.add_type(IntersectionType {
      parts: alloc::vec![vector2_mul_vector, vector2_mul_number],
    });
    table_prop(
      vector2_meta_type,
      "__add",
      Property::rw_type_id(vector2_add),
    );
    table_prop(
      vector2_meta_type,
      "__mul",
      Property::rw_type_id(vector2_mul),
    );

    // CallableClass：元表含 __call: (CallableClass, string) -> number
    let callable_class_meta_type = arena.add_type(TableType::new());
    let callable_class_type = arena.add_type(extern_type_full(
      "CallableClass",
      None,
      Some(callable_class_meta_type),
      None,
    ));
    let callable_call = free_function(arena, &[callable_class_type, string_type], &[number_type]);
    table_prop(
      callable_class_meta_type,
      "__call",
      Property::rw_type_id(callable_call),
    );

    // IndexableClass：元表 + { [string | number]: number } 索引器
    let indexable_class_meta_type = arena.add_type(TableType::new());
    let indexable_key_type = arena.add_type(UnionType {
      options: alloc::vec![string_type, number_type],
    });
    let indexable_class_type = arena.add_type(extern_type_full(
      "IndexableClass",
      None,
      Some(indexable_class_meta_type),
      Some(TableIndexer {
        index_type: indexable_key_type,
        index_result_type: number_type,
        is_read_only: false,
      }),
    ));

    // IndexableNumericKeyClass：元表 + { [number]: number } 索引器
    let indexable_numeric_key_class_meta_type = arena.add_type(TableType::new());
    let indexable_numeric_key_class_type = arena.add_type(extern_type_full(
      "IndexableNumericKeyClass",
      None,
      Some(indexable_numeric_key_class_meta_type),
      Some(TableIndexer {
        index_type: number_type,
        index_result_type: number_type,
        is_read_only: false,
      }),
    ));

    // 同名 "BaseClass" 派生实例（歧义消费场景）
    let duplicate_base_class_instance_type =
      arena.add_type(extern_type("BaseClass", Some(base_class_instance_type)));
    let duplicate_method = method(
      arena,
      duplicate_base_class_instance_type,
      &[],
      &[string_type],
    );
    extern_prop(
      duplicate_base_class_instance_type,
      "Method",
      Property::rw_type_id(duplicate_method),
    );

    // ClassWithGenericMethod { identity: <T>(T) -> T }
    let generic_t = arena.add_type(GenericType::generic_type_name_polarity(
      &String::from("T"),
      Polarity::Mixed,
    ));
    let identity_args = arena.add_type_pack_t(TypePack::new(alloc::vec![generic_t], None));
    let identity_rets = arena.add_type_pack_t(TypePack::new(alloc::vec![generic_t], None));
    let identity = arena.add_type(FunctionType::new_with_generics(
      alloc::vec![generic_t],
      alloc::vec![],
      identity_args,
      identity_rets,
      None,
      false,
    ));
    let class_with_generic_method_type =
      arena.add_type(extern_type("ClassWithGenericMethod", None));
    extern_prop(
      class_with_generic_method_type,
      "identity",
      Property::readonly(identity),
    );

    (
      base_class_instance_type,
      base_class_type,
      child_class_instance_type,
      child_class_type,
      grand_child_instance_type,
      another_child_instance_type,
      unrelated_class_instance_type,
      unrelated_class_type,
      vector2_type,
      vector2_instance_type,
      callable_class_type,
      indexable_class_type,
      indexable_numeric_key_class_type,
      duplicate_base_class_instance_type,
      class_with_generic_method_type,
    )
  };

  // 全局 scope 导出类型绑定（cpp exportedTypeBindings 同款）
  let exported_bindings = [
    ("BaseClass", base_class_instance_type),
    ("ChildClass", child_class_instance_type),
    ("GrandChild", grand_child_instance_type),
    ("AnotherChild", another_child_instance_type),
    ("UnrelatedClass", unrelated_class_instance_type),
    ("Vector2", vector2_instance_type),
    ("CallableClass", callable_class_type),
    ("IndexableClass", indexable_class_type),
    ("IndexableNumericKeyClass", indexable_numeric_key_class_type),
    ("ClassWithGenericMethod", class_with_generic_method_type),
  ];
  for (name, ty) in exported_bindings {
    export_type_binding(globals, name, ty);
  }

  // 全局值绑定（cpp bindings 同款；GrandChild/AnotherChild 指向 ChildClass 静态侧）
  for (name, ty) in [
    ("BaseClass", base_class_type),
    ("ChildClass", child_class_type),
    ("GrandChild", child_class_type),
    ("AnotherChild", child_class_type),
    ("UnrelatedClass", unrelated_class_type),
    ("Vector2", vector2_type),
    (
      "confusingBaseClassInstance",
      duplicate_base_class_instance_type,
    ),
    ("ClassWithGenericMethod", class_with_generic_method_type),
  ] {
    add_global_binding_value(globals, name, binding(ty));
  }

  for ty in [
    base_class_instance_type,
    child_class_instance_type,
    grand_child_instance_type,
    another_child_instance_type,
    unrelated_class_instance_type,
    vector2_instance_type,
    callable_class_type,
    indexable_class_type,
    indexable_numeric_key_class_type,
    duplicate_base_class_instance_type,
    class_with_generic_method_type,
  ] {
    persist(ty);
  }

  freeze(globals.global_types_mut());
  (vector2_type, vector2_instance_type)
}
