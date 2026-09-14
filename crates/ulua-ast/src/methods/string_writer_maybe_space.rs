use crate::records::{position::Position, string_writer::StringWriter};

pub fn string_writer_maybe_space(writer: &mut StringWriter, new_pos: &Position, reserve: i32) {
  writer.maybe_space(new_pos, reserve);
}
