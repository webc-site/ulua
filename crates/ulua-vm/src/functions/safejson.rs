#[inline]
pub(crate) fn safejson(ch: u8) -> bool {
  (32..128).contains(&ch) && ch != b'\\' && ch != b'\"'
}
