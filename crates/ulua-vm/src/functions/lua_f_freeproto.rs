use core::mem::size_of;

use crate::{
  functions::{lua_m_free::lua_m_free, lua_m_freegco::lua_m_freegco},
  records::{
    feedback_vector_slot::FeedbackVectorSlot, gc_object::GCObject, loc_var::LocVar,
    lua_page::lua_Page, lua_state::LuaState, proto::Proto, t_string::tstring,
  },
  type_aliases::{instruction::Instruction, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`f` 须指向一个即将销毁、字段自洽的 `Proto`：其 `code/p/k/locvars/upvalues` 及可空
/// `lineinfo/debuginsn/execdata/typeinfo/feedbackvec` 各段须由对应 `size*` 字段界定的已分配区间（用 `(*f).hdr.memcat` 记账）；
/// `page` 允许 NULL（交由 `lua_m_freegco` 归还）。execdata 非空时回调 `ecb.destroy`。释放后 `f` 不得再被引用，须在无并发访问该 proto 时调用。
/// cpp VM/src/lfunc.cpp:173
pub unsafe fn lua_f_freeproto(l: *mut LuaState, f: *mut Proto, page: *mut lua_Page) {
  unsafe {
    // cpp lfunc.cpp:29 luaF_freeproto：各字段读取集中在 free 前，绑定引用消除重复解引用
    let p = &*f;
    lua_m_free(
      l,
      p.code as *mut u8,
      p.sizecode as usize * size_of::<Instruction>(),
      p.hdr.memcat,
    );
    lua_m_free(
      l,
      p.p as *mut u8,
      p.sizep as usize * size_of::<*mut Proto>(),
      p.hdr.memcat,
    );
    lua_m_free(
      l,
      p.k as *mut u8,
      p.sizek as usize * size_of::<TValue>(),
      p.hdr.memcat,
    );
    if !p.lineinfo.is_null() {
      lua_m_free(
        l,
        p.lineinfo,
        p.sizelineinfo as usize * size_of::<u8>(),
        p.hdr.memcat,
      );
    }
    lua_m_free(
      l,
      p.locvars as *mut u8,
      p.sizelocvars as usize * size_of::<LocVar>(),
      p.hdr.memcat,
    );
    lua_m_free(
      l,
      p.upvalues as *mut u8,
      p.sizeupvalues as usize * size_of::<*mut tstring>(),
      p.hdr.memcat,
    );
    if !p.debuginsn.is_null() {
      lua_m_free(
        l,
        p.debuginsn,
        p.sizecode as usize * size_of::<u8>(),
        p.hdr.memcat,
      );
    }

    if !p.execdata.is_null()
      && let Some(destroy) = (*(*l).global).ecb.destroy
    {
      destroy(l, f);
    }

    if !p.typeinfo.is_null() {
      lua_m_free(
        l,
        p.typeinfo,
        p.sizetypeinfo as usize * size_of::<u8>(),
        p.hdr.memcat,
      );
    }

    if !p.feedbackvec.is_null() {
      lua_m_free(
        l,
        p.feedbackvec as *mut u8,
        p.feedbackvecsize as usize * size_of::<FeedbackVectorSlot>(),
        p.hdr.memcat,
      );
    }

    lua_m_freegco(
      l,
      f as *mut GCObject,
      size_of::<Proto>(),
      p.hdr.memcat,
      page,
    );
  }
}
