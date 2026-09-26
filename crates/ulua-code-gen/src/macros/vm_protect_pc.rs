use ulua_vm::{
  macros::vm_protect_pc::vm_protect_pc as VM_PROTECT_PC_VM, records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn vm_protect_pc(l: *mut LuaState, pc: *const u32) {
  // Safety: 本 unsafe fn 的 `# Safety` 契约保证 l 为存活 LuaState*、pc 为界内指令指针，
  // 与被转发的 VM_PROTECT_PC_VM 前置条件完全一致，转发既未收紧也未放宽该契约。
  unsafe {
    VM_PROTECT_PC_VM(l, pc);
  }
}
