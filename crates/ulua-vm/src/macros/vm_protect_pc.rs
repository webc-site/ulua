use crate::records::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn vm_protect_pc(l: *mut lua_State, pc: *const u32) {
  unsafe {
    (*(*l).ci).savedpc = pc;
  }
}

pub use vm_protect_pc as VM_PROTECT_PC;
