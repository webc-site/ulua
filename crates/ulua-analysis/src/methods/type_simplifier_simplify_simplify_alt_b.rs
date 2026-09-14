use core::mem::zeroed;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, negation_type::NegationType, never_type::NeverType,
    recursion_limiter::RecursionLimiter, table_type::TableType, type_simplifier::TypeSimplifier,
    union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};
impl TypeSimplifier {
  pub fn simplify_type_id_dense_hash_set_type_id(
    &mut self,
    ty: TypeId,
    seen: &mut DenseHashSet<TypeId>,
  ) -> TypeId {
    let mut rl = RecursionLimiter {
      base: unsafe { zeroed() },
      native_stack_guard: unsafe { zeroed() },
    };
    rl.recursion_limiter_recursion_limiter(
      "TypeSimplifier::simplify",
      &mut self.recursion_depth,
      60,
    );

    let ty = follow_type_id(ty);
    if seen.contains(&ty) {
      return ty;
    }
    seen.insert(ty);

    if let Some(nt) = get_type_id::<NegationType>(ty) {
      let negated_ty = follow_type_id(nt.ty);
      let bt = unsafe { &*self.builtin_types };
      if get_type_id::<AnyType>(negated_ty).is_some() {
        let arena = unsafe { &mut *self.arena.cast_mut() };
        return arena.add_type(UnionType {
          options: alloc::vec![bt.never_type, bt.error_type],
        });
      } else if get_type_id::<UnknownType>(negated_ty).is_some() {
        return bt.never_type;
      } else if get_type_id::<NeverType>(negated_ty).is_some() {
        return bt.unknown_type;
      }
      if let Some(nnt) = get_type_id::<NegationType>(negated_ty) {
        return self.simplify_type_id_dense_hash_set_type_id(nnt.ty, seen);
      }
    }

    if let Some(tt) = get_type_id::<TableType>(ty)
      && tt.props.len() == 1
      && let Some(read_ty) = tt.props.values().next().unwrap().read_ty
    {
      let prop_ty = self.simplify_type_id_dense_hash_set_type_id(read_ty, seen);
      if get_type_id::<NeverType>(prop_ty).is_some() {
        let bt = unsafe { &*self.builtin_types };
        return bt.never_type;
      }
    }
    ty
  }
}
