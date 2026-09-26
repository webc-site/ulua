//! Faithful Rust port of `RefinementExternTypeFixture::getFrontend`'s extern-type
//! registration body (`tests/TypeInfer.refinements.test.cpp` lines 82-146).
//!
//! Adds the Roblox-flavoured extern types the refinement tests need (Vector3,
//! Instance with the magic `IsA`, ExternScriptConnection, Folder, Part,
//! WeldConstraint) to the frontend's global type arena/scope.

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{
    attach_magic_function::attach_magic_function, freeze::freeze, persist_type::persist,
    unfreeze::unfreeze,
  },
  records::{
    frontend::Frontend, function_type::FunctionType, property_type::Property, union_type::UnionType,
  },
};
use ulua_common::fflag;

use crate::functions::{
  extern_type_fixture::{export_type_binding, extern_prop, extern_type},
  make_magic_instance_is_a::make_magic_instance_is_a,
};

pub fn register_refinement_extern_type_fixture_types(frontend: &mut Frontend) {
  // 经 `Frontend::builtin_types_ref` chokepoint 取内建单例共享引用，仅拷出若干
  // TypeId 标量，借用不绑定 `frontend`，与随后 `&mut frontend.globals` 不重叠。
  let builtins = frontend.builtin_types_ref();
  let number_type = builtins.number_type;
  let string_type = builtins.string_type;
  let boolean_type = builtins.boolean_type;
  let nil_type = builtins.nil_type;
  let empty_type_pack = builtins.empty_type_pack;
  let root_super = Some(builtins.extern_type);

  let globals = &mut frontend.globals;
  let global_scope = globals.global_scope();

  let arena = globals.global_types_mut();
  unfreeze(arena);

  // Vector3 { X: number, Y: number, Z: number }
  let vec3 = arena.add_type(extern_type("Vector3", root_super));
  extern_prop(vec3, "X", Property::rw_type_id(number_type));
  extern_prop(vec3, "Y", Property::rw_type_id(number_type));
  extern_prop(vec3, "Z", Property::rw_type_id(number_type));

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

  extern_prop(inst, "Name", Property::rw_type_id(string_type));
  extern_prop(inst, "IsA", Property::rw_type_id(is_a));

  // ExternScriptConnection { Disconnect: (ExternScriptConnection) -> () }
  let script_connection = arena.add_type(extern_type("ExternScriptConnection", Some(inst)));
  let disconnect_args = arena.add_type_pack_initializer_list_type_id(&[script_connection]);
  let disconnect = arena.add_type(FunctionType::function_type_new(
    disconnect_args,
    empty_type_pack,
    None,
    false,
  ));
  extern_prop(
    script_connection,
    "Disconnect",
    Property::rw_type_id(disconnect),
  );

  // Folder, Part { Position: Vector3 }
  let folder = arena.add_type(extern_type("Folder", Some(inst)));
  let part = arena.add_type(extern_type("Part", Some(inst)));
  extern_prop(part, "Position", Property::rw_type_id(vec3));

  // WeldConstraint { Part0: Part?, Part1: Part? }
  let optional_part = arena.add_type(UnionType {
    options: alloc::vec![part, nil_type],
  });
  let weld_constraint = arena.add_type(extern_type("WeldConstraint", Some(inst)));
  extern_prop(
    weld_constraint,
    "Part0",
    Property::rw_type_id(optional_part),
  );
  extern_prop(
    weld_constraint,
    "Part1",
    Property::rw_type_id(optional_part),
  );

  let exported_bindings = [
    ("Vector3", vec3),
    ("Instance", inst),
    ("ExternScriptConnection", script_connection),
    ("Folder", folder),
    ("Part", part),
    ("WeldConstraint", weld_constraint),
  ];
  for (name, ty) in exported_bindings {
    export_type_binding(globals, name, ty);
  }

  // 读走 `Arc<Scope>` 共享引用（与被测侧 snapshot/遍历 exportedTypeBindings 同款
  // 读法），无需裸指针解引用。
  for tf in global_scope.exported_type_bindings.values() {
    persist(tf.r#type());
  }

  let mode = if fflag::DebugLuauForceOldSolver.get() {
    SolverMode::Old
  } else {
    SolverMode::New
  };
  frontend.set_luau_solver_mode(mode);

  freeze(frontend.globals.global_types_mut());
}
