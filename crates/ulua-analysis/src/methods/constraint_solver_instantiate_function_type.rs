//! `TypeId ConstraintSolver::instantiateFunctionType(TypeId functionTypeId,
//!   const std::vector<TypeId>& type_arguments, const std::vector<TypePackId>& typePackArguments,
//!   NotNull<Scope> scope, const Location& location)`
//! (`Analysis/src/ConstraintSolver.cpp:3193-3267`, hand-ported faithfully).

use core::ptr::null;
use std::ptr::eq;

use ulua_ast::records::location::Location;
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::polarity::Polarity,
  functions::{
    follow_type::follow_type_id, fresh_type::fresh_type, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, shallow_clone_clone_alt_b::shallow_clone,
  },
  records::{
    clone_state::CloneState, constraint_solver::ConstraintSolver, function_type::FunctionType,
    replacer::Replacer, scope::Scope,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl ConstraintSolver {
  pub fn instantiate_function_type(
    &mut self,
    function_type_id: TypeId,
    type_arguments: &[TypeId],
    type_pack_arguments: &[TypePackId],
    scope: *mut Scope,
    _location: &Location,
  ) -> TypeId {
    let function_type_id = follow_type_id(function_type_id);

    // no work to be done if we're not instantiating with anything
    if type_arguments.is_empty() && type_pack_arguments.is_empty() {
      return function_type_id;
    }

    let Some(ft) = get_type_id::<FunctionType>(function_type_id) else {
      return function_type_id;
    };

    let mut replacements: DenseHashMap<TypeId, TypeId> = DenseHashMap::new(null());
    let generics = &ft.generics;
    let mut type_parameters_iter = 0usize;

    for &type_argument in type_arguments.iter() {
      if type_parameters_iter == generics.len() {
        break;
      }

      *replacements.get_or_insert(generics[type_parameters_iter]) = type_argument;
      type_parameters_iter += 1;
    }

    while type_parameters_iter != generics.len() {
      let fresh = fresh_type(
        unsafe { &mut *self.arena },
        unsafe { &*self.builtin_types },
        scope,
        Polarity::Mixed,
      );
      *replacements.get_or_insert(generics[type_parameters_iter]) = fresh;
      type_parameters_iter += 1;
    }

    let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> = DenseHashMap::new(null());
    let generic_packs = &ft.generic_packs;
    for (&type_pack_argument, &generic_pack) in type_pack_arguments.iter().zip(generic_packs.iter())
    {
      *replacement_packs.get_or_insert(generic_pack) = type_pack_argument;
    }

    let mut r = Replacer::new(
      self.arena,
      &mut replacements as *mut DenseHashMap<TypeId, TypeId>,
      &mut replacement_packs as *mut DenseHashMap<TypePackId, TypePackId>,
    );

    let mut cs = CloneState {
      builtin_types: self.builtin_types,
      seen_types: DenseHashMap::new(null()),
      seen_type_packs: DenseHashMap::new(null()),
    };

    // We clone persistent types here to enable instantiation for generic
    // builtins like `table.find`; otherwise, the lines after would
    // immediately corrupt the definitions of the original function.
    let cloned_function_type_id = unsafe {
      shallow_clone(
        function_type_id,
        &mut *self.arena,
        &mut cs,
        /* clonePersistentTypes */ true,
      )
    };
    // C++ 在 assert 后直接解引用 ft2（shallow_clone 一个 FunctionType 必然仍是
    // FunctionType），此处同契约，用 unwrap 表达必命中。
    let ft2 = get_mutable_type_id::<FunctionType>(cloned_function_type_id)
      .expect("shallow_clone 保留了 FunctionType 变体");
    LUAU_ASSERT!(!eq(ft as *const FunctionType, ft2 as *const FunctionType));

    // We instantiate all generics, replacing any with free types.
    ft2.generics.clear();

    // However, we only instantiate as many type pack arguments as are given.
    if !ft2.generic_packs.is_empty() && type_pack_arguments.len() < ft2.generic_packs.len() {
      ft2.generic_packs.drain(0..type_pack_arguments.len());
    } else {
      ft2.generic_packs.clear();
    }

    match r.substitute_type_id(cloned_function_type_id) {
      Some(result) => result,
      None => unsafe { (*self.builtin_types).error_type },
    }
  }
}
