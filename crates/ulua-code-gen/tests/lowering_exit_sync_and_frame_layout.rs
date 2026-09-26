//! `IrLoweringX64` 构造期不变量 + X64 栈帧布局忠实性测试（审计轮 code-gen/lowering 条目 1、4）。
//!
//! 对齐 cpp：`CodeGen/include/Luau/IrLoweringX64.h:111-112`、
//! `CodeGen/src/EmitCommonX64.h:36-46,80-81,88-89`。

use core::ptr::null_mut;

use ulua_code_gen::{
  enums::abix_64::ABIX64,
  functions::{
    get_full_stack_size::{
      K_STACK_EXTRA_ARGUMENT_STORAGE, K_STACK_LOCAL_STORAGE, K_STACK_OFFSET_TO_LOCALS,
      K_STACK_OFFSET_TO_SPILL_SLOTS, K_STACK_REG_HOME_STORAGE, K_STACK_SPILL_STORAGE,
    },
    s_closure::s_closure,
    s_code::s_code,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{
      K_FUNCTION_ALIGNMENT, K_SPILL_SLOTS, R_BASE, R_CONSTANTS, R_NATIVE_CONTEXT, R_STATE,
    },
    ir_data::K_INVALID_INST_IDX,
    ir_function::IrFunction,
    ir_lowering_x_64::IrLoweringX64,
    module_helpers::ModuleHelpers,
    register_x_64::RegisterX64,
  },
};

/// 条目 1 的回归点：`exitSyncInstIdx` 初值必须是 `kInvalidInstIdx`。
///
/// cpp `IrLoweringX64.h:112` 是 `uint32_t exitSyncInstIdx = kInvalidInstIdx;`；
/// 若初始化成 0，函数**第 0 条指令**上的 ExitSync 分支会把"尚未记录过"误判为
/// "同一指令已记录过"，跳过 token 快照（`jumpOrAbortOnUndefNoFinalize` 的 else 分支），
/// 于是 `exitSync()` 的 alloc-token 校验拿到的是恒 0 的假快照。
#[test]
fn ctor_initializes_exit_sync_state_as_invalid() {
  let mut build =
    AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(false, ABIX64::Windows, 0);
  let mut helpers = ModuleHelpers::default();
  let mut function = IrFunction::default();

  let lowering = IrLoweringX64::ir_lowering_x_64_ir_lowering_x_64(
    &mut build,
    &mut helpers,
    &mut function,
    null_mut(),
  );

  assert_eq!(
    lowering.exit_sync_inst_idx, K_INVALID_INST_IDX,
    "exitSyncInstIdx 初值必须等于 kInvalidInstIdx，否则 index 0 的 ExitSync 会跳过 token 快照"
  );
  // cpp `IrLoweringX64.h:111` `uint32_t exitSyncAllocToken = 0;`
  assert_eq!(lowering.exit_sync_alloc_token, 0);
  // 二者不得初值相同：否则快照判断退化
  assert_ne!(lowering.exit_sync_inst_idx, lowering.exit_sync_alloc_token);
}

/// cpp `EmitCommonX64.h:46`
/// `static_assert((kExtraLocals + kSpillSlots) * 8 % 16 == 0, "locals have to preserve 16 byte alignment");`
/// Rust 侧没有编译期求值等价物来守护这条，只能靠测试锁定。
#[test]
fn frame_locals_area_preserves_16_byte_alignment() {
  assert_eq!(
    (K_STACK_LOCAL_STORAGE + K_STACK_SPILL_STORAGE) % 16,
    0,
    "locals 区必须保持 16 字节对齐（上游 static_assert）"
  );
}

/// 条目 4 的回归点：帧布局常量必须逐字等于上游数值，不能靠注释自证。
/// 锚点依次为 EmitCommonX64.h:45、:61、:62、:63、:64、:80、:81、:36。
#[test]
fn frame_layout_constants_match_upstream_literals() {
  // `inline constexpr unsigned kSpillSlots = 23;` —— 曾被注释写成 13
  assert_eq!(K_SPILL_SLOTS, 23, "EmitCommonX64.h:45");
  assert_eq!(K_STACK_LOCAL_STORAGE, 8 * 3, "EmitCommonX64.h:61");
  assert_eq!(K_STACK_SPILL_STORAGE, 8 * 23, "EmitCommonX64.h:62");
  assert_eq!(K_STACK_EXTRA_ARGUMENT_STORAGE, 2 * 8, "EmitCommonX64.h:63");
  assert_eq!(K_STACK_REG_HOME_STORAGE, 4 * 8, "EmitCommonX64.h:64");
  assert_eq!(
    K_STACK_OFFSET_TO_LOCALS,
    K_STACK_EXTRA_ARGUMENT_STORAGE + K_STACK_REG_HOME_STORAGE,
    "EmitCommonX64.h:80"
  );
  assert_eq!(K_STACK_OFFSET_TO_LOCALS, 48, "EmitCommonX64.h:80");
  assert_eq!(
    K_STACK_OFFSET_TO_SPILL_SLOTS,
    K_STACK_OFFSET_TO_LOCALS + K_STACK_LOCAL_STORAGE,
    "EmitCommonX64.h:81"
  );
  assert_eq!(K_STACK_OFFSET_TO_SPILL_SLOTS, 72, "EmitCommonX64.h:81");
  assert_eq!(K_FUNCTION_ALIGNMENT, 32, "EmitCommonX64.h:36");
}

/// 条目 3 的回归点：`lowerInst` 里曾被本地复制的一份常量/寄存器，现在只能来自单一来源。
/// 锚点为 EmitCommonX64.h:39-42（rState/rBase/rNativeContext/rConstants）。
#[test]
fn non_volatility_registers_and_stack_slots_have_single_source() {
  assert_eq!(R_STATE, RegisterX64::R15, "EmitCommonX64.h:39");
  assert_eq!(R_BASE, RegisterX64::R14, "EmitCommonX64.h:40");
  assert_eq!(R_NATIVE_CONTEXT, RegisterX64::R13, "EmitCommonX64.h:41");
  assert_eq!(R_CONSTANTS, RegisterX64::R12, "EmitCommonX64.h:42");

  // sClosure/sCode 的位移必须仍等于 kStackOffsetToLocals + 0/8（EmitCommonX64.h:88-89）
  assert_eq!(s_closure().imm, 48, "EmitCommonX64.h:88");
  assert_eq!(s_code().imm, 56, "EmitCommonX64.h:89");
}
