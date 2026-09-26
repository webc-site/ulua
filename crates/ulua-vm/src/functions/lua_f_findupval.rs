use core::{ffi::c_void, mem::size_of, ptr::addr_of_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::lua_m_newgco,
  macros::{isblack::isblack, isdead::isdead, lua_c_init::luaC_init, upisopen::upisopen},
  records::{gc_object::GCObject, lua_state::LuaState, up_val::UpVal},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为处于活动调用中的存活状态（断言 `isactive`、自身非黑）；`level` 必须落在 `l` 当前栈区且
/// 指向活跃局部槽——链表按 `(*pp).v >= level` 比较裸栈指针。`l.openupval`/`g.uvhead` 双链须自洽，
/// 否则解引用悬垂 UpVal 或写坏全局 uv 环。新建 upval 时可抛 ERR_MEM。cpp lfunc.cpp:99。
pub unsafe fn lua_f_findupval(l: *mut LuaState, level: StkId) -> *mut UpVal {
  unsafe {
    let g = (*l).global;
    let mut pp: *mut *mut UpVal = addr_of_mut!((*l).openupval);

    while !(*pp).is_null() && (*(*pp)).v >= level {
      let p = *pp;
      LUAU_ASSERT!(!isdead!(g, p as *mut GCObject));
      LUAU_ASSERT!(upisopen!(p));
      if (*p).v == level {
        return p;
      }

      pp = addr_of_mut!((*p).u.open.threadnext);
    }

    LUAU_ASSERT!((*l).isactive);
    LUAU_ASSERT!(!isblack!(l as *mut GCObject));

    let uv = lua_m_newgco(l, size_of::<UpVal>(), (*l).activememcat) as *mut UpVal;

    luaC_init!(l, uv, LuaType::Upval as i32);
    let u = &mut *uv;
    u.markedopen = 0;
    u.v = level;

    u.u.open.threadnext = *pp;
    *pp = uv;

    let uvhead = addr_of_mut!((*g).uvhead);
    u.u.open.prev = uvhead;
    u.u.open.next = (*uvhead).u.open.next;
    (*u.u.open.next).u.open.prev = uv;
    (*uvhead).u.open.next = uv;

    LUAU_ASSERT!((*u.u.open.next).u.open.prev == uv && (*u.u.open.prev).u.open.next == uv);

    uv
  }
}

/// # Safety
/// C ABI 导出壳：`level` 必须是由 `StkId` 擦除为 `c_void` 前真实的栈槽指针，其余前提同内层
/// `lua_f_findupval`；返回的 `*mut c_void` 需由调用方还原回 `UpVal*` 使用。cpp lfunc.cpp:99。
pub unsafe extern "C-unwind" fn lua_f_findupval_export(
  l: *mut LuaState,
  level: StkId,
) -> *mut c_void {
  // Safety: 导出壳原样转发同契约 `lua_f_findupval`；`l` 存活且 open 链表读取在界内，返回指针按 c_void 宽化
  unsafe { lua_f_findupval(l, level).cast() }
}
