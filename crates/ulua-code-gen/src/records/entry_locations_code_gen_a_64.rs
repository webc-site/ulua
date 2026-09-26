use crate::records::label::Label;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EntryLocations {
  pub start: Label,
  pub prologue_end: Label,
  pub epilogue_start: Label,
}
