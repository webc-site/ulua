#[cfg(any())]
impl FindNode {
  pub fn new(pos: Position, document_end: Position) -> Self {
    Self {
      pos,
      document_end,
      best: core::ptr::null_mut(),
    }
  }
}
