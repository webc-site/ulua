use crate::{
  enums::event_type::EventType,
  records::{
    event::{Event, EventData},
    thread_context::ThreadContext,
  },
};

impl ThreadContext {
  pub fn event_argument(&mut self, name: &str, value: &str) {
    let pos = self.data.len() as u32;
    self.data.extend_from_slice(name.as_bytes());
    self.data.push(0);
    self.events.push(Event {
      r#type: EventType::ArgName,
      token: 0,
      data: EventData { microsec: pos },
    });

    let pos = self.data.len() as u32;
    self.data.extend_from_slice(value.as_bytes());
    self.data.push(0);
    self.events.push(Event {
      r#type: EventType::ArgValue,
      token: 0,
      data: EventData { microsec: pos },
    });

    if self.events.len() > Self::K_EVENT_FLUSH_LIMIT {
      self.flush_events();
    }
  }
}
