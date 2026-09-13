use alloc::string::String;
use std::iter::repeat_n;

const K_DETAILS_ALIGN_COLUMN: i32 = 60;

pub(crate) fn pad_to_detail_column(result: &mut String, line_start: usize) {
  let pad = K_DETAILS_ALIGN_COLUMN - (result.len() as i32 - line_start as i32);
  if pad > 0 {
    // Append `pad` spaces.
    result.extend(repeat_n(' ', pad as usize));
  }
}
