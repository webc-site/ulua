//! Source: `Common/include/Luau/TimeTrace.h:82-93` (hand-ported)
//! C++ `ThreadContext::flushEvents()`.
use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};

use crate::{
  enums::event_type::EventType,
  functions::{
    create_token::create_token, flush_events::flush_events,
    get_clock_microseconds::get_clock_microseconds,
  },
  records::{
    event::{Event, EventData},
    thread_context::ThreadContext,
  },
};

impl ThreadContext {
  pub fn flush_events(&mut self) {
    // `static uint16_t flushToken = createToken(*globalContext, "flushEvents", "TimeTrace");`
    static FLUSH_TOKEN: AtomicU16 = AtomicU16::new(0);
    static INITIALIZED: AtomicBool = AtomicBool::new(false);

    // `GlobalContext` guards its mutable state behind a Mutex, so the shared
    // Arc handle is enough for both `createToken` and `flushEvents`.
    let global_context = self.global_context.clone();

    if !INITIALIZED.load(Ordering::Relaxed) {
      let token = create_token(&global_context, "flushEvents", "TimeTrace");
      FLUSH_TOKEN.store(token, Ordering::Relaxed);
      INITIALIZED.store(true, Ordering::Relaxed);
    }

    let flush_token = FLUSH_TOKEN.load(Ordering::Relaxed);

    // events.push_back({EventType::Enter, flushToken, {getClockMicroseconds()}});
    self.events.push(Event {
      r#type: EventType::Enter,
      token: flush_token,
      data: EventData {
        microsec: get_clock_microseconds(),
      },
    });

    // TimeTrace::flushEvents(*globalContext, threadId, events, data);
    flush_events(&global_context, self.thread_id, &self.events, &self.data);

    // events.clear(); data.clear();
    self.events.clear();
    self.data.clear();

    // events.push_back({EventType::Leave, 0, {getClockMicroseconds()}});
    self.events.push(Event {
      r#type: EventType::Leave,
      token: 0,
      data: EventData {
        microsec: get_clock_microseconds(),
      },
    });
  }
}
