use alloc::collections::BTreeMap;

use crate::{
  functions::simplify_intersection_simplify::simplify_intersection,
  records::{
    builtin_types::BuiltinTypes, non_strict_context::NonStrictContext, type_arena::TypeArena,
  },
};

pub fn non_strict_context_conjunction(
  builtins: *mut BuiltinTypes,
  arena: *mut TypeArena,
  left: &NonStrictContext,
  right: &NonStrictContext,
) -> NonStrictContext {
  let mut conj = NonStrictContext {
    context: BTreeMap::new(),
  };

  for (&def, &left_ty) in &left.context {
    if let Some(right_ty) = right.find_def(def) {
      let result = simplify_intersection(builtins, arena, left_ty, right_ty);
      conj.context.insert(def, result.result);
    }
  }

  conj
}
