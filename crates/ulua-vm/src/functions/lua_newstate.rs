//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:184:lua_newstate`
//! Source: `VM/src/lstate.cpp:184-290` (hand-ported)

use core::{
  ffi::c_void,
  mem::{size_of, zeroed},
  ptr::null_mut,
};

use ulua_common::FFlag;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    close_state::close_state, f_luaopen::f_luaopen,
    lua_d_rawrunprotected_ldo::luaD_rawrunprotected, preinit_state::preinit_state,
  },
  macros::{
    bit_2_mask::bit2mask, fixedbit::FIXEDBIT, gc_spause::GCSPAUSE, luai_gcgoal::LUAI_GCGOAL,
    luai_gcstepmul::LUAI_GCSTEPMUL, luai_gcstepsize::LUAI_GCSTEPSIZE, registry::registry,
    setnilvalue::setnilvalue, white_0_bit::WHITE0BIT,
  },
  records::{global_state::global_State, lg::LG},
  type_aliases::{lua_alloc::LuaAlloc, lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_newstate(f: LuaAlloc, ud: *mut c_void) -> *mut lua_State {
  unsafe {
    let Some(falloc) = f else {
      return null_mut();
    };
    let l = falloc(ud, null_mut(), 0, size_of::<LG>());
    if l.is_null() {
      return null_mut();
    }
    let l = l as *mut lua_State;
    let g = &mut (*(l as *mut LG)).g as *mut global_State;

    (*l).hdr.tt = LuaType::Thread as u8;
    (*g).currentwhite = bit2mask(WHITE0BIT, FIXEDBIT) as u8;
    (*l).hdr.marked = (*g).currentwhite;
    (*l).hdr.memcat = 0;
    preinit_state(l, g);
    (*g).frealloc = f;
    (*g).ud = ud;
    (*g).mainthread = l;
    (*g).uvhead.u.open.prev = &mut (*g).uvhead;
    (*g).uvhead.u.open.next = &mut (*g).uvhead;
    (*g).gc_threshold = 0; // mark it as unfinished state
    (*g).registryfree = 0;
    (*g).errorjmp = null_mut();
    (*g).rngstate = 0;
    (*g).ptrenckey[0] = 1;
    (*g).ptrenckey[1] = 0;
    (*g).ptrenckey[2] = 0;
    (*g).ptrenckey[3] = 0;
    (*g).strt.size = 0;
    (*g).strt.nuse = 0;
    (*g).strt.hash = null_mut();
    setnilvalue!(&mut (*g).pseudotemp as *mut TValue);
    setnilvalue!(registry!(l) as *const TValue as *mut TValue);
    (*g).gcstate = GCSPAUSE as u8;
    (*g).gray = null_mut();
    (*g).grayagain = null_mut();
    (*g).weak = null_mut();
    (*g).totalbytes = size_of::<LG>();
    (*g).gcgoal = LUAI_GCGOAL;
    (*g).gcstepmul = LUAI_GCSTEPMUL;
    (*g).gcstepsize = LUAI_GCSTEPSIZE << 10;

    for page in (*g).freepages.iter_mut() {
      *page = null_mut();
    }
    for page in (*g).freegcopages.iter_mut() {
      *page = null_mut();
    }

    (*g).allpages = null_mut();
    (*g).allgcopages = null_mut();
    (*g).sweepgcopage = null_mut();

    for mt in (*g).mt.iter_mut() {
      *mt = null_mut();
    }

    for gc in (*g).udatagc.iter_mut() {
      *gc = None;
    }
    for mt in (*g).udatamt.iter_mut() {
      *mt = null_mut();
    }

    for udatadirect in (*g).udatadirect.iter_mut() {
      setnilvalue!(&mut udatadirect.indextm as *mut TValue);
      setnilvalue!(&mut udatadirect.newindextm as *mut TValue);
      setnilvalue!(&mut udatadirect.namecalltm as *mut TValue);
      udatadirect.index = None;
      udatadirect.newindex = None;
      udatadirect.namecall = None;
    }

    for name in (*g).lightuserdataname.iter_mut() {
      *name = null_mut();
    }

    if FFlag::LuauDirectFieldGet.get() {
      for field in (*g).udatadirectfields.iter_mut() {
        *field = null_mut();
      }
    }

    for bytes in (*g).memcatbytes.iter_mut() {
      *bytes = 0;
    }

    (*g).memcatbytes[0] = size_of::<LG>();

    (*g).cb = zeroed(); // lua_Callbacks()
    (*g).ecb = zeroed(); // lua_ExecutionCallbacks()

    (*g).ecbdata = zeroed();

    (*g).gcstats = Default::default(); // GCStats()
    (*g).lastprotoid = 1;

    if luaD_rawrunprotected(l, Some(f_luaopen), null_mut()) != 0 {
      // memory allocation error: free partial state
      close_state(l);
      return null_mut();
    }

    ulua_common::LUAU_ASSERT!((*g).gc_threshold != 0);
    l
  }
}
