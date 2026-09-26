use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::validateobjref::validateobjref,
  macros::{blackbit::BLACKBIT, checkliveness::checkliveness, upisopen::upisopen},
  records::{call_info::CallInfo, global_state::global_State, lua_state::LuaState, up_val::UpVal},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn validatestack(g: *mut global_State, l: *mut LuaState) {
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
      // open upvalue 永远不是黑色（BLACKBIT）
      LUAU_ASSERT!(((*uv).hdr.marked & (1u8 << BLACKBIT)) == 0);
      uv = (*uv).u.open.threadnext;
    }
  }
}
