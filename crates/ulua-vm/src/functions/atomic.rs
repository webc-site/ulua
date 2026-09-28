use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    cleartable::cleartable, clearupvals::clearupvals, markmt::markmt, marktaggetmt::marktaggetmt,
    markudatadirect::markudatadirect, propagateall::propagateall, remarkupvals::remarkupvals,
  },
  macros::{
    gc_satomic::{GCSATOMIC, GCSSWEEP},
    iswhite::iswhite,
    markobject::markobject,
    otherwhite::otherwhite,
  },
  records::{gc_object::GCObject, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn atomic(l: *mut LuaState) -> usize {
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
    // cpp lgc.cpp:1016：标签 userdata 元表需在 atomic 阶段补标（again）
    marktaggetmt(g);
    // cpp lgc.cpp:1018-1022：补标 udatadirect 元方法缓存与直接字段派发表
    markudatadirect(g);
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
