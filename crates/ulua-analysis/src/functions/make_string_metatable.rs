use alloc::{string::ToString, sync::Arc, vec, vec::Vec};
use core::ptr::NonNull;
// MagicFunction::refine has a default no-op implementation in C++.
use core::ptr::null_mut;

use crate::{
  enums::{solver_mode::SolverMode, table_state::TableState},
  functions::{
    assign_prop_documentation_symbols::assign_prop_documentation_symbols,
    attach_magic_function::attach_magic_function, get_mutable_type::get_mutable_type_id,
    make_function_builtin_definitions_alt_b::make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_type_id_bool as make_function_poly,
  },
  methods::{
    magic_find_handle_old_solver::magic_find_handle_old_solver,
    magic_format_handle_old_solver::magic_format_handle_old_solver,
    magic_format_infer::magic_format_infer, magic_format_type_check::magic_format_type_check,
    magic_gmatch_handle_old_solver::magic_gmatch_handle_old_solver,
    magic_match_handle_old_solver::magic_match_handle_old_solver,
    magic_match_infer::magic_match_infer,
  },
  records::{
    builtin_types::BuiltinTypes, function_type::FunctionType, magic_find::MagicFind,
    magic_function::MagicFunction, magic_function_call_context::MagicFunctionCallContext,
    magic_gmatch::MagicGmatch, magic_refinement_context::MagicRefinementContext,
    property_type::Property, table_indexer::TableIndexer, table_type::TableType,
    type_arena::TypeArena, type_level::TypeLevel, type_pack::TypePack, union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{props_type::Props, type_id::TypeId, type_pack_id::TypePackId},
};
fn noop_refine(_context: &MagicRefinementContext) {}

// The `infer` logic for gmatch / find is translated as a method on the wrapper struct that
// ignores `self`. Provide free-fn shims so the magic vtable can hold a function pointer.
fn gmatch_infer_shim(context: &MagicFunctionCallContext) -> bool {
  MagicGmatch {
    base: MagicFunction {
      handle_old_solver: magic_gmatch_handle_old_solver,
      infer: gmatch_infer_shim,
      refine: noop_refine,
      type_check: |_| false,
    },
  }
  .infer(context)
}

fn find_infer_shim(context: &MagicFunctionCallContext) -> bool {
  MagicFind {
    base: MagicFunction {
      handle_old_solver: magic_find_handle_old_solver,
      infer: find_infer_shim,
      refine: noop_refine,
      type_check: |_| false,
    },
    handle_old_solver: magic_find_handle_old_solver,
    infer: find_infer_shim,
  }
  .infer(context)
}

fn make_format_magic() -> Arc<MagicFunction> {
  Arc::new(MagicFunction {
    handle_old_solver: magic_format_handle_old_solver,
    infer: magic_format_infer,
    refine: noop_refine,
    type_check: magic_format_type_check,
  })
}

fn make_gmatch_magic() -> Arc<MagicFunction> {
  Arc::new(MagicFunction {
    handle_old_solver: magic_gmatch_handle_old_solver,
    infer: gmatch_infer_shim,
    refine: noop_refine,
    type_check: |_| false,
  })
}

fn make_match_magic() -> Arc<MagicFunction> {
  Arc::new(MagicFunction {
    handle_old_solver: magic_match_handle_old_solver,
    infer: magic_match_infer,
    refine: noop_refine,
    type_check: |_| false,
  })
}

fn make_find_magic() -> Arc<MagicFunction> {
  Arc::new(MagicFunction {
    handle_old_solver: magic_find_handle_old_solver,
    infer: find_infer_shim,
    refine: noop_refine,
    type_check: |_| false,
  })
}

fn read_prop(ty: TypeId) -> Property {
  Property::rw_type_id(ty)
}

pub fn make_string_metatable(mut builtin_types: NonNull<BuiltinTypes>, mode: SolverMode) -> TypeId {
  // C++ `NotNull<TypeArena> arena{builtinTypes->arena.get()}` — a mutable handle into the Box's contents.
  let arena: &mut TypeArena =
    unsafe { &mut *(&mut *builtin_types.as_mut().arena as *mut TypeArena) };
  let builtin_types = unsafe { builtin_types.as_ref() };

  let nil_type = builtin_types.nil_type;
  let number_type = builtin_types.number_type;
  let boolean_type = builtin_types.boolean_type;
  let string_type = builtin_types.string_type;

  let optional_number = arena.add_type(UnionType {
    options: vec![nil_type, number_type],
  });
  let optional_string = arena.add_type(UnionType {
    options: vec![nil_type, string_type],
  });
  let optional_boolean = arena.add_type(UnionType {
    options: vec![nil_type, boolean_type],
  });

  let one_string_pack = arena.add_type_pack_initializer_list_type_id(&[string_type]);
  let any_type_pack = builtin_types.any_type_pack;

  let variadic_tail_pack: TypePackId = if mode == SolverMode::New {
    builtin_types.unknown_type_pack
  } else {
    any_type_pack
  };
  let empty_pack = arena.add_type_pack_initializer_list_type_id(&[]);
  let string_variadic_list = arena.add_type_pack_t(VariadicTypePack {
    ty: string_type,
    hidden: false,
  });
  let number_variadic_list = arena.add_type_pack_t(VariadicTypePack {
    ty: number_type,
    hidden: false,
  });

  let mut format_ftv = FunctionType::function_type_new(
    arena.add_type_pack_t(TypePack {
      head: vec![string_type],
      tail: Some(variadic_tail_pack),
    }),
    one_string_pack,
    None,
    false,
  );
  format_ftv.is_checked_function = true;
  let format_fn = arena.add_type(format_ftv);
  attach_magic_function(format_fn, make_format_magic());

  let string_to_string_type = make_function_poly(
    arena,
    None,
    Vec::new(),
    Vec::new(),
    vec![string_type],
    vec![string_type],
    /* checked */ true,
  );

  let repl_table = arena.add_type(TableType {
    props: Props::default(),
    indexer: Some(TableIndexer {
      index_type: string_type,
      index_result_type: string_type,
      is_read_only: false,
    }),
    state: TableState::Generic,
    level: TypeLevel::default(),
    scope: null_mut(),
    name: None,
    synthetic_name: None,
    instantiated_type_params: Vec::new(),
    instantiated_type_pack_params: Vec::new(),
    definition_module_name: Default::default(),
    definition_location: Default::default(),
    bound_to: None,
    tags: Default::default(),
    remaining_props: 0,
  });
  let repl_fn = make_function_poly(
    arena,
    None,
    Vec::new(),
    Vec::new(),
    vec![string_type],
    vec![string_type],
    /* checked */ false,
  );
  let repl_arg_type = arena.add_type(UnionType {
    options: vec![string_type, repl_table, repl_fn],
  });
  let gsub_func = make_function_poly(
    arena,
    Some(string_type),
    Vec::new(),
    Vec::new(),
    vec![string_type, repl_arg_type, optional_number],
    vec![string_type, number_type],
    /* checked */ false,
  );

  let gmatch_iter_ret = arena.add_type(FunctionType::function_type_new(
    empty_pack,
    string_variadic_list,
    None,
    false,
  ));
  let gmatch_func = make_function_poly(
    arena,
    Some(string_type),
    Vec::new(),
    Vec::new(),
    vec![string_type],
    vec![gmatch_iter_ret],
    /* checked */ true,
  );
  attach_magic_function(gmatch_func, make_gmatch_magic());

  let mut match_func_ty = FunctionType::function_type_new(
    arena.add_type_pack_initializer_list_type_id(&[string_type, string_type, optional_number]),
    arena.add_type_pack_t(VariadicTypePack {
      ty: string_type,
      hidden: false,
    }),
    None,
    false,
  );
  match_func_ty.is_checked_function = true;
  let match_func = arena.add_type(match_func_ty);
  attach_magic_function(match_func, make_match_magic());

  let mut find_func_ty = FunctionType::function_type_new(
    arena.add_type_pack_initializer_list_type_id(&[
      string_type,
      string_type,
      optional_number,
      optional_boolean,
    ]),
    arena.add_type_pack_t(TypePack {
      head: vec![optional_number, optional_number],
      tail: Some(string_variadic_list),
    }),
    None,
    false,
  );
  find_func_ty.is_checked_function = true;
  let find_func = arena.add_type(find_func_ty);
  attach_magic_function(find_func, make_find_magic());

  // string.byte : string -> number? -> number? -> ...number
  let mut string_dot_byte = FunctionType::function_type_new(
    arena.add_type_pack_initializer_list_type_id(&[string_type, optional_number, optional_number]),
    number_variadic_list,
    None,
    false,
  );
  string_dot_byte.is_checked_function = true;

  // string.char : .... number -> string
  let mut string_dot_char = FunctionType::function_type_new(
    number_variadic_list,
    arena.add_type_pack_initializer_list_type_id(&[string_type]),
    None,
    false,
  );
  string_dot_char.is_checked_function = true;

  // string.unpack : string -> string -> number? -> ...any
  let mut string_dot_unpack = FunctionType::function_type_new(
    arena.add_type_pack_t(TypePack {
      head: vec![string_type, string_type, optional_number],
      tail: None,
    }),
    variadic_tail_pack,
    None,
    false,
  );
  string_dot_unpack.is_checked_function = true;

  let byte_fn = arena.add_type(string_dot_byte);
  let char_fn = arena.add_type(string_dot_char);
  let len_fn = make_function_poly(
    arena,
    Some(string_type),
    Vec::new(),
    Vec::new(),
    Vec::new(),
    vec![number_type],
    /* checked */ true,
  );
  let rep_fn = make_function_poly(
    arena,
    Some(string_type),
    Vec::new(),
    Vec::new(),
    vec![number_type],
    vec![string_type],
    /* checked */ true,
  );
  let sub_fn = make_function_poly(
    arena,
    Some(string_type),
    Vec::new(),
    Vec::new(),
    vec![number_type, optional_number],
    vec![string_type],
    /* checked */ true,
  );
  let split_ret_table = arena.add_type(TableType {
    props: Props::default(),
    indexer: Some(TableIndexer {
      index_type: number_type,
      index_result_type: string_type,
      is_read_only: false,
    }),
    state: TableState::Sealed,
    level: TypeLevel::default(),
    scope: null_mut(),
    name: None,
    synthetic_name: None,
    instantiated_type_params: Vec::new(),
    instantiated_type_pack_params: Vec::new(),
    definition_module_name: Default::default(),
    definition_location: Default::default(),
    bound_to: None,
    tags: Default::default(),
    remaining_props: 0,
  });
  let split_fn = make_function_poly(
    arena,
    Some(string_type),
    Vec::new(),
    Vec::new(),
    vec![optional_string],
    vec![split_ret_table],
    /* checked */ true,
  );
  let pack_arg_pack = arena.add_type_pack_t(TypePack {
    head: vec![string_type],
    tail: Some(variadic_tail_pack),
  });
  let pack_fn = arena.add_type(FunctionType::function_type_new(
    pack_arg_pack,
    one_string_pack,
    None,
    false,
  ));
  let packsize_fn = make_function_poly(
    arena,
    Some(string_type),
    Vec::new(),
    Vec::new(),
    Vec::new(),
    vec![number_type],
    /* checked */ true,
  );
  let unpack_fn = arena.add_type(string_dot_unpack);

  let mut string_lib = Props::default();
  string_lib.insert("byte".to_string(), read_prop(byte_fn));
  string_lib.insert("char".to_string(), read_prop(char_fn));
  string_lib.insert("find".to_string(), read_prop(find_func));
  string_lib.insert("format".to_string(), read_prop(format_fn)); // FIXME
  string_lib.insert("gmatch".to_string(), read_prop(gmatch_func));
  string_lib.insert("gsub".to_string(), read_prop(gsub_func));
  string_lib.insert("len".to_string(), read_prop(len_fn));
  string_lib.insert("lower".to_string(), read_prop(string_to_string_type));
  string_lib.insert("match".to_string(), read_prop(match_func));
  string_lib.insert("rep".to_string(), read_prop(rep_fn));
  string_lib.insert("reverse".to_string(), read_prop(string_to_string_type));
  string_lib.insert("sub".to_string(), read_prop(sub_fn));
  string_lib.insert("upper".to_string(), read_prop(string_to_string_type));
  string_lib.insert("split".to_string(), read_prop(split_fn));
  string_lib.insert("pack".to_string(), read_prop(pack_fn));
  string_lib.insert("packsize".to_string(), read_prop(packsize_fn));
  string_lib.insert("unpack".to_string(), read_prop(unpack_fn));

  assign_prop_documentation_symbols(&mut string_lib, "@luau/global/string");

  let table_type = arena.add_type(TableType {
    props: string_lib,
    indexer: None,
    state: TableState::Sealed,
    level: TypeLevel::default(),
    scope: null_mut(),
    name: None,
    synthetic_name: None,
    instantiated_type_params: Vec::new(),
    instantiated_type_pack_params: Vec::new(),
    definition_module_name: Default::default(),
    definition_location: Default::default(),
    bound_to: None,
    tags: Default::default(),
    remaining_props: 0,
  });

  if let Some(ttv) = get_mutable_type_id::<TableType>(table_type) {
    ttv.name = Some("typeof(string)".to_string());
  }

  let mut index_props = Props::default();
  index_props.insert("__index".to_string(), Property::rw_type_id(table_type));
  arena.add_type(TableType {
    props: index_props,
    indexer: None,
    state: TableState::Sealed,
    level: TypeLevel::default(),
    scope: null_mut(),
    name: None,
    synthetic_name: None,
    instantiated_type_params: Vec::new(),
    instantiated_type_pack_params: Vec::new(),
    definition_module_name: Default::default(),
    definition_location: Default::default(),
    bound_to: None,
    tags: Default::default(),
    remaining_props: 0,
  })
}
