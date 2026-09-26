use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    correctstack::correctstack, lua_d_throw_ldo::lua_d_throw, lua_m_realloc::lua_m_realloc_,
    lua_m_toobig::lua_m_toobig,
  },
  macros::{extra_stack::EXTRA_STACK, max_stack_size::MAX_STACK_SIZE, setnilvalue::setnilvalue},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈不变式成立（`LUAU_ASSERT stack_last-stack == stacksize-EXTRA_STACK`）；
/// `newsize` 为目标逻辑栈大小，`newsize+EXTRA_STACK` 乘 `size_of::<TValue>()` 不得溢出（否则 `luaM_toobig`
/// 抛错），`luaM_realloc_` 移动栈后 `correctstack` 重定位所有指针；`fornewci!=0` 表示已压入待回退的新 CI。
/// OOM 时经 `luaD_throw(ErrMem)` 抛错，须受保护帧。cpp `ldo.cpp:183`。
pub unsafe fn lua_d_reallocstack(l: *mut LuaState, newsize: i32, fornewci: i32) {
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

    (*l).stack = lua_m_realloc_(
      l,
      (*l).stack as *mut u8,
      oldsize_bytes,
      newsize_bytes,
      // cpp ldo.cpp:204：按线程自身 GC 头的 memcat 记账，而非 activememcat（后者可被 lua_setmemcat 运行时修改）
      (*l).hdr.memcat,
    ) as *mut TValue;

    let newstack = (*l).stack;

    for i in (*l).stacksize as usize..realsize as usize {
      setnilvalue!(newstack.add(i));
    }

    (*l).stacksize = realsize;
    (*l).stack_last = newstack.add(newsize as usize);

    correctstack(l, oldstack);
  }
}
