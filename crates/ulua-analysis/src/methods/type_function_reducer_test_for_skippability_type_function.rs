use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  enums::{
    skip_test_result::SkipTestResult, type_function_instance_state::TypeFunctionInstanceState,
  },
  functions::{follow_type::follow, follow_type_pack, get_type, get_type_pack},
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:258 testForSkippability(TypeId)`。
  pub fn test_for_skippability_type_id(&mut self, ty: TypeId) -> SkipTestResult {
    let mut queue: VecDeque<TypeId> = VecDeque::new();
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();

    queue.push_back(follow(ty));

    while !queue.empty() {
      let t = *queue.front();
      queue.pop_front();

      if seen.contains(&t) {
        continue;
      }

      if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(t) {
        let state = tfit.state;
        if state == TypeFunctionInstanceState::Stuck {
          return SkipTestResult::Stuck;
        } else if state == TypeFunctionInstanceState::Solved {
          return SkipTestResult::Generic;
        }

        for cyclic_ty in &self.cyclic_type_functions {
          if t == *cyclic_ty {
            return SkipTestResult::CyclicTypeFunction;
          }
        }

        if !self.irreducible.contains(&(t as *const ())) {
          return SkipTestResult::Defer;
        }

        return SkipTestResult::Irreducible;
      } else if get_type::get::<GenericType>(t).is_some() {
        return SkipTestResult::Generic;
      } else if let Some(it) = get_type::get::<IntersectionType>(t) {
        for part in &it.parts {
          queue.push_back(follow(*part));
        }
      }

      seen.insert(t);
    }

    SkipTestResult::Okay
  }

  pub(crate) fn test_for_skippability_type_pack_id(&self, ty: TypePackId) -> SkipTestResult {
    let ty = follow_type_pack::follow(ty);

    if get_type_pack::get::<TypeFunctionInstanceTypePack>(ty).is_some() {
      if !self.irreducible.contains(&(ty as *const ())) {
        return SkipTestResult::Defer;
      } else {
        return SkipTestResult::Irreducible;
      }
    } else if get_type_pack::get::<GenericTypePack>(ty).is_some() {
      return SkipTestResult::Generic;
    }

    SkipTestResult::Okay
  }
}
