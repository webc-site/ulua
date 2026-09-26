use crate::records::label::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct EntryLocations {
  pub start: Label,
  pub prologue_end: Label,
  pub epilogue_start: Label,
}
