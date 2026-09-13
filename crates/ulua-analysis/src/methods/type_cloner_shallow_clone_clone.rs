use core::ptr::null_mut;

use crate::{
  enums::follow_option::FollowOption,
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type_alt_c::follow_type_id_follow_option,
    get_mutable_type::get_mutable_type_id,
  },
  records::{
    free_type::FreeType, generic_type::GenericType, table_type::TableType, r#type::Type,
    type_cloner::TypeCloner,
  },
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack},
};
impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
    // We want to [`Luau::follow`] but without forcing the expansion of [`LazyType`]s.
    let ty = unsafe { follow_type_id_follow_option(ty, FollowOption::DisableLazyTypeThunks) };

    if let Some(clone) = self.find_type_id(ty) {
      return clone;
    } else if unsafe { (*ty).persistent } && ty != self.force_ty {
      return ty;
    }

    let target = unsafe { (*self.arena).add_type(Type::new((*ty).ty.clone())) };
    unsafe {
      (*as_mutable_type_id(target)).documentation_symbol = (*ty).documentation_symbol.clone();
    }

    // `replacement_for_null_scope` is null for ordinary clones (Clone.cpp:171-176,
    // free/table scope -> null) and carries the fragment cloner's fresh scope for
    // the `FragmentAutocompleteTypeCloner` override (Clone.cpp:508-513). Generic
    // types always get a null scope in both paths.
    if let Some(generic) = get_mutable_type_id::<GenericType>(target) {
      generic.scope = null_mut();
    } else if let Some(free) = get_mutable_type_id::<FreeType>(target) {
      free.scope = self.replacement_for_null_scope;
    } else if let Some(table) = get_mutable_type_id::<TableType>(target) {
      table.scope = self.replacement_for_null_scope;
    }

    unsafe { (*self.types).insert(ty, target) };
    self.queue.push(TypeOrPack::V0(target));
    target
  }
}
