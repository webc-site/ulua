use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::records::{
  ast_array::AstArray, ast_local::AstLocal, binding::Binding, location::Location, name::Name,
  parser::Parser, position::Position, temp_vector::TempVector,
};

impl Parser {
  pub fn prepare_function_arguments(
    &mut self,
    start: &Location,
    hasself: bool,
    args: &TempVector<'_, Binding>,
  ) -> (*mut AstLocal, AstArray<*mut AstLocal>) {
    let mut self_local: *mut AstLocal = null_mut();

    if hasself {
      // C++: push_local(Binding(Name(name_self, start), nullptr));
      // `Parser::Name { name, location }` is the (AstName, Location) pair.
      let binding = Binding::new(
        Name {
          name: self.name_self,
          location: *start,
        },
        null_mut(),
        Position::default(),
        false,
      );
      self_local = self.push_local(&binding);
    }

    // C++ uses a `TempVector<AstLocal*> vars(scratch_local)` here; a local Vec
    // produces an identical `copy` result and avoids holding a borrow of
    // `self.scratch_local` across the `self.push_local` calls (the scratch
    // Buffer is only an allocation-reuse optimization, not observable).
    let mut vars: Vec<*mut AstLocal> = Vec::new();
    for arg in args.iter() {
      let local = self.push_local(arg);
      vars.push(local);
    }

    let copied = self.copy_initializer_list_t(&vars);
    (self_local, copied)
  }
}
