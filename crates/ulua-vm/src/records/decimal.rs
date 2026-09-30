#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Decimal {
  pub s: u64,
  pub k: i32,
}
