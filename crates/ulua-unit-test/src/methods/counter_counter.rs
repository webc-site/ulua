use core::sync::atomic::{AtomicI32, Ordering};

use crate::records::counter::Counter;

static INSTANCE_COUNT: AtomicI32 = AtomicI32::new(0);

impl Counter {
  /// C++: `Counter() { ++instanceCount; id = instanceCount; }`
  /// (tests/Parser.test.cpp:41). An associated constructor — each call bumps
  /// the static instance count and records it as this instance's `id`.
  pub fn counter_counter() -> Self {
    let id = INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    Counter { id }
  }

  /// C++ tests reset the static counter directly via `Counter::instanceCount = 0;`
  /// (tests/Parser.test.cpp:98). Rust has no assignable associated static, so the
  /// reset is exposed as a method.
  pub fn reset_instance_count() {
    INSTANCE_COUNT.store(0, Ordering::SeqCst);
  }
}
