use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::validateobjref::validateobjref,
  macros::{checkliveness::checkliveness, upisopen::upisopen},
  records::{call_info::CallInfo, global_state::global_State, lua_state::lua_State, up_val::UpVal},
  type_aliases::stk_id::StkId,
};

pub(crate) unsafe fn validatestack(g: *mut global_State, l: *mut lua_State) {
  unsafe {
    validateobjref(g, l as *mut _, (*l).gt as *mut _);

    let mut ci: *mut CallInfo = (*l).base_ci;
    while ci <= (*l).ci {
      LUAU_ASSERT!((*l).stack <= (*ci).base);
      LUAU_ASSERT!((*ci).func <= (*ci).base && (*ci).base <= (*ci).top);
      LUAU_ASSERT!((*ci).top <= (*l).stack_last);
      ci = ci.wrapping_add(1);
    }

    let mut o: StkId = (*l).stack;
    while o < (*l).top {
      checkliveness!(g, o);
      o = o.wrapping_add(1);
    }

    if !(*l).namecall.is_null() {
      validateobjref(g, l as *mut _, (*l).namecall as *mut _);
    }

    let mut uv: *mut UpVal = (*l).openupval;
    while !uv.is_null() {
      LUAU_ASSERT!((*uv).hdr.tt == LuaType::Upval as u8);
      LUAU_ASSERT!(upisopen!(uv));
      LUAU_ASSERT!(
        (*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv
      );
      LUAU_ASSERT!(((*uv).hdr.marked & (1 << 2)) == 0); // open upvalues are never black (BLACKBIT = 2)
      uv = (*uv).u.open.threadnext;
    }
  }
}
