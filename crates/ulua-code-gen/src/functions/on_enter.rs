use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::{
  functions::get_code_gen_context::get_code_gen_context, macros::codegen_assert::CODEGEN_ASSERT,
};

/// 回跳入口（ecb.enter 槽位实现）：按当前 pc 取原生码跳转目标并交棒给 gate。
///
/// # Safety
/// `extern "C-unwind"` FFI 边界，由 VM（装入 `ecb.enter` 的一方）保证：`l` 为存活 `LuaState`、
/// `proto` 为其当前帧正在执行且已绑定 `execdata` 的存活 `Proto`。断言保证 `execdata` 非空、
/// `savedpc ∈ [code, code+sizecode)`，故 `pc_offset = savedpc - code` 有界，
/// `(*proto).execdata as *mut u32` 指向 `NativeProtoExecDataHeader` 之后的指令偏移数组，
/// `instruction_offsets.add(pc_offset)` 在 `sizecode` 项界内。`gate_entry` 由 `init_header_functions`
/// 在 JIT 启用前注册为 `Some`（类型化 ABI 契约见 `records::native_fn::GateFn`），两处 `expect` 的
/// 不可触发前提即上方断言对应的 VM 注册契约；`&mut ctx.context` 为串行回调窗口内的唯一借用。
pub unsafe extern "C-unwind" fn on_enter(l: *mut LuaState, proto: *mut Proto) -> i32 {
  // Safety: `get_code_gen_context` 依其 `# Safety` 在 VM 串行回调窗口内返回独占借用或 None。
  let code_gen_context = unsafe { get_code_gen_context(l) };

  // 上下文存在性判定（Option::is_some 为 safe）；后三条含存活对象字段直读。
  CODEGEN_ASSERT!(code_gen_context.is_some());
  CODEGEN_ASSERT!(unsafe { !(*proto).execdata.is_null() });
  CODEGEN_ASSERT!(unsafe { (*(*l).ci).savedpc >= (*proto).code });
  CODEGEN_ASSERT!(unsafe { (*(*l).ci).savedpc < (*proto).code.add((*proto).sizecode as usize) });

  let pc_offset = unsafe { (*(*l).ci).savedpc.offset_from((*proto).code) as usize };
  let instruction_offsets = unsafe { (*proto).execdata as *mut u32 };
  let target = unsafe { (*proto).exectarget + *instruction_offsets.add(pc_offset) as usize };

  // Safety: gate 由 init_header_functions 在 JIT 启用前注册，实参与 `&mut ctx.context` 唯一借用
  // 见函数头契约；未注册时 panic 而非跳空指针。
  let ctx = code_gen_context.expect("JIT code-gen context must be registered when ecb.enter fires");
  let gate = ctx
    .context
    .gate_entry
    .expect("JIT gate entry not registered");
  unsafe { gate(l, proto, target, &mut ctx.context) }
}
