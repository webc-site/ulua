use ulua_ast::records::hot_comment::HotComment;

pub fn has_native_comment_directive(hotcomments: &[HotComment]) -> bool {
  for hc in hotcomments {
    if hc.content.is_empty()
      || hc.content.as_bytes().first() == Some(&b' ')
      || hc.content.as_bytes().first() == Some(&b'\t')
    {
      continue;
    }

    if hc.header {
      let bytes = hc.content.as_bytes();
      let space_pos = bytes.iter().position(|&b| b == b' ' || b == b'\t');

      let first = if let Some(pos) = space_pos {
        &hc.content[..pos]
      } else {
        &hc.content[..]
      };

      if first == "native" {
        return true;
      }
    }
  }

  false
}
