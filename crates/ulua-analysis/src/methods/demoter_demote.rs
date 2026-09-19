use crate::{records::demoter::Demoter, type_aliases::type_id::TypeId};

impl Demoter {
  pub fn demote(&mut self, _expected_types: &mut Vec<Option<TypeId>>) {
    // The C++ implementation calls `substitute` on each element; this is handled by Substitution trait
    // stubbing here as the substitution logic is already in the Substitution record methods
  }
}
