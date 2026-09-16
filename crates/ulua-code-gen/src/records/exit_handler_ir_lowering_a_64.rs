use crate::records::label::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct ExitHandler {
  pub self_: Label,
  pub pcpos: u32,
}
