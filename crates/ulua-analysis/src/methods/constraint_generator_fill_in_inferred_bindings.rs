//! @interface-stub
use alloc::{string::String, vec::Vec};

use ulua_ast::records::{ast_stat_block::AstStatBlock, location::Location};

use crate::{
  records::{
    binding::Binding, constraint_generator::ConstraintGenerator, scope::Scope, symbol::Symbol,
  },
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
      let ty = if tys.len() == 1 {
        tys[0]
      } else {
        self.make_union_vector_type_id(tys)
      };

      unsafe {
        (*scope).bindings.insert(
          symbol,
          Binding {
            type_id: ty,
            location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }
    }
  }
}
