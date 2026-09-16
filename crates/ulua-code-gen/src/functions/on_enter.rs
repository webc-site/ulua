use core::mem::transmute;

use ulua_vm::records::{lua_state::lua_State, proto::Proto};

use crate::{
  functions::get_code_gen_context::get_code_gen_context, macros::codegen_assert::CODEGEN_ASSERT,
  records::native_context::NativeContext,
};

// JIT 机器码入口签名：gate_entry 指向生成的机器码，只能经 C ABI 调用
type GateFn =
  unsafe extern "C-unwind" fn(*mut lua_State, *mut Proto, usize, *mut NativeContext) -> i32;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
///
/// ABI 与 VM 的 `ecb.enter` 槽位一致（C++ `onEnter` 为 C ABI），可直接装入
/// `lua_ExecutionCallbacks::enter`，无需 transmute。
pub unsafe extern "C-unwind" fn on_enter(l: *mut lua_State, proto: *mut Proto) -> i32 {
  unsafe {
    let code_gen_context = get_code_gen_context(l);

    CODEGEN_ASSERT!(!code_gen_context.is_null());
    CODEGEN_ASSERT!(!(*proto).execdata.is_null());
    CODEGEN_ASSERT!((*(*l).ci).savedpc >= (*proto).code);
    CODEGEN_ASSERT!((*(*l).ci).savedpc < (*proto).code.add((*proto).sizecode as usize));

    let pc_offset = (*(*l).ci).savedpc.offset_from((*proto).code) as usize;
    let instruction_offsets = (*proto).execdata as *mut u32;
    let target = (*proto).exectarget + *instruction_offsets.add(pc_offset) as usize;

    let gate: GateFn = transmute((*code_gen_context).context.gate_entry);
    gate(l, proto, target, &mut (*code_gen_context).context)
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_on_enter")]
pub unsafe extern "C-unwind" fn on_enter_export(l: *mut lua_State, proto: *mut Proto) -> i32 {
  unsafe { on_enter(l, proto) }
}
