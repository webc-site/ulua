use core::fmt::{self, Debug, Formatter};

use crate::enums::event_type::EventType;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Event {
  pub(crate) r#type: EventType,
  pub(crate) token: u16,
  pub(crate) data: EventData,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union EventData {
  pub(crate) microsec: u32,
  pub(crate) data_pos: u32,
}

impl Debug for EventData {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("EventData")
      .field("microsec/data_pos", unsafe { &self.microsec })
      .finish()
  }
}
