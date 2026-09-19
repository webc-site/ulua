use crate::{
  enums::event_type::EventType,
  records::{
    event::{Event, EventData},
    thread_context::ThreadContext,
  },
};

impl ThreadContext {
  pub fn event_leave_u32(&mut self, microsec: u32) {
    self.events.push(Event {
      r#type: EventType::Leave,
      token: 0,
      data: EventData { microsec },
    });

    if self.events.len() > Self::K_EVENT_FLUSH_LIMIT {
      self.flush_events();
    }
  }
}
