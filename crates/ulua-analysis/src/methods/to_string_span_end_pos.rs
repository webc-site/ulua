use crate::records::to_string_span::ToStringSpan;

impl ToStringSpan {
  pub fn end_pos(&self) -> usize {
    self.end_pos
  }
}
