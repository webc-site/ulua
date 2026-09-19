//! Source: `Analysis/include/Luau/Module.h`

use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_ast::{
  enums::mode::Mode,
  records::{
    allocator::Allocator, ast_expr::AstExpr, ast_name_table::AstNameTable, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_type::AstType, ast_type_pack::AstTypePack,
    location::Location,
  },
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::type_file_resolver::Type,
  records::{
    def_arena::DefArena, lint_result::LintResult, refinement_key_arena::RefinementKeyArena,
    scope::Scope, r#type, type_arena::TypeArena, type_fun::TypeFun,
  },
  type_aliases::{
    collections::{HashMap, HashMapExt},
    error_vec::ErrorVec,
    module_name_type::ModuleName,
    name_type::Name,
    scope_ptr_type::ScopePtr,
    type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
#[derive(Debug)]
pub struct Module {
  pub checked_in_new_solver: bool,

  pub name: ModuleName,
  pub human_readable_name: String,

  pub interface_types: TypeArena,
  pub internal_types: TypeArena,

  pub allocator: Option<Arc<Allocator>>,
  pub names: Option<Arc<AstNameTable>>,
  pub root: *mut AstStatBlock,

  pub scopes: Vec<(Location, ScopePtr)>,

  pub ast_types: DenseHashMap<*const AstExpr, TypeId>,
  pub ast_type_packs: DenseHashMap<*const AstExpr, TypePackId>,
  pub ast_expected_types: DenseHashMap<*const AstExpr, TypeId>,

  pub ast_original_call_types: DenseHashMap<*const AstNode, TypeId>,
  pub ast_overload_resolved_types: DenseHashMap<*const AstNode, TypeId>,
  pub ast_for_in_next_types: DenseHashMap<*const AstNode, TypeId>,

  pub ast_resolved_types: DenseHashMap<*const AstType, TypeId>,
  pub ast_resolved_type_packs: DenseHashMap<*const AstTypePack, TypePackId>,

  pub ast_compound_assign_result_types: DenseHashMap<*const AstStat, TypeId>,

  pub upper_bound_contributors: DenseHashMap<TypeId, Vec<(Location, TypeId)>>,

  pub ast_scopes: DenseHashMap<*const AstNode, *mut Scope>,

  pub type_function_aliases: Vec<Box<TypeFun>>,

  pub declared_globals: HashMap<Name, TypeId>,
  pub errors: ErrorVec,
  pub lint_result: LintResult,
  pub mode: Mode,
  pub r#type: Type,
  pub check_duration_sec: f64,
  pub timeout: bool,
  pub cancelled: bool,

  pub return_type: TypePackId,
  pub exported_type_bindings: HashMap<Name, TypeFun>,

  pub def_arena: DefArena,
  pub key_arena: RefinementKeyArena,

  pub constraint_generation_did_not_complete: bool,
}

impl Default for Module {
  fn default() -> Self {
    // Faithful port of the C++ `Module` default ctor (`Module.h:76`), which
    // default-initializes every member via its in-class initializer.
    Self {
      checked_in_new_solver: false,

      name: ModuleName::new(),
      human_readable_name: String::new(),

      interface_types: TypeArena::default(),
      internal_types: TypeArena::default(),

      allocator: None,
      names: None,
      root: null_mut(),

      scopes: Vec::new(),

      // `DenseHashMap<...>{nullptr}` — pointer-keyed maps use the null
      // sentinel as their empty key.
      ast_types: DenseHashMap::new(null::<AstExpr>()),
      ast_type_packs: DenseHashMap::new(null::<AstExpr>()),
      ast_expected_types: DenseHashMap::new(null::<AstExpr>()),

      ast_original_call_types: DenseHashMap::new(null::<AstNode>()),
      ast_overload_resolved_types: DenseHashMap::new(null::<AstNode>()),
      ast_for_in_next_types: DenseHashMap::new(null::<AstNode>()),

      ast_resolved_types: DenseHashMap::new(null::<AstType>()),
      ast_resolved_type_packs: DenseHashMap::new(null::<AstTypePack>()),

      ast_compound_assign_result_types: DenseHashMap::new(null::<AstStat>()),

      upper_bound_contributors: DenseHashMap::new(null::<r#type::Type>()),

      ast_scopes: DenseHashMap::new(null::<AstNode>()),

      type_function_aliases: Vec::new(),

      declared_globals: HashMap::new(),
      errors: ErrorVec::new(),
      lint_result: LintResult::default(),
      // C++ `Mode mode;` has no in-class initializer; the first enumerator
      // (`NoCheck`) is the deterministic zero-init value.
      mode: Mode::NoCheck,
      // C++ `SourceCode::Type type;` — first enumerator is `None`.
      r#type: Type::None,
      check_duration_sec: 0.0,
      timeout: false,
      cancelled: false,

      // `TypePackId returnType = nullptr;` — TypePackId is `*const TypePackVar`.
      return_type: null(),
      exported_type_bindings: HashMap::new(),

      def_arena: DefArena::default(),
      key_arena: RefinementKeyArena::default(),

      constraint_generation_did_not_complete: true,
    }
  }
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}
