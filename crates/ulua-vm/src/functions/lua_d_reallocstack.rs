use core::{ffi::c_int, mem::size_of};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    correctstack::correctstack, lua_d_throw_ldo::lua_d_throw, lua_m_realloc::lua_m_realloc_,
    lua_m_toobig::lua_m_toobig,
  },
  macros::{
    cast_to::cast_to, extra_stack::EXTRA_STACK, max_stack_size::MAX_STACK_SIZE,
    setnilvalue::setnilvalue,
  },
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_reallocstack(l: *mut lua_State, newsize: c_int, fornewci: c_int) {
  unsafe {
    // throw 'out of memory' error because space for a custom error message cannot be guaranteed here
    if newsize > MAX_STACK_SIZE {
      // reallocation was performed to setup a new CallInfo frame, which we have to remove
      if fornewci != 0 {
        let cip = (*l).ci.wrapping_offset(-1);

        (*l).ci = cip;
        (*l).base = (*cip).base;
        (*l).top = (*cip).top;
      }

      lua_d_throw(l, LuaStatus::ErrMem as i32);
    }

    let oldstack = (*l).stack;
    let realsize = newsize + EXTRA_STACK;
    LUAU_ASSERT!(
      (*l).stack_last.offset_from((*l).stack) == ((*l).stacksize - EXTRA_STACK) as isize
    );

    let oldsize_bytes = (*l).stacksize as usize * size_of::<TValue>();
    let newsize_bytes = if realsize as usize <= usize::MAX / size_of::<TValue>() {
      realsize as usize * size_of::<TValue>()
    } else {
      lua_m_toobig(l)
    };

    (*l).stack = cast_to!(
      *mut TValue,
      lua_m_realloc_(
        l,
        (*l).stack as *mut u8,
        oldsize_bytes,
        newsize_bytes,
        (*l).activememcat
      )
    );

    let newstack = (*l).stack;

    for i in (*l).stacksize as usize..realsize as usize {
      setnilvalue!(newstack.add(i));
    }

    (*l).stacksize = realsize;
    (*l).stack_last = newstack.add(newsize as usize);

    correctstack(l, oldstack);
  }
}

pub use lua_d_reallocstack as luaD_reallocstack;
