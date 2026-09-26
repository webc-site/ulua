use alloc::vec::Vec;
use core::iter::repeat_with;

use ulua_ast::records::location::Location;

use crate::{
  functions::{arc_as_mut::arc_as_mut, follow_type, get_mutable_type_pack},
  records::{
    count_mismatch::CountMismatchContext, type_checker::TypeChecker, type_pack, type_pack::TypePack,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};
impl TypeChecker {
  pub fn un_type_pack(
    &mut self,
    scope: &ScopePtr,
    tp: TypePackId,
    expected_length: usize,
    location: &Location,
  ) -> Vec<TypeId> {
    let expected_type_pack = self.add_type_pack_type_pack(TypePack::empty());
    // 刚以 TypePack 变体分配，下转必然成功（C++ LUAU_ASSERT(expectedPack)）。
    let expected_pack =
      get_mutable_type_pack::get_mutable::<type_pack::TypePack>(expected_type_pack)
        .expect("刚以 TypePack 变体分配，下转必命中（C++ LUAU_ASSERT(expectedPack)）");
    expected_pack
      .head
      .extend(repeat_with(|| self.fresh_type_scope_ptr(scope.clone())).take(expected_length));
    let old_errors_size = self.expect_current_module().errors.len();
    self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
      tp,
      expected_type_pack,
      scope,
      location,
      CountMismatchContext::Arg,
    );
    unsafe {
      (*(arc_as_mut(self.expect_current_module())))
        .errors
        .truncate(old_errors_size)
    };
    let mut result = expected_pack.head.clone();
    for ty in &mut result {
      *ty = follow_type::follow(*ty);
    }
    result
  }
}
