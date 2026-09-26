use crate::{
  enums::polarity::Polarity,
  functions::{arc_as_mut::arc_as_mut, fresh_type::fresh_type},
  records::constraint_generator::ConstraintGenerator,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  pub fn fresh_type(&mut self, scope: &ScopePtr, polarity: Polarity) -> TypeId {
    let ft = {
      fresh_type(
        self.arena.get_mut(),
        self.builtin_types.get(),
        arc_as_mut(scope),
        polarity,
      )
    };

    if let Some(interior_free_types) = self.interior_free_types.last_mut() {
      interior_free_types.types.push(ft);
    }

    self.free_types.insert_type_id(ft);
    ft
  }
}
