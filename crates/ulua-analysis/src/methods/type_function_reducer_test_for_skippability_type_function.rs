use core::ffi::c_void;

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  enums::{
    skip_test_result::SkipTestResult, type_function_instance_state::TypeFunctionInstanceState,
  },
  functions::{follow_type::follow, get_type_alt_j::get_type_id},
  records::{
    generic_type::GenericType, intersection_type::IntersectionType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::type_id::TypeId,
};
impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:258 testForSkippability(TypeId)`。
  pub fn test_for_skippability_type_id(&mut self, ty: TypeId) -> SkipTestResult {
    let mut queue: VecDeque<TypeId> = VecDeque::new();
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(TypeId::default());

    queue.push_back(follow(ty));

    while !queue.empty() {
      let t = *queue.front();
      queue.pop_front();

      if seen.contains(&t) {
        continue;
      }

      if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(t) {
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

        if !self.irreducible.contains(&(t as *const c_void)) {
          return SkipTestResult::Defer;
        }

        return SkipTestResult::Irreducible;
      } else if get_type_id::<GenericType>(t).is_some() {
        return SkipTestResult::Generic;
      } else if let Some(it) = get_type_id::<IntersectionType>(t) {
        for part in &it.parts {
          queue.push_back(follow(*part));
        }
      }

      seen.insert(t);
    }

    SkipTestResult::Okay
  }
}
