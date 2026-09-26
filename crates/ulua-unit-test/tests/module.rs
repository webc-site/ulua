use ulua_analysis::type_aliases::module_name_type::ModuleName;

extern crate alloc;

// Source: `tests/Module.test.cpp`
#[test]
fn module_any_persistance_does_not_leak() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::frontend_options::FrontendOptions,
    type_aliases::module_name_type::ModuleName,
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let module_name = ModuleName::from("Module/A");

  fixture.file_resolver.source.insert(
    module_name.clone(),
    String::from(
      r#"
export type A = B
type B = A
    "#,
    ),
  );

  let opts = FrontendOptions {
    retain_full_type_graphs: false,
    ..Default::default()
  };
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_name, Some(opts));
  assert!(!result.errors.is_empty(), "{:?}", result.errors);

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_name);
  let binding = module
    .exported_type_bindings
    .get("A")
    .expect("expected exported type binding A");

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!("any", to_string_type_id(binding.r#type()));
  } else {
    assert_eq!("*error-type*", to_string_type_id(binding.r#type()));
  }
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_builtin_types_point_into_global_types_arena() {
  use ulua_analysis::{
    functions::{first::first, get_type},
    records::table_type::TableType,
  };
  use ulua_unit_test::{
    functions::is_in_arena::is_in_arena, records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();
  let module_name = ModuleName::from("MainModule");

  fixture.base.file_resolver.source.insert(
    module_name.clone(),
    String::from(
      r#"
        return {sign=math.sign}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_name, None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_name);
  let exports = first(module.return_type, true).expect("expected module return type");

  assert!(is_in_arena(exports, &module.interface_types));

  let exports_table = get_type::get::<TableType>(exports).expect("expected return table");

  let sign_type = exports_table
    .props
    .get("sign")
    .and_then(|prop| prop.read_ty)
    .expect("expected sign property read type");

  assert!(!is_in_arena(sign_type, &module.interface_types));
  assert!(is_in_arena(
    sign_type,
    fixture.get_frontend().globals.global_types_mut()
  ));
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_a_bound_type_to_a_persistent_type() {
  use ulua_analysis::{
    functions::{clone_clone::clone_type_id as clone_type, follow_type::follow},
    records::{clone_state::CloneState, type_arena::TypeArena},
    type_aliases::bound_type::BoundType,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  let mut arena = TypeArena::default();
  let bound_to = arena.add_type(BoundType {
    bound_to: fixture.base.get_builtins().number_type,
  });

  assert!(unsafe { (*fixture.base.get_builtins().number_type).persistent });

  let mut dest = TypeArena::default();
  let mut state = CloneState::new(fixture.base.get_builtins());
  let res = clone_type(bound_to, &mut dest, &mut state);

  assert_eq!(res, follow(bound_to));
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_a_bound_typepack_to_a_persistent_typepack() {
  use ulua_analysis::{
    functions::{clone_clone::clone as clone_pack, follow_type_pack::follow},
    records::{clone_state::CloneState, type_arena::TypeArena},
    type_aliases::bound_type_pack::BoundTypePack,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  let mut arena = TypeArena::default();
  let bound_to = arena.add_type_pack_t(BoundTypePack {
    bound_to: fixture.base.get_builtins().never_type_pack,
  });

  assert!(unsafe { (*fixture.base.get_builtins().never_type_pack).is_persistent() });

  let mut dest = TypeArena::default();
  let mut state = CloneState::new(fixture.base.get_builtins());
  let res = clone_pack(bound_to, &mut dest, &mut state);

  assert_eq!(res, follow(bound_to));
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_class() {
  use ulua_analysis::{
    functions::{clone_clone::clone_type_id as clone_type, get_type},
    records::{
      clone_state::CloneState, extern_type::ExternType, property_type::Property, r#type::Type,
      type_arena::TypeArena,
    },
    type_aliases::props_type::Props,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let any_type = fixture.get_builtins().any_type;
  let number_type = fixture.get_builtins().number_type;
  let string_type = fixture.get_builtins().string_type;

  let mut metaclass_props = Props::default();
  metaclass_props.insert("__add".into(), Property::readonly(any_type));
  let example_meta_class = Type::from(ExternType {
    name: "ExampleClassMeta".into(),
    props: metaclass_props,
    parent: None,
    metatable: None,
    tags: Default::default(),
    user_data: None,
    definition_module_name: "Test".into(),
    definition_location: None,
    indexer: None,
    relation: None,
  });
  let example_meta_class_id = &example_meta_class as *const Type;

  let mut class_props = Props::default();
  class_props.insert("PropOne".into(), Property::readonly(number_type));
  class_props.insert("PropTwo".into(), Property::readonly(string_type));
  let example_class = Type::from(ExternType {
    name: "ExampleClass".into(),
    props: class_props,
    parent: None,
    metatable: Some(example_meta_class_id),
    tags: Default::default(),
    user_data: None,
    definition_module_name: "Test".into(),
    definition_location: None,
    indexer: None,
    relation: None,
  });
  let example_class_id = &example_class as *const Type;

  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());

  let cloned = clone_type(example_class_id, &mut dest, &mut clone_state);
  let etv = get_type::get::<ExternType>(cloned).expect("expected extern type");

  let metatable_ty = etv.metatable.expect("expected cloned metatable");
  let metatable = get_type::get::<ExternType>(metatable_ty).expect("expected extern metatable");

  assert_eq!("ExampleClass", etv.name);
  assert_eq!("ExampleClassMeta", metatable.name);
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_cyclic_union() {
  use ulua_analysis::{
    functions::{clone_clone::clone_type_id as clone_type, get_mutable_type, get_type},
    records::{clone_state::CloneState, type_arena::TypeArena, union_type::UnionType},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let number_type = fixture.get_builtins().number_type();
  let string_type = fixture.get_builtins().string_type();

  let mut src = TypeArena::default();
  let u = src.add_type(UnionType {
    options: alloc::vec![number_type, string_type],
  });
  let uu = get_mutable_type::get_mutable::<UnionType>(u).expect("expected source union");

  uu.options.push(u);

  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());

  let cloned = clone_type(u, &mut dest, &mut clone_state);
  assert!(!cloned.is_null());

  let cloned_union = get_type::get::<UnionType>(cloned).expect("expected cloned union");
  assert_eq!(3, cloned_union.options.len());

  assert_eq!(number_type, cloned_union.options[0]);
  assert_eq!(string_type, cloned_union.options[1]);
  assert_eq!(cloned, cloned_union.options[2]);
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_free_tables() {
  use ulua_analysis::{
    enums::table_state::TableState,
    functions::{clone_clone::clone_type_id as clone_type, get_mutable_type, get_type},
    records::{
      clone_state::CloneState, table_type::TableType, r#type::Type, type_arena::TypeArena,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let mut table_ty = Type::from(TableType::new());
  let table_ty_id = &mut table_ty as *mut Type as *const Type;
  let ttv =
    get_mutable_type::get_mutable::<TableType>(table_ty_id).expect("expected source table type");
  ttv.state = TableState::Free;

  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());

  let cloned = clone_type(table_ty_id, &mut dest, &mut clone_state);
  let cloned_ttv = get_type::get::<TableType>(cloned).expect("expected cloned table type");

  assert_eq!(cloned_ttv.state, TableState::Free);
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_free_types() {
  use core::ptr::null_mut;

  use ulua_analysis::{
    enums::polarity::Polarity,
    functions::{
      clone_clone::{clone as clone_pack, clone_type_id as clone_type},
      fresh_type::fresh_type,
      get_type, get_type_pack,
    },
    records::{
      clone_state::CloneState, free_type::FreeType, free_type_pack::FreeTypePack,
      type_arena::TypeArena, type_level::TypeLevel, type_pack_var::TypePackVar,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let mut arena = TypeArena::default();
  let free_ty = fresh_type(
    &mut arena,
    fixture.get_builtins(),
    null_mut(),
    Polarity::Unknown,
  );
  let free_tp = TypePackVar::from(FreeTypePack::new(TypeLevel::default()));
  let free_tp_id = &free_tp as *const TypePackVar;

  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());

  let cloned_ty = clone_type(free_ty, &mut dest, &mut clone_state);
  assert!(get_type::get::<FreeType>(cloned_ty).is_some());

  clone_state = CloneState::new(fixture.get_builtins());
  let cloned_tp = clone_pack(free_tp_id, &mut dest, &mut clone_state);
  assert!(get_type_pack::get::<FreeTypePack>(cloned_tp).is_some());
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_iteration_limit() {
  use ulua_analysis::{
    functions::{clone_clone::clone_type_id as clone_type, get_mutable_type, get_type},
    records::{
      clone_state::CloneState, property_type::Property, table_type::TableType,
      type_arena::TypeArena,
    },
    type_aliases::error_type::ErrorType,
  };
  use ulua_common::fint;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  let _sfi = ScopedFastInt::new(&fint::LuauTypeCloneIterationLimit, 2000);
  let mut fixture = Fixture::fixture_bool(false);

  let mut src = TypeArena::default();

  let table = src.add_type(TableType::new());
  let mut nested = table;

  let nesting = 2500;
  for _ in 0..nesting {
    let child = src.add_type(TableType::new());
    let ttv =
      get_mutable_type::get_mutable::<TableType>(nested).expect("expected nested table type");
    ttv.props.insert("a".into(), Property::rw_type_id(child));
    nested = child;
  }

  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());

  let ty = clone_type(table, &mut dest, &mut clone_state);
  assert!(get_type::get::<ErrorType>(ty).is_some());

  // Cloning it again is an important test.
  let ty2 = clone_type(table, &mut dest, &mut clone_state);
  assert!(get_type::get::<ErrorType>(ty2).is_some());
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_self_property() {
  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_common::fflag;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  // CLI-117082 ModuleTests.clone_self_property we don't infer self correctly,
  // instead replacing it with unknown.
  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        --!nonstrict
        local a = {}
        function a:foo(x: number)
            return -x;
        end
        return a;
    "#,
    ),
  );

  let module_a = ModuleName::from("Module/A");
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_a, None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  fixture.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        --!nonstrict
        local a = require(script.Parent.A)
        return a.foo(5)
    "#,
    ),
  );

  let module_b = ModuleName::from("Module/B");
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b, None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "This function must be called with self. Did you mean to use a colon instead of a dot?",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_clone_table_bound_to_table_bound_to_table() {
  use core::ptr::null_mut;

  use ulua_analysis::{
    enums::table_state::TableState,
    functions::{clone_clone::clone_type_id as clone_type, get_mutable_type, get_type},
    records::{
      clone_state::CloneState, table_type::TableType, type_arena::TypeArena, type_level::TypeLevel,
    },
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  let mut arena = TypeArena::default();

  let a = arena.add_type(TableType::table_type_table_state_type_level_scope(
    TableState::Free,
    TypeLevel::default(),
    null_mut(),
  ));
  get_mutable_type::get_mutable::<TableType>(a)
    .expect("expected table a")
    .name = Some("a".into());

  let b = arena.add_type(TableType::table_type_table_state_type_level_scope(
    TableState::Free,
    TypeLevel::default(),
    null_mut(),
  ));
  get_mutable_type::get_mutable::<TableType>(b)
    .expect("expected table b")
    .name = Some("b".into());

  let c = arena.add_type(TableType::table_type_table_state_type_level_scope(
    TableState::Free,
    TypeLevel::default(),
    null_mut(),
  ));
  get_mutable_type::get_mutable::<TableType>(c)
    .expect("expected table c")
    .name = Some("c".into());

  get_mutable_type::get_mutable::<TableType>(a)
    .expect("expected table a")
    .bound_to = Some(b);
  get_mutable_type::get_mutable::<TableType>(b)
    .expect("expected table b")
    .bound_to = Some(c);

  let mut dest = TypeArena::default();
  let mut state = CloneState::new(fixture.base.get_builtins());
  let res = clone_type(a, &mut dest, &mut state);

  assert_eq!(1, dest.types.size());

  let table_a = get_type::get::<TableType>(res).expect("expected cloned table");
  assert_eq!(Some("c"), table_a.name.as_deref());
  assert!(table_a.bound_to.is_none());
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_deep_clone_cyclic_table() {
  use alloc::string::String;

  use ulua_analysis::{
    functions::{
      clone_clone::clone_type_id as clone_type, first::first, get_mutable_type, get_type,
      to_string_to_string::to_string_type_id,
    },
    records::{
      clone_state::CloneState, function_type::FunctionType, table_type::TableType,
      type_arena::TypeArena,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local Cyclic = {}
        function Cyclic.get()
            return Cyclic
        end
    "#,
    None,
  );

  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let ty = fixture.require_type_string("Cyclic");

  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());
  let clone_ty = clone_type(ty, &mut dest, &mut clone_state);

  let ttv =
    get_mutable_type::get_mutable::<TableType>(clone_ty).expect("expected cloned table type");

  assert_eq!(Some(String::from("Cyclic")), ttv.synthetic_name.clone());

  let method_type = ttv
    .props
    .get("get")
    .and_then(|prop| prop.read_ty)
    .expect("expected get method type");

  let ftv = get_type::get::<FunctionType>(method_type).expect("expected get method function type");

  let method_return_type = first(ftv.ret_types(), true).expect("expected method return type");

  assert!(
    method_return_type == clone_ty,
    "{} should be pointer identical to {}",
    to_string_type_id(method_type),
    to_string_type_id(clone_ty)
  );
  assert_eq!(2, dest.type_packs.size());
  assert_eq!(2, dest.types.size());
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_deep_clone_cyclic_table_2() {
  use alloc::string::String;

  use ulua_analysis::{
    functions::{
      clone_clone::clone_type_id as clone_type, first::first, get_mutable_type, get_type,
    },
    records::{
      clone_state::CloneState, function_type::FunctionType, table_type::TableType,
      type_arena::TypeArena,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let mut src = TypeArena::default();

  let table_ty = src.add_type(TableType::new());
  let arg_pack = src.add_type_pack_initializer_list_type_id(&[]);
  let ret_pack = src.add_type_pack_initializer_list_type_id(&[table_ty]);
  let method_ty = src.add_type(FunctionType::function_type_new(
    arg_pack, ret_pack, None, false,
  ));

  let tt =
    get_mutable_type::get_mutable::<TableType>(table_ty).expect("expected source table type");
  tt.props
    .entry(String::from("get"))
    .or_default()
    .set_type(method_ty);

  let mut dest = TypeArena::default();

  let mut clone_state = CloneState::new(fixture.get_builtins());
  let clone_ty = clone_type(table_ty, &mut dest, &mut clone_state);
  let ctt =
    get_mutable_type::get_mutable::<TableType>(clone_ty).expect("expected cloned table type");

  let cloned_method_type = ctt
    .props
    .get("get")
    .and_then(|prop| prop.read_ty)
    .expect("expected cloned get method type");

  let cmf = get_type::get::<FunctionType>(cloned_method_type)
    .expect("expected cloned get method function type");

  let clone_method_return_type =
    first(cmf.ret_types(), true).expect("expected cloned method return type");

  assert!(clone_method_return_type == clone_ty);
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_deep_clone_intersection() {
  use ulua_analysis::{
    functions::{
      clone_clone::clone_type_id as clone_type, freeze::freeze,
      to_string_to_string::to_string_type_id, unfreeze::unfreeze,
    },
    records::{
      clone_state::CloneState, intersection_type::IntersectionType, type_arena::TypeArena,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());
  let number_type = fixture.get_builtins().number_type();
  let string_type = fixture.get_builtins().string_type();

  let old_intersection = {
    let frontend = fixture.get_frontend();
    let global_types = frontend.globals.global_types_mut();
    unfreeze(global_types);
    let old_intersection = global_types.add_type(IntersectionType {
      parts: alloc::vec![number_type, string_type],
    });
    freeze(global_types);
    old_intersection
  };

  let new_intersection = clone_type(old_intersection, &mut dest, &mut clone_state);

  assert_ne!(new_intersection, old_intersection);
  assert_eq!("number & string", to_string_type_id(new_intersection));
  assert_eq!(1, dest.types.size());
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_deep_clone_non_persistent_primitive() {
  use ulua_analysis::{
    functions::{
      clone_clone::clone_type_id as clone_type, freeze::freeze,
      to_string_to_string::to_string_type_id, unfreeze::unfreeze,
    },
    records::{
      clone_state::CloneState,
      primitive_type::{PrimitiveType, Type},
      type_arena::TypeArena,
    },
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());

  // Create a new number type that isn't persistent.
  let old_number = {
    let frontend = fixture.get_frontend();
    let global_types = frontend.globals.global_types_mut();
    unfreeze(global_types);
    let old_number = global_types.add_type(PrimitiveType {
      r#type: Type::Number,
      metatable: None,
    });
    freeze(global_types);
    old_number
  };

  let new_number = clone_type(old_number, &mut dest, &mut clone_state);

  assert_ne!(new_number, old_number);
  assert_eq!("number", to_string_type_id(old_number));
  assert_eq!("number", to_string_type_id(new_number));
  assert_eq!(1, dest.types.size());
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_deep_clone_union() {
  use ulua_analysis::{
    functions::{
      clone_clone::clone_type_id as clone_type, freeze::freeze,
      to_string_to_string::to_string_type_id, unfreeze::unfreeze,
    },
    records::{clone_state::CloneState, type_arena::TypeArena, union_type::UnionType},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let mut dest = TypeArena::default();
  let mut clone_state = CloneState::new(fixture.get_builtins());
  let number_type = fixture.get_builtins().number_type();
  let string_type = fixture.get_builtins().string_type();

  let old_union = {
    let frontend = fixture.get_frontend();
    let global_types = frontend.globals.global_types_mut();
    unfreeze(global_types);
    let old_union = global_types.add_type(UnionType {
      options: alloc::vec![number_type, string_type],
    });
    freeze(global_types);
    old_union
  };

  let new_union = clone_type(old_union, &mut dest, &mut clone_state);

  assert_ne!(new_union, old_union);
  assert_eq!("number | string", to_string_type_id(new_union));
  assert_eq!(1, dest.types.size());
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_do_not_clone_reexports() {
  use ulua_analysis::{functions::get_type, records::table_type::TableType};
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        export type A = {p : number}
        return {}
    "#,
    ),
  );
  fixture.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        local a = require(script.Parent.A)
        export type B = {q : a.A}
        return {}
    "#,
    ),
  );

  let module_b_name = ModuleName::from("Module/B");
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b_name, None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module_a_name = ModuleName::from("Module/A");
  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_a_name);
  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_b_name);

  let type_a = module_a
    .exported_type_bindings
    .get("A")
    .expect("expected exported type A")
    .r#type();
  let type_b = module_b
    .exported_type_bindings
    .get("B")
    .expect("expected exported type B")
    .r#type();

  let table_b = get_type::get::<TableType>(type_b).expect("expected table type B");
  assert_eq!(
    Some(type_a),
    table_b.props.get("q").and_then(|prop| prop.read_ty)
  );
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_do_not_clone_types_of_reexported_values() {
  use ulua_analysis::{
    functions::{first::first, get_type, to_string_to_string::to_string_type_id},
    records::table_type::TableType,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        local exports = {a={p=5}}
        return exports
    "#,
    ),
  );
  fixture.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        local a = require(script.Parent.A)
        local exports = {b=a.a}
        return exports
    "#,
    ),
  );

  let module_b_name = ModuleName::from("Module/B");
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b_name, None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module_a_name = ModuleName::from("Module/A");
  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_a_name);
  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_b_name);

  let type_a = first(module_a.return_type, true).expect("expected Module/A return type");
  let type_b = first(module_b.return_type, true).expect("expected Module/B return type");

  let table_a = get_type::get::<TableType>(type_a)
    .unwrap_or_else(|| panic!("expected table, got {}", to_string_type_id(type_a)));
  let table_b = get_type::get::<TableType>(type_b)
    .unwrap_or_else(|| panic!("expected table, got {}", to_string_type_id(type_b)));

  let prop_a = table_a.props.get("a").expect("expected property a");
  let prop_b = table_b.props.get("b").expect("expected property b");

  assert_eq!(prop_a.read_ty, prop_b.read_ty);
  assert_eq!(prop_a.write_ty, prop_b.write_ty);
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_dont_clone_persistent_primitive() {
  use ulua_analysis::{
    functions::clone_clone::clone_type_id as clone_type,
    records::{clone_state::CloneState, type_arena::TypeArena},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let mut dest = TypeArena::default();
  let number_type = fixture.get_builtins().number_type();
  let mut clone_state = CloneState::new(fixture.get_builtins());

  // number_type is persistent. We leave it as-is.
  let new_number = clone_type(number_type, &mut dest, &mut clone_state);
  assert_eq!(new_number, number_type);
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_is_within_comment() {
  use alloc::string::String;

  use ulua_analysis::functions::is_within_comment_module::is_within_comment_source_module_position;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let source = String::from(
    r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:
        ]] foo:bar()

        --[[]]--[[]] -- Two distinct comments that have zero characters of space between them.
    "#,
  );

  fixture.check_string_optional_frontend_options(&source, None);

  let source_module = fixture.main_source_module();

  assert_eq!(5, source_module.comment_locations.len());

  assert!(is_within_comment_source_module_position(
    source_module,
    Position::new(1, 15)
  ));
  assert!(is_within_comment_source_module_position(
    source_module,
    Position::new(6, 16)
  ));
  assert!(is_within_comment_source_module_position(
    source_module,
    Position::new(9, 13)
  ));
  assert!(is_within_comment_source_module_position(
    source_module,
    Position::new(9, 14)
  ));

  assert!(!is_within_comment_source_module_position(
    source_module,
    Position::new(2, 15)
  ));
  assert!(!is_within_comment_source_module_position(
    source_module,
    Position::new(7, 10)
  ));
  assert!(!is_within_comment_source_module_position(
    source_module,
    Position::new(7, 11)
  ));
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_is_within_comment_parse_result() {
  use alloc::string::String;

  use ulua_analysis::functions::is_within_comment_module::is_within_comment_parse_result_position;
  use ulua_ast::records::{
    allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
    parser::Parser, position::Position,
  };

  let src = String::from(
    r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:
        ]] foo:bar()

        --[[]]--[[]] -- Two distinct comments that have zero characters of space between them.
    "#,
  );

  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut alloc = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut alloc);
  let parse_options = ParseOptions {
    capture_comments: true,
    ..Default::default()
  };
  let parse_result = Parser::parse(&src, &mut names, &mut alloc, parse_options);

  assert_eq!(5, parse_result.comment_locations.len());

  assert!(is_within_comment_parse_result_position(
    &parse_result,
    Position::new(1, 15)
  ));
  assert!(is_within_comment_parse_result_position(
    &parse_result,
    Position::new(6, 16)
  ));
  assert!(is_within_comment_parse_result_position(
    &parse_result,
    Position::new(9, 13)
  ));
  assert!(is_within_comment_parse_result_position(
    &parse_result,
    Position::new(9, 14)
  ));

  assert!(!is_within_comment_parse_result_position(
    &parse_result,
    Position::new(2, 15)
  ));
  assert!(!is_within_comment_parse_result_position(
    &parse_result,
    Position::new(7, 10)
  ));
  assert!(!is_within_comment_parse_result_position(
    &parse_result,
    Position::new(7, 11)
  ));
}

// Source: `tests/Module.test.cpp`
#[test]
fn module_old_solver_correctly_populates_child_scopes() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  fixture.check_string_optional_frontend_options(
    r#"
--!strict
if true then
end

if false then
end

if true then
else
end

local x = {}
for i,v in x do
end
"#,
    None,
  );

  let module_name = ModuleName::from("MainModule");
  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_name);
  assert_eq!(7, module.get_module_scope().children.len());
}
