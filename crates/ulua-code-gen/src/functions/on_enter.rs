use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::{
  functions::get_code_gen_context::get_code_gen_context, macros::codegen_assert::CODEGEN_ASSERT,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
///
/// 仅被同文件 `on_enter_export` 转发调用；C ABI 由该导出壳承载，本体用 Rust ABI。
pub unsafe fn on_enter(l: *mut LuaState, proto: *mut Proto) -> i32 {
  // Safety: l 为活 LuaState、proto 为其当前帧正在执行的活 Proto(VM 在 ecb.enter 处调用)。
  // 断言保证 execdata 非空、savedpc∈[code, code+sizecode), 故 pc_offset=savedpc-code 有界,
  // (*proto).execdata as *mut u32 指向 NativeProtoExecDataHeader 之后的指令偏移数组,
  // instruction_offsets.add(pc_offset) 在 sizecode 项界内。gate_entry 的类型化 ABI 契约见
  // records::native_fn::GateFn; &mut context 唯一借用。
  // 以下各窄块统一援引本契约。
  let code_gen_context = unsafe { get_code_gen_context(l) };

  // 指针判空为 safe 读（宏展开为 safe assert!）；后三条含存活对象字段直读。
  CODEGEN_ASSERT!(!code_gen_context.is_null());
  CODEGEN_ASSERT!(unsafe { !(*proto).execdata.is_null() });
  CODEGEN_ASSERT!(unsafe { (*(*l).ci).savedpc >= (*proto).code });
  CODEGEN_ASSERT!(unsafe { (*(*l).ci).savedpc < (*proto).code.add((*proto).sizecode as usize) });

  let pc_offset = unsafe { (*(*l).ci).savedpc.offset_from((*proto).code) as usize };
  let instruction_offsets = unsafe { (*proto).execdata as *mut u32 };
  let target = unsafe { (*proto).exectarget + *instruction_offsets.add(pc_offset) as usize };

  // Safety: gate_entry 由 init_header_functions 在 JIT 启用前注册为 Some（生成码入口，
  // 地址→Option<GateFn> 的一次收口在写入处完成），expect 取出的即该已知 ABI 入口，
  // 实参 l/proto/target 与 &mut context 唯一借用如上契约；未注册时 panic 而非跳空指针。
  let gate = unsafe {
    (*code_gen_context)
      .context
      .gate_entry
      .expect("JIT gate entry not registered")
  };
  unsafe { gate(l, proto, target, &mut (*code_gen_context).context) }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn on_enter_export(l: *mut LuaState, proto: *mut Proto) -> i32 {
  // Safety: 导出 C ABI 入口原样转发 l/proto 给同契约 unsafe fn on_enter; 调用方(VM 装入
  // ecb.enter)按 ABI 保证 l 活、proto 为其当前执行且有 execdata 的活 Proto, 满足被调前置条件。
  unsafe { on_enter(l, proto) }
}
