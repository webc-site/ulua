use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::functions::split::split;
pub fn string_at_location(source: &str, location: &Location) -> String {
  let lines: Vec<&str> = split(source, '\n');

  ulua_common::LUAU_ASSERT!(
    lines.len() > location.begin.line as usize && lines.len() > location.end.line as usize
  );

  let mut byte_start: i32 = -1;
  let mut byte_end: i32 = -1;
  let mut bytes_sum: i32 = 0;

  for (line_no, line) in lines.iter().enumerate() {
    if line_no as u32 == location.begin.line {
      byte_start = bytes_sum + location.begin.column as i32;
    }

    if line_no as u32 == location.end.line {
      byte_end = bytes_sum + location.end.column as i32;
      break;
    }

    bytes_sum += line.len() as i32 + 1;
  }

  ulua_common::LUAU_ASSERT!(byte_start != -1);
  ulua_common::LUAU_ASSERT!(byte_end != -1);

  let start = byte_start as usize;
  let end = byte_end as usize;
  source[start..end].to_string()
}
