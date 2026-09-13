use crate::records::{position::Position, string_writer::StringWriter};

pub fn string_writer_advance(writer: &mut StringWriter, new_pos: &Position) {
  writer.advance(new_pos);
}
