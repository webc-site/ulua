use crate::{
  enums::event_type::EventType,
  records::{
    event::{Event, EventData},
    thread_context::ThreadContext,
  },
};

impl ThreadContext {
  pub fn event_enter_u16_u32(&mut self, token: u16, microsec: u32) {
    self.events.push(Event {
      r#type: EventType::Enter,
      token,
      data: EventData { microsec },
    });
  }
}
