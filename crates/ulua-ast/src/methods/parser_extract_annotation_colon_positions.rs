use alloc::vec::Vec;

use crate::records::{
  ast_array::AstArray, binding::Binding, parser::Parser, position::Position,
  temp_vector::TempVector,
};

impl Parser {
  pub(crate) fn extract_annotation_colon_positions(
    &mut self,
    bindings: &TempVector<'_, Binding>,
  ) -> AstArray<Position> {
    // C++ uses `TempVector<Position>(scratch_position)`; a local Vec yields the
    // same `copy` and avoids borrowing `self.scratch_position` across the
    // `&mut self` copy call (scratch reuse is a non-observable optimization).
    let mut annotation_colon_positions: Vec<Position> = Vec::new();
    for binding in bindings.iter() {
      annotation_colon_positions.push(binding.colon_position);
    }

    self.copy_initializer_list_t(&annotation_colon_positions)
  }
}
