use crate::{
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::{def_id_def::DefId, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn update_r_value_refinements_scope_ptr_def_id_type_id(
    &self,
    scope: &ScopePtr,
    def: DefId,
    ty: TypeId,
  ) {
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
    unsafe { self.update_r_value_refinements_scope_def_id_type_id(scope_raw, def, ty) };
  }
}
