//! Source: `VM/src/lgc.cpp` (lgc.cpp:858-890, hand-ported)

use core::{mem::size_of, ptr::addr_of_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_f_closeupval::lua_f_closeupval,
  macros::{
    gcvalue::gcvalue, isblack::isblack, iscollectable::iscollectable, isgray::isgray,
    iswhite::iswhite, upisopen::upisopen,
  },
  records::{gc_object::GCObject, lua_state::LuaState, up_val::UpVal},
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global` 指向有效 global_State，其 `uvhead` 打开 upvalue 双向链表完整
/// （每个 `uv` 的 `u.open.next`/`prev` 互指，见 `LUAU_ASSERT`）；链表内每个 `UpVal` 存活且 `(*uv).v` 可解引用，
/// `luaF_closeupval` 会写这些对象。须在 GC atomic/swept 阶段（无并发 mutation）调用。cpp/VM/src/lgc.cpp:967 clearupvals。
pub(crate) unsafe fn clearupvals(l: *mut LuaState) -> usize {
  unsafe {
    let g = (*l).global;

    let mut work: usize = 0;

    let uvhead = addr_of_mut!((*g).uvhead);
    let mut uv = (*g).uvhead.u.open.next;
    while uv != uvhead {
      work += size_of::<UpVal>();

      LUAU_ASSERT!(upisopen!(uv));
      LUAU_ASSERT!(
        (*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv
      );
      // open upvalues are never black
      LUAU_ASSERT!(!isblack!(uv as *mut GCObject));
      LUAU_ASSERT!(
        iswhite!(uv as *mut GCObject) || !iscollectable!((*uv).v) || !iswhite!(gcvalue!((*uv).v))
      );

      if (*uv).markedopen != 0 {
        // upvalue is still open (belongs to alive thread)
        LUAU_ASSERT!(isgray!(uv as *mut GCObject));
        (*uv).markedopen = 0; // for next cycle
        uv = (*uv).u.open.next;
      } else {
        // upvalue is either dead, or alive but the thread is dead; unlink and close
        let next = (*uv).u.open.next;
        lua_f_closeupval(l, uv, /* dead= */ iswhite!(uv as *mut GCObject));
        uv = next;
      }
    }

    work
  }
}
