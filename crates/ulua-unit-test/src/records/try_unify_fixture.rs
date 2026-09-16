//! Ported from `tests/TypeInfer.tryUnify.test.cpp`.

use alloc::{boxed::Box, vec::Vec};
use core::ptr::null;

use ulua_analysis::{
  enums::{solver_mode::SolverMode, table_state::TableState, variance::Variance},
  functions::get_type_alt_j::get_type_id,
  records::{
    builtin_types::BuiltinTypes, count_mismatch::CountMismatchContext,
    free_type_pack::FreeTypePack, function_type::FunctionType,
    internal_error_reporter::InternalErrorReporter, normalizer::Normalizer,
    property_type::Property, scope::Scope, table_type::TableType, txn_log::TxnLog,
    type_arena::TypeArena, unifier::Unifier, unifier_shared_state::UnifierSharedState,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{props_type::Props, type_id::TypeId, type_pack_id::TypePackId},
};
use ulua_ast::records::location::Location;

use crate::records::fixture::Fixture;
#[derive(Debug)]
pub struct TryUnifyFixture {
  pub state: Unifier,
  pub normalizer: Box<Normalizer>,
  pub unifier_state: Box<UnifierSharedState>,
  pub ice_handler: Box<InternalErrorReporter>,
  pub global_scope: Box<Scope>,
  pub arena: Box<TypeArena>,
  // Boxed (like the other members above) so its contents — in particular the
  // Frontend's inline `builtin_types_` — keep a stable heap address. `state`
  // (the Unifier) and `normalizer` cache a `*mut BuiltinTypes` into it at
  // construction; with `base` inline, returning the fixture by value relocated
  // `builtin_types_` and dangled those pointers, so every unification that read
  // `builtin_types.any_type`/`.never_type` did an invalid read — benign on Linux
  // by layout luck, a SIGSEGV on macOS/Windows
  // (type_infer_try_unify_cli_50320_follow_in_any_unification, found via valgrind).
  pub base: Box<Fixture>,
}

impl TryUnifyFixture {
  pub fn new() -> Self {
    let mut base = Box::new(Fixture::fixture_bool(false));
    let builtin_types = base.get_builtins() as *mut BuiltinTypes;

    let mut arena = Box::new(TypeArena::default());
    let return_pack = arena.add_type_pack_vector_type_id_optional_type_pack_id(vec![null()], None);
    let mut global_scope = Box::new(Scope::scope_type_pack_id(return_pack));
    let mut ice_handler = Box::new(InternalErrorReporter::default());
    let mut unifier_state = Box::new(UnifierSharedState::new(
      &mut *ice_handler as *mut InternalErrorReporter,
    ));
    let mut normalizer = Box::new(Normalizer::new(
      &mut *arena as *mut TypeArena,
      builtin_types,
      &mut *unifier_state as *mut UnifierSharedState,
      SolverMode::Old,
      false,
    ));

    let state = Unifier {
      types: &mut *arena as *mut TypeArena,
      builtin_types,
      normalizer: &mut *normalizer as *mut Normalizer,
      scope: &mut *global_scope as *mut Scope,
      log: TxnLog::new(),
      failure: false,
      errors: Vec::new(),
      location: Location::default(),
      variance: Variance::Covariant,
      normalize: true,
      check_inhabited: true,
      ctx: CountMismatchContext::Arg,
      shared_state: &mut *unifier_state as *mut UnifierSharedState,
      blocked_types: Vec::new(),
      blocked_type_packs: Vec::new(),
      first_pack_error_pos: None,
    };

    Self {
      state,
      normalizer,
      unifier_state,
      ice_handler,
      global_scope,
      arena,
      base,
    }
  }

  pub fn get_builtins(&mut self) -> &mut BuiltinTypes {
    self.base.get_builtins()
  }

  pub fn fresh_type(&mut self) -> TypeId {
    let builtins = unsafe { &*self.state.builtin_types };
    self
      .arena
      .fresh_type_not_null_builtin_types_type_level(builtins, self.global_scope.level)
  }

  pub fn type_pack(&mut self, head: Vec<TypeId>) -> TypePackId {
    self
      .arena
      .add_type_pack_vector_type_id_optional_type_pack_id(head, None)
  }

  pub fn type_pack_with_tail(&mut self, head: Vec<TypeId>, tail: TypePackId) -> TypePackId {
    self
      .arena
      .add_type_pack_vector_type_id_optional_type_pack_id(head, Some(tail))
  }

  pub fn free_type_pack(&mut self) -> TypePackId {
    self
      .arena
      .add_type_pack_t(FreeTypePack::new(self.global_scope.level))
  }

  pub fn variadic_type_pack(&mut self, ty: TypeId) -> TypePackId {
    self.arena.add_type_pack_t(VariadicTypePack::new(ty))
  }

  pub fn function_type(&mut self, args: Vec<TypeId>, rets: Vec<TypeId>) -> TypeId {
    let arg_pack = self.type_pack(args);
    let ret_pack = self.type_pack(rets);
    self.arena.add_type(FunctionType::function_type_new(
      arg_pack, ret_pack, None, false,
    ))
  }

  pub fn unsealed_table_type(&mut self, props: &[(&str, TypeId)]) -> TypeId {
    let mut mapped = Props::new();

    for (name, ty) in props {
      mapped.insert((*name).to_string(), Property::rw_type_id(*ty));
    }

    self.arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &mapped,
        None,
        self.global_scope.level,
        TableState::Unsealed,
      ),
    )
  }

  pub fn table_prop_type(&self, table: TypeId, name: &str) -> TypeId {
    let table = get_type_id::<TableType>(table).expect("expected TableType");
    table
      .props
      .get(name)
      .expect("expected table prop")
      .type_deprecated()
  }
}

impl Default for TryUnifyFixture {
  fn default() -> Self {
    Self::new()
  }
}
