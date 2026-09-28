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
      // memchr2 与 `iter().position(|&b| matches!(b, b' ' | b'\t'))` 逐位等价：
      // 同为「首个空格或制表符的下标，无则 None」，仅查找内核换 memchr 的 SIMD 实现。
      let space_pos = memchr::memchr2(b' ', b'\t', bytes);

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
