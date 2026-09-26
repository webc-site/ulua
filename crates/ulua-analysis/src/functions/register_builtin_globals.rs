use alloc::{string::ToString, sync::Arc, vec, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

// MagicFunction::refine has a default no-op implementation in C++.
use crate::functions::arc_as_mut::arc_as_mut;
use crate::{
  enums::{polarity::Polarity, solver_mode::SolverMode, table_state::TableState},
  functions::{
    add_global_binding_builtin_definitions::add_global_binding_builtin_definitions,
    attach_magic_function::attach_magic_function,
    attach_tag_type::attach_tag,
    finalize_global_bindings::finalize_global_bindings,
    follow_type,
    get_builtin_definition_source::get_builtin_definition_source,
    get_global_binding::get_global_binding,
    get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_mutable_type, get_type,
    get_type_function_definition_source::get_type_function_definition_source,
    make_function_builtin_definitions::{
      make_function,
      make_function_type_arena_optional_type_id_initializer_list_type_id_initializer_list_type_pack_id_initializer_list_type_id_initializer_list_type_id_bool as make_function_poly,
    },
    make_intersection::make_intersection,
    make_option::make_option,
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
    arena_handle::Handle, builtin_types::BuiltinTypes, extern_type::ExternType, frontend::Frontend,
    function_type::FunctionType, generic_type::GenericType, global_types::GlobalTypes,
    magic_function::MagicFunction,
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

/// cpp `registerBuiltinGlobals(Frontend&, GlobalTypes&, bool)` 的实现本体。
///
/// 形参对 `(&mut Frontend, &mut GlobalTypes)` 是同一对象的整体+字段别名，
/// 属刻意保留的重叠借用（被调方内 `frontend.load_definition_file(globals, …)`
/// 同时经两者写入），故本函数只在 crate 内可见：外部调用方一律走单
/// `&mut Frontend` 门面 [`Frontend::register_builtin_globals`] /
/// [`Frontend::register_builtin_globals_for_autocomplete`]
/// （`methods/frontend_register_builtin_globals.rs`），重叠别名窗口收在
/// 那唯一一处。
pub(crate) fn register_builtin_globals(
  frontend: &mut Frontend,
  globals: &mut GlobalTypes,
  type_check_for_autocomplete: bool,
) {
  LUAU_ASSERT!(!globals.global_types.types.is_frozen());
  LUAU_ASSERT!(!globals.global_types.type_packs.is_frozen());

  // C++ `TypeArena& arena` 局部引用直译：原形态为 `&mut *(&mut globals.global_types
  // as *mut TypeArena)` 裸化再借用，现用既有 `Handle::from_mut` + `get_mut` 单行
  // 收敛（与本文件 `add_to_scope` 调用同一 wave2 句柄形态），借用窗口与原裸指针
  // 同构。soundness 不变量：`arena` 存续期间，中途把 `&mut globals` 整体传出的
  // 调用（load_definition_file、add_global_binding_builtin_definitions 等）与经
  // `arena` 的写入在时间上严格串行——任一瞬间只有一条路径触碰 `global_types`
  // 字段，字段级不相交编译器无法推导，沿用 C++ 中 `TypeArena&` 与 `GlobalTypes&`
  // 并存的引用别名格局，且全库补全/注册链路单线程运行。
  let arena: &mut TypeArena = Handle::from_mut(&mut globals.global_types).get_mut();
  // 经 `GlobalTypes::builtin_types_ref` chokepoint 取内建单例共享引用（自指针
  // 布线契约集中于 chokepoint，调用点免 unsafe）。`make_option` 仍以 `NonNull`
  // 形参直传（其内部只读），由共享引用安全转铸取句柄，不再触碰 `globals`
  // 的裸字段。
  let builtin_types_ref: &BuiltinTypes = globals.builtin_types_ref();
  let builtin_types = NonNull::from(builtin_types_ref);
  let global_scope_ptr: *mut Scope = arc_as_mut(&globals.global_scope);

  if frontend.get_luau_solver_mode() == SolverMode::New {
    let type_functions = &builtin_types_ref.type_functions;
    // Safety: `add_to_scope` 的 `scope` 裸指针契约——`global_scope_ptr` 源自
    // `arc_as_mut(&globals.global_scope)`，该 Arc 由 `globals` 持有且写入为
    // 单线程独占；`arena` 经 `&mut` 再借用包成 Handle（同一 provenance、此刻
    // 无其他借用者）；`type_functions` 仅读。
    unsafe { type_functions.add_to_scope(Handle::from_mut(&mut *arena), global_scope_ptr) };
  }

  let load_result = {
    let scope = globals.global_scope.clone();
    frontend.load_definition_file_into_globals(
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

  let string_metatable_ty =
    get_metatable_type_id_not_null_builtin_types(builtin_types_ref.string_type, builtin_types_ref);
  LUAU_ASSERT!(string_metatable_ty.is_some());
  let string_metatable_table = get_type::get::<TableType>(follow_type::follow(
    string_metatable_ty
      .expect("cpp LUAU_ASSERT：string 内建元表由 BuiltinDefinitions 恒建为 MetatableType"),
  ));
  LUAU_ASSERT!(string_metatable_table.is_some());
  let string_metatable_table =
    string_metatable_table.expect("cpp LUAU_ASSERT：元表 follow 后恒为 TableType");

  let index_prop = string_metatable_table.props.get("__index");
  LUAU_ASSERT!(index_prop.is_some());
  let index_prop = index_prop.expect("cpp LUAU_ASSERT：string 元表恒带 __index 属性");

  add_global_binding_builtin_definitions(
    globals,
    "string",
    index_prop
      .read_ty
      .expect("string 元表 __index 的 read_ty 由内建定义接线（cpp 直 deref）"),
    "@luau",
  );
  add_global_binding_builtin_definitions(
    globals,
    "string",
    index_prop
      .write_ty
      .expect("string 元表 __index 的 write_ty 由内建定义接线（cpp 直 deref）"),
    "@luau",
  );

  // Setup 'vector' metatable
  let vector_binding = globals
    .global_scope
    .exported_type_bindings
    .get("vector")
    .map(|tf| tf.r#type());
  if let Some(vector_ty) = vector_binding {
    let vector_cls = get_mutable_type::get_mutable::<ExternType>(vector_ty);
    if let Some(vector_cls) = vector_cls {
      let metatable = arena.add_type(TableType::table_type_table_state_type_level_scope(
        TableState::Sealed,
        TypeLevel::default(),
        null_mut(),
      ));
      vector_cls.metatable = Some(metatable);

      let number_type = builtin_types_ref.number_type;
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

      let metatable_ty = get_mutable_type::get_mutable::<TableType>(metatable);
      // metatable 刚由 arena.add_type 创建为 TableType，LUAU_ASSERT 必命中
      let metatable_ty =
        metatable_ty.expect("metatable 刚由 arena.add_type 建为 TableType（cpp LUAU_ASSERT）");
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
  let next_args_type_pack =
    arena.add_type_pack_t(TypePack::from_vec(vec![map_of_k_to_v, next_arg_k]));
  let next_ret_k = make_option(builtin_types, arena, generic_k);
  let next_rets_type_pack = arena.add_type_pack_t(TypePack::from_vec(vec![next_ret_k, generic_v]));
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
  let nil_type = builtin_types_ref.nil_type;
  let pairs_return_type_pack = arena.add_type_pack_t(TypePack::from_vec(vec![
    pairs_next,
    map_of_k_to_v,
    nil_type,
  ]));

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
        &builtin_types_ref.type_functions.getmetatable_func,
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
        &builtin_types_ref.type_functions.setmetatable_func,
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
    let args_pack = arena.add_type_pack_t(TypePack::from_vec(vec![tab_ty, generic_mt]));
    let ret_pack = arena.add_type_pack_t(TypePack::single(table_meta_mt));
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

    let falsy_type = builtin_types_ref.falsy_type;
    let negation = arena.add_type(NegationType { ty: falsy_type });
    let refined_ty = arena.add_type(
      TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
        &builtin_types_ref.type_functions.intersect_func,
        vec![generic_t2, negation],
      ),
    );

    let optional_string_type = builtin_types_ref.optional_string_type;
    let args_pack =
      arena.add_type_pack_t(TypePack::from_vec(vec![generic_t2, optional_string_type]));
    let ret_pack = arena.add_type_pack_t(TypePack::single(refined_ty));
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
  let ttv = get_mutable_type::get_mutable::<TableType>(table_binding);
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
  let global_type_function_scope_ptr: *mut Scope = arc_as_mut(&globals.global_type_function_scope);
  // Safety: 指针由 `arc_as_mut` 从 `globals.global_type_function_scope` 的 Arc 导出,
  // 满足其「Arc 存活 + 单线程独占写」契约——该 Arc 被 `globals` 持有直至函数结束;
  // 读侧 `globals.global_scope` 是另一枚 Arc<Scope> 对象,克隆进目标字段的映射为
  // 独立 owned 值,不存在自赋值别名;写窗口仅限本块,块外无人同时借用该 Scope。
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
    let ast_name = globals.global_names.names.get_str(name);
    LUAU_ASSERT!(!ast_name.is_null());

    let symbol = Symbol::from_global(ast_name);
    let binding = globals.global_scope.bindings.get(&symbol).cloned();
    if let Some(binding) = binding {
      // Safety: 目标仍是 `arc_as_mut` 导出的 `global_type_function_scope`——Arc 由
      // `globals` 持有、循环全程存活且单线程独占写；插入的 `binding` 已从
      // `global_scope.bindings`（另一枚 Scope）克隆为 owned 值，写入不与其他
      // 借用该 Scope 的引用重叠，符合 `arc_as_mut` 的裸指针使用契约。
      unsafe {
        (*global_type_function_scope_ptr)
          .bindings
          .insert(symbol, binding);
      }
    }
  }

  let type_function_load_result = {
    let scope = globals.global_type_function_scope.clone();
    frontend.load_definition_file_into_globals(
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
