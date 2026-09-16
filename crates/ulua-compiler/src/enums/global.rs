#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum Global {
  #[default]
  Default = 0,
  Mutable,
  Written,
}
