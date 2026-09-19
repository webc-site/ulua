use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    apply_mapped_generics::ApplyMappedGenerics, generic_type::GenericType,
    intersection_builder::IntersectionBuilder, union_builder::UnionBuilder,
  },
  type_aliases::type_id::TypeId,
};

impl ApplyMappedGenerics {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let bounds = unsafe { (*self.env).get_mapped_type_bounds(ty, self.ice_reporter) };
    let lower_bound = &bounds.lower_bound;
    let upper_bound = &bounds.upper_bound;

    if upper_bound.empty() && lower_bound.empty() {
      // No bounds for the generic we're mapping.
      // In this case, unknown vs never is an arbitrary choice:
      // ie, does it matter if we map add<A, A> to add<unknown, unknown> or add<never, never> in the context of subtyping?
      // We choose unknown here, since it's closest to the original behavior.
      unsafe { (*self.builtin_types).unknown_type }
    } else if !upper_bound.empty() {
      let mut ib = IntersectionBuilder::new(self.arena, self.builtin_types);
      for &ub in &upper_bound.order {
        // NOTE: The original implementation skips over generic
        // types, but that seems incorrect to me.
        if get_type_id::<GenericType>(ub).is_none() {
          ib.add(ub);
        }
      }
      ib.build()
    } else if !lower_bound.empty() {
      let mut ub_builder = UnionBuilder::new(self.arena, self.builtin_types);
      for &lb in &lower_bound.order {
        // NOTE: The original implementation skips over generic
        // types, but that seems incorrect to me.
        if get_type_id::<GenericType>(lb).is_none() {
          ub_builder.add(lb);
        }
      }
      ub_builder.build()
    } else {
      LUAU_ASSERT!(false);
      unsafe { (*self.builtin_types).unknown_type }
    }
  }
}
