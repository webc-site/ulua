//! Source: `Common/include/Luau/TimeTrace.h:82-93` (hand-ported)
//! C++ `ThreadContext::flushEvents()`.
use std::sync::OnceLock;

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
    // C++ magic static 保证只注册一次；OnceLock 提供同样的线程安全语义
    // （Relaxed 原子量在双线程同时首次调用时会重复注册 token）。
    static FLUSH_TOKEN: OnceLock<u16> = OnceLock::new();

    // `GlobalContext` guards its mutable state behind a Mutex, so the shared
    // Arc handle is enough for both `createToken` and `flushEvents`.
    let global_context = self.global_context.clone();

    let flush_token =
      *FLUSH_TOKEN.get_or_init(|| create_token(&global_context, "flushEvents", "TimeTrace"));

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
