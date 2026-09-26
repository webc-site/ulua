#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
  NoCheck,
  Nonstrict,
  Strict,
  Definition,
}
