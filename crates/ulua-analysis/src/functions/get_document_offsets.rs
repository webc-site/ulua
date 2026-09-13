use core::cmp::{max, min};

use ulua_ast::records::position::Position;
pub(crate) fn get_document_offsets(
  src: &str,
  start_pos: &Position,
  end_pos: &Position,
) -> (usize, usize) {
  let mut line_count: u32 = 0;
  let mut col_count: u32 = 0;

  let mut doc_offset: usize = 0;
  let mut start_offset: usize = 0;
  let mut end_offset: usize = 0;
  let mut found_start = false;
  let mut found_end = false;

  for c in src.chars() {
    if found_start && found_end {
      break;
    }

    if start_pos.line == line_count && start_pos.column == col_count {
      found_start = true;
      start_offset = doc_offset;
    }

    if end_pos.line == line_count && end_pos.column == col_count {
      end_offset = doc_offset;
      found_end = true;
    }

    // We put a cursor position that extends beyond the extents of the current line
    if found_start && !found_end && (line_count > end_pos.line) {
      found_end = true;
      end_offset = doc_offset.saturating_sub(1);
    }

    if c == '\n' {
      line_count += 1;
      col_count = 0;
    } else {
      col_count += 1;
    }
    doc_offset += c.len_utf8();
  }

  if found_start && !found_end {
    end_offset = src.len();
  }

  let min = min(start_offset, end_offset);
  let max = max(start_offset, end_offset);
  let len = max - min;

  (min, len)
}
