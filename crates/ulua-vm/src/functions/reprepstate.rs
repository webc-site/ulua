use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{macros::luai_maxccalls::LUAI_MAXCCALLS, records::match_state::MatchState};

pub(crate) fn reprepstate(ms: *mut MatchState) {
  unsafe {
    (*ms).level = 0;
    LUAU_ASSERT!((*ms).matchdepth == LUAI_MAXCCALLS);
  }
}
