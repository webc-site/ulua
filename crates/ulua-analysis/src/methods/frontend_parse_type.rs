//! C++ `Frontend::parseType` (`Analysis/src/Frontend.cpp:2059-2183`).
use alloc::{rc::Rc, sync::Arc};
use core::ptr::null_mut;

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_common::{fflag, fint, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::{polarity::Polarity, solver_mode::SolverMode},
  functions::arc_as_mut::arc_as_mut,
  methods::constraint_generator_constraint_generator::ConstraintGeneratorArgs,
  records::{
    arena_handle::Handle, constraint_generator::ConstraintGenerator,
    constraint_graph::ConstraintGraph, data_flow_graph_builder::DataFlowGraphBuilder,
    frontend::Frontend, internal_error_reporter::InternalErrorReporter, module::Module,
    module_resolver::ModuleResolverRef, normalizer::Normalizer,
    null_module_resolver::NullModuleResolver, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::{
    frontend_callbacks::ModuleScopeCallback, module_ptr_module::ModulePtr, type_id::TypeId,
  },
};
impl Frontend {
  pub fn parse_type(
    &mut self,
    allocator: &mut Allocator,
    name_table: &mut AstNameTable,
    ice_handler: &mut InternalErrorReporter,
    limits: TypeCheckLimits,
    arena: &mut TypeArena,
    source: &str,
  ) -> TypeId {
    let parse_result = Parser::parse_type_source(
      source,
      name_table,
      allocator,
      ParseOptions::default(),
    );

    if parse_result.root.is_null() {
      ice_handler.ice_string("Frontend::parseType was given an unparseable type");
    }

    if let Some(error) = parse_result.errors.first() {
      ice_handler.ice_string(&alloc::format!(
        "Frontend::parseType error: {}",
        error.get_message()
      ));
    }

    let module: ModulePtr = Arc::new(Module::default());

    let mut unifier_state = UnifierSharedState::new(ice_handler);
    unifier_state.counters.recursion_limit = fint::LuauTypeInferRecursionLimit.get();
    unifier_state.counters.iteration_limit = limits
      .unifier_iteration_limit()
      .unwrap_or_else(|| fint::LuauTypeInferIterationLimit.get());

    let mut normalizer = Normalizer::new(
      Some(Handle::from_mut(arena)),
      self.builtin_types_handle(),
      Some(Handle::from_mut(&mut unifier_state)),
      SolverMode::New,
      false,
    );

    let mut type_function_runtime =
      TypeFunctionRuntime::new(ice_handler, &limits, self.globals.global_scope.clone());
    type_function_runtime.allow_evaluation = true;

    let mut module_resolver = NullModuleResolver::new();
    // 具体解析器 → `ModuleResolverRef` 具体分派句柄（由 `&mut` 构造，薄指针 + match 分派）
    let resolver = ModuleResolverRef::from(&mut module_resolver);

    let mut dfg = DataFlowGraphBuilder::empty();

    let mut cgraph_storage = if fflag::LuauConstraintGraph.get() {
      Some(ConstraintGraph {
        dependencies: DenseHashMap::new(Default::default()),
        reverse_dependencies: DenseHashMap::new(Default::default()),
        constraint_lists: Default::default(),
      })
    } else {
      None
    };
    let cgraph = cgraph_storage
      .as_mut()
      .map(|cgraph| cgraph as *mut ConstraintGraph)
      .unwrap_or(null_mut());

    let prepare_module_scope: ModuleScopeCallback = Rc::new(|_, _| {});

    let mut cg = ConstraintGenerator::new(ConstraintGeneratorArgs {
      module,
      normalizer: Handle::from_mut(&mut normalizer),
      type_function_runtime: Handle::from_mut(&mut type_function_runtime),
      // 具体解析器 → `ModuleResolverRef` 分派句柄，构造器内按值转存。
      module_resolver: resolver,
      builtin_types: self.builtin_types_handle(),
      ice: Handle::from_mut(&mut *ice_handler),
      global_scope: self.globals.global_scope.clone(),
      type_function_scope: self.globals.global_scope.clone(),
      prepare_module_scope,
      logger: None,
      dfg: Handle::from_mut(&mut dfg),
      require_cycles: &[],
      cgraph,
    });

    let ty = cg.resolve_type(
      arc_as_mut(&self.globals.global_scope),
      parse_result.root,
      false,
      false,
      Polarity::Positive,
    );

    if !cg.constraints.is_empty() {
      ice_handler.ice_string("Not yet implemented: parseType cannot reduce other type aliases");
    }

    ty
  }
}
