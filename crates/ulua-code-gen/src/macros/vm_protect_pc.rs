use ulua_vm::{
  macros::vm_protect_pc::vm_protect_pc as VM_PROTECT_PC_VM, records::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn vm_protect_pc(l: *mut lua_State, pc: *const u32) {
  unsafe {
    VM_PROTECT_PC_VM(l, pc);
  }
}
