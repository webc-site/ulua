use alloc::string::String;
pub fn normalize_state_offsets(text: &mut String) {
  let mut result = String::with_capacity(text.len());
  let mut pending_reg = String::new();

  let mut pos = 0;
  while pos < text.len() {
    let eol = text[pos..]
      .find('\n')
      .map(|i| pos + i)
      .unwrap_or(text.len());
    let line = &text[pos..eol];

    let mut new_line = line.to_string();

    let r15_pattern = "[r15+";
    if let Some(start) = line.find(r15_pattern) {
      if let Some(end) = line[start + r15_pattern.len()..].find(']')
        && let Some(comma) = line[..start].rfind(',')
        && let Some(reg) = line[..comma].split_whitespace().last()
      {
        pending_reg = reg.to_string();
        let end = start + r15_pattern.len() + end;
        new_line.replace_range(start..=end, "[r15+<offset>]");
      }
    } else if !pending_reg.is_empty() {
      let deref_start = format!("[{}+", pending_reg);
      if let Some(start) = line.find(&deref_start)
        && let Some(end) = line[start + deref_start.len()..].find(']')
      {
        let end = start + deref_start.len() + end;
        new_line.replace_range(start..=end, &format!("[{}+<offset>]", pending_reg));
      }
    }

    result.push_str(&new_line);
    if eol < text.len() {
      result.push('\n');
    }
    pos = eol + 1;
  }

  *text = result;
}
