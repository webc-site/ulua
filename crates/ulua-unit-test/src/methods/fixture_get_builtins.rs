use ulua_analysis::records::builtin_types::BuiltinTypes;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn get_builtins(&mut self) -> &mut BuiltinTypes {
    // `Frontend` owns `builtin_types_` inline and exposes `builtin_types` as a
    // self-referential pointer at it (see `Frontend::wire_self_pointers`). The
    // test `Fixture` is routinely returned by value — e.g.
    // `SimplifyFixture::default()` constructs then moves the whole struct to
    // the caller's slot — which relocates `builtin_types_` and leaves the
    // cached `self.builtin_types` (and `frontend.builtin_types`) dangling.
    //
    // `get_frontend` re-runs `wire_self_pointers` and refreshes
    // `self.builtin_types` against the frontend's *current* address, so we must
    // route every access through it rather than trusting the cached pointer.
    // Reading the stale pointer was layout-dependent UB: it happened to hold
    // valid data on some targets (macOS arm64) and returned clobbered garbage
    // on others (x86_64 Linux), crashing e.g. `simplify_simplify_stops_at_cycles`.
    self.get_frontend();

    unsafe { &mut *self.builtin_types }
  }
}
