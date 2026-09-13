use crate::records::label::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct EntryLocations {
  pub start: Label,
  pub prologue_end: Label,
  pub epilogue_start: Label,
}

impl EntryLocations {
  pub const START: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const PROLOGUE_END: Label = Label {
    id: 0,
    location: !0u32,
  };
  pub const EPILOGUE_START: Label = Label {
    id: 0,
    location: !0u32,
  };
}
