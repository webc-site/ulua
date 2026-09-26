use alloc::{
  boxed::Box,
  vec::{Vec, Vec as AllocVec},
};
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::variance::Variance,
  functions::arc_as_mut::arc_as_mut,
  records::{
    arena_handle::Handle,
    count_mismatch::CountMismatchContext,
    scope::Scope,
    txn_log::{SeenStorage, TxnLog, TypeOrPackId},
    type_checker::TypeChecker,
    unifier::Unifier,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn mk_unifier(&mut self, scope: &ScopePtr, location: &Location) -> Unifier {
    // C++ `Unifier::tryUnify` (the public entry) resets `iterationCount = 0`
    // at the start of every top-level unification (Unifier.cpp:385/1396).
    // `iteration_count` lives in the TypeChecker-wide `unifier_state.counters`
    // shared across every unify; child unifiers (`make_child_unifier`) share it
    // so a single top-level unify accumulates correctly, but a NEW top-level
    // unify must start from zero. `mk_unifier` is the one boundary where a
    // fresh top-level Unifier is created (children bypass it), so resetting
    // here — rather than in each of the several `TypeChecker::{unify, try_unify,
    // ...}` wrappers that call the recursive `tryUnify_` directly — gives every
    // top-level unify a fresh budget, matching C++. Without it the counter
    // leaked across the whole module check and spuriously tripped
    // LuauTypeInferIterationLimit (luau_subtyping_is_np_hard).
    self.unifier_state.counters.iteration_count = 0;

    let module = arc_as_mut(self.expect_current_module());
    let types = unsafe { &mut (*module).internal_types as *mut _ };
    self.normalizer.arena = Handle::from_opt_ptr(types);
    self.normalizer.shared_state = Some(Handle::from_mut(&mut self.unifier_state));

    let scope_ptr: *mut Scope = arc_as_mut(scope);
    // Own the seen set in a boxed Vec freed when this Unifier's log drops,
    // instead of leaking it via `Box::into_raw` on every top-level unify (the
    // leak the fuzz suite's LeakSanitizer flagged here).
    let mut seen_box = Box::new(SeenStorage(Vec::new()));
    let shared_seen: *mut Vec<(TypeOrPackId, TypeOrPackId)> = &mut seen_box.0;

    Unifier {
      types: Handle::from_ptr(types),
      builtin_types: self.builtin_types,
      normalizer: Handle::from_mut(&mut self.normalizer),
      scope: Handle::from_ptr(scope_ptr),
      log: TxnLog {
        type_var_changes: DenseHashMap::default(),
        type_pack_changes: DenseHashMap::default(),
        parent: null_mut(),
        owned_seen: Vec::new(),
        shared_seen,
        owned_seen_box: Some(seen_box),
        radioactive: false,
      },
      failure: false,
      errors: Vec::new(),
      location: *location,
      variance: Variance::Covariant,
      normalize: true,
      check_inhabited: true,
      ctx: CountMismatchContext::Arg,
      shared_state: Handle::from_mut(&mut self.unifier_state),
      blocked_types: AllocVec::new(),
      blocked_type_packs: AllocVec::new(),
      first_pack_error_pos: None,
    }
  }
}
