use alloc::{sync::Arc, vec::Vec};

use ulua_ast::records::location::Location;

use crate::{
  functions::{follow_type::follow_type_id, get_mutable_type_pack::get_mutable_type_pack_id},
  records::{
    count_mismatch::CountMismatchContext, module::Module, type_checker::TypeChecker, type_pack,
    type_pack::TypePack,
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
    let expected_type_pack = self.add_type_pack_type_pack(TypePack {
      head: Vec::new(),
      tail: None,
    });
    // 刚以 TypePack 变体分配，下转必然成功（C++ LUAU_ASSERT(expectedPack)）。
    let expected_pack =
      get_mutable_type_pack_id::<type_pack::TypePack>(expected_type_pack).unwrap();
    for _ in 0..expected_length {
      expected_pack
        .head
        .push(self.fresh_type_scope_ptr(scope.clone()));
    }
    let old_errors_size = { self.current_module.as_ref().unwrap().errors.len() };
    self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
      tp,
      expected_type_pack,
      scope,
      location,
      CountMismatchContext::Arg,
    );
    unsafe {
      (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
        .errors
        .truncate(old_errors_size)
    };
    let mut result = expected_pack.head.clone();
    for ty in &mut result {
      *ty = follow_type_id(*ty);
    }
    result
  }
}
