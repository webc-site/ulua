use crate::{
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::{def_id_def::DefId, l_value::LValue, type_id::TypeId},
};

impl ConstraintGenerator {
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
