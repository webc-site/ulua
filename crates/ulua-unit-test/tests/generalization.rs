extern crate alloc;

use ulua_analysis::functions::follow_type;

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_a_a() {
  use ulua_analysis::records::function_type::FunctionType;
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let free_ty = fixture.fresh_type().0;
  let args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[free_ty]);
  let rets = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[free_ty]);
  let fn_ty = fixture
    .arena
    .add_type(FunctionType::function_type_new(args, rets, None, false));

  fixture.generalize(fn_ty);

  assert_eq!("<a>(a) -> a", fixture.to_string_type_id(fn_ty));
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_a_b() {
  use ulua_analysis::records::{
    function_type::FunctionType, table_indexer::TableIndexer, table_type::TableType,
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let (a_ty, a_free) = fixture.fresh_type();
  let (b_ty, _) = fixture.fresh_type();

  let mut tt = TableType::new();
  tt.indexer = Some(TableIndexer {
    index_type: fixture.builtin_types.number_type,
    index_result_type: b_ty,
    is_read_only: false,
  });

  let table_ty = fixture.arena.add_type(tt);
  unsafe {
    (*a_free).upper_bound = table_ty;
  }

  let args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[a_ty]);
  let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
    args,
    fixture.builtin_types.empty_type_pack,
    None,
    false,
  ));

  fixture.generalize(function_ty);

  assert_eq!("<a>({a}) -> ()", fixture.to_string_type_id(function_ty));
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_a_number_string_string() {
  use ulua_analysis::records::{function_type::FunctionType, union_type::UnionType};
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let (a_ty, a_free) = fixture.fresh_type();

  let upper_bound = fixture.arena.add_type(UnionType {
    options: vec![
      fixture.builtin_types.number_type,
      fixture.builtin_types.string_type,
    ],
  });
  unsafe {
    (*a_free).upper_bound = upper_bound;
  }

  let args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[a_ty]);
  let rets = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.optional_string_type]);
  let fn_type = fixture
    .arena
    .add_type(FunctionType::function_type_new(args, rets, None, false));

  fixture.generalize(fn_type);

  assert_eq!(
    "(number | string) -> string?",
    fixture.to_string_type_id(fn_type)
  );
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_avoid_cross_module_mutation_in_bidirectional_inference() {
  use std::sync::Arc;

  use ulua_analysis::{
    functions::freeze::freeze, records::module::Module, type_aliases::module_name_type::ModuleName,
  };
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.base.file_resolver.source.insert(
    ModuleName::from("Module/ListFns"),
    String::from(
      r#"
        local mod = {}
        function mod.findWhere(list, predicate): number?
            for i = 1, #list do
                if predicate(list[i], i) then
                    return i
                end
            end
            return nil
        end
        return mod
    "#,
    ),
  );
  fixture.base.file_resolver.source.insert(
    ModuleName::from("Module/B"),
    String::from(
      r#"
        local funs = require(script.Parent.ListFns)
        local accessories = funs.findWhere(getList(), function(accessory)
            return accessory.AccessoryType ~= accessoryTypeEnum
        end)
        return {}
    "#,
    ),
  );

  let module_list_fns = ModuleName::from("Module/ListFns");
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_list_fns, None);
  let mod_list_fns = fixture
    .get_frontend()
    .module_resolver
    .get_module(&module_list_fns);

  unsafe {
    let module = Arc::as_ptr(&mod_list_fns) as *mut Module;
    freeze(&mut (*module).interface_types);
    freeze(&mut (*module).internal_types);
  }

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  let module_b = ModuleName::from("Module/B");
  let _result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b, None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_b_t1_a_t1_t1_where_t1_a_t1_c() {
  use ulua_analysis::records::{
    function_type::FunctionType, table_indexer::TableIndexer, table_type::TableType,
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let (a_ty, a_free) = fixture.fresh_type();
  let (b_ty, b_free) = fixture.fresh_type();
  let (c_ty, c_free) = fixture.fresh_type();

  unsafe {
    (*a_free).upper_bound = c_ty;
    (*c_free).lower_bound = a_ty;
  }

  let mut tt = TableType::new();
  tt.indexer = Some(TableIndexer {
    index_type: fixture.builtin_types.number_type,
    index_result_type: c_ty,
    is_read_only: false,
  });

  let table_ty = fixture.arena.add_type(tt);
  unsafe {
    (*b_free).upper_bound = table_ty;
  }

  let args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[b_ty, a_ty]);
  let rets = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[c_ty]);
  let function_ty = fixture
    .arena
    .add_type(FunctionType::function_type_new(args, rets, None, false));

  fixture.generalize(function_ty);

  assert_eq!("<a>({a}, a) -> a", fixture.to_string_type_id(function_ty));
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_cache_fully_generalized_types() {
  use alloc::collections::BTreeMap;

  use ulua_analysis::records::property_type::Property;
  use ulua_unit_test::{
    functions::add_sealed_table_type::add_sealed_table_type,
    records::generalization_fixture::GeneralizationFixture,
  };

  let mut fixture = GeneralizationFixture::new();
  assert!(fixture.generalized_types.empty());

  let mut props = BTreeMap::new();
  props.insert(
    String::from("one"),
    Property::rw_type_id(fixture.builtin_types.number_type),
  );
  props.insert(
    String::from("two"),
    Property::rw_type_id(fixture.builtin_types.string_type),
  );
  let tiny_table = add_sealed_table_type(&mut fixture.arena, &props, None);

  fixture.generalize(tiny_table);

  assert!(fixture.generalized_types.contains(&tiny_table));
  assert!(
    fixture
      .generalized_types
      .contains(&fixture.builtin_types.number_type)
  );
  assert!(
    fixture
      .generalized_types
      .contains(&fixture.builtin_types.string_type)
  );
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_dont_cache_types_that_arent_done_yet() {
  use alloc::collections::BTreeMap;
  use std::sync::Arc;

  use ulua_analysis::{
    enums::polarity::Polarity,
    records::{
      free_type::FreeType, function_type::FunctionType, property_type::Property, scope::Scope,
    },
  };
  use ulua_unit_test::{
    functions::add_sealed_table_type::add_sealed_table_type,
    records::generalization_fixture::GeneralizationFixture,
  };

  let mut fixture = GeneralizationFixture::new();
  let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;
  let free_ty = fixture
    .arena
    .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
      global_scope,
      fixture.builtin_types.never_type,
      fixture.builtin_types.string_type,
      Polarity::Unknown,
    ));

  let fn_ret = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.number_type]);
  let fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
    fixture.builtin_types.empty_type_pack,
    fn_ret,
    None,
    false,
  ));

  let mut props = BTreeMap::new();
  props.insert(
    String::from("one"),
    Property::rw_type_id(fixture.builtin_types.number_type),
  );
  props.insert(String::from("two"), Property::rw_type_id(free_ty));
  props.insert(String::from("three"), Property::rw_type_id(fn_ty));
  let table_ty = add_sealed_table_type(&mut fixture.arena, &props, None);

  fixture.generalize(table_ty);

  assert!(fixture.generalized_types.contains(&fn_ty));
  assert!(
    fixture
      .generalized_types
      .contains(&fixture.builtin_types.number_type)
  );
  assert!(
    fixture
      .generalized_types
      .contains(&fixture.builtin_types.never_type)
  );
  assert!(
    fixture
      .generalized_types
      .contains(&fixture.builtin_types.string_type)
  );
  assert!(!fixture.generalized_types.contains(&free_ty));
  assert!(!fixture.generalized_types.contains(&table_ty));
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_dont_traverse_into_class_types_when_generalizing() {
  use alloc::collections::BTreeMap;

  use ulua_analysis::{
    functions::get_type,
    records::{extern_type::ExternType, free_type::FreeType, property_type::Property},
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let (prop_ty, _) = fixture.fresh_type();

  let mut props = BTreeMap::new();
  props.insert(String::from("oh_no"), Property::readonly(prop_ty));
  let cursed_extern_type = fixture.arena.add_type(ExternType {
    name: String::from("Cursed"),
    props,
    parent: None,
    metatable: None,
    tags: Default::default(),
    user_data: None,
    definition_module_name: Default::default(),
    definition_location: None,
    indexer: None,
    relation: None,
  });

  let gen_extern_type = fixture.generalize(cursed_extern_type);
  assert!(gen_extern_type.is_some());

  let gen_extern_type = gen_extern_type.unwrap();
  let extern_type = get_type::get::<ExternType>(gen_extern_type).unwrap();
  let gen_prop_ty = extern_type.props.get("oh_no").unwrap().read_ty.unwrap();
  assert!(get_type::get::<FreeType>(gen_prop_ty).is_some());
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_functions_containing_cyclic_tables_can_be_cached() {
  use alloc::collections::BTreeMap;

  use ulua_analysis::{
    enums::table_state::TableState,
    functions::as_mutable_type::as_mutable_type_id,
    records::{
      blocked_type::BlockedType, function_type::FunctionType, property_type::Property,
      table_type::TableType, type_level::TypeLevel,
    },
    type_aliases::type_variant::TypeVariant,
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let self_ty = fixture.arena.add_type(BlockedType::default());

  let method_args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[self_ty]);
  let method_rets = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.number_type]);
  let method_ty = fixture.arena.add_type(FunctionType::function_type_new(
    method_args,
    method_rets,
    None,
    false,
  ));

  let mut props = BTreeMap::new();
  props.insert(
    String::from("count"),
    Property::rw_type_id(fixture.builtin_types.number_type),
  );
  props.insert(String::from("method"), Property::rw_type_id(method_ty));
  unsafe {
    (*as_mutable_type_id(self_ty)).ty = TypeVariant::Table(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        None,
        TypeLevel::default(),
        TableState::Sealed,
      ),
    );
  }

  fixture.generalize(method_ty);

  assert!(fixture.generalized_types.contains(&method_ty));
  assert!(fixture.generalized_types.contains(&self_ty));
  assert!(
    fixture
      .generalized_types
      .contains(&fixture.builtin_types.number_type)
  );
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generalization_fuzzer_crash() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        type function t0<A>(l0,...):""
        type t0 = any
        do
        _()
        _ = {_=...,}
        _ = {_=rawget({_=_,l0,},_,- _),}
        end
        end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generalization_should_not_leak_free_type() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flag = ScopedFastFlag::new(&fflag::DebugLuauForbidInternalTypes, true);
  let mut fixture = BuiltinsFixture::default();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        function foo()

            local productButtonPairs = {}
            local func
            local dir = -1

            local function updateSearch()
                for product, button in pairs(productButtonPairs) do
                    -- This line may have a floating free type pack.
                    button.LayoutOrder = func(product) * dir
                end
            end

            function(mode)
                if mode == 'New'then
                    func = function(p)
                        return p.id
                    end
                elseif mode == 'Price'then
                    func = function(p)
                        return p.price
                    end
                end
            end
        end
    "#,
    None,
  );
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generalization_traversal_should_re_traverse_unions_if_they_change_type() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
function byId(p)
 return p.id
end

function foo()

 local productButtonPairs = {}
 local func = byId
 local dir = -1

 local function updateSearch()
  for product, button in pairs(productButtonPairs) do
   button.LayoutOrder = func(product) * dir
  end
 end

  function(mode)
   if mode == 'Name'then
   else
    if mode == 'New'then
     func = function(p)
      return p.id
     end
    elseif mode == 'Price'then
     func = function(p)
      return p.price
     end
    end

   end
  end
end
"#,
    None,
  );
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type() {
  use ulua_analysis::functions::follow_type;
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let (t1, ft1) = fixture.fresh_type();
  let (t2, ft2) = fixture.fresh_type();

  unsafe {
    (*ft1).lower_bound = t2;
    (*ft2).upper_bound = t1;
    (*ft2).lower_bound = fixture.builtin_types.unknown_type;
  }

  let t2_generalized = fixture.generalize(t2);
  assert!(t2_generalized.is_some());

  assert_eq!(follow_type::follow(t1), follow_type::follow(t2));

  let t1_generalized = fixture.generalize(t1);
  assert!(t1_generalized.is_some());

  assert_eq!(fixture.builtin_types.unknown_type, follow_type::follow(t1));
  assert_eq!(fixture.builtin_types.unknown_type, follow_type::follow(t2));
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generalize_a_type_that_is_bounded_by_another_generalizable_type_in_reverse_order()
{
  use ulua_analysis::functions::follow_type;
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let (t1, ft1) = fixture.fresh_type();
  let (t2, ft2) = fixture.fresh_type();

  unsafe {
    (*ft1).lower_bound = t2;
    (*ft2).upper_bound = t1;
    (*ft2).lower_bound = fixture.builtin_types.unknown_type;
  }

  let t1_generalized = fixture.generalize(t1);
  assert!(t1_generalized.is_some());

  assert_eq!(follow_type::follow(t1), follow_type::follow(t2));

  let t2_generalized = fixture.generalize(t2);
  assert!(t2_generalized.is_some());

  assert_eq!(fixture.builtin_types.unknown_type, follow_type::follow(t1));
  assert_eq!(fixture.builtin_types.unknown_type, follow_type::follow(t2));
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generic_argument_with_singleton_oss_1808() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function test<T>(value: false | (T) -> T)
            return value
        end
        test(false)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generics_dont_leak_into_callback() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local func: <T>(T, (T) -> ()) -> () = nil :: any
        func({}, function(obj)
            local _ = obj
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "unknown",
    to_string_type_id(fixture.require_type_at_position_position(Position {
      line: 3,
      column: 23,
    }))
  );
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_generics_dont_leak_into_callback_2() {
  use ulua_analysis::{
    functions::{get_error::get_type_error, to_string_to_string::to_string_type_id},
    records::type_mismatch::TypeMismatch,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
local func: <T>(T, (T) -> ()) -> () = nil :: any
local foobar: (number) -> () = nil :: any
func({}, function(obj)
    foobar(obj)
end)
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(err.wanted_type));
  assert_eq!("{  }", to_string_type_id(err.given_type));
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_intersection_type_traversal_doesnt_crash() {
  use std::sync::Arc;

  use ulua_analysis::{
    functions::get_mutable_type,
    records::{free_type::FreeType, intersection_type::IntersectionType, scope::Scope},
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;

  let i = fixture
    .arena
    .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
  let h = fixture
    .arena
    .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
  let j = fixture
    .arena
    .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
  let intersection_type = fixture
    .arena
    .add_type(IntersectionType { parts: vec![h, j] });

  get_mutable_type::get_mutable::<FreeType>(h)
    .unwrap()
    .upper_bound = i;
  get_mutable_type::get_mutable::<FreeType>(h)
    .unwrap()
    .lower_bound = fixture.builtin_types.never_type;
  get_mutable_type::get_mutable::<FreeType>(i)
    .unwrap()
    .upper_bound = fixture.builtin_types.unknown_type;
  get_mutable_type::get_mutable::<FreeType>(i)
    .unwrap()
    .lower_bound = intersection_type;
  get_mutable_type::get_mutable::<FreeType>(j)
    .unwrap()
    .upper_bound = i;
  get_mutable_type::get_mutable::<FreeType>(j)
    .unwrap()
    .lower_bound = fixture.builtin_types.never_type;

  fixture.generalize(intersection_type);
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_t1_t1_b_where_t1_a_t1_b_number_number() {
  use ulua_analysis::records::{
    function_type::FunctionType, intersection_type::IntersectionType, table_indexer::TableIndexer,
    table_type::TableType,
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let mut tt = TableType::new();
  tt.indexer = Some(TableIndexer {
    index_type: fixture.builtin_types.number_type,
    index_result_type: fixture.builtin_types.number_type,
    is_read_only: false,
  });
  let number_array = fixture.arena.add_type(tt);

  let (a_ty, a_free) = fixture.fresh_type();
  let (b_ty, b_free) = fixture.fresh_type();

  let upper_bound = fixture.arena.add_type(IntersectionType {
    parts: vec![b_ty, number_array, number_array],
  });
  unsafe {
    (*a_free).upper_bound = upper_bound;
    (*b_free).lower_bound = a_ty;
  }

  let args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[a_ty, b_ty]);
  let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
    args,
    fixture.builtin_types.empty_type_pack,
    None,
    false,
  ));

  fixture.generalize(function_ty);

  assert_eq!(
    "(unknown & {number}, unknown) -> ()",
    fixture.to_string_type_id(function_ty)
  );
}

// Source: `tests/Generalization.test.cpp`
#[test]
fn generalization_union_type_traversal_doesnt_crash() {
  use std::sync::Arc;

  use ulua_analysis::{
    functions::get_mutable_type,
    records::{free_type::FreeType, scope::Scope, union_type::UnionType},
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;

  let mut fixture = GeneralizationFixture::new();
  let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;

  let i = fixture
    .arena
    .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
  let h = fixture
    .arena
    .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
  let j = fixture
    .arena
    .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
  let union_type = fixture.arena.add_type(UnionType {
    options: vec![h, j],
  });

  get_mutable_type::get_mutable::<FreeType>(h)
    .unwrap()
    .upper_bound = i;
  get_mutable_type::get_mutable::<FreeType>(h)
    .unwrap()
    .lower_bound = fixture.builtin_types.never_type;
  get_mutable_type::get_mutable::<FreeType>(i)
    .unwrap()
    .upper_bound = fixture.builtin_types.unknown_type;
  get_mutable_type::get_mutable::<FreeType>(i)
    .unwrap()
    .lower_bound = union_type;
  get_mutable_type::get_mutable::<FreeType>(j)
    .unwrap()
    .upper_bound = i;
  get_mutable_type::get_mutable::<FreeType>(j)
    .unwrap()
    .lower_bound = fixture.builtin_types.never_type;

  fixture.generalize(union_type);
}

// Source: `tests/Generalization.test.cpp:489-504`
//
// 已知缺件（对照 cpp `Generalization.test.cpp`）：
// - `searching_for_free_types_does_not_use_the_native_stack`（:568）未移植——
//   依赖上游 FFlag `LuauIterativeTypeSearcher`，本端口尚未 sync 该 flag 与
//   迭代式搜索实现，faithful 移植不可表达。
// - `mixed_polarity_is_recovered`（:585）未移植——断言读
//   `FunctionType::generics`，Rust 侧该字段为 `pub(crate)`，外部测试访问须
//   先泄漏内部 API（review §8：被迫 pub 才能迁的不迁），待上游提供公开
//   accessor 后补回。
#[test]
fn collapse_two_type_direct_cycle() {
  use ulua_analysis::records::function_type::FunctionType;
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;
  let mut fixture = GeneralizationFixture::new();
  let (t1, ft1) = fixture.fresh_type();
  let (t2, ft2) = fixture.fresh_type();

  // t1.upper = t2, t2.lower = t1 —— 直接二元环
  unsafe {
    (*ft1).upper_bound = t2;
    (*ft2).lower_bound = t1;
  }

  let args = fixture.arena.add_type_pack_initializer_list_type_id(&[t1]);
  let rets = fixture.arena.add_type_pack_initializer_list_type_id(&[t2]);
  let function_ty = fixture
    .arena
    .add_type(FunctionType::function_type_new(args, rets, None, false));

  fixture.generalize(function_ty);

  // 双方解析到同一代表
  assert_eq!(follow_type::follow(t1), follow_type::follow(t2));
}

// Source: `tests/Generalization.test.cpp:506-523`
#[test]
fn collapse_cycle_with_external_bound() {
  use ulua_analysis::records::function_type::FunctionType;
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;
  let mut fixture = GeneralizationFixture::new();
  let (t1, ft1) = fixture.fresh_type();
  let (t2, ft2) = fixture.fresh_type();

  // t1.upper = t2, t1.lower = number, t2.lower = t1 —— 环带外部界
  unsafe {
    (*ft1).upper_bound = t2;
    (*ft1).lower_bound = fixture.builtin_types.number_type;
    (*ft2).lower_bound = t1;
  }

  let rets = fixture.arena.add_type_pack_initializer_list_type_id(&[t1]);
  let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
    fixture.builtin_types.empty_type_pack,
    rets,
    None,
    false,
  ));

  fixture.generalize(function_ty);

  // 环塌缩后代表应泛化到 number
  assert_eq!("number", fixture.to_string_type_id(follow_type::follow(t1)));
  assert_eq!("number", fixture.to_string_type_id(follow_type::follow(t2)));
}

// Source: `tests/Generalization.test.cpp:525-543`
//
// 曾为 Rust 侧真实行为缺口：缺 `cpp/Analysis/src/Generalization.cpp:824-978`
// 的 `collapseInvariantFreeType` + `collapseDirectBoundCycleAt` 预处理
// （把环成员从 union/intersection 界中 `removeType` 剥掉后再合并外部界）。
// 现由 `ulua_analysis::functions::collapse_free_type_cycles` 移植后启用。
#[test]
fn collapse_cycle_with_external_bound_in_union() {
  use alloc::vec;

  use ulua_analysis::records::{function_type::FunctionType, union_type::UnionType};
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;
  let mut fixture = GeneralizationFixture::new();
  let (t1, ft1) = fixture.fresh_type();
  let (t2, ft2) = fixture.fresh_type();

  // t1.upper = t2, t1.lower = number | t2, t2.lower = t1, t2.upper = t1
  unsafe {
    (*ft1).upper_bound = t2;
  }
  let union_ty = fixture.arena.add_type(UnionType {
    options: vec![fixture.builtin_types.number_type, t2],
  });
  unsafe {
    (*ft1).lower_bound = union_ty;
    (*ft2).lower_bound = t1;
    (*ft2).upper_bound = t1;
  }

  let rets = fixture.arena.add_type_pack_initializer_list_type_id(&[t1]);
  let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
    fixture.builtin_types.empty_type_pack,
    rets,
    None,
    false,
  ));

  fixture.generalize(function_ty);

  assert_eq!("number", fixture.to_string_type_id(follow_type::follow(t1)));
  assert_eq!("number", fixture.to_string_type_id(follow_type::follow(t2)));
}

// Source: `tests/Generalization.test.cpp:545-566`
#[test]
fn no_spurious_cycle_through_intersection() {
  use alloc::vec;

  use ulua_analysis::records::{
    function_type::FunctionType, intersection_type::IntersectionType, table_indexer::TableIndexer,
    table_type::TableType,
  };
  use ulua_unit_test::records::generalization_fixture::GeneralizationFixture;
  let mut fixture = GeneralizationFixture::new();

  let mut tt = TableType::new();
  tt.indexer = Some(TableIndexer {
    index_type: fixture.builtin_types.number_type,
    index_result_type: fixture.builtin_types.number_type,
    is_read_only: false,
  });
  let number_array = fixture.arena.add_type(tt);

  let (t1, ft1) = fixture.fresh_type();
  let (t2, ft2) = fixture.fresh_type();

  // t1.upper = t2 & number[]（交集包裹 t2，非直接界）；t2.lower = t1（直接）
  // 二者不应成环
  let intersection_ty = fixture.arena.add_type(IntersectionType {
    parts: vec![t2, number_array],
  });
  unsafe {
    (*ft1).upper_bound = intersection_ty;
    (*ft2).lower_bound = t1;
  }

  let args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[t1, t2]);
  let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
    args,
    fixture.builtin_types.empty_type_pack,
    None,
    false,
  ));

  fixture.generalize(function_ty);

  // t1 与 t2 应保持互异，不得被误塌缩为一体
  assert_ne!(
    fixture.to_string_type_id(follow_type::follow(t1)),
    fixture.to_string_type_id(follow_type::follow(t2))
  );
}

// Source: `tests/Generalization.test.cpp:609-626`（Fixture，新 solver 专用）
#[test]
fn respect_useless_user_authored_generics() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  // cpp `DOES_NOT_PASS_OLD_SOLVER_GUARD()`：强制新 solver
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        local function getitem<T>(tbl: { read item: T })
            local _ = tbl.item
        end
    "#,
    None,
  );
  assert!(result.errors.is_empty(), "{:?}", result.errors);

  let getitem = fixture.require_type_string("getitem");
  assert_eq!("<T>({ read item: T }) -> ()", to_string_type_id(getitem));
}
