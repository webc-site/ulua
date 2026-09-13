//! C++ `Frontend::parseType` (`Analysis/src/Frontend.cpp:2059-2183`).
use alloc::{rc::Rc, string::String, sync::Arc};
use core::ptr::{NonNull, null_mut};

use ulua_ast::records::{
  allocator::Allocator, ast_expr::AstExpr, ast_name_table::AstNameTable,
  parse_options::ParseOptions, parser::Parser,
};
use ulua_common::{FFlag, FInt, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::{polarity::Polarity, solver_mode::SolverMode},
  methods::constraint_generator_constraint_generator::ConstraintGeneratorArgs,
  records::{
    constraint_generator::ConstraintGenerator,
    constraint_graph::ConstraintGraph,
    data_flow_graph_builder::DataFlowGraphBuilder,
    frontend::Frontend,
    internal_error_reporter::InternalErrorReporter,
    module::Module,
    module_info::ModuleInfo,
    module_resolver::{ModuleResolver, ModuleResolverVtable},
    normalizer::Normalizer,
    scope::Scope,
    type_arena::TypeArena,
    type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::{
    frontend_callbacks::ModuleScopeCallback, module_name_type::ModuleName,
    module_ptr_module::ModulePtr, type_id::TypeId,
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
    let parse_result = Parser::parse_type_c_char_usize_ast_name_table_allocator_parse_options(
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
    unifier_state.counters.recursion_limit = FInt::LuauTypeInferRecursionLimit.get();
    unifier_state.counters.iteration_limit = limits
      .unifier_iteration_limit()
      .unwrap_or_else(|| FInt::LuauTypeInferIterationLimit.get());

    let mut normalizer = Normalizer::new(
      arena as *mut TypeArena,
      self.builtin_types,
      &mut unifier_state as *mut UnifierSharedState,
      SolverMode::New,
      false,
    );

    let mut type_function_runtime =
      TypeFunctionRuntime::new(ice_handler, &limits, self.globals.global_scope.clone());
    type_function_runtime.allow_evaluation = true;

    let mut module_resolver = null_module_resolver();

    let mut dfg = DataFlowGraphBuilder::empty();

    let mut cgraph_storage = if FFlag::LuauConstraintGraph.get() {
      Some(ConstraintGraph {
        builtin_types: NonNull::new(self.builtin_types).expect("builtinTypes must not be null"),
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
      normalizer: NonNull::new(&mut normalizer).unwrap(),
      type_function_runtime: NonNull::new(&mut type_function_runtime).unwrap(),
      module_resolver: NonNull::new(&mut module_resolver).unwrap(),
      builtin_types: NonNull::new(self.builtin_types).expect("builtinTypes must not be null"),
      ice: NonNull::new(ice_handler).expect("iceHandler must not be null"),
      global_scope: self.globals.global_scope.clone(),
      type_function_scope: self.globals.global_scope.clone(),
      prepare_module_scope,
      logger: null_mut(),
      dfg: NonNull::new(&mut dfg).unwrap(),
      require_cycles: &[],
      cgraph,
    });

    let ty = cg.resolve_type(
      Arc::as_ptr(&self.globals.global_scope) as *mut Scope,
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

fn null_module_resolver() -> ModuleResolver {
  ModuleResolver {
    vtable: ModuleResolverVtable {
      resolve_module_info: null_resolve_module_info,
      get_module: null_get_module,
      module_exists: null_module_exists,
      get_human_readable_module_name: null_get_human_readable_module_name,
    },
  }
}

unsafe fn null_resolve_module_info(
  _this: *mut ModuleResolver,
  _current_module_name: &ModuleName,
  _path_expr: *const AstExpr,
) -> Option<ModuleInfo> {
  None
}

unsafe fn null_get_module(
  _this: *const ModuleResolver,
  _module_name: &ModuleName,
) -> Option<ModulePtr> {
  None
}

unsafe fn null_module_exists(_this: *const ModuleResolver, _module_name: &ModuleName) -> bool {
  false
}

unsafe fn null_get_human_readable_module_name(
  _this: *const ModuleResolver,
  module_name: &ModuleName,
) -> String {
  module_name.clone()
}
