use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    cleartable::cleartable, clearupvals::clearupvals, markmt::markmt, propagateall::propagateall,
    remarkupvals::remarkupvals,
  },
  macros::{
    gc_satomic::{GCSATOMIC, GCSSWEEP},
    iswhite::iswhite,
    markobject::markobject,
    otherwhite::otherwhite,
  },
  records::gc_object::GCObject,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn atomic(l: *mut lua_State) -> usize {
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!((*g).gcstate as i32 == GCSATOMIC);

    let mut work = 0usize;

    work += remarkupvals(g);
    work += propagateall(g);

    (*g).gray = (*g).weak;
    (*g).weak = null_mut();
    LUAU_ASSERT!(!iswhite!((*g).mainthread as *mut GCObject));
    markobject!(g, l);
    markmt(g);
    work += propagateall(g);

    (*g).gray = (*g).grayagain;
    (*g).grayagain = null_mut();
    work += propagateall(g);

    work += cleartable(l, (*g).weak);
    (*g).weak = null_mut();

    work += clearupvals(l);

    (*g).currentwhite = otherwhite!(g) as u8;
    (*g).sweepgcopage = (*g).allgcopages;
    (*g).gcstate = GCSSWEEP as u8;

    work
  }
}
