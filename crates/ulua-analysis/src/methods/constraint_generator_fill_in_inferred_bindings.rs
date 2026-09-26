use alloc::vec::Vec;

use ulua_ast::records::{ast_stat_block::AstStatBlock, location::Location};

use super::constraint_generator_prototype_type_definitions::make_binding;
use crate::{
  records::{constraint_generator::ConstraintGenerator, scope::Scope, symbol::Symbol},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  pub fn fill_in_inferred_bindings(&mut self, _global_scope: &ScopePtr, _block: *mut AstStatBlock) {
    let inferred_bindings: Vec<(Symbol, *mut Scope, Location, Vec<TypeId>)> = self
      .inferred_bindings
      .iter()
      .map(|(symbol, p)| (symbol.clone(), p.scope, p.location, p.types.order.clone()))
      .collect();

    for (symbol, scope, location, tys) in inferred_bindings {
      let ty = match tys.as_slice() {
        [only] => *only,
        _ => self.make_union_vector_type_id(tys),
      };

      unsafe {
        (*scope).bindings.insert(symbol, make_binding(ty, location));
      }
    }
  }
}
