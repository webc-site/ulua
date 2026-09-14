use alloc::{ffi::CString, string::ToString, sync::Arc, vec, vec::Vec};
// MagicFunction::refine has a default no-op implementation in C++.
use core::ptr::null_mut;

use ulua_ast::records::ast_name_table::AstNameTable;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{polarity::Polarity, solver_mode::SolverMode, table_state::TableState},
  functions::{
    add_global_binding_builtin_definitions::add_global_binding_builtin_definitions,
    attach_magic_function::attach_magic_function, attach_tag_type::attach_tag,
    finalize_global_bindings::finalize_global_bindings, follow_type::follow_type_id,
    get_builtin_definition_source::get_builtin_definition_source,
    get_global_binding::get_global_binding,
    get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
    get_type_function_definition_source::get_type_function_definition_source,
    make_function_builtin_definitions::make_function,
    make_function_builtin_definitions_alt_b::make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_type_id_bool as make_function_poly,
    make_intersection::make_intersection, make_option::make_option,
    to_string_error::to_string_type_error,
  },
  methods::{
    magic_assert_handle_old_solver::magic_assert_handle_old_solver,
    magic_assert_infer::magic_assert_infer,
    magic_clone_handle_old_solver::magic_clone_handle_old_solver,
    magic_clone_infer::magic_clone_infer,
    magic_freeze_handle_old_solver::magic_freeze_handle_old_solver,
    magic_freeze_infer::magic_freeze_infer, magic_freeze_type_check::magic_freeze_type_check,
    magic_pack_handle_old_solver::magic_pack_handle_old_solver, magic_pack_infer::magic_pack_infer,
    magic_pcall_handle_old_solver::magic_pcall_handle_old_solver,
    magic_pcall_infer::magic_pcall_infer,
    magic_require_handle_old_solver::magic_require_handle_old_solver,
    magic_require_infer::magic_require_infer,
    magic_select_handle_old_solver::magic_select_handle_old_solver,
    magic_select_infer::magic_select_infer,
    magic_set_metatable_handle_old_solver::magic_set_metatable_handle_old_solver,
    magic_set_metatable_infer::magic_set_metatable_infer,
  },
  records::{
    extern_type::ExternType, frontend::Frontend, function_type::FunctionType,
    generic_type::GenericType, global_types::GlobalTypes, magic_function::MagicFunction,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    magic_refinement_context::MagicRefinementContext, metatable_type::MetatableType,
    negation_type::NegationType, property_type::Property, scope::Scope, symbol::Symbol,
    table_indexer::TableIndexer, table_type::TableType, type_arena::TypeArena,
    type_function_instance_type::TypeFunctionInstanceType, type_level::TypeLevel,
    type_pack::TypePack,
  },
  type_aliases::{props_type::Props, type_id::TypeId},
};
fn noop_refine(_context: &MagicRefinementContext) {}
fn noop_type_check(_context: &MagicFunctionTypeCheckContext) -> bool {
  false
}

fn read_prop(ty: TypeId) -> Property {
  Property::rw_type_id(ty)
}

fn read_prop_doc(ty: TypeId, doc: &str) -> Property {
  Property {
    read_ty: Some(ty),
    documentation_symbol: Some(doc.to_string()),
    ..Property::default()
  }
}

pub fn register_builtin_globals(
  frontend: &mut Frontend,
  globals: &mut GlobalTypes,
  type_check_for_autocomplete: bool,
) {
  LUAU_ASSERT!(!globals.global_types.types.is_frozen());
  LUAU_ASSERT!(!globals.global_types.type_packs.is_frozen());

  let arena: &mut TypeArena = unsafe { &mut *(&mut globals.global_types as *mut TypeArena) };
  let builtin_types = globals.builtin_types;
  let global_scope_ptr: *mut Scope = Arc::as_ptr(&globals.global_scope) as *mut Scope;

  if frontend.get_luau_solver_mode() == SolverMode::New {
    let type_functions = unsafe { &builtin_types.as_ref().type_functions };
    unsafe { type_functions.add_to_scope(arena as *mut TypeArena, global_scope_ptr) };
  }

  let load_result = {
    let scope = globals.global_scope.clone();
    frontend.load_definition_file(
      globals,
      scope,
      &get_builtin_definition_source(),
      "@luau".to_string(),
      /* captureComments */ false,
      type_check_for_autocomplete,
    )
  };
  if !load_result.success
    && let Some(module) = &load_result.module
  {
    for error in &module.errors {
      eprintln!("builtin definition error: {}", to_string_type_error(error));
    }
  }
  LUAU_ASSERT!(load_result.success);

  let generic_k = arena.add_type(GenericType::generic_type_scope_name_polarity(
    global_scope_ptr,
    "K".into(),
    Polarity::Mixed,
  ));
  let generic_v = arena.add_type(GenericType::generic_type_scope_name_polarity(
    global_scope_ptr,
    "V".into(),
    Polarity::Mixed,
  ));
  let global_level = globals.global_scope.level;
  let map_of_k_to_v = arena.add_type(
    TableType::table_type_props_optional_table_indexer_type_level_table_state(
      &Props::default(),
      Some(TableIndexer {
        index_type: generic_k,
        index_result_type: generic_v,
        is_read_only: false,
      }),
      global_level,
      TableState::Generic,
    ),
  );

  let string_metatable_ty = get_metatable_type_id_not_null_builtin_types(
    unsafe { builtin_types.as_ref() }.string_type,
    unsafe { builtin_types.as_ref() },
  );
  LUAU_ASSERT!(string_metatable_ty.is_some());
  let string_metatable_table =
    get_type_id::<TableType>(follow_type_id(string_metatable_ty.unwrap()));
  LUAU_ASSERT!(string_metatable_table.is_some());
  let string_metatable_table = string_metatable_table.unwrap();

  let index_prop = string_metatable_table.props.get("__index");
  LUAU_ASSERT!(index_prop.is_some());
  let index_prop = index_prop.unwrap();

  add_global_binding_builtin_definitions(globals, "string", index_prop.read_ty.unwrap(), "@luau");
  add_global_binding_builtin_definitions(globals, "string", index_prop.write_ty.unwrap(), "@luau");

  // Setup 'vector' metatable
  let vector_binding = globals
    .global_scope
    .exported_type_bindings
    .get("vector")
    .map(|tf| tf.r#type());
  if let Some(vector_ty) = vector_binding {
    let vector_cls = get_mutable_type_id::<ExternType>(vector_ty);
    if let Some(vector_cls) = vector_cls {
      let metatable = arena.add_type(TableType::table_type_table_state_type_level_scope(
        TableState::Sealed,
        TypeLevel::default(),
        null_mut(),
      ));
      vector_cls.metatable = Some(metatable);

      let number_type = unsafe { builtin_types.as_ref() }.number_type;
      let add_fn = make_function(
        arena,
        Some(vector_ty),
        vec![vector_ty],
        vec![vector_ty],
        false,
      );
      let sub_fn = make_function(
        arena,
        Some(vector_ty),
        vec![vector_ty],
        vec![vector_ty],
        false,
      );
      let unm_fn = make_function(arena, Some(vector_ty), Vec::new(), vec![vector_ty], false);
      let mul_a = make_function(
        arena,
        Some(vector_ty),
        vec![vector_ty],
        vec![vector_ty],
        false,
      );
      let mul_b = make_function(
        arena,
        Some(vector_ty),
        vec![number_type],
        vec![vector_ty],
        false,
      );
      let mul_intersect = make_intersection(arena, vec![mul_a, mul_b]);
      let div_a = make_function(
        arena,
        Some(vector_ty),
        vec![vector_ty],
        vec![vector_ty],
        false,
      );
      let div_b = make_function(
        arena,
        Some(vector_ty),
        vec![number_type],
        vec![vector_ty],
        false,
      );
      let div_intersect = make_intersection(arena, vec![div_a, div_b]);
      let idiv_a = make_function(
        arena,
        Some(vector_ty),
        vec![vector_ty],
        vec![vector_ty],
        false,
      );
      let idiv_b = make_function(
        arena,
        Some(vector_ty),
        vec![number_type],
        vec![vector_ty],
        false,
      );
      let idiv_intersect = make_intersection(arena, vec![idiv_a, idiv_b]);

      let metatable_ty = get_mutable_type_id::<TableType>(metatable);
      // metatable 刚由 arena.add_type 创建为 TableType，LUAU_ASSERT 必命中
      let metatable_ty = metatable_ty.unwrap();
      metatable_ty
        .props
        .insert("__add".to_string(), read_prop(add_fn));
      metatable_ty
        .props
        .insert("__sub".to_string(), read_prop(sub_fn));
      metatable_ty
        .props
        .insert("__unm".to_string(), read_prop(unm_fn));
      metatable_ty
        .props
        .insert("__mul".to_string(), read_prop(mul_intersect));
      metatable_ty
        .props
        .insert("__div".to_string(), read_prop(div_intersect));
      metatable_ty
        .props
        .insert("__idiv".to_string(), read_prop(idiv_intersect));
    }
  }

  // next<K, V>(t: Table<K, V>, i: K?) -> (K?, V)
  let next_arg_k = make_option(builtin_types, arena, generic_k);
  let next_args_type_pack = arena.add_type_pack_t(TypePack {
    head: vec![map_of_k_to_v, next_arg_k],
    tail: None,
  });
  let next_ret_k = make_option(builtin_types, arena, generic_k);
  let next_rets_type_pack = arena.add_type_pack_t(TypePack {
    head: vec![next_ret_k, generic_v],
    tail: None,
  });
  let mut next_ftv =
    FunctionType::function_type_new(next_args_type_pack, next_rets_type_pack, None, false);
  next_ftv.generics = vec![generic_k, generic_v];
  let next_ty = arena.add_type(next_ftv);
  add_global_binding_builtin_definitions(globals, "next", next_ty, "@luau");

  let pairs_args_type_pack = arena.add_type_pack_initializer_list_type_id(&[map_of_k_to_v]);

  let pairs_next = arena.add_type(FunctionType::function_type_new(
    next_args_type_pack,
    next_rets_type_pack,
    None,
    false,
  ));
  let nil_type = unsafe { builtin_types.as_ref() }.nil_type;
  let pairs_return_type_pack = arena.add_type_pack_t(TypePack {
    head: vec![pairs_next, map_of_k_to_v, nil_type],
    tail: None,
  });

  // pairs<K, V>(t: Table<K, V>) -> ((Table<K, V>, K?) -> (K, V), Table<K, V>, nil)
  let mut pairs_ftv =
    FunctionType::function_type_new(pairs_args_type_pack, pairs_return_type_pack, None, false);
  pairs_ftv.generics = vec![generic_k, generic_v];
  let pairs_ty = arena.add_type(pairs_ftv);
  add_global_binding_builtin_definitions(globals, "pairs", pairs_ty, "@luau");

  let generic_mt = arena.add_type(GenericType::generic_type_scope_name_polarity(
    global_scope_ptr,
    "MT".into(),
    Polarity::Mixed,
  ));

  let tab_ty = arena.add_type(TableType::table_type_table_state_type_level_scope(
    TableState::Generic,
    global_level,
    null_mut(),
  ));

  let table_meta_mt = arena.add_type(MetatableType {
    table: tab_ty,
    metatable: generic_mt,
    synthetic_name: None,
  });

  let generic_t = arena.add_type(GenericType::generic_type_scope_name_polarity(
    global_scope_ptr,
    "T".into(),
    Polarity::Mixed,
  ));

  if frontend.get_luau_solver_mode() == SolverMode::New {
    // getmetatable : <T>(T) -> getmetatable<T>
    let getmt_return = arena.add_type(
      TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
        unsafe { &builtin_types.as_ref().type_functions.getmetatable_func },
        vec![generic_t],
      ),
    );
    let f = make_function_poly(
      arena,
      None,
      vec![generic_t],
      Vec::new(),
      vec![generic_t],
      vec![getmt_return],
      false,
    );
    add_global_binding_builtin_definitions(globals, "getmetatable", f, "@luau");
  } else {
    // getmetatable : <MT>({ @metatable MT, {+ +} }) -> MT
    let f = make_function_poly(
      arena,
      None,
      vec![generic_mt],
      Vec::new(),
      vec![table_meta_mt],
      vec![generic_mt],
      false,
    );
    add_global_binding_builtin_definitions(globals, "getmetatable", f, "@luau");
  }

  if frontend.get_luau_solver_mode() == SolverMode::New {
    // setmetatable<T: {}, MT>(T, MT) -> setmetatable<T, MT>
    let setmt_return = arena.add_type(
      TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
        unsafe { &builtin_types.as_ref().type_functions.setmetatable_func },
        vec![generic_t, generic_mt],
      ),
    );
    let f = make_function_poly(
      arena,
      None,
      vec![generic_t, generic_mt],
      Vec::new(),
      vec![generic_t, generic_mt],
      vec![setmt_return],
      false,
    );
    add_global_binding_builtin_definitions(globals, "setmetatable", f, "@luau");
  } else {
    // setmetatable<T: {}, MT>(T, MT) -> { @metatable MT, T }
    let args_pack = arena.add_type_pack_t(TypePack {
      head: vec![tab_ty, generic_mt],
      tail: None,
    });
    let ret_pack = arena.add_type_pack_t(TypePack {
      head: vec![table_meta_mt],
      tail: None,
    });
    let mut ftv = FunctionType::function_type_new(args_pack, ret_pack, None, false);
    ftv.generics = vec![generic_mt];
    let f = arena.add_type(ftv);
    add_global_binding_builtin_definitions(globals, "setmetatable", f, "@luau");
  }

  finalize_global_bindings(globals.global_scope.clone());

  let assert_binding = get_global_binding(globals, "assert");
  attach_magic_function(
    assert_binding,
    Arc::new(MagicFunction {
      handle_old_solver: magic_assert_handle_old_solver,
      infer: magic_assert_infer,
      refine: noop_refine,
      type_check: noop_type_check,
    }),
  );
  let pcall_binding = get_global_binding(globals, "pcall");
  attach_magic_function(
    pcall_binding,
    Arc::new(MagicFunction {
      handle_old_solver: magic_pcall_handle_old_solver,
      infer: magic_pcall_infer,
      refine: noop_refine,
      type_check: noop_type_check,
    }),
  );

  if frontend.get_luau_solver_mode() == SolverMode::New {
    // declare function assert<T>(value: T, errorMessage: string?): intersect<T, ~(false?)>
    let generic_t2 = arena.add_type(GenericType::generic_type_scope_name_polarity(
      global_scope_ptr,
      "T".into(),
      Polarity::Mixed,
    ));

    let falsy_type = unsafe { builtin_types.as_ref() }.falsy_type;
    let negation = arena.add_type(NegationType { ty: falsy_type });
    let refined_ty = arena.add_type(
      TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
        unsafe { &builtin_types.as_ref().type_functions.intersect_func },
        vec![generic_t2, negation],
      ),
    );

    let optional_string_type = unsafe { builtin_types.as_ref() }.optional_string_type;
    let args_pack = arena.add_type_pack_t(TypePack {
      head: vec![generic_t2, optional_string_type],
      tail: None,
    });
    let ret_pack = arena.add_type_pack_t(TypePack {
      head: vec![refined_ty],
      tail: None,
    });
    let mut assert_ftv = FunctionType::function_type_new(args_pack, ret_pack, None, false);
    assert_ftv.generics = vec![generic_t2];
    let assert_ty = arena.add_type(assert_ftv);
    add_global_binding_builtin_definitions(globals, "assert", assert_ty, "@luau");
  }

  let setmetatable_binding = get_global_binding(globals, "setmetatable");
  attach_magic_function(
    setmetatable_binding,
    Arc::new(MagicFunction {
      handle_old_solver: magic_set_metatable_handle_old_solver,
      infer: magic_set_metatable_infer,
      refine: noop_refine,
      type_check: noop_type_check,
    }),
  );
  let select_binding = get_global_binding(globals, "select");
  attach_magic_function(
    select_binding,
    Arc::new(MagicFunction {
      handle_old_solver: magic_select_handle_old_solver,
      infer: magic_select_infer,
      refine: noop_refine,
      type_check: noop_type_check,
    }),
  );

  let table_binding = get_global_binding(globals, "table");
  let ttv = get_mutable_type_id::<TableType>(table_binding);
  if let Some(ttv) = ttv {
    if frontend.get_luau_solver_mode() == SolverMode::New {
      // CLI-114044 - The new solver does not yet support generic tables; model with unconstrained generics.
      let generic_ty = arena.add_type(GenericType::generic_type_scope_name_polarity(
        global_scope_ptr,
        "T".into(),
        Polarity::Mixed,
      ));
      let the_pack = arena.add_type_pack_initializer_list_type_id(&[generic_ty]);
      let mut id_with_magic_ftv = FunctionType::function_type_new(the_pack, the_pack, None, false);
      id_with_magic_ftv.generics = vec![generic_ty];
      let id_ty_with_magic = arena.add_type(id_with_magic_ftv);
      ttv.props.insert(
        "freeze".to_string(),
        read_prop_doc(id_ty_with_magic, "@luau/global/table.freeze"),
      );

      let mut id_ftv = FunctionType::function_type_new(the_pack, the_pack, None, false);
      id_ftv.generics = vec![generic_ty];
      let id_ty = arena.add_type(id_ftv);
      ttv.props.insert(
        "clone".to_string(),
        read_prop_doc(id_ty, "@luau/global/table.clone"),
      );
    } else {
      // tabTy is a generic table type which we can't express via declaration syntax yet
      let freeze_fn = make_function(arena, None, vec![tab_ty], vec![tab_ty], false);
      let clone_fn = make_function(arena, None, vec![tab_ty], vec![tab_ty], false);
      ttv.props.insert(
        "freeze".to_string(),
        read_prop_doc(freeze_fn, "@luau/global/table.freeze"),
      );
      ttv.props.insert(
        "clone".to_string(),
        read_prop_doc(clone_fn, "@luau/global/table.clone"),
      );
    }

    if let Some(p) = ttv.props.get_mut("getn") {
      p.deprecated = true;
      p.deprecated_suggestion = "#".to_string();
    }
    if let Some(p) = ttv.props.get_mut("foreach") {
      p.deprecated = true;
    }
    if let Some(p) = ttv.props.get_mut("foreachi") {
      p.deprecated = true;
    }

    let pack_ty = ttv.props.get("pack").and_then(|p| p.read_ty);
    let clone_ty = ttv.props.get("clone").and_then(|p| p.read_ty);
    let freeze_ty = ttv.props.get("freeze").and_then(|p| p.read_ty);

    if let Some(pack_ty) = pack_ty {
      attach_magic_function(
        pack_ty,
        Arc::new(MagicFunction {
          handle_old_solver: magic_pack_handle_old_solver,
          infer: magic_pack_infer,
          refine: noop_refine,
          type_check: noop_type_check,
        }),
      );
    }
    if let Some(clone_ty) = clone_ty {
      attach_magic_function(
        clone_ty,
        Arc::new(MagicFunction {
          handle_old_solver: magic_clone_handle_old_solver,
          infer: magic_clone_infer,
          refine: noop_refine,
          type_check: noop_type_check,
        }),
      );
    }
    if let Some(freeze_ty) = freeze_ty {
      attach_magic_function(
        freeze_ty,
        Arc::new(MagicFunction {
          handle_old_solver: magic_freeze_handle_old_solver,
          infer: magic_freeze_infer,
          refine: noop_refine,
          type_check: magic_freeze_type_check,
        }),
      );
    }
  }

  let require_ty = get_global_binding(globals, "require");
  attach_tag(require_ty, "require");
  attach_magic_function(
    require_ty,
    Arc::new(MagicFunction {
      handle_old_solver: magic_require_handle_old_solver,
      infer: magic_require_infer,
      refine: noop_refine,
      type_check: noop_type_check,
    }),
  );

  // Global scope cannot be the parent of the type checking environment because it can be changed by the embedder
  let global_type_function_scope_ptr: *mut Scope =
    Arc::as_ptr(&globals.global_type_function_scope) as *mut Scope;
  unsafe {
    (*global_type_function_scope_ptr).exported_type_bindings =
      globals.global_scope.exported_type_bindings.clone();
    (*global_type_function_scope_ptr).builtin_type_names =
      globals.global_scope.builtin_type_names.clone();
  }

  // Type function runtime also removes a few standard libraries and globals, so we will take only the ones that are defined
  let type_function_runtime_bindings: [&str; 24] = [
    // Libraries
    "math",
    "table",
    "string",
    "bit32",
    "utf8",
    "buffer",
    // Globals
    "assert",
    "error",
    "print",
    "next",
    "ipairs",
    "pairs",
    "select",
    "unpack",
    "getmetatable",
    "setmetatable",
    "rawget",
    "rawset",
    "rawlen",
    "rawequal",
    "tonumber",
    "tostring",
    "type",
    "typeof",
  ];

  for name in type_function_runtime_bindings.iter() {
    let name_cstr = CString::new(*name).unwrap();
    let ast_name = unsafe {
      (*(Arc::as_ptr(&globals.global_names.names) as *mut AstNameTable)).get(name_cstr.as_ptr())
    };
    LUAU_ASSERT!(!ast_name.value.is_null());

    let symbol = Symbol::from_global(ast_name);
    let binding = globals.global_scope.bindings.get(&symbol).cloned();
    if let Some(binding) = binding {
      unsafe {
        (*global_type_function_scope_ptr)
          .bindings
          .insert(symbol, binding);
      }
    }
  }

  let type_function_load_result = {
    let scope = globals.global_type_function_scope.clone();
    frontend.load_definition_file(
      globals,
      scope,
      &get_type_function_definition_source(),
      "@luau".to_string(),
      /* captureComments */ false,
      false,
    )
  };
  if !type_function_load_result.success
    && let Some(module) = &type_function_load_result.module
  {
    for error in &module.errors {
      eprintln!(
        "type function definition error: {}",
        to_string_type_error(error)
      );
    }
  }
  LUAU_ASSERT!(type_function_load_result.success);

  finalize_global_bindings(globals.global_type_function_scope.clone());
}
