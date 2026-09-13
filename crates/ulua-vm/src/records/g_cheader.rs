#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[derive(Default)]
pub struct GCheader {
  pub tt: u8,
  pub marked: u8,
  pub memcat: u8,
}
