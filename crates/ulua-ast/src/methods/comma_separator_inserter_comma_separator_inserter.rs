use crate::records::{
  comma_separator_inserter::CommaSeparatorInserter, position::Position, writer::Writer,
};

impl CommaSeparatorInserter {
  // The implementation of CommaSeparatorInserter::new already exists in the record file.
  // As per the dependency policy for already-implemented containers and foundation types,
  // we provide a minimal stub here to avoid duplicate definition errors.
}

pub fn comma_separator_inserter_comma_separator_inserter(
  writer: &mut dyn Writer,
  comma_position: *const Position,
) -> CommaSeparatorInserter {
  CommaSeparatorInserter::new(writer, comma_position)
}
