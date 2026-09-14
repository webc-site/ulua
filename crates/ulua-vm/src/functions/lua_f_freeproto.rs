use core::mem::size_of;

use crate::{
  functions::{lua_m_free::luaM_free_, lua_m_freegco::luaM_freegco_},
  records::{
    feedback_vector_slot::FeedbackVectorSlot, gc_object::GCObject, loc_var::LocVar,
    lua_page::lua_Page, proto::Proto, t_string::tstring,
  },
  type_aliases::{instruction::Instruction, lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_freeproto(l: *mut lua_State, f: *mut Proto, page: *mut lua_Page) {
  unsafe {
    luaM_free_(
      l,
      (*f).code as *mut u8,
      (*f).sizecode as usize * size_of::<Instruction>(),
      (*f).hdr.memcat,
    );
    luaM_free_(
      l,
      (*f).p as *mut u8,
      (*f).sizep as usize * size_of::<*mut Proto>(),
      (*f).hdr.memcat,
    );
    luaM_free_(
      l,
      (*f).k as *mut u8,
      (*f).sizek as usize * size_of::<TValue>(),
      (*f).hdr.memcat,
    );
    if !(*f).lineinfo.is_null() {
      luaM_free_(
        l,
        (*f).lineinfo,
        (*f).sizelineinfo as usize * size_of::<u8>(),
        (*f).hdr.memcat,
      );
    }
    luaM_free_(
      l,
      (*f).locvars as *mut u8,
      (*f).sizelocvars as usize * size_of::<LocVar>(),
      (*f).hdr.memcat,
    );
    luaM_free_(
      l,
      (*f).upvalues as *mut u8,
      (*f).sizeupvalues as usize * size_of::<*mut tstring>(),
      (*f).hdr.memcat,
    );
    if !(*f).debuginsn.is_null() {
      luaM_free_(
        l,
        (*f).debuginsn,
        (*f).sizecode as usize * size_of::<u8>(),
        (*f).hdr.memcat,
      );
    }

    if !(*f).execdata.is_null()
      && let Some(destroy) = (*(*l).global).ecb.destroy
    {
      destroy(l, f);
    }

    if !(*f).typeinfo.is_null() {
      luaM_free_(
        l,
        (*f).typeinfo,
        (*f).sizetypeinfo as usize * size_of::<u8>(),
        (*f).hdr.memcat,
      );
    }

    if !(*f).feedbackvec.is_null() {
      luaM_free_(
        l,
        (*f).feedbackvec as *mut u8,
        (*f).feedbackvecsize as usize * size_of::<FeedbackVectorSlot>(),
        (*f).hdr.memcat,
      );
    }

    luaM_freegco_(
      l,
      f as *mut GCObject,
      size_of::<Proto>(),
      (*f).hdr.memcat,
      page,
    );
  }
}

pub use lua_f_freeproto as luaF_freeproto;
