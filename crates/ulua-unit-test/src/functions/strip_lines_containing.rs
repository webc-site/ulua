use alloc::string::String;

pub fn strip_lines_containing(text: &mut String, needle: &str) {
  let needle_bytes = needle.as_bytes();
  if needle_bytes.is_empty() {
    return;
  }

  let mut pos = 0;
  while pos < text.len() {
    let bytes = text.as_bytes();
    if let Some(found_idx) = bytes[pos..]
      .windows(needle_bytes.len())
      .position(|w| w == needle_bytes)
    {
      let actual_pos = pos + found_idx;

      let line_start = match bytes[..actual_pos].iter().rposition(|&b| b == b'\n') {
        None => 0,
        Some(idx) => idx + 1,
      };

      let line_end = bytes[actual_pos..]
        .iter()
        .position(|&b| b == b'\n')
        .map(|idx| actual_pos + idx);

      match line_end {
        None => {
          text.truncate(line_start);
        }
        Some(end_idx) => {
          text.drain(line_start..=end_idx);
        }
      }

      pos = line_start;
    } else {
      break;
    }
  }
}
