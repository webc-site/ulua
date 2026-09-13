//! Faithful Rust port of `RefinementExternTypeFixture::getFrontend`'s extern-type
//! registration body (`tests/TypeInfer.refinements.test.cpp` lines 82-146).
//!
//! Adds the Roblox-flavoured extern types the refinement tests need (Vector3,
//! Instance with the magic `IsA`, ExternScriptConnection, Folder, Part,
//! WeldConstraint) to the frontend's global type arena/scope.

use alloc::{string::String, sync::Arc};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{
    attach_magic_function::attach_magic_function, freeze::freeze,
    get_mutable_type::get_mutable_type_id, persist_type::persist, unfreeze::unfreeze,
  },
  records::{
    extern_type::ExternType, frontend::Frontend, function_type::FunctionType,
    property_type::Property, scope::Scope, type_fun::TypeFun, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
/// C++ `ExternType{name, {}, parent, std::nullopt, {}, nullptr, "Test", {}}`.
use ulua_common::FFlag;

use crate::functions::make_magic_instance_is_a::make_magic_instance_is_a;
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

pub fn register_refinement_extern_type_fixture_types(frontend: &mut Frontend) {
  let builtins = unsafe { &*frontend.builtin_types };
  let number_type = builtins.number_type;
  let string_type = builtins.string_type;
  let boolean_type = builtins.boolean_type;
  let nil_type = builtins.nil_type;
  let empty_type_pack = builtins.empty_type_pack;
  let root_super = Some(builtins.extern_type);

  let globals = &mut frontend.globals;
  let scope_ptr = Arc::as_ptr(&globals.global_scope()) as *mut Scope;

  let arena = globals.global_types_mut();
  unfreeze(arena);

  // Vector3 { X: number, Y: number, Z: number }
  let vec3 = arena.add_type(extern_type("Vector3", root_super));
  {
    let v = get_mutable_type_id::<ExternType>(vec3).expect("expected Vector3 extern type");
    v.props
      .insert(String::from("X"), Property::rw_type_id(number_type));
    v.props
      .insert(String::from("Y"), Property::rw_type_id(number_type));
    v.props
      .insert(String::from("Z"), Property::rw_type_id(number_type));
  }

  // Instance { Name: string, IsA: (Instance, string) -> boolean (magic) }
  let inst = arena.add_type(extern_type("Instance", root_super));

  let is_a_params = arena.add_type_pack_initializer_list_type_id(&[inst, string_type]);
  let is_a_rets = arena.add_type_pack_initializer_list_type_id(&[boolean_type]);
  let is_a = arena.add_type(FunctionType::function_type_new(
    is_a_params,
    is_a_rets,
    None,
    false,
  ));
  attach_magic_function(is_a, make_magic_instance_is_a());

  {
    let i = get_mutable_type_id::<ExternType>(inst).expect("expected Instance extern type");
    i.props
      .insert(String::from("Name"), Property::rw_type_id(string_type));
    i.props
      .insert(String::from("IsA"), Property::rw_type_id(is_a));
  }

  // ExternScriptConnection { Disconnect: (ExternScriptConnection) -> () }
  let script_connection = arena.add_type(extern_type("ExternScriptConnection", Some(inst)));
  let disconnect_args = arena.add_type_pack_initializer_list_type_id(&[script_connection]);
  let disconnect = arena.add_type(FunctionType::function_type_new(
    disconnect_args,
    empty_type_pack,
    None,
    false,
  ));
  {
    let s = get_mutable_type_id::<ExternType>(script_connection)
      .expect("expected ExternScriptConnection extern type");
    s.props
      .insert(String::from("Disconnect"), Property::rw_type_id(disconnect));
  }

  // Folder, Part { Position: Vector3 }
  let folder = arena.add_type(extern_type("Folder", Some(inst)));
  let part = arena.add_type(extern_type("Part", Some(inst)));
  {
    let p = get_mutable_type_id::<ExternType>(part).expect("expected Part extern type");
    p.props
      .insert(String::from("Position"), Property::rw_type_id(vec3));
  }

  // WeldConstraint { Part0: Part?, Part1: Part? }
  let optional_part = arena.add_type(UnionType {
    options: alloc::vec![part, nil_type],
  });
  let weld_constraint = arena.add_type(extern_type("WeldConstraint", Some(inst)));
  {
    let w = get_mutable_type_id::<ExternType>(weld_constraint)
      .expect("expected WeldConstraint extern type");
    w.props
      .insert(String::from("Part0"), Property::rw_type_id(optional_part));
    w.props
      .insert(String::from("Part1"), Property::rw_type_id(optional_part));
  }

  unsafe {
    let exported = &mut (*scope_ptr).exported_type_bindings;
    exported.insert(String::from("Vector3"), TypeFun::type_fun_type_id(vec3));
    exported.insert(String::from("Instance"), TypeFun::type_fun_type_id(inst));
    exported.insert(
      String::from("ExternScriptConnection"),
      TypeFun::type_fun_type_id(script_connection),
    );
    exported.insert(String::from("Folder"), TypeFun::type_fun_type_id(folder));
    exported.insert(String::from("Part"), TypeFun::type_fun_type_id(part));
    exported.insert(
      String::from("WeldConstraint"),
      TypeFun::type_fun_type_id(weld_constraint),
    );

    for tf in (*scope_ptr).exported_type_bindings.values() {
      persist(tf.r#type());
    }
  }

  let mode = if FFlag::DebugLuauForceOldSolver.get() {
    SolverMode::Old
  } else {
    SolverMode::New
  };
  frontend.set_luau_solver_mode(mode);

  freeze(frontend.globals.global_types_mut());
}
