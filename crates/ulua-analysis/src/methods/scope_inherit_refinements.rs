use crate::{
  functions::get_base_symbol::get_base_symbol,
  records::{scope::Scope, symbol::Symbol},
  type_aliases::scope_ptr_type::ScopePtr,
};

impl Scope {
  // Updates the `this` scope with the refinements from the `childScope`
  // excluding ones that don't exist in `this` (Scope.cpp:226).
  pub fn inherit_refinements(&mut self, child_scope: &ScopePtr) {
    for (k, a) in { &child_scope.rvalue_refinements }.iter() {
      if self.lookup_def_id(*k).is_some() {
        *self.rvalue_refinements.get_or_insert(*k) = *a;
      }
    }

    for (k, a) in { &child_scope.refinements } {
      let symbol: Symbol = get_base_symbol(k);
      if self.lookup_symbol(symbol).is_some() {
        self.refinements.insert(k.clone(), *a);
      }
    }
  }
}
