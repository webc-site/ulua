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

/// GC atomic/swept 阶段清闭 `uvhead` 打开 upvalue 链表（w6e 收形：`l` 折为 `&mut`
/// 引用形参，诚实降级为安全 fn；真实链表裸导航与 closeupval 写入下沉到唯一逐句窄
/// `unsafe` 块）。
///
/// 调用序契约（正确性，非内存安全）：`l`/`l.global` 的存活已由引用形参与
/// `LuaState` 自持不变量承载；调用方仍须保证 `(*l).global.uvhead` 打开 upvalue 双向
/// 链表完整（每个 `uv` 的 `u.open.next`/`prev` 互指，见 `LUAU_ASSERT`），链表内每个
/// `UpVal` 存活且 `(*uv).v` 可解引用（`luaF_closeupval` 会写这些对象），且须在 GC
/// atomic/swept 阶段（无并发 mutation）调用。cpp/VM/src/lgc.cpp:967 clearupvals。
pub(crate) fn clearupvals(l: &mut LuaState) -> usize {
  let g = l.global;

  let mut work: usize = 0;

  // SAFETY: 上方调用序契约——`g` 指向有效 global_State，`uvhead` 双向链表完整且各
  // `UpVal` 存活；块内仅沿该链表走查、读标记并经 `lua_f_closeupval`（`# Safety`
  // 契约，透传本函数持有的 `l`）摘链关闭，不释放、不越链。
  unsafe {
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
  }

  work
}
