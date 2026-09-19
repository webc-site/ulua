use core::ptr::{addr_of_mut, null_mut};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    cleartable::cleartable, clearupvals::clearupvals, markmt::markmt, marktaggetmt::marktaggetmt,
    propagateall::propagateall, remarkupvals::remarkupvals,
  },
  macros::{
    gc_satomic::{GCSATOMIC, GCSSWEEP},
    iswhite::iswhite,
    markobject::markobject,
    markvalue::markvalue,
    otherwhite::otherwhite,
  },
  records::gc_object::GCObject,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
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
    // cpp lgc.cpp:1016：标签 userdata 元表需在 atomic 阶段补标（again）
    marktaggetmt(g);
    // cpp lgc.cpp:1018-1019（markudatadirectaccess）：atomic 阶段补标直接访问元方法
    if fflag::LuauUdataDirectAccess6.get() {
      for udatadirect in (*g).udatadirect.iter_mut() {
        markvalue!(g, addr_of_mut!(udatadirect.indextm));
        markvalue!(g, addr_of_mut!(udatadirect.newindextm));
        markvalue!(g, addr_of_mut!(udatadirect.namecalltm));
      }
    }
    // cpp lgc.cpp:854-860 + 1021-1022（markudatadirectfields）：atomic 阶段补标直接字段派发表
    if fflag::LuauDirectFieldGet.get() {
      for &field in (*g).udatadirectfields.iter() {
        if !field.is_null() {
          markobject!(g, field);
        }
      }
    }
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
