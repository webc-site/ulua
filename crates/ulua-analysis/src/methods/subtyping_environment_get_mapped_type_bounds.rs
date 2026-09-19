use ulua_common::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow,
  methods::subtyping_bind_generic::dense_hash_map_find_mut_no_default,
  records::{
    generic_bounds::GenericBounds, internal_error_reporter::InternalErrorReporter,
    subtyping_environment::SubtypingEnvironment,
  },
  type_aliases::type_id::TypeId,
};
impl SubtypingEnvironment {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_mapped_type_bounds(
    &mut self,
    ty: TypeId,
    ice_reporter: *mut InternalErrorReporter,
  ) -> &mut GenericBounds {
    let ty = follow(ty);
    if let Some(bounds) = dense_hash_map_find_mut_no_default(&mut self.mapped_generics, &ty)
      && !bounds.is_empty()
    {
      return bounds.last_mut().unwrap();
    }

    if !self.parent.is_null() {
      return unsafe { (*self.parent).get_mapped_type_bounds(ty, ice_reporter) };
    }

    LUAU_ASSERT!(false);
    unsafe {
      (*ice_reporter).ice_string("Trying to access bounds for a type with no in-scope bounds");
    }
    unreachable!()
  }
}
