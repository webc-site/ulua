use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::{def_id_def::DefId, l_value::LValue, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  pub(crate) fn update_r_value_refinements_scope_ptr_def_id_type_id(
    &self,
    scope: &ScopePtr,
    def: DefId,
    ty: TypeId,
  ) {
    let scope_raw = arc_as_mut(scope);
    // Safety: `scope_raw` 由 `arc_as_mut` 自调用方借用的 `&ScopePtr`（Arc<Scope>）
    // 派生，该 Arc 在整条语句内存活且指针对应 Scope 全程有效；被调方契约「scope
    // 为存活、本线程可独占写的 Scope」由此满足，且调用点单线程、无其它借用并存。
    unsafe { self.update_r_value_refinements_scope_def_id_type_id(scope_raw, def, ty) };
  }

  // ConstraintGenerator::updateRValueRefinements(Scope*, DefId, TypeId) const
  // (ConstraintGenerator.cpp:5139).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn update_r_value_refinements_scope_def_id_type_id(
    &self,
    scope: *mut Scope,
    def: DefId,
    ty: TypeId,
  ) {
    // Safety: `scope` 依本 `unsafe fn` 契约与全部调用点（Arc 持有的 root/child
    // Scope 句柄或 `arc_as_mut` 派生指针）非空且指向本次 check 期间存活的 Scope；
    // 两处写入分别落 `rvalue_refinements`/`refinements` 两张表，单线程独占写、
    // 借用窗口止于本函数，`dfg` 经 `Option::as_ref` 判空后只读查 symbol。
    unsafe {
      *(*scope).rvalue_refinements.get_or_insert(def) = ty;

      // C++ (ConstraintGenerator.cpp:5142-5143):
      //     if (auto sym = dfg->getSymbolFromDef(def))
      //         scope->refinements[*sym] = ty;
      // Only locals/globals are mapped in `defToSymbol`; for any other def
      // (e.g. a property-access def from `checkIndexName`) `getSymbolFromDef`
      // returns nullopt and the refinement write is skipped. Do NOT fall back
      // to `def->name` — that would associate an index result with the base
      // symbol's `refinements` entry, which later corrupts the fragment clone.
      if let Some(sym) = self
        .dfg
        .as_ref()
        .and_then(|dfg| dfg.get_symbol_from_def(def))
      {
        (*scope).refinements.insert(LValue::Symbol(sym), ty);
      }
    }
  }
}
