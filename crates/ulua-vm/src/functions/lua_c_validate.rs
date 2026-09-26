//! Source: `VM/src/lgcdebug.cpp:274` (hand-ported)

use core::{
  ffi::c_void,
  ptr::{addr_of_mut, null_mut},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_m_visitgco::lua_m_visitgco, validategco::validategco, validategraylist::validategraylist,
  },
  macros::{checkliveness::checkliveness, isblack::isblack, isdead::isdead, upisopen::upisopen},
  records::{
    gc_object::GCObject, global_state::global_State, lua_page::lua_Page, lua_state::LuaState,
    up_val::UpVal,
  },
};

/// # Safety
/// `l` 须为存活主 LuaState 且 `(*l).global` 有效，GC 处于一致状态：`mainthread`/`registry`/`mt[]`/各灰链表
/// （`weak`/`gray`/`grayagain`）指针均指向存活对象，`uvhead` 打开 upvalue 链表完整。本函数纯断言校验（失败即 abort），
/// 须在 GC 阶段边界独占调用。cpp/VM/src/lgcdebug.cpp:274 luaC_validate。
pub unsafe fn lua_c_validate(l: *mut LuaState) {
  unsafe {
    let g: *mut global_State = (*l).global;

    // 移植记录以 hdr.tt 字段替代 ttype! 方法入口，mainthread/uv 等
    // 具体类型指针须先提升到 GCObject* 再走 isdead!/isblack!（等价 obj2gco）
    LUAU_ASSERT!(!isdead!(g, (*g).mainthread as *mut GCObject));
    checkliveness!(g, &(*g).registry);

    for &mt in (*g).mt.iter() {
      if !mt.is_null() {
        LUAU_ASSERT!(!isdead!(g, mt as *mut GCObject));
      }
    }

    validategraylist(g, (*g).weak as *mut GCObject);
    validategraylist(g, (*g).gray as *mut GCObject);
    validategraylist(g, (*g).grayagain as *mut GCObject);

    validategco(
      l as *mut c_void,
      null_mut::<lua_Page>(),
      (*g).mainthread as *mut GCObject,
    );

    lua_m_visitgco(l, l as *mut c_void, validategco);

    // 哨兵地址比较：用 addr_of_mut 取裸地址，避免为比较制造瞬时 &mut 独占借用
    let uvhead = addr_of_mut!((*g).uvhead);
    let mut uv: *mut UpVal = (*g).uvhead.u.open.next;
    while uv != uvhead {
      LUAU_ASSERT!((*uv).hdr.tt == LuaType::Upval as u8);
      LUAU_ASSERT!(upisopen!(uv));
      LUAU_ASSERT!(
        (*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv
      );
      // open upvalues are never black
      LUAU_ASSERT!(!isblack!(uv as *mut GCObject));
      uv = (*uv).u.open.next;
    }
  }
}
