use alloc::vec::Vec;

use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    interior_free_types::InteriorFreeTypes,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_variant::TypeVariant},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  // ConstraintGenerator::visitFragmentRoot (ConstraintGenerator.cpp).
  pub(crate) fn visit_fragment_root(&mut self, resume_scope: &ScopePtr, block: *mut AstStatBlock) {
    // We prepopulate global data in the resumeScope to avoid writing data into the old modules scopes
    let global_scope = self.global_scope.clone().unwrap();
    unsafe {
      self.prepopulate_global_scope_for_fragment_typecheck(&global_scope, resume_scope, block)
    };
    // Pre
    self.interior_free_types.push(InteriorFreeTypes::default());
    unsafe {
      self.visit_block_without_child_scope(resume_scope.as_ref() as *const _ as *mut _, block)
    };
    // Post
    self.interior_free_types.pop();

    self.fill_in_inferred_bindings(resume_scope, block);

    if !self.logger.is_null() {
      unsafe {
        (*self.logger).capture_generation_module(self.module.clone().unwrap());
      }
    }

    let local_types_pairs: Vec<(TypeId, Vec<TypeId>)> = self
      .local_types
      .iter()
      .map(|(ty, domain)| (*ty, domain.order.clone()))
      .collect();
    for (ty, domain) in local_types_pairs {
      // FIXME: This isn't the most efficient thing.
      let mut domain_ty = unsafe { (*self.builtin_types).never_type };
      for d in domain {
        let d_followed = follow_type_id(d);
        if d_followed == ty {
          continue;
        }
        domain_ty = self.simplify_union(
          resume_scope.clone(),
          resume_scope.as_ref().location,
          domain_ty,
          d_followed,
        );
      }

      LUAU_ASSERT!(get_type_id::<BlockedType>(ty).is_some());
      unsafe {
        (*as_mutable_type_id(ty)).ty = TypeVariant::Bound(domain_ty);
      }
    }
  }
}
