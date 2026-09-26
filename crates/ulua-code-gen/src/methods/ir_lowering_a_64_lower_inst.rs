//! Source: `CodeGen/src/IrLoweringA64.cpp:308`
use core::{
  ffi::c_void,
  mem::{align_of, offset_of, size_of, size_of_val},
  ptr,
};

use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  macros::{blackbit::BLACKBIT, lua_multret::LUA_MULTRET},
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
    g_cheader::GCheader,
    global_state::global_State,
    lua_node::LuaNode,
    lua_state::LuaState,
    lua_t_value::TValue,
    lua_table::LuaTable,
    luau_buffer::LuauBuffer,
    proto::Proto,
    t_string::tstring,
    udata::Udata,
    up_val::UpVal,
  },
  type_aliases::{instruction::Instruction, luau_fast_function::LuauFastFunction, value::Value},
};

use crate::{
  enums::{
    address_kind_a_64::AddressKindA64, condition_a_64::ConditionA64, features_a_64::FeaturesA64,
    ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind,
    kind_a_64::KindA64,
  },
  macros::{
    codegen_assert::{CODEGEN_ASSERT, unsupported_instruction_form},
    ir_operand::{HAS_OP_B, HAS_OP_C, HAS_OP_D, HAS_OP_E},
  },
  records::{
    address_a_64::AddressA64,
    assembly_builder_a_64::{AssemblyBuilderA64, K_MAX_IMMEDIATE},
    interrupt_handler_ir_lowering_a_64::InterruptHandler,
    ir_block::IrBlock,
    ir_const::IrConst,
    ir_data::{K_INVALID_INST_IDX, K_NATIVE_PTR_SIZE},
    ir_inst::IrInst,
    ir_lowering_a_64::IrLoweringA64,
    ir_op::IrOp,
    label::Label,
    native_context::NativeContext,
    register_a_64::{RegisterA64, reg},
  },
};
// 本地寄存器/地址常量（对应 EmitCommonA64.h）
const INT_MAX: i32 = i32::MAX;
const K_TVALUE_SIZE_LOG2: i32 = 4;
const K_LUA_NODE_SIZE_LOG2: i32 = 5;
const K_OFFSET_OF_INSTRUCTION_C: i32 = 3;
// 结构体偏移由编译器按 Rust 布局计算, 对齐 cpp 的 offsetof(TString, len) / offsetof(Buffer, len)
const K_TSTRING_LEN_OFFSET: i32 = offset_of!(tstring, len) as i32;
const K_BUFFER_LEN_OFFSET: i32 = offset_of!(LuauBuffer, len) as i32;
const K_OFFSET_OF_TKEY_TAG_NEXT: i32 = 12;
const K_TKEY_TAG_BITS: i32 = 4;
const FEATURE_JSCVT: u32 = FeaturesA64::FeatureJscvt as u32;
const FEATURE_ADV_SIMD: u32 = FeaturesA64::FeatureAdvSimd as u32;
const LUA_TNIL: u8 = LuaType::Nil as u8;
const LUA_TBOOLEAN: u8 = LuaType::Boolean as u8;
const LUA_TNUMBER: u8 = LuaType::Number as u8;
const LUA_TINTEGER: u8 = LuaType::Integer as u8;
const LUA_TVECTOR: u8 = LuaType::Vector as u8;
const LUA_TSTRING: u8 = LuaType::String as u8;
const LUA_TUPVAL: u8 = LuaType::Upval as u8;
const K_TVALUE_VALUE_GC_OFFSET: i32 = (offset_of!(TValue, value) + offset_of!(Value, gc)) as i32;
const K_TVALUE_VALUE_N_OFFSET: i32 = (offset_of!(TValue, value) + offset_of!(Value, n)) as i32;
const K_TVALUE_VALUE_L_OFFSET: i32 = (offset_of!(TValue, value) + offset_of!(Value, l)) as i32;
const K_TVALUE_VALUE_P_OFFSET: i32 = (offset_of!(TValue, value) + offset_of!(Value, p)) as i32;
const K_CLOSURE_L_P_OFFSET: i32 = (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32;
const K_CLOSURE_L_UPREFS_OFFSET: i32 =
  (offset_of!(Closure, inner) + offset_of!(LClosure, uprefs)) as i32;

const X0: RegisterA64 = reg(KindA64::X, 0);
const X1: RegisterA64 = reg(KindA64::X, 1);
const X2: RegisterA64 = reg(KindA64::X, 2);
const X3: RegisterA64 = reg(KindA64::X, 3);
const X4: RegisterA64 = reg(KindA64::X, 4);
const X5: RegisterA64 = reg(KindA64::X, 5);
const X6: RegisterA64 = reg(KindA64::X, 6);
const W0: RegisterA64 = reg(KindA64::W, 0);
const W1: RegisterA64 = reg(KindA64::W, 1);
const W2: RegisterA64 = reg(KindA64::W, 2);
const W3: RegisterA64 = reg(KindA64::W, 3);
const W5: RegisterA64 = reg(KindA64::W, 5);
const D0: RegisterA64 = reg(KindA64::D, 0);
const D1: RegisterA64 = reg(KindA64::D, 1);
const Q0: RegisterA64 = reg(KindA64::Q, 0);
const WZR: RegisterA64 = reg(KindA64::W, 31);
const XZR: RegisterA64 = reg(KindA64::X, 31);
const NOREG: RegisterA64 = RegisterA64::NOREG;
const R_STATE: RegisterA64 = reg(KindA64::X, 19);
const R_NATIVE_CONTEXT: RegisterA64 = reg(KindA64::X, 20);
const R_GLOBAL_STATE: RegisterA64 = reg(KindA64::X, 21);
const R_CONSTANTS: RegisterA64 = reg(KindA64::X, 22);
const R_CLOSURE: RegisterA64 = reg(KindA64::X, 23);
const R_CODE: RegisterA64 = reg(KindA64::X, 24);
const R_BASE: RegisterA64 = reg(KindA64::X, 25);
const fn r_base() -> RegisterA64 {
  R_BASE
}
const fn r_constants() -> RegisterA64 {
  R_CONSTANTS
}
use crate::functions::{
  cast_reg::cast_reg,
  condition_op::condition_op,
  emit_abort::emit_abort,
  emit_add_offset::emit_add_offset as emit_add_offset_impl,
  emit_builtin_ir_lowering_a_64::emit_builtin_assembly_builder_a_64_ir_function_ir_reg_alloc_a_64_i32_i32_i32_i32 as emit_builtin,
  emit_fallback_ir_lowering_a_64::emit_fallback_assembly_builder_a_64_i32_i32 as emit_fallback_impl,
  emit_update_base_emit_common_a_64::emit_update_base,
  float_bits::{get_double_bits, get_float_bits},
  get_cmd_value_kind::get_cmd_value_kind,
  get_condition_fp::get_condition_fp,
  get_condition_int_64::get_condition_int_64,
  get_condition_int_ir_lowering_a_64::get_condition_int,
  get_inverse_condition_condition_a_64::get_inverse_condition,
  get_native_context_offset::get_native_context_offset,
  get_negated_condition_ir_utils::get_negated_condition_ir_condition as get_negated_condition,
  is_gco::is_gco,
  nvalue::nvalue,
  produces_dirty_high_register_bits::produces_dirty_high_register_bits,
  vm_const_op::vm_const_op,
  vm_reg_op::vm_reg_op,
  vm_upvalue_op::vm_upvalue_op as vm_upvalue_op_raw,
};

trait MemArg {
  fn address(self, base: RegisterA64) -> AddressA64;
}

impl MemArg for RegisterA64 {
  fn address(self, base: RegisterA64) -> AddressA64 {
    AddressA64::address_a_64_register_a_64_register_a_64(base, self)
  }
}

impl MemArg for i32 {
  fn address(self, base: RegisterA64) -> AddressA64 {
    AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(base, self, AddressKindA64::Imm)
  }
}

impl MemArg for u32 {
  fn address(self, base: RegisterA64) -> AddressA64 {
    (self as i32).address(base)
  }
}

impl MemArg for usize {
  fn address(self, base: RegisterA64) -> AddressA64 {
    (self as i32).address(base)
  }
}

fn mem<T: MemArg>(base: RegisterA64, data: T) -> AddressA64 {
  data.address(base)
}

fn mem_kind(base: RegisterA64, data: i32, kind: AddressKindA64) -> AddressA64 {
  AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(base, data, kind)
}

/// `mem(r_base, vmReg(op) * sizeof(TValue))`：VmReg 槽位的直接寻址形态。
/// 自由函数而不挂 `&mut self`，是为了在 `emit_ldr(.., vm_reg_addr(..))` 的实参位与
/// two-phase 借用共存；与化前的内联 `mem(r_base(), vm_reg_op(..) * size_of::<TValue>())` 逐位等价。
fn vm_reg_addr(op: IrOp) -> AddressA64 {
  mem(r_base(), vm_reg_op(op) * (size_of::<TValue>() as i32))
}

/// `mem(R_NATIVE_CONTEXT, off)`：NativeContext 字段的直接寻址形态（VM helper 函数指针、
/// 快速调用表基址等），化前为逐点 7 行内联展开的同形样板。
/// 入参取 `offset_of!` 的 `usize` 域，经 `MemArg for usize` 走与化前 `as i32` 相同的编码路径。
fn native_ctx(off: usize) -> AddressA64 {
  mem(R_NATIVE_CONTEXT, off)
}

/// `IrCmd::DoArith` 的算术 tag method → `NativeContext` 里对应 VM helper 字段偏移的静态分发表。
/// 化前是 8 条形如 `x if x == TMS::TmAdd as u32 => build_mut().ldr(X4, mem(R_NATIVE_CONTEXT, ..))`
/// 的 match 臂（每臂 7 行）；`TmAdd..TmUnm` 在 `TMS` 中 discriminant 连续，故以 `tm - TmAdd`
/// 直接索引本表，命中项与原臂的 `offset_of!` 常量逐一对应，分派语义不变。
const DOARITH_HELPER_OFFSETS: [usize; 8] = [
  offset_of!(NativeContext, lua_v_doarithadd),
  offset_of!(NativeContext, lua_v_doarithsub),
  offset_of!(NativeContext, lua_v_doarithmul),
  offset_of!(NativeContext, lua_v_doarithdiv),
  offset_of!(NativeContext, lua_v_doarithidiv),
  offset_of!(NativeContext, lua_v_doarithmod),
  offset_of!(NativeContext, lua_v_doarithpow),
  offset_of!(NativeContext, lua_v_doarithunm),
];

/// 编译期守住上表「连续 discriminant」前提：`TmAdd` 起第 8 个必须是 `TmUnm`。
const _: () =
  assert!(TMS::TmUnm as u32 - TMS::TmAdd as u32 == DOARITH_HELPER_OFFSETS.len() as u32 - 1);

trait OffsetArg {
  fn to_usize(self) -> usize;
}

impl OffsetArg for usize {
  fn to_usize(self) -> usize {
    self
  }
}

impl OffsetArg for u32 {
  fn to_usize(self) -> usize {
    self as usize
  }
}

impl OffsetArg for i32 {
  fn to_usize(self) -> usize {
    self as usize
  }
}

fn emit_add_offset<T: OffsetArg>(
  build: &mut AssemblyBuilderA64,
  dst: RegisterA64,
  src: RegisterA64,
  offset: T,
) {
  emit_add_offset_impl(build, dst, src, offset.to_usize());
}

trait I32Arg {
  fn to_i32(self) -> i32;
}

impl I32Arg for i32 {
  fn to_i32(self) -> i32 {
    self
  }
}

impl I32Arg for u32 {
  fn to_i32(self) -> i32 {
    self as i32
  }
}

fn emit_fallback<T: I32Arg>(build: &mut AssemblyBuilderA64, offset: i32, pcpos: T) {
  emit_fallback_impl(build, offset, pcpos.to_i32());
}

fn vm_upvalue_op(op: IrOp) -> i32 {
  vm_upvalue_op_raw(op) as i32
}

trait MovArg {
  fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64);
}

impl MovArg for RegisterA64 {
  fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.mov_register_a_64_register_a_64(dst, self);
  }
}

impl MovArg for i32 {
  fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.mov_register_a_64_i32(dst, self);
  }
}

// u32/u16/u8 三臂逐字同形（`build.mov_register_a_64_i32(dst, self as i32)`），坍缩进宏。
// 寄存器臂与无 cast 的 i32 臂（直接 `self`）保留手写。
crate::a64_arg_impls! {
  MovArg for u32 as i32 { mov_to: (dst: RegisterA64) => mov_register_a_64_i32; }
  MovArg for u16 as i32 { mov_to: (dst: RegisterA64) => mov_register_a_64_i32; }
  MovArg for u8 as i32 { mov_to: (dst: RegisterA64) => mov_register_a_64_i32; }
}

trait AddSubArg {
  fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
  fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
}

impl AddSubArg for RegisterA64 {
  fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.add_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
  }
  fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.sub_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
  }
}

impl AddSubArg for u16 {
  fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.add_register_a_64_register_a_64_u16(dst, src1, self);
  }
  fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.sub_register_a_64_register_a_64_u16(dst, src1, self);
  }
}

// i32/u32 两臂逐字同形(add/sub *_u16(dst, src1, self as u16)),坍缩进宏;
// 寄存器臂与无 cast 的 u16 臂(直接 self)保留手写。
crate::a64_arg_impls! {
  AddSubArg for i32 as u16 {
    add_to: (dst: RegisterA64, src1: RegisterA64) => add_register_a_64_register_a_64_u16;
    sub_from: (dst: RegisterA64, src1: RegisterA64) => sub_register_a_64_register_a_64_u16;
  }
  AddSubArg for u32 as u16 {
    add_to: (dst: RegisterA64, src1: RegisterA64) => add_register_a_64_register_a_64_u16;
    sub_from: (dst: RegisterA64, src1: RegisterA64) => sub_register_a_64_register_a_64_u16;
  }
}

trait LogicArg {
  fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
  fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
  fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
}

impl LogicArg for RegisterA64 {
  fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.and_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
  }
  fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.orr_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
  }
  fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.eor_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
  }
}

impl LogicArg for u32 {
  fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.and_register_a_64_register_a_64_u32(dst, src1, self);
  }
  fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.orr_register_a_64_register_a_64_u32(dst, src1, self);
  }
  fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.eor_register_a_64_register_a_64_u32(dst, src1, self);
  }
}

impl LogicArg for i32 {
  fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    (self as u32).and_to(build, dst, src1);
  }
  fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    (self as u32).orr_to(build, dst, src1);
  }
  fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    (self as u32).eor_to(build, dst, src1);
  }
}

trait ShiftArg {
  fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
  fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
  fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
  fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
}

impl ShiftArg for RegisterA64 {
  fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.lsl_register_a_64_register_a_64_register_a_64(dst, src1, self);
  }
  fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.lsr_register_a_64_register_a_64_register_a_64(dst, src1, self);
  }
  fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.asr_register_a_64_register_a_64_register_a_64(dst, src1, self);
  }
  fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.ror_register_a_64_register_a_64_register_a_64(dst, src1, self);
  }
}

impl ShiftArg for u8 {
  fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.lsl_register_a_64_register_a_64_u8(dst, src1, self);
  }
  fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.lsr_register_a_64_register_a_64_u8(dst, src1, self);
  }
  fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.asr_register_a_64_register_a_64_u8(dst, src1, self);
  }
  fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.ror_register_a_64_register_a_64_u8(dst, src1, self);
  }
}

impl ShiftArg for i32 {
  fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    (self as u8).lsl_to(build, dst, src1);
  }
  fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    (self as u8).lsr_to(build, dst, src1);
  }
  fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    (self as u8).asr_to(build, dst, src1);
  }
  fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    (self as u8).ror_to(build, dst, src1);
  }
}

trait CmpArg {
  fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64);
}

impl CmpArg for RegisterA64 {
  fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
    build.cmp_register_a_64_register_a_64(src1, self);
  }
}

impl CmpArg for u16 {
  fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
    build.cmp_register_a_64_u16(src1, self);
  }
}

// i32/u32 两臂逐字同形(cmp_register_a_64_u16(src1, self as u16)),坍缩进宏;
// 寄存器臂与无 cast 的 u16 臂(直接 self)保留手写。
crate::a64_arg_impls! {
  CmpArg for i32 as u16 { cmp_with: (src1: RegisterA64) => cmp_register_a_64_u16; }
  CmpArg for u32 as u16 { cmp_with: (src1: RegisterA64) => cmp_register_a_64_u16; }
}

trait FmovArg {
  fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64);
}

impl FmovArg for RegisterA64 {
  fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.fmov_register_a_64_register_a_64(dst, self);
  }
}

impl FmovArg for f32 {
  fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.fmov_register_a_64_f32(dst, self);
  }
}

impl FmovArg for f64 {
  fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.fmov_register_a_64_f64(dst, self);
  }
}

trait CcmnArg {
  fn ccmn_with(
    self,
    build: &mut AssemblyBuilderA64,
    src1: RegisterA64,
    cond: ConditionA64,
    nzcv: u8,
  );
}

impl CcmnArg for RegisterA64 {
  fn ccmn_with(
    self,
    build: &mut AssemblyBuilderA64,
    src1: RegisterA64,
    cond: ConditionA64,
    nzcv: u8,
  ) {
    build.ccmn_register_a_64_register_a_64_condition_a_64_u8(src1, self, cond, nzcv);
  }
}

impl CcmnArg for u8 {
  fn ccmn_with(
    self,
    build: &mut AssemblyBuilderA64,
    src1: RegisterA64,
    cond: ConditionA64,
    nzcv: u8,
  ) {
    build.ccmn_register_a_64_u8_condition_a_64_u8(src1, self, cond, nzcv);
  }
}

impl CcmnArg for i32 {
  fn ccmn_with(
    self,
    build: &mut AssemblyBuilderA64,
    src1: RegisterA64,
    cond: ConditionA64,
    nzcv: u8,
  ) {
    (self as u8).ccmn_with(build, src1, cond, nzcv);
  }
}

trait TstArg {
  fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64);
}

impl TstArg for RegisterA64 {
  fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
    build.tst_register_a_64_register_a_64_i32(src1, self, 0);
  }
}

impl TstArg for u32 {
  fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
    build.tst_register_a_64_u32(src1, self);
  }
}

impl TstArg for i32 {
  fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
    (self as u32).tst_with(build, src1);
  }
}

trait LowerInstBuilderExt {
  fn b(&mut self, label: &mut Label);
  fn mov<T: MovArg>(&mut self, dst: RegisterA64, src: T);
  fn add<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn sub<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn and_<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn orr<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn eor<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn lsl<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn lsr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn asr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn ror<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
  fn cmp<T: CmpArg>(&mut self, src1: RegisterA64, src2: T);
  fn fmov<T: FmovArg>(&mut self, dst: RegisterA64, src: T);
  fn ccmn<T: CcmnArg>(&mut self, src1: RegisterA64, src2: T, cond: ConditionA64, nzcv: u8);
  fn tst<T: TstArg>(&mut self, src1: RegisterA64, src2: T);
  fn sbfx(&mut self, dst: RegisterA64, src: RegisterA64, offset: u8, size: u8);
}

impl LowerInstBuilderExt for AssemblyBuilderA64 {
  fn b(&mut self, label: &mut Label) {
    self.b_label(label);
  }
  fn mov<T: MovArg>(&mut self, dst: RegisterA64, src: T) {
    src.mov_to(self, dst);
  }
  fn add<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.add_to(self, dst, src1);
  }
  fn sub<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.sub_from(self, dst, src1);
  }
  fn and_<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.and_to(self, dst, src1);
  }
  fn orr<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.orr_to(self, dst, src1);
  }
  fn eor<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.eor_to(self, dst, src1);
  }
  fn lsl<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.lsl_to(self, dst, src1);
  }
  fn lsr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.lsr_to(self, dst, src1);
  }
  fn asr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.asr_to(self, dst, src1);
  }
  fn ror<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    src2.ror_to(self, dst, src1);
  }
  fn cmp<T: CmpArg>(&mut self, src1: RegisterA64, src2: T) {
    src2.cmp_with(self, src1);
  }
  fn fmov<T: FmovArg>(&mut self, dst: RegisterA64, src: T) {
    src.fmov_to(self, dst);
  }
  fn ccmn<T: CcmnArg>(&mut self, src1: RegisterA64, src2: T, cond: ConditionA64, nzcv: u8) {
    src2.ccmn_with(self, src1, cond, nzcv);
  }
  fn tst<T: TstArg>(&mut self, src1: RegisterA64, src2: T) {
    src2.tst_with(self, src1);
  }
  fn sbfx(&mut self, dst: RegisterA64, src: RegisterA64, offset: u8, size: u8) {
    self.sbfx_register_a_64_register_a_64_u8_u8(dst, src, offset, size);
  }
}

fn get_condition_int64(cond: IrCondition) -> ConditionA64 {
  get_condition_int_64(cond)
}

fn get_condition_f_p(cond: IrCondition) -> ConditionA64 {
  get_condition_fp(cond)
}

impl IrLoweringA64 {
  // 以下操作数取值门面统一经 `function_ref` 视图访问器读取 IrFunction（裸指针解引用收口在
  // records 的 impl 注释契约），本组站点自身不再出现 `(*self.function)` 别名解引用。
  fn int_op(&self, op: IrOp) -> i32 {
    self.function_ref().int_op(op)
  }

  fn uint_op(&self, op: IrOp) -> u32 {
    self.function_ref().uint_op(op)
  }

  fn int_64_op(&mut self, op: IrOp) -> i64 {
    self.function_ref().int64_op(op)
  }

  fn double_op(&self, op: IrOp) -> f64 {
    self.function_ref().double_op(op)
  }

  fn tag_op(&self, op: IrOp) -> u8 {
    self.function_ref().tag_op(op)
  }

  fn const_op(&self, op: IrOp) -> IrConst {
    self.function_ref().const_op(op)
  }

  fn import_op(&self, op: IrOp) -> u32 {
    self.function_ref().import_op(op)
  }

  fn reg_op(&mut self, op: IrOp) -> RegisterA64 {
    self.ir_lowering_a_64_reg_op(op)
  }

  fn label_op(&mut self, op: IrOp) -> &mut Label {
    self.ir_lowering_a_64_label_op(op)
  }

  fn temp_addr(&mut self, op: IrOp, offset: i32) -> AddressA64 {
    self.ir_lowering_a_64_temp_addr(op, offset, RegisterA64::NOREG)
  }

  fn temp_double(&mut self, op: IrOp) -> RegisterA64 {
    self.ir_lowering_a_64_temp_double(op)
  }

  fn temp_int(&mut self, op: IrOp) -> RegisterA64 {
    self.ir_lowering_a_64_temp_int(op)
  }

  fn temp_uint(&mut self, op: IrOp) -> RegisterA64 {
    self.ir_lowering_a_64_temp_uint(op)
  }

  fn temp_int64(&mut self, op: IrOp) -> RegisterA64 {
    self.ir_lowering_a_64_temp_int_64(op)
  }

  /// `CmpTag` / `JumpEqTag` 共用：把操作数的 tag 物化到寄存器。
  /// `Inst` 操作数直接复用产出该值的寄存器；`VmReg` 操作数从栈槽 temp 里 `ldr` tt；
  /// `Constant` 操作数无寄存器可用（由调用方按 `tag_op` 内联立即数比较），返回 `NOREG`。
  fn tag_operand_reg(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      return self.reg_op(op);
    }
    if op.kind() == IrOpKind::VmReg {
      let reg = self.regs.alloc_temp(KindA64::W);
      let addr = self.temp_addr(op, (offset_of!(TValue, tt) as i32));
      self.build_mut().ldr(reg, addr);
      return reg;
    }
    CODEGEN_ASSERT!(op.kind() == IrOpKind::Constant);
    NOREG
  }

  fn temp_addr_buffer(&mut self, buffer_op: IrOp, index_op: IrOp, tag: u8) -> AddressA64 {
    self.ir_lowering_a_64_temp_addr_buffer(buffer_op, index_op, tag)
  }

  fn temp_float(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let val = self.double_op(op) as f32;

      // 注：build 指针契约见 `build_mut` 访问器（records 唯一 unsafe 收口点），
      // 此处仅调用其查询方法判定该 f32 常量是否可被 fmov 直接编码。
      if self.build_mut().is_fmov_supported_fp_32(val) {
        let temp = self.regs.alloc_temp(KindA64::S);
        self.build_mut().fmov_register_a_64_f32(temp, val);
        temp
      } else {
        let temp = self.regs.alloc_temp(KindA64::S);
        let vali = get_float_bits(val);

        if (vali & 0xffff) == 0 {
          let temp2 = self.regs.alloc_temp(KindA64::W);
          // 注：连续向 builder 追加 movz + fmov；temp/temp2 是刚分配、仍在作用域内的
          // 临时寄存器，不悬垂；指针契约见 `build_mut`。
          self.build_mut().movz(temp2, (vali >> 16) as u16, 16);
          self
            .build_mut()
            .fmov_register_a_64_register_a_64(temp, temp2);
        } else {
          let temp2 = self.regs.alloc_temp(KindA64::X);
          // 注：追加 adr + ldr；adr 写入的常量槽由 builder 自身管理，temp2 为刚分配且
          // 未释放的临时寄存器；指针契约见 `build_mut`。
          self.build_mut().adr_register_a_64_f32(temp2, val);
          self.build_mut().ldr(temp, mem(temp2, 0));
        }

        temp
      }
    } else {
      unsupported_instruction_form();
      RegisterA64::NOREG
    }
  }

  fn check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
    self.ir_lowering_a_64_check_safe_env(target, index, next)
  }

  /// `block_op` 的只读共享视图门面（unsafe 收口见 records 的
  /// `ir_lowering_a_64_block_op_ref`）。
  fn block_op_ref(&self, op: IrOp) -> &IrBlock {
    self.ir_lowering_a_64_block_op_ref(op)
  }

  fn is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
    self.ir_lowering_a_64_is_fallthrough_block(target, next)
  }

  fn jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
    self.ir_lowering_a_64_jump_or_fallthrough(target, next)
  }

  fn jump_or_fallthrough_op(&mut self, op: IrOp, next: &IrBlock) {
    self.with_block_mut(op, |s, t| s.jump_or_fallthrough(t, next));
  }

  fn check_object_barrier_conditions(
    &mut self,
    object: RegisterA64,
    temp: RegisterA64,
    ra: RegisterA64,
    ra_op: IrOp,
    ratag: i32,
    skip: &mut Label,
  ) {
    self.ir_lowering_a_64_check_object_barrier_conditions(object, temp, ra, ra_op, ratag, skip)
  }

  /// A64 浮点四则同构骨架（Idiv 的 Num/Vec 侧共享）：结果复用任一源，
  /// 双源经 `temp` 装载后发射 `emit`；装载与发射顺序与展开版逐位一致。
  fn lower_fa_bin(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    kind: KindA64,
    temp: fn(&mut Self, IrOp) -> RegisterA64,
    emit: fn(&mut Self, RegisterA64, RegisterA64, RegisterA64),
  ) {
    inst.reg_a64 = self
      .regs
      .alloc_reuse(kind, index, &[inst.op(0), inst.op(1)]);
    let src1 = temp(self, inst.op(0));
    let src2 = temp(self, inst.op(1));
    emit(self, inst.reg_a64, src1, src2);
  }

  /// A64 浮点一元同构骨架（Unm/Floor/Ceil/Round/Sqrt/Abs 的 Num/Float/Vec 侧共享）：
  /// 结果复用 op0，源经 `temp` 装载后发射 `emit`。
  fn lower_fa_un(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    kind: KindA64,
    temp: fn(&mut Self, IrOp) -> RegisterA64,
    emit: fn(&mut Self, RegisterA64, RegisterA64),
  ) {
    inst.reg_a64 = self.regs.alloc_reuse(kind, index, &[inst.op(0)]);
    let src = temp(self, inst.op(0));
    emit(self, inst.reg_a64, src);
  }

  /// Uint 位逻辑二元同构骨架（Bitand/Bitxor/Bitor 共享）：A 为 Inst 且 B 为可编码掩码
  /// 常量时直接发射立即数形式，否则经 temp_uint 装载后发射寄存器形式；发射顺序与展开版一致。
  fn lower_bit_logic_uint(&mut self, inst: &mut IrInst, index: u32) {
    inst.reg_a64 = self
      .regs
      .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);
    let op0 = inst.op(0);
    let op1 = inst.op(1);

    if op0.kind() == IrOpKind::Inst && op1.kind() == IrOpKind::Constant {
      let imm = self.int_op(op1) as u32;
      if self.emit_is_mask_supported(imm) {
        let src = self.reg_op(op0);
        match inst.cmd {
          IrCmd::BitandUint => self.build_mut().and_(inst.reg_a64, src, imm),
          IrCmd::BitxorUint => self.build_mut().eor(inst.reg_a64, src, imm),
          IrCmd::BitorUint => self.build_mut().orr(inst.reg_a64, src, imm),
          _ => unreachable!(),
        }
        return;
      }
    }
    let temp1 = self.temp_uint(op0);
    let temp2 = self.temp_uint(op1);
    match inst.cmd {
      IrCmd::BitandUint => self.build_mut().and_(inst.reg_a64, temp1, temp2),
      IrCmd::BitxorUint => self.build_mut().eor(inst.reg_a64, temp1, temp2),
      IrCmd::BitorUint => self.build_mut().orr(inst.reg_a64, temp1, temp2),
      _ => unreachable!(),
    }
  }

  /// Uint 移位同构骨架（Bitlshift/Bitrshift/Bitarshift/Bitrrotate 共享）：A 为 Inst 且
  /// B 为常量时按 (常量 & 31) 发射立即数移位，否则经 temp_uint 装载后发射寄存器移位；
  /// BitlrotateUint 因取负环绕语义与复用集不同保持展开。
  fn lower_bit_shift_uint(&mut self, inst: &mut IrInst, index: u32) {
    inst.reg_a64 = self
      .regs
      .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);
    let op0 = inst.op(0);
    let op1 = inst.op(1);

    if op0.kind() == IrOpKind::Inst && op1.kind() == IrOpKind::Constant {
      let src = self.reg_op(op0);
      let amount = ((self.int_op(op1) as u32) & 31) as u8;
      match inst.cmd {
        IrCmd::BitlshiftUint => self.build_mut().lsl(inst.reg_a64, src, amount),
        IrCmd::BitrshiftUint => self.build_mut().lsr(inst.reg_a64, src, amount),
        IrCmd::BitarshiftUint => self.build_mut().asr(inst.reg_a64, src, amount),
        IrCmd::BitrrotateUint => self.build_mut().ror(inst.reg_a64, src, amount),
        _ => unreachable!(),
      }
    } else {
      let temp1 = self.temp_uint(op0);
      let temp2 = self.temp_uint(op1);
      match inst.cmd {
        IrCmd::BitlshiftUint => self.build_mut().lsl(inst.reg_a64, temp1, temp2),
        IrCmd::BitrshiftUint => self.build_mut().lsr(inst.reg_a64, temp1, temp2),
        IrCmd::BitarshiftUint => self.build_mut().asr(inst.reg_a64, temp1, temp2),
        IrCmd::BitrrotateUint => self.build_mut().ror(inst.reg_a64, temp1, temp2),
        _ => unreachable!(),
      }
    }
  }

  fn lower_bit_logic_int_64(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderA64, RegisterA64, RegisterA64, RegisterA64),
  ) {
    inst.reg_a64 = self
      .regs
      .alloc_reuse(KindA64::X, index, &[(inst.op(0)), (inst.op(1))]);
    let temp1 = self.temp_int64(inst.op(0));
    let temp2 = self.temp_int64(inst.op(1));
    emit(self.build_mut(), inst.reg_a64, temp1, temp2);
  }

  /// Int64 移位对偶骨架（Bitlshift/Bitrshift 共享，两臂互为精确镜像）：先以
  /// `(amount + 63) > 126` 判 |amount| > 63（结果清零），再按 amount 符号分支——
  /// 正移位发射 `emit_main`，负移位取负后发射 `emit_reverse`；(emit_main, emit_reverse)
  /// 取 (lsl, lsr) / (lsr, lsl) 即得左右移两臂。BitarshiftInt64 因越界语义不同
  /// （正越界按符号填充而非清零）保持展开。
  fn lower_bit_shift_int_64(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit_main: fn(&mut AssemblyBuilderA64, RegisterA64, RegisterA64, RegisterA64),
    emit_reverse: fn(&mut AssemblyBuilderA64, RegisterA64, RegisterA64, RegisterA64),
  ) {
    inst.reg_a64 = self
      .regs
      .alloc_reuse(KindA64::X, index, &[(inst.op(0)), (inst.op(1))]);
    let source = self.temp_int64(inst.op(0));
    let amount = self.temp_int64(inst.op(1));
    let temp = self.regs.alloc_temp(KindA64::X);

    let mut done = Label::default();
    let mut negative = Label::default();
    let mut out_of_range = Label::default();

    // (amount + 63) > 126 = |amount| > 63
    self.build_mut().add(temp, amount, 63_u16);
    self.build_mut().cmp(temp, 126_u16);
    self.emit_bcond(ConditionA64::UnsignedGreater, &mut out_of_range);

    // 检查 amount 符号
    self.build_mut().cmp(amount, 0_u16);
    self.emit_bcond(ConditionA64::Less, &mut negative);

    // 主方向移位
    emit_main(self.build_mut(), inst.reg_a64, source, amount);
    self.build_mut().b(&mut done);

    // 按 -amount 反向移位
    self.build_mut().set_label_label(&mut negative);
    self.build_mut().neg(temp, amount);
    emit_reverse(self.build_mut(), inst.reg_a64, source, temp);
    self.build_mut().b(&mut done);

    self.build_mut().set_label_label(&mut out_of_range);
    self.build_mut().mov(inst.reg_a64, 0);

    self.build_mut().set_label_label(&mut done);
  }

  fn lower_buffer_read_int(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderA64, RegisterA64, AddressA64),
  ) {
    inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(1))]);
    let addr = self.temp_addr_buffer(inst.op(0), inst.op(1), self.tag_op(inst.op(2)));
    emit(self.build_mut(), inst.reg_a64, addr);
  }

  fn lower_buffer_write_int(
    &mut self,
    inst: &mut IrInst,
    emit: fn(&mut AssemblyBuilderA64, RegisterA64, AddressA64),
  ) {
    let temp = self.temp_int(inst.op(2));
    let addr = self.temp_addr_buffer(inst.op(0), inst.op(1), self.tag_op(inst.op(3)));
    emit(self.build_mut(), temp, addr);
  }

  fn lower_buffer_read_reg(&mut self, inst: &mut IrInst, index: u32, kind: KindA64) {
    inst.reg_a64 = self.regs.alloc_reg(kind, index);
    let addr = self.temp_addr_buffer(inst.op(0), inst.op(1), self.tag_op(inst.op(2)));
    self.build_mut().ldr(inst.reg_a64, addr);
  }

  fn lower_buffer_write_reg(&mut self, inst: &mut IrInst, temp: RegisterA64) {
    let addr = self.temp_addr_buffer(inst.op(0), inst.op(1), self.tag_op(inst.op(3)));
    self.build_mut().str(temp, addr);
  }

  /// LoadTag/LoadPointer/LoadDouble/LoadInt/LoadInt64/LoadFloat/LoadTvalue 七胞胎：
  /// 按臂选目标寄存器尺度与 TValue 内偏移（LoadTvalue 带 opb 时按其字节偏移取整只
  /// TValue），经 temp_addr 取栈/常量地址后 ldr 装载；发射序与展开版逐条一致。
  fn lower_load_scalar(&mut self, inst: &mut IrInst, index: u32) {
    let (kind, off) = match inst.cmd {
      IrCmd::LoadTag => (KindA64::W, offset_of!(TValue, tt) as i32),
      IrCmd::LoadPointer => (KindA64::X, K_TVALUE_VALUE_GC_OFFSET),
      IrCmd::LoadDouble => (KindA64::D, K_TVALUE_VALUE_N_OFFSET),
      IrCmd::LoadInt => (KindA64::W, offset_of!(TValue, value) as i32),
      IrCmd::LoadInt64 => (KindA64::X, K_TVALUE_VALUE_L_OFFSET),
      IrCmd::LoadFloat => (KindA64::S, self.int_op(inst.op(1))),
      _ => (
        KindA64::Q,
        if HAS_OP_B!(inst) {
          self.int_op(inst.op(1))
        } else {
          0
        },
      ),
    };
    inst.reg_a64 = self.regs.alloc_reg(kind, index);
    let addr = self.temp_addr(inst.op(0), off);
    self.build_mut().ldr(inst.reg_a64, addr);
  }

  pub fn ir_lowering_a_64_lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
    // 本函数是安全的：对 build/function/helpers/label 裸指针的每次解引用都收在
    // `records/ir_lowering_a_64.rs` 的视图/闭包窗门面（`build_mut`/`with_op_label`/
    // `with_target_label`/`with_helper_label` 等，各带 Safety 契约）内，
    // 本巨文件内不再出现 unsafe 与裸指针解引用。
    self.regs.curr_inst_idx = index;

    self.value_tracker.before_inst_lowering(inst);
    match inst.cmd {
      IrCmd::LoadTag => self.lower_load_scalar(inst, index),
      IrCmd::LoadPointer => self.lower_load_scalar(inst, index),
      IrCmd::LoadDouble => self.lower_load_scalar(inst, index),
      IrCmd::LoadInt => self.lower_load_scalar(inst, index),
      IrCmd::LoadInt64 => self.lower_load_scalar(inst, index),
      IrCmd::LoadFloat => self.lower_load_scalar(inst, index),
      IrCmd::LoadTvalue => self.lower_load_scalar(inst, index),
      IrCmd::LoadEnv => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
        self.build_mut().ldr(
          inst.reg_a64,
          mem(R_CLOSURE, (offset_of!(Closure, env) as i32)),
        );
      }
      IrCmd::GetArrAddr => {
        {
          inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
          let hoist_0 = self.reg_op(inst.op(0));
          self.emit_ldr(
            inst.reg_a64,
            mem(hoist_0, (offset_of!(LuaTable, array) as i32)),
          );

          if (inst.op(1)).kind() == IrOpKind::Inst {
            let hoist_1 = self.reg_op(inst.op(1));
            self.emit_add_register_a_64_register_a_64_register_a_64_i32(
              inst.reg_a64,
              inst.reg_a64,
              hoist_1,
              K_TVALUE_SIZE_LOG2,
            ); // implicit uxtw
          } else if (inst.op(1)).kind() == IrOpKind::Constant {
            if self.int_op(inst.op(1)) == 0 {
              // 无需偏移
            } else {
              // cpp `intOp(OP_B(inst)) * sizeof(TValue)`：int 提升为 size_t 后
              // 无符号 64 位乘法与比较（IrLoweringA64.cpp GET_ARR_ADDR 同臂），
              // 超出 imm 范围时 mov 回退加载 32 位截断值
              let stride =
                (self.int_op(inst.op(1)) as i64 as u64).wrapping_mul(size_of::<TValue>() as u64);
              if stride <= K_MAX_IMMEDIATE as u64 {
                self
                  .build_mut()
                  .add(inst.reg_a64, inst.reg_a64, stride as u16);
              } else {
                let temp = self.regs.alloc_temp(KindA64::X);
                self.build_mut().mov(temp, stride as u32 as i32);
                self.build_mut().add(inst.reg_a64, inst.reg_a64, temp);
              }
            }
          } else {
            unsupported_instruction_form();
          }
        }
      }
      IrCmd::GetSlotNodeAddr => {
        {
          inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp1w = cast_reg(KindA64::W, temp1);
          let temp2 = self.regs.alloc_temp(KindA64::W);
          let temp2x = cast_reg(KindA64::X, temp2);

          // 注：load 的 stride 与目标寄存器尺寸相同，故可对数组下标而非字节偏移做范围检查
          if self.uint_op(inst.op(1)) <= AddressA64::K_MAX_OFFSET as u32 {
            self.emit_ldr(
              temp1w,
              mem(
                R_CODE,
                (self.uint_op(inst.op(1)) as i32) * (size_of::<Instruction>() as i32),
              ),
            );
          } else {
            self.emit_mov(
              temp1,
              (self.uint_op(inst.op(1)) as i32) * (size_of::<Instruction>() as i32),
            );
            self.build_mut().ldr(temp1w, mem(R_CODE, temp1));
          }

          // C 字段只要位于指令字的最高字节就可以移位
          CODEGEN_ASSERT!(K_OFFSET_OF_INSTRUCTION_C == 3);
          let hoist_2 = self.reg_op(inst.op(0));
          self.emit_ldrb(
            temp2,
            mem(hoist_2, (offset_of!(LuaTable, nodemask8) as i32)),
          );
          self
            .build_mut()
            .and_register_a_64_register_a_64_register_a_64_i32(temp2, temp2, temp1w, -24);

          // 注：这可能 clobber (inst.op(0))，此后切记不要再使用它
          let hoist_3 = self.reg_op(inst.op(0));
          self.emit_ldr(
            inst.reg_a64,
            mem(hoist_3, (offset_of!(LuaTable, node) as i32)),
          );
          self
            .build_mut()
            .add_register_a_64_register_a_64_register_a_64_i32(
              inst.reg_a64,
              inst.reg_a64,
              temp2x,
              K_LUA_NODE_SIZE_LOG2,
            ); // "zero extend" temp2 to get a larger shift (top 32 bits are zero)
        }
      }
      IrCmd::GetHashNodeAddr => {
        {
          inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
          let temp1 = self.regs.alloc_temp(KindA64::W);
          let temp2 = self.regs.alloc_temp(KindA64::W);
          let temp2x = cast_reg(KindA64::X, temp2);

          // hash & ((1 << lsizenode) - 1) == hash & !(-1 << lsizenode)
          self.build_mut().mov(temp1, -1);
          let hoist_4 = self.reg_op(inst.op(0));
          self.emit_ldrb(
            temp2,
            mem(hoist_4, (offset_of!(LuaTable, lsizenode) as i32)),
          );
          self.build_mut().lsl(temp1, temp1, temp2);
          self.emit_mov(temp2, self.uint_op(inst.op(1)));
          self.build_mut().bic(temp2, temp2, temp1, 0);

          // 注：这可能 clobber (inst.op(0))，此后切记不要再使用它
          let hoist_5 = self.reg_op(inst.op(0));
          self.emit_ldr(
            inst.reg_a64,
            mem(hoist_5, (offset_of!(LuaTable, node) as i32)),
          );
          self
            .build_mut()
            .add_register_a_64_register_a_64_register_a_64_i32(
              inst.reg_a64,
              inst.reg_a64,
              temp2x,
              K_LUA_NODE_SIZE_LOG2,
            ); // "zero extend" temp2 to get a larger shift (top 32 bits are zero)
        }
      }
      IrCmd::GetClosureUpvalAddr => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
        let cl = if (inst.op(0)).kind() == IrOpKind::Undef {
          R_CLOSURE
        } else {
          self.reg_op(inst.op(0))
        };

        self.build_mut().add(
          inst.reg_a64,
          cl,
          (K_CLOSURE_L_UPREFS_OFFSET + (size_of::<TValue>() as i32) * vm_upvalue_op(inst.op(1)))
            as u16,
        );
      }
      IrCmd::StoreTag | IrCmd::StoreExtra => {
        // tt/extra 双胞胎：小立即数为零直写 WZR，非零先落 W 临时再 str
        let (off, v) = if inst.cmd == IrCmd::StoreTag {
          (
            offset_of!(TValue, tt) as i32,
            self.tag_op(inst.op(1)) as i32,
          )
        } else {
          (offset_of!(TValue, extra) as i32, self.int_op(inst.op(1)))
        };

        let addr = self.temp_addr(inst.op(0), off);
        if v == 0 {
          self.build_mut().str(WZR, addr);
        } else {
          let temp = self.regs.alloc_temp(KindA64::W);
          self.emit_mov(temp, v);
          self.build_mut().str(temp, addr);
        }
      }
      IrCmd::StorePointer => {
        let addr = self.temp_addr(inst.op(0), (offset_of!(TValue, value) as i32));
        if (inst.op(1)).kind() == IrOpKind::Constant {
          CODEGEN_ASSERT!(self.int_op(inst.op(1)) == 0);
          self.build_mut().str(XZR, addr);
        } else {
          let hoist_6 = self.reg_op(inst.op(1));
          self.emit_str(hoist_6, addr);
        }
      }
      IrCmd::StoreDouble => {
        let addr = self.temp_addr(inst.op(0), (offset_of!(TValue, value) as i32));
        if (inst.op(1)).kind() == IrOpKind::Constant
          && get_double_bits(self.double_op(inst.op(1))) == 0
        {
          self.build_mut().str(XZR, addr);
        } else {
          let temp = self.temp_double(inst.op(1));
          self.build_mut().str(temp, addr);
        }
      }
      IrCmd::StoreInt => {
        let addr = self.temp_addr(inst.op(0), (offset_of!(TValue, value) as i32));
        if (inst.op(1)).kind() == IrOpKind::Constant && self.int_op(inst.op(1)) == 0 {
          self.build_mut().str(WZR, addr);
        } else {
          let temp = self.temp_int(inst.op(1));
          self.build_mut().str(temp, addr);
        }
      }
      IrCmd::StoreInt64 => {
        let addr = self.temp_addr(inst.op(0), (offset_of!(TValue, value) as i32));
        if (inst.op(1)).kind() == IrOpKind::Constant && self.int_64_op(inst.op(1)) == 0 {
          self.build_mut().str(XZR, addr);
        } else {
          let temp = self.temp_int64(inst.op(1));
          self.build_mut().str(temp, addr);
        }
      }
      IrCmd::StoreVector => {
        let temp1 = self.temp_float(inst.op(1));
        let temp2 = self.temp_float(inst.op(2));
        let temp3 = self.temp_float(inst.op(3));

        let addr = self.temp_addr(inst.op(0), (offset_of!(TValue, value) as i32));
        CODEGEN_ASSERT!(
          addr.kind == AddressKindA64::Imm
            && addr.data % 4 == 0
            && ((addr.data + 8) as u32) / 4 <= AddressA64::K_MAX_OFFSET as u32
        );

        self.build_mut().str(temp1, mem(addr.base, addr.data));
        self.build_mut().str(temp2, mem(addr.base, addr.data + 4));
        self.build_mut().str(temp3, mem(addr.base, addr.data + 8));

        if HAS_OP_E!(inst) {
          let temp = self.regs.alloc_temp(KindA64::W);
          self.emit_mov(temp, self.tag_op(inst.op(4)));
          let hoist_7 = self.temp_addr(inst.op(0), (offset_of!(TValue, tt) as i32));
          self.emit_str(temp, hoist_7);
        }
      }
      IrCmd::StoreTvalue => {
        let addr_offset = if HAS_OP_C!(inst) {
          self.int_op(inst.op(2))
        } else {
          0
        };
        let addr = self.temp_addr(inst.op(0), addr_offset);
        let hoist_8 = self.reg_op(inst.op(1));
        self.emit_str(hoist_8, addr);
      }
      IrCmd::StoreSplitTvalue => {
        {
          let addr_offset = if HAS_OP_D!(inst) {
            self.int_op(inst.op(3))
          } else {
            0
          };

          let tempt = self.regs.alloc_temp(KindA64::W);
          let addrt = self.temp_addr(inst.op(0), (offset_of!(TValue, tt) as i32) + addr_offset);
          self.emit_mov(tempt, self.tag_op(inst.op(1)));
          self.build_mut().str(tempt, addrt);

          let addr = self.temp_addr(inst.op(0), (offset_of!(TValue, value) as i32) + addr_offset);

          if self.tag_op(inst.op(1)) == LUA_TBOOLEAN {
            if (inst.op(2)).kind() == IrOpKind::Constant {
              // 注：true 布尔值复用 tag 临时寄存器作值，false 值用内置零寄存器
              CODEGEN_ASSERT!(LUA_TBOOLEAN == 1);
              self.emit_str(
                if self.int_op(inst.op(2)) != 0 {
                  tempt
                } else {
                  WZR
                },
                addr,
              );
            } else {
              let hoist_9 = self.reg_op(inst.op(2));
              self.emit_str(hoist_9, addr);
            }
          } else if self.tag_op(inst.op(1)) == LUA_TNUMBER {
            let temp = self.temp_double(inst.op(2));
            self.build_mut().str(temp, addr);
          } else if self.tag_op(inst.op(1)) == LUA_TINTEGER {
            let temp = self.temp_int64(inst.op(2));
            self.build_mut().str(temp, addr);
          } else if is_gco(self.tag_op(inst.op(1))) {
            let hoist_10 = self.reg_op(inst.op(2));
            self.emit_str(hoist_10, addr);
          } else {
            unsupported_instruction_form();
          }
        }
      }
      IrCmd::AddInt => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);
        if (inst.op(1)).kind() == IrOpKind::Constant
          && ((self.int_op(inst.op(1))) as u32) <= K_MAX_IMMEDIATE as u32
        {
          let hoist_11 = self.reg_op(inst.op(0));
          self.emit_add(inst.reg_a64, hoist_11, (self.int_op(inst.op(1))) as u16);
        } else if (inst.op(0)).kind() == IrOpKind::Constant
          && ((self.int_op(inst.op(0))) as u32) <= K_MAX_IMMEDIATE as u32
        {
          let hoist_12 = self.reg_op(inst.op(1));
          self.emit_add(inst.reg_a64, hoist_12, (self.int_op(inst.op(0))) as u16);
        } else {
          let temp1 = self.temp_int(inst.op(0));
          let temp2 = self.temp_int(inst.op(1));
          self.build_mut().add(inst.reg_a64, temp1, temp2);
        }
      }
      IrCmd::SubInt => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);
        if (inst.op(1)).kind() == IrOpKind::Constant
          && ((self.int_op(inst.op(1))) as u32) <= K_MAX_IMMEDIATE as u32
        {
          let hoist_13 = self.reg_op(inst.op(0));
          self.emit_sub(inst.reg_a64, hoist_13, (self.int_op(inst.op(1))) as u16);
        } else {
          let temp1 = self.temp_int(inst.op(0));
          let temp2 = self.temp_int(inst.op(1));
          self.build_mut().sub(inst.reg_a64, temp1, temp2);
        }
      }
      IrCmd::AddInt64 => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::X, index, &[(inst.op(0)), (inst.op(1))]);
        if (inst.op(1)).kind() == IrOpKind::Constant
          && ((self.int_64_op(inst.op(1))) as u64) <= K_MAX_IMMEDIATE as u64
        {
          let hoist_14 = self.temp_int64(inst.op(0));
          let hoist_0 = self.int_64_op(inst.op(1));
          self.emit_add(inst.reg_a64, hoist_14, hoist_0 as u16);
        } else if (inst.op(0)).kind() == IrOpKind::Constant
          && ((self.int_64_op(inst.op(0))) as u64) <= K_MAX_IMMEDIATE as u64
        {
          let hoist_15 = self.temp_int64(inst.op(1));
          let hoist_1 = self.int_64_op(inst.op(0));
          self.emit_add(inst.reg_a64, hoist_15, hoist_1 as u16);
        } else {
          let temp1 = self.temp_int64(inst.op(0));
          let temp2 = self.temp_int64(inst.op(1));
          self.build_mut().add(inst.reg_a64, temp1, temp2);
        }
      }
      IrCmd::SubInt64 => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::X, index, &[(inst.op(0)), (inst.op(1))]);
        if (inst.op(1)).kind() == IrOpKind::Constant
          && ((self.int_64_op(inst.op(1))) as u64) <= K_MAX_IMMEDIATE as u64
        {
          let hoist_16 = self.temp_int64(inst.op(0));
          let hoist_2 = self.int_64_op(inst.op(1));
          self.emit_sub(inst.reg_a64, hoist_16, hoist_2 as u16);
        } else {
          let temp1 = self.temp_int64(inst.op(0));
          let temp2 = self.temp_int64(inst.op(1));
          self.build_mut().sub(inst.reg_a64, temp1, temp2);
        }
      }
      IrCmd::MulInt64 | IrCmd::DivInt64 | IrCmd::UdivInt64 => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::X, index, &[(inst.op(0)), (inst.op(1))]);
        let temp1 = self.temp_int64(inst.op(0));
        let temp2 = self.temp_int64(inst.op(1));
        match inst.cmd {
          IrCmd::MulInt64 => self.build_mut().mul(inst.reg_a64, temp1, temp2),
          IrCmd::DivInt64 => self.build_mut().sdiv(inst.reg_a64, temp1, temp2),
          IrCmd::UdivInt64 => self.build_mut().udiv(inst.reg_a64, temp1, temp2),
          _ => unreachable!(),
        }
      }
      IrCmd::IdivInt64 => {
        // 向下取整除法：q = a / b，若 (q < 0 && a % b != 0) 则 q -= 1
        inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index); // can't reuse: both operands needed for remainder
        {
          let temp1 = self.temp_int64(inst.op(0));
          let temp2 = self.temp_int64(inst.op(1));
          let temp_rem = self.regs.alloc_temp(KindA64::X);
          let temp_adj = self.regs.alloc_temp(KindA64::X);

          self.build_mut().sdiv(inst.reg_a64, temp1, temp2); // result = a / b
          self.build_mut().mov(temp_rem, inst.reg_a64); // copy quotient; rem requires dst to initially hold quotient
          self.build_mut().rem(temp_rem, temp1, temp2);

          self.build_mut().sub(temp_adj, inst.reg_a64, 1_u16); // adjusted = result - 1

          self.build_mut().cmp(temp_rem, 0_u16);
          self
            .build_mut()
            .csel(temp_adj, temp_adj, inst.reg_a64, ConditionA64::NotEqual); // (remainder != 0) ? result-1 : result

          self.build_mut().cmp(inst.reg_a64, 0_u16);
          self
            .build_mut()
            .csel(inst.reg_a64, temp_adj, inst.reg_a64, ConditionA64::Less);
          // (result < 0) ? temp_adj : result
        }
      }
      IrCmd::CheckDivInt64 => {
        {
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let fail = self.ir_lowering_a_64_get_target_label(inst.op(2), index, &mut fresh);

          // 防护除零
          let reg_b = self.temp_int64(inst.op(1));
          self.with_target_label(fail, |s, l| s.build_mut().cbz(reg_b, l));

          // 防护 a 为 -2^63 且 b 为 -1 的情形
          let reg_a = self.temp_int64(inst.op(0));
          let temp_rotate = self.regs.alloc_temp(KindA64::X);

          // 位技巧：若为 integer.minsigned (0x8000000000000000)，旋转 63 位即得 1
          self.build_mut().ror(temp_rotate, reg_a, 63);

          self.build_mut().cmp(temp_rotate, 1_u16);

          // nzcv = 0000 EQ
          // nzcv = 0001 NE
          self
            .build_mut()
            .ccmn(reg_b, 1, get_condition_int64(IrCondition::Equal), 1);
          self.with_target_label(fail, |s, l| {
            s.build_mut()
              .b_condition_a_64_label(get_condition_int64(IrCondition::Equal), l)
          });

          self.ir_lowering_a_64_finalize_target_label(inst.op(2), index, &mut fresh);
        }
      }
      IrCmd::RemInt64 | IrCmd::UremInt64 => {
        // C 截断模：先取商（符号性按臂分派），再 msub 得余数
        inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
        {
          let temp1 = self.temp_int64(inst.op(0));
          let temp2 = self.temp_int64(inst.op(1));
          match inst.cmd {
            IrCmd::RemInt64 => self.build_mut().sdiv(inst.reg_a64, temp1, temp2),
            _ => self.build_mut().udiv(inst.reg_a64, temp1, temp2),
          }
          self.build_mut().rem(inst.reg_a64, temp1, temp2);
        }
      }
      IrCmd::ModInt64 => {
        // 向下取整模运算：rem = a % b（C 截断）；若 (rem != 0 && sign(rem) != sign(b)) 则 rem += b
        inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index); // can't reuse: dividend (temp1) needed after sdiv
        {
          let temp1 = self.temp_int64(inst.op(0));
          let temp2 = self.temp_int64(inst.op(1));
          let temp_rem = self.regs.alloc_temp(KindA64::X);
          let temp_adj = self.regs.alloc_temp(KindA64::X);

          self.build_mut().sdiv(inst.reg_a64, temp1, temp2); // quotient = a / b
          self.build_mut().mov(temp_rem, inst.reg_a64); // temp_rem = quotient
          self.build_mut().rem(temp_rem, temp1, temp2); // temp_rem = C-style remainder

          self.build_mut().add(temp_adj, temp_rem, temp2); // temp_adj = rem + b (floored candidate)
          self.build_mut().eor(inst.reg_a64, temp_rem, temp2); // sign check: negative if signs differ

          self.build_mut().cmp(inst.reg_a64, 0_u16);
          self
            .build_mut()
            .csel(temp_adj, temp_adj, temp_rem, ConditionA64::Less); // if signs differ then rem+b else rem

          self.build_mut().cmp(temp_rem, 0_u16);
          self
            .build_mut()
            .csel(inst.reg_a64, temp_adj, temp_rem, ConditionA64::NotEqual);
          // rem != 0 时取调整值，否则为 0
        }
      }
      IrCmd::Sexti8Int => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);

        let hoist_17 = self.reg_op(inst.op(0));
        self.emit_sbfx(inst.reg_a64, hoist_17, 0, 8);
        // sextb
      }
      IrCmd::Sexti16Int => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);

        let hoist_18 = self.reg_op(inst.op(0));
        self.emit_sbfx(inst.reg_a64, hoist_18, 0, 16);
        // sexth
      }
      IrCmd::AddNum | IrCmd::SubNum | IrCmd::MulNum | IrCmd::DivNum => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::D, index, &[(inst.op(0)), (inst.op(1))]);
        let temp1 = self.temp_double(inst.op(0));
        let temp2 = self.temp_double(inst.op(1));
        match inst.cmd {
          IrCmd::AddNum => self.build_mut().fadd(inst.reg_a64, temp1, temp2),
          IrCmd::SubNum => self.build_mut().fsub(inst.reg_a64, temp1, temp2),
          IrCmd::MulNum => self.build_mut().fmul(inst.reg_a64, temp1, temp2),
          IrCmd::DivNum => self.build_mut().fdiv(inst.reg_a64, temp1, temp2),
          _ => unreachable!(),
        }
      }
      IrCmd::IdivNum => {
        self.lower_fa_bin(inst, index, KindA64::D, Self::temp_double, Self::emit_fdiv);
        self.build_mut().frintm(inst.reg_a64, inst.reg_a64);
      }
      IrCmd::ModNum => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index); // can't allocReuse because both A and B are used twice
          let temp1 = self.temp_double(inst.op(0));
          let temp2 = self.temp_double(inst.op(1));
          self.build_mut().fdiv(inst.reg_a64, temp1, temp2);
          self.build_mut().frintm(inst.reg_a64, inst.reg_a64);
          self.build_mut().fmul(inst.reg_a64, inst.reg_a64, temp2);
          self.build_mut().fsub(inst.reg_a64, temp1, inst.reg_a64);
        }
      }
      IrCmd::MuladdNum => {
        let temp_a = self.temp_double(inst.op(0));
        let temp_b = self.temp_double(inst.op(1));
        let temp_c = self.temp_double(inst.op(2));

        if (self.build_mut().features & FEATURE_ADV_SIMD) != 0 {
          inst.reg_a64 = self.regs.alloc_reuse(KindA64::D, index, &[(inst.op(2))]);
          if inst.reg_a64 != temp_c {
            self.build_mut().fmov(inst.reg_a64, temp_c);
          }
          self.build_mut().fmla(inst.reg_a64, temp_b, temp_a);
        } else {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
          self.build_mut().fmul(inst.reg_a64, temp_b, temp_a);
          self.build_mut().fadd(inst.reg_a64, inst.reg_a64, temp_c);
        }
      }
      IrCmd::MinNum => {
        self.lower_min_max(
          inst,
          index,
          KindA64::D,
          Self::temp_double,
          IrCondition::Less,
        );
      }
      IrCmd::MaxNum => {
        self.lower_min_max(
          inst,
          index,
          KindA64::D,
          Self::temp_double,
          IrCondition::Greater,
        );
      }
      IrCmd::UnmNum => {
        self.lower_fa_un(inst, index, KindA64::D, Self::temp_double, Self::emit_fneg);
      }
      IrCmd::FloorNum => {
        self.lower_fa_un(
          inst,
          index,
          KindA64::D,
          Self::temp_double,
          Self::emit_frintm,
        );
      }
      IrCmd::CeilNum => {
        self.lower_fa_un(
          inst,
          index,
          KindA64::D,
          Self::temp_double,
          Self::emit_frintp,
        );
      }
      IrCmd::RoundNum => {
        self.lower_fa_un(
          inst,
          index,
          KindA64::D,
          Self::temp_double,
          Self::emit_frinta,
        );
      }
      IrCmd::SqrtNum => {
        self.lower_fa_un(inst, index, KindA64::D, Self::temp_double, Self::emit_fsqrt);
      }
      IrCmd::AbsNum => {
        self.lower_fa_un(inst, index, KindA64::D, Self::temp_double, Self::emit_fabs);
      }
      IrCmd::SignNum => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::D, index, &[(inst.op(0))]);

        let temp = self.temp_double(inst.op(0));
        let temp0 = self.regs.alloc_temp(KindA64::D);
        let temp1 = self.regs.alloc_temp(KindA64::D);

        self.build_mut().fcmpz(temp);
        self.build_mut().fmov(temp0, 0.0);
        self.build_mut().fmov(temp1, 1.0);
        self.build_mut().fcsel(
          inst.reg_a64,
          temp1,
          temp0,
          get_condition_f_p(IrCondition::Greater),
        );
        self.build_mut().fmov(temp1, -1.0);
        self.build_mut().fcsel(
          inst.reg_a64,
          temp1,
          inst.reg_a64,
          get_condition_f_p(IrCondition::Less),
        );
      }
      IrCmd::AddFloat | IrCmd::SubFloat | IrCmd::MulFloat | IrCmd::DivFloat => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::S, index, &[(inst.op(0)), (inst.op(1))]);
        let temp1 = self.temp_float(inst.op(0));
        let temp2 = self.temp_float(inst.op(1));
        match inst.cmd {
          IrCmd::AddFloat => self.build_mut().fadd(inst.reg_a64, temp1, temp2),
          IrCmd::SubFloat => self.build_mut().fsub(inst.reg_a64, temp1, temp2),
          IrCmd::MulFloat => self.build_mut().fmul(inst.reg_a64, temp1, temp2),
          IrCmd::DivFloat => self.build_mut().fdiv(inst.reg_a64, temp1, temp2),
          _ => unreachable!(),
        }
      }
      IrCmd::MinFloat => {
        self.lower_min_max(inst, index, KindA64::S, Self::temp_float, IrCondition::Less);
      }
      IrCmd::MaxFloat => {
        self.lower_min_max(
          inst,
          index,
          KindA64::S,
          Self::temp_float,
          IrCondition::Greater,
        );
      }
      IrCmd::UnmFloat => {
        self.lower_fa_un(inst, index, KindA64::S, Self::temp_float, Self::emit_fneg);
      }
      IrCmd::FloorFloat => {
        self.lower_fa_un(inst, index, KindA64::S, Self::temp_float, Self::emit_frintm);
      }
      IrCmd::CeilFloat => {
        self.lower_fa_un(inst, index, KindA64::S, Self::temp_float, Self::emit_frintp);
      }
      IrCmd::SqrtFloat => {
        self.lower_fa_un(inst, index, KindA64::S, Self::temp_float, Self::emit_fsqrt);
      }
      IrCmd::AbsFloat => {
        self.lower_fa_un(inst, index, KindA64::S, Self::temp_float, Self::emit_fabs);
      }
      IrCmd::SignFloat => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::S, index, &[(inst.op(0))]);

        let temp = self.temp_float(inst.op(0));
        let temp0 = self.regs.alloc_temp(KindA64::S);
        let temp1 = self.regs.alloc_temp(KindA64::S);

        self.build_mut().fcmpz(temp);
        // cpp 用 float 变体立即数（IrLoweringA64.cpp:1056-1059 的 0.0f/1.0f/-1.0f）
        self.build_mut().fmov(temp0, 0.0f32);
        self.build_mut().fmov(temp1, 1.0f32);
        self.build_mut().fcsel(
          inst.reg_a64,
          temp1,
          temp0,
          get_condition_f_p(IrCondition::Greater),
        );
        self.build_mut().fmov(temp1, -1.0f32);
        self.build_mut().fcsel(
          inst.reg_a64,
          temp1,
          inst.reg_a64,
          get_condition_f_p(IrCondition::Less),
        );
      }
      IrCmd::SelectNum => {
        inst.reg_a64 = self.regs.alloc_reuse(
          KindA64::D,
          index,
          &[(inst.op(0)), (inst.op(1)), (inst.op(2)), (inst.op(3))],
        );

        let temp1 = self.temp_double(inst.op(0));
        let temp2 = self.temp_double(inst.op(1));
        let temp3 = self.temp_double(inst.op(2));
        let temp4 = self.temp_double(inst.op(3));

        self.build_mut().fcmp(temp3, temp4);
        self.build_mut().fcsel(
          inst.reg_a64,
          temp2,
          temp1,
          get_condition_f_p(IrCondition::Equal),
        );
      }
      IrCmd::SelectInt64 => {
        let cond = condition_op(inst.op(4));

        inst.reg_a64 = self.regs.alloc_reuse(
          KindA64::X,
          index,
          &[(inst.op(0)), (inst.op(1)), (inst.op(2)), (inst.op(3))],
        );

        let temp1 = self.temp_int64(inst.op(0));
        let temp2 = self.temp_int64(inst.op(1));
        let temp3 = self.temp_int64(inst.op(2));
        let temp4 = self.temp_int64(inst.op(3));

        self.build_mut().cmp(temp3, temp4);
        self
          .build_mut()
          .csel(inst.reg_a64, temp2, temp1, get_condition_int64(cond));
      }
      IrCmd::SelectVec => {
        {
          // `(inst.op(1))` 不能复用为返回值，因为它可能在首次使用前就被 A 覆写
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::Q,
            index,
            &[(inst.op(0)), (inst.op(2)), (inst.op(3))],
          );

          let temp1 = self.reg_op(inst.op(0));
          let temp2 = self.reg_op(inst.op(1));
          let temp3 = self.reg_op(inst.op(2));
          let temp4 = self.reg_op(inst.op(3));

          let mask = self.regs.alloc_temp(KindA64::Q);

          // 求谓词并计算 mask。
          self.build_mut().fcmeq_4s(mask, temp3, temp4);
          // 把 A mov 到 res 寄存器
          self.build_mut().mov(inst.reg_a64, temp1);
          // 两数相等时在 res 寄存器用 B 覆盖 A。
          self.build_mut().bit(inst.reg_a64, temp2, mask);
        }
      }
      IrCmd::SelectIfTruthy => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);

          // 先放 lhs 作为结果，稍后若 'A' 为 falsy 再用 rhs 覆盖
          let hoist_19 = self.reg_op(inst.op(1));
          self.emit_mov(inst.reg_a64, hoist_19);

          // 尽早取 rhs 寄存器，让可能的 restore 发生在条件控制流两侧之外
          let c = self.reg_op(inst.op(2));

          let temp = self.regs.alloc_temp(KindA64::W);
          let mut save_rhs = Label::default();
          let mut exit = Label::default();

          // 先检查 tag
          let hoist_20 = self.reg_op(inst.op(0));
          self.emit_umov_4s(temp, hoist_20, 3);
          self.build_mut().cmp(temp, (LUA_TBOOLEAN) as u16);

          self.emit_bcond(ConditionA64::UNSIGNED_LESS, &mut save_rhs); // rhs if 'A' is nil
          self.emit_bcond(ConditionA64::UnsignedGreater, &mut exit); // Keep lhs if 'A' is not a boolean

          // 检查 boolean 值
          let hoist_21 = self.reg_op(inst.op(0));
          self.emit_umov_4s(temp, hoist_21, 0);
          self.build_mut().cbnz(temp, &mut exit); // Keep lhs if 'A' is true

          self.build_mut().set_label_label(&mut save_rhs);
          self.build_mut().mov(inst.reg_a64, c);

          self.build_mut().set_label_label(&mut exit);
        }
      }
      IrCmd::MuladdVec => {
        let temp_a = self.reg_op(inst.op(0));
        let temp_b = self.reg_op(inst.op(1));
        let temp_c = self.reg_op(inst.op(2));

        if (self.build_mut().features & FEATURE_ADV_SIMD) != 0 {
          inst.reg_a64 = self.regs.alloc_reuse(KindA64::Q, index, &[(inst.op(2))]);
          if inst.reg_a64 != temp_c {
            self.build_mut().mov(inst.reg_a64, temp_c);
          }
          self.build_mut().fmla(inst.reg_a64, temp_b, temp_a);
        } else {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);
          self.build_mut().fmul(inst.reg_a64, temp_b, temp_a);
          self.build_mut().fadd(inst.reg_a64, inst.reg_a64, temp_c);
        }
      }
      IrCmd::AddVec | IrCmd::SubVec | IrCmd::MulVec | IrCmd::DivVec => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::Q, index, &[(inst.op(0)), (inst.op(1))]);

        let hoist_0 = self.reg_op(inst.op(0));
        let hoist_1 = self.reg_op(inst.op(1));
        match inst.cmd {
          IrCmd::AddVec => self.emit_fadd(inst.reg_a64, hoist_0, hoist_1),
          IrCmd::SubVec => self.emit_fsub(inst.reg_a64, hoist_0, hoist_1),
          IrCmd::MulVec => self.emit_fmul(inst.reg_a64, hoist_0, hoist_1),
          IrCmd::DivVec => self.emit_fdiv(inst.reg_a64, hoist_0, hoist_1),
          _ => unreachable!(),
        }
      }
      IrCmd::IdivVec => {
        self.lower_fa_bin(inst, index, KindA64::Q, Self::reg_op, Self::emit_fdiv);
        self.build_mut().frintm(inst.reg_a64, inst.reg_a64);
      }
      IrCmd::UnmVec => {
        self.lower_fa_un(inst, index, KindA64::Q, Self::reg_op, Self::emit_fneg);
      }
      IrCmd::MinVec | IrCmd::MaxVec => {
        self.lower_min_max_vec(inst, index, inst.cmd == IrCmd::MaxVec);
      }
      IrCmd::FloorVec => {
        self.lower_fa_un(inst, index, KindA64::Q, Self::reg_op, Self::emit_frintm);
      }
      IrCmd::CeilVec => {
        self.lower_fa_un(inst, index, KindA64::Q, Self::reg_op, Self::emit_frintp);
      }
      IrCmd::AbsVec => {
        self.lower_fa_un(inst, index, KindA64::Q, Self::reg_op, Self::emit_fabs);
      }
      IrCmd::DotVec => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);

          let temp = self.regs.alloc_temp(KindA64::Q);
          let temps = cast_reg(KindA64::S, temp);

          let hoist_36 = self.reg_op(inst.op(0));
          let hoist_37 = self.reg_op(inst.op(1));
          self.emit_fmul(temp, hoist_36, hoist_37);
          self.build_mut().faddp(inst.reg_a64, temps); // x+y
          self.build_mut().dup_4s(temp, temp, 2);
          self.build_mut().fadd(inst.reg_a64, inst.reg_a64, temps); // +z
        }
      }
      IrCmd::ExtractVec => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);

          if self.int_op(inst.op(1)) == 0 {
            // Lane vN.s[0] 可直接按 sN 读取
            let hoist_38 = self.reg_op(inst.op(0));
            self.emit_fmov(inst.reg_a64, cast_reg(KindA64::S, hoist_38));
          } else {
            let hoist_39 = self.reg_op(inst.op(0));
            self.emit_dup_4s(inst.reg_a64, hoist_39, self.int_op(inst.op(1)) as u8);
          }
        }
      }
      IrCmd::NotAny => {
        {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);

          if (inst.op(0)).kind() == IrOpKind::Constant {
            // 其他情形应已被常量折叠
            CODEGEN_ASSERT!(self.tag_op(inst.op(0)) == LUA_TBOOLEAN);
            let hoist_40 = self.reg_op(inst.op(1));
            self.emit_eor(inst.reg_a64, hoist_40, 1);
          } else {
            let mut not_bool = Label::default();
            let mut exit = Label::default();

            // 利用 NIL 是唯一小于 BOOLEAN 的 tag，一次完成两个 tag 比较
            CODEGEN_ASSERT!(LUA_TNIL == 0 && LUA_TBOOLEAN == 1);
            let hoist_41 = self.reg_op(inst.op(0));
            self.emit_cmp(hoist_41, (LUA_TBOOLEAN) as u16);
            self.emit_bcond(ConditionA64::NotEqual, &mut not_bool);

            if (inst.op(1)).kind() == IrOpKind::Constant {
              self.emit_mov(
                inst.reg_a64,
                if self.int_op(inst.op(1)) == 0 { 1 } else { 0 },
              );
            } else {
              let hoist_42 = self.reg_op(inst.op(1));
              self.emit_eor(inst.reg_a64, hoist_42, 1);
            } // boolean => invert value

            self.build_mut().b(&mut exit);

            // 非 boolean => 仅当 tag 为 nil 时结果为真
            self.build_mut().set_label_label(&mut not_bool);
            self.build_mut().cset(inst.reg_a64, ConditionA64::Less);

            self.build_mut().set_label_label(&mut exit);
          }
        }
      }
      IrCmd::CmpInt => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);

        let cond = condition_op(inst.op(2));

        if (inst.op(0)).kind() == IrOpKind::Constant {
          if ((self.int_op(inst.op(0))) as u32) <= K_MAX_IMMEDIATE as u32 {
            let hoist_43 = self.reg_op(inst.op(1));
            self.emit_cmp(hoist_43, (self.int_op(inst.op(0))) as u16);
          } else {
            let hoist_44 = self.reg_op(inst.op(1));
            let hoist_45 = self.temp_int(inst.op(0));
            self.emit_cmp(hoist_44, hoist_45);
          }

          self
            .build_mut()
            .cset(inst.reg_a64, get_inverse_condition(get_condition_int(cond)));
        } else if (inst.op(0)).kind() == IrOpKind::Inst {
          if ((self.int_op(inst.op(1))) as u32) <= K_MAX_IMMEDIATE as u32 {
            let hoist_46 = self.reg_op(inst.op(0));
            self.emit_cmp(hoist_46, (self.int_op(inst.op(1))) as u16);
          } else {
            let hoist_47 = self.reg_op(inst.op(0));
            let hoist_48 = self.temp_int(inst.op(1));
            self.emit_cmp(hoist_47, hoist_48);
          }

          self.build_mut().cset(inst.reg_a64, get_condition_int(cond));
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::CmpInt64 => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);

        let cond = condition_op(inst.op(2));

        if (inst.op(0)).kind() == IrOpKind::Constant {
          if ((self.int_64_op(inst.op(0))) as u64) <= K_MAX_IMMEDIATE as u64 {
            let hoist_49 = self.reg_op(inst.op(1));
            let hoist_3 = self.int_64_op(inst.op(0));
            self.emit_cmp(hoist_49, hoist_3 as u16);
          } else {
            let hoist_50 = self.reg_op(inst.op(1));
            let hoist_51 = self.temp_int64(inst.op(0));
            self.emit_cmp(hoist_50, hoist_51);
          }

          self.build_mut().cset(
            inst.reg_a64,
            get_inverse_condition(get_condition_int64(cond)),
          );
        } else if (inst.op(0)).kind() == IrOpKind::Inst {
          if (inst.op(1)).kind() == IrOpKind::Constant
            && ((self.int_64_op(inst.op(1))) as u64) <= K_MAX_IMMEDIATE as u64
          {
            let hoist_52 = self.reg_op(inst.op(0));
            let hoist_4 = self.int_64_op(inst.op(1));
            self.emit_cmp(hoist_52, hoist_4 as u16);
          } else {
            let hoist_53 = self.reg_op(inst.op(0));
            let hoist_54 = self.temp_int64(inst.op(1));
            self.emit_cmp(hoist_53, hoist_54);
          }

          self
            .build_mut()
            .cset(inst.reg_a64, get_condition_int64(cond));
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::CmpAny => {
        {
          CODEGEN_ASSERT!(
            (inst.op(0)).kind() == IrOpKind::VmReg && (inst.op(1)).kind() == IrOpKind::VmReg
          );
          let cond = condition_op(inst.op(2));

          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);

          let mut skip = Label::default();
          let mut exit = Label::default();

          // 等值比较时，'luaV_equalval' 要求调用前 tag 相等
          if cond == IrCondition::Equal {
            let tempa = self.regs.alloc_temp(KindA64::W);
            let tempb = self.regs.alloc_temp(KindA64::W);

            let hoist_55 = self.temp_addr(inst.op(0), (offset_of!(TValue, tt) as i32));
            self.emit_ldr(tempa, hoist_55);
            let hoist_56 = self.temp_addr(inst.op(1), (offset_of!(TValue, tt) as i32));
            self.emit_ldr(tempb, hoist_56);
            self.build_mut().cmp(tempa, tempb);

            // tag 不相等则跳过调用并把结果置 0
            self.emit_bcond(ConditionA64::NotEqual, &mut skip);
          }

          // 结果寄存器已预留，现在释放它以免被记入 spill 序列
          self.regs.free_reg(inst.reg_a64);

          let spills = self.spill_regs(index, &[]);

          self.build_mut().mov(X0, R_STATE);
          self.emit_vm_reg_addr(X1, inst.op(0));
          self.emit_vm_reg_addr(X2, inst.op(1));

          if cond == IrCondition::LessEqual {
            self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_v_lessequal)));
          } else if cond == IrCondition::Less {
            self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_v_lessthan)));
          } else if cond == IrCondition::Equal {
            self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_v_equalval)));
          } else {
            CODEGEN_ASSERT!(false, "Unsupported condition");
          }

          self.build_mut().blr(X3);

          if inst.reg_a64 != W0 {
            self.build_mut().mov(inst.reg_a64, W0);
          }

          inst.reg_a64 = self.regs.take_reg(inst.reg_a64, index);

          emit_update_base(self.build_mut());

          self.regs.restore_usize(spills);

          if cond == IrCondition::Equal {
            self.build_mut().b(&mut exit);
            self.build_mut().set_label_label(&mut skip);

            self.build_mut().mov(inst.reg_a64, 0);
            self.build_mut().set_label_label(&mut exit);
          }

          // 若发生过调用，跳过清高寄存器位；唯一消费者 JUMP_CMP_INT 不读它们
        }
      }
      IrCmd::CmpTag => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);

        let cond = condition_op(inst.op(2));
        CODEGEN_ASSERT!(matches!(cond, IrCondition::Equal | IrCondition::NotEqual));
        let a_reg = self.tag_operand_reg(inst.op(0));
        let b_reg = self.tag_operand_reg(inst.op(1));

        if (inst.op(0)).kind() == IrOpKind::Constant {
          self.emit_cmp(b_reg, (self.tag_op(inst.op(0))) as u16);
          self
            .build_mut()
            .cset(inst.reg_a64, get_inverse_condition(get_condition_int(cond)));
        } else if (inst.op(1)).kind() == IrOpKind::Constant {
          self.emit_cmp(a_reg, (self.tag_op(inst.op(1))) as u16);
          self.build_mut().cset(inst.reg_a64, get_condition_int(cond));
        } else {
          self.build_mut().cmp(a_reg, b_reg);
          self.build_mut().cset(inst.reg_a64, get_condition_int(cond));
        }
      }
      IrCmd::CmpSplitTvalue => {
        {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(inst.op(0)), (inst.op(1))]);

          // 本指令的第二个操作数必须是常量
          // 没有常量类型，lowering 时就不知道正确的值比较方式
          CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::Constant);

          let cond = condition_op(inst.op(4));
          CODEGEN_ASSERT!(matches!(cond, IrCondition::Equal | IrCondition::NotEqual));

          // 先检查 tag 相等
          let temp = self.regs.alloc_temp(KindA64::W);

          if (inst.op(0)).kind() != IrOpKind::Constant {
            let hoist_57 = self.reg_op(inst.op(0));
            self.emit_cmp(hoist_57, (self.tag_op(inst.op(1))) as u16);
            self.build_mut().cset(temp, get_condition_int(cond));
          } else {
            // 不同常量 tag 本应由常量折叠处理掉
            CODEGEN_ASSERT!(self.tag_op(inst.op(0)) == self.tag_op(inst.op(1)));
          }

          if self.tag_op(inst.op(1)) == LUA_TBOOLEAN {
            if (inst.op(2)).kind() == IrOpKind::Constant {
              CODEGEN_ASSERT!(self.int_op(inst.op(2)) == 0 || self.int_op(inst.op(2)) == 1);
              let hoist_58 = self.reg_op(inst.op(3));
              self.emit_cmp(hoist_58, (self.int_op(inst.op(2))) as u16);
            // swapped arguments
            } else if (inst.op(3)).kind() == IrOpKind::Constant {
              CODEGEN_ASSERT!(self.int_op(inst.op(3)) == 0 || self.int_op(inst.op(3)) == 1);
              let hoist_59 = self.reg_op(inst.op(2));
              self.emit_cmp(hoist_59, (self.int_op(inst.op(3))) as u16);
            } else {
              let hoist_60 = self.reg_op(inst.op(2));
              let hoist_61 = self.reg_op(inst.op(3));
              self.emit_cmp(hoist_60, hoist_61);
            }

            self.build_mut().cset(inst.reg_a64, get_condition_int(cond));
          } else if self.tag_op(inst.op(1)) == LUA_TSTRING {
            let hoist_62 = self.reg_op(inst.op(2));
            let hoist_63 = self.reg_op(inst.op(3));
            self.emit_cmp(hoist_62, hoist_63);
            self.build_mut().cset(inst.reg_a64, get_condition_int(cond));
          } else if self.tag_op(inst.op(1)) == LUA_TNUMBER {
            let temp1 = self.temp_double(inst.op(2));
            let temp2 = self.temp_double(inst.op(3));

            self.build_mut().fcmp(temp1, temp2);
            self.build_mut().cset(inst.reg_a64, get_condition_f_p(cond));
          } else if self.tag_op(inst.op(1)) == LUA_TINTEGER {
            let temp1 = self.temp_int64(inst.op(2));
            let temp2 = self.temp_int64(inst.op(3));

            self.build_mut().cmp(temp1, temp2);
            self
              .build_mut()
              .cset(inst.reg_a64, get_condition_int64(cond));
          } else {
            CODEGEN_ASSERT!(false, "unsupported type tag in CMP_SPLIT_TVALUE");
          }

          if (inst.op(0)).kind() != IrOpKind::Constant {
            if cond == IrCondition::Equal {
              self.build_mut().and_(inst.reg_a64, inst.reg_a64, temp);
            } else {
              self.build_mut().orr(inst.reg_a64, inst.reg_a64, temp);
            }
          }
        }
      }
      IrCmd::JUMP => {
        if (inst.op(0)).kind() == IrOpKind::Undef || (inst.op(0)).kind() == IrOpKind::VmExit {
          let mut fresh = Label::default();
          let target = self.ir_lowering_a_64_get_target_label(inst.op(0), index, &mut fresh);
          self.with_target_label(target, |s, l| s.build_mut().b(l));
          self.ir_lowering_a_64_finalize_target_label(inst.op(0), index, &mut fresh);
        } else {
          self.jump_or_fallthrough_op(inst.op(0), next);
        }
      }
      IrCmd::JumpIfTruthy => {
        self.jump_if_branch(true, inst.op(0), inst.op(1), inst.op(2), next);
      }
      IrCmd::JumpIfFalsy => {
        self.jump_if_branch(false, inst.op(0), inst.op(1), inst.op(2), next);
      }
      IrCmd::JumpEqTag => {
        let mut zr = NOREG;
        let a_reg = self.tag_operand_reg(inst.op(0));
        let b_reg = self.tag_operand_reg(inst.op(1));

        if (inst.op(0)).kind() == IrOpKind::Constant && self.tag_op(inst.op(0)) == 0 {
          zr = b_reg;
        } else if (inst.op(1)).kind() == IrOpKind::Constant && self.tag_op(inst.op(1)) == 0 {
          zr = a_reg;
        } else if (inst.op(1)).kind() == IrOpKind::Constant {
          self.emit_cmp(a_reg, (self.tag_op(inst.op(1))) as u16);
        } else if (inst.op(0)).kind() == IrOpKind::Constant {
          self.emit_cmp(b_reg, (self.tag_op(inst.op(0))) as u16);
        } else {
          self.build_mut().cmp(a_reg, b_reg);
        }

        if self.is_fallthrough_block(self.block_op_ref(inst.op(3)), next) {
          if zr != NOREG {
            self.with_op_label(inst.op(2), |s, l| s.build_mut().cbz(zr, l));
          } else {
            self.with_op_label(inst.op(2), |s, l| {
              s.build_mut().b_condition_a_64_label(ConditionA64::Equal, l)
            });
          }
          self.jump_or_fallthrough_op(inst.op(3), next);
        } else {
          if zr != NOREG {
            self.with_op_label(inst.op(3), |s, l| s.build_mut().cbnz(zr, l));
          } else {
            self.with_op_label(inst.op(3), |s, l| {
              s.build_mut()
                .b_condition_a_64_label(ConditionA64::NotEqual, l)
            });
          }
          self.jump_or_fallthrough_op(inst.op(2), next);
        }
      }
      IrCmd::JumpCmpInt => {
        let cond = condition_op(inst.op(2));

        if cond == IrCondition::Equal && self.int_op(inst.op(1)) == 0 {
          let hoist_64 = self.reg_op(inst.op(0));
          self.with_op_label(inst.op(3), |s, l| s.build_mut().cbz(hoist_64, l));
        } else if cond == IrCondition::NotEqual && self.int_op(inst.op(1)) == 0 {
          let hoist_65 = self.reg_op(inst.op(0));
          self.with_op_label(inst.op(3), |s, l| s.build_mut().cbnz(hoist_65, l));
        } else {
          CODEGEN_ASSERT!(((self.int_op(inst.op(1))) as u32) <= K_MAX_IMMEDIATE as u32);
          let hoist_66 = self.reg_op(inst.op(0));
          self.emit_cmp(hoist_66, (self.int_op(inst.op(1))) as u16);
          self.with_op_label(inst.op(3), |s, l| {
            s.build_mut()
              .b_condition_a_64_label(get_condition_int(cond), l)
          });
        }
        self.jump_or_fallthrough_op(inst.op(4), next);
      }
      IrCmd::JumpCmpInt64 => {
        // cpp IrLoweringA64.cpp:1729
        let cond = condition_op(inst.op(2));

        // 常量可能在任一侧；以下形式均与右操作数比较，
        // 因此交换操作数并反转条件（Equal/NotEqual 不受影响）
        let mut lhs = inst.op(0);
        let mut rhs = inst.op(1);
        let swapped = lhs.kind() == IrOpKind::Constant;

        if swapped {
          lhs = inst.op(1);
          rhs = inst.op(0);
        }

        if cond == IrCondition::Equal
          && rhs.kind() == IrOpKind::Constant
          && self.int_64_op(rhs) == 0
        {
          let hoist_67 = self.reg_op(lhs);
          self.with_op_label(inst.op(3), |s, l| s.build_mut().cbz(hoist_67, l));
        } else if cond == IrCondition::NotEqual
          && rhs.kind() == IrOpKind::Constant
          && self.int_64_op(rhs) == 0
        {
          let hoist_68 = self.reg_op(lhs);
          self.with_op_label(inst.op(3), |s, l| s.build_mut().cbnz(hoist_68, l));
        } else {
          if rhs.kind() == IrOpKind::Constant
            && (self.int_64_op(rhs) as u64) <= K_MAX_IMMEDIATE as u64
          {
            let hoist_69 = self.reg_op(lhs);
            let hoist_70 = self.int_64_op(rhs);
            self.emit_cmp(hoist_69, hoist_70 as u16);
          } else {
            let temp = self.temp_int64(rhs);
            let hoist_71 = self.reg_op(lhs);
            self.emit_cmp(hoist_71, temp);
          }

          let cc = get_condition_int64(cond);
          self.with_op_label(inst.op(3), |s, l| {
            s.build_mut().b_condition_a_64_label(
              if swapped {
                get_inverse_condition(cc)
              } else {
                cc
              },
              l,
            )
          });
        }
        self.jump_or_fallthrough_op(inst.op(4), next);
      }
      IrCmd::JumpEqPointer => {
        let hoist_72 = self.reg_op(inst.op(0));
        let hoist_73 = self.reg_op(inst.op(1));
        self.emit_cmp(hoist_72, hoist_73);
        self.with_op_label(inst.op(2), |s, l| {
          s.build_mut().b_condition_a_64_label(ConditionA64::Equal, l)
        });
        self.jump_or_fallthrough_op(inst.op(3), next);
      }
      IrCmd::JumpCmpNum => {
        let cond = condition_op(inst.op(2));

        if (inst.op(1)).kind() == IrOpKind::Constant && self.double_op(inst.op(1)) == 0.0 {
          let temp = self.temp_double(inst.op(0));

          self.build_mut().fcmpz(temp);
        } else {
          let temp1 = self.temp_double(inst.op(0));
          let temp2 = self.temp_double(inst.op(1));

          self.build_mut().fcmp(temp1, temp2);
        }

        self.with_op_label(inst.op(3), |s, l| {
          s.build_mut()
            .b_condition_a_64_label(get_condition_f_p(cond), l)
        });
        self.jump_or_fallthrough_op(inst.op(4), next);
      }
      IrCmd::JumpCmpFloat => {
        let cond = condition_op(inst.op(2));

        if (inst.op(1)).kind() == IrOpKind::Constant && ((self.double_op(inst.op(1))) as f32) == 0.0
        {
          let temp = self.temp_float(inst.op(0));

          self.build_mut().fcmpz(temp);
        } else {
          let temp1 = self.temp_float(inst.op(0));
          let temp2 = self.temp_float(inst.op(1));

          self.build_mut().fcmp(temp1, temp2);
        }

        self.with_op_label(inst.op(3), |s, l| {
          s.build_mut()
            .b_condition_a_64_label(get_condition_f_p(cond), l)
        });
        self.jump_or_fallthrough_op(inst.op(4), next);
      }
      IrCmd::JumpFornLoopCond => {
        {
          let index = self.temp_double(inst.op(0));
          let limit = self.temp_double(inst.op(1));
          let step = self.temp_double(inst.op(2));

          let mut direct = Label::default();

          // step > 0
          self.build_mut().fcmpz(step);
          self.emit_bcond(get_condition_f_p(IrCondition::Greater), &mut direct);

          // !(limit <= index)
          self.build_mut().fcmp(limit, index);
          self.with_op_label(inst.op(4), |s, l| {
            s.build_mut()
              .b_condition_a_64_label(get_condition_f_p(IrCondition::NotLessEqual), l)
          });
          self.with_op_label(inst.op(3), |s, l| s.build_mut().b(l));

          // !(index <= limit)
          self.build_mut().set_label_label(&mut direct);

          self.build_mut().fcmp(index, limit);
          self.with_op_label(inst.op(4), |s, l| {
            s.build_mut()
              .b_condition_a_64_label(get_condition_f_p(IrCondition::NotLessEqual), l)
          });
          self.jump_or_fallthrough_op(inst.op(3), next);
        }
        // IrCmd::JUMP_SLOT_MATCH 在下方实现
      }
      IrCmd::TableLen => {
        {
          let reg = self.reg_op(inst.op(0)); // note: we need to call regOp before spill so that we don't do redundant reloads
          self.spill_regs(index, &[reg]);
          self.build_mut().mov(X0, reg);
          self.emit_ldr(X1, native_ctx(offset_of!(NativeContext, lua_h_getn)));
          self.build_mut().blr(X1);

          inst.reg_a64 = self.regs.take_reg(W0, index);

          self.build_mut().ubfx(inst.reg_a64, inst.reg_a64, 0, 32); // Ensure high register bits are cleared
        }
      }
      IrCmd::StringLen => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);

        let hoist_74 = self.reg_op(inst.op(0));
        self.emit_ldr(inst.reg_a64, mem(hoist_74, K_TSTRING_LEN_OFFSET));
      }
      IrCmd::TableSetnum => {
        {
          // 注：先调 regOp 再 spill，避免冗余 reload
          let table = self.reg_op(inst.op(0));
          let key = self.reg_op(inst.op(1));
          let temp = self.regs.alloc_temp(KindA64::W);

          self.spill_regs(index, &[table, key]);

          if W1 != key {
            self.build_mut().mov(X1, table);
            self.build_mut().mov(W2, key);
          } else {
            self.build_mut().mov(temp, W1);
            self.build_mut().mov(X1, table);
            self.build_mut().mov(W2, temp);
          }

          self.build_mut().mov(X0, R_STATE);
          self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_h_setnum)));
          self.build_mut().blr(X3);
          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
      }
      IrCmd::NewTable => {
        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);
        self.emit_mov(X1, self.uint_op(inst.op(0)));
        self.emit_mov(X2, self.uint_op(inst.op(1)));
        self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_h_new)));
        self.build_mut().blr(X3);
        inst.reg_a64 = self.regs.take_reg(X0, index);
      }
      IrCmd::DupTable => {
        {
          let reg = self.reg_op(inst.op(0)); // note: we need to call regOp before spill so that we don't do redundant reloads
          self.spill_regs(index, &[reg]);
          self.build_mut().mov(X1, reg);
          self.build_mut().mov(X0, R_STATE);
          self.emit_ldr(X2, native_ctx(offset_of!(NativeContext, lua_h_clone)));
          self.build_mut().blr(X2);
          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
      }
      IrCmd::TryNumToIndex => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
          let temp1 = self.temp_double(inst.op(0));

          if (self.build_mut().features & FEATURE_JSCVT) != 0 {
            self.build_mut().fjcvtzs(inst.reg_a64, temp1); // fjcvtzs sets PSTATE.Z (equal) iff conversion is exact
            self.with_op_label(inst.op(1), |s, l| {
              s.build_mut()
                .b_condition_a_64_label(ConditionA64::NotEqual, l)
            });
          } else {
            let temp2 = self.regs.alloc_temp(KindA64::D);

            self.build_mut().fcvtzs(inst.reg_a64, temp1);
            self.build_mut().scvtf(temp2, inst.reg_a64);
            self.build_mut().fcmp(temp1, temp2);
            self.with_op_label(inst.op(1), |s, l| {
              s.build_mut()
                .b_condition_a_64_label(ConditionA64::NotEqual, l)
            });
          }
        }
      }
      IrCmd::TryCallFastgettm => {
        {
          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp2 = self.regs.alloc_temp(KindA64::W);

          let hoist_75 = self.reg_op(inst.op(0));
          self.emit_ldr(
            temp1,
            mem(hoist_75, (offset_of!(LuaTable, metatable) as i32)),
          );
          self.with_op_label(inst.op(2), |s, l| s.build_mut().cbz(temp1, l)); // no metatable

          self
            .build_mut()
            .ldrb(temp2, mem(temp1, (offset_of!(LuaTable, tmcache) as i32)));
          self.emit_tst(temp2, 1 << self.int_op(inst.op(1))); // can't use tbz/tbnz because their jump offsets are too short
          self.with_op_label(inst.op(2), |s, l| {
            s.build_mut()
              .b_condition_a_64_label(ConditionA64::NotEqual, l)
          }); // Equal = Zero after tst; tmcache caches *absence* of metamethods

          self.spill_regs(index, &[temp1]);
          self.build_mut().mov(X0, temp1);
          self.emit_mov(W1, self.int_op(inst.op(1)));
          self.emit_ldr(
            X2,
            mem(
              R_GLOBAL_STATE,
              (offset_of!(global_State, tmname) as i32)
                + self
                  .int_op(inst.op(1))
                  .wrapping_mul(K_NATIVE_PTR_SIZE as i32),
            ),
          );
          self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_t_gettm)));
          self.build_mut().blr(X3);

          self.with_op_label(inst.op(2), |s, l| s.build_mut().cbz(X0, l)); // no tag method

          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
      }
      IrCmd::NewUserdata => {
        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);
        self.emit_mov(X1, self.int_op(inst.op(0)));
        self.emit_mov(X2, self.int_op(inst.op(1)));
        self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, new_userdata)));
        self.build_mut().blr(X3);
        inst.reg_a64 = self.regs.take_reg(X0, index);
      }
      IrCmd::Int64ToNum => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
        let temp = self.temp_int64(inst.op(0));
        self.build_mut().scvtf(inst.reg_a64, temp);
      }
      IrCmd::IntToNum => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
        let temp = self.temp_int(inst.op(0));
        self.build_mut().scvtf(inst.reg_a64, temp);
      }
      IrCmd::UintToNum => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
        let temp = self.temp_int(inst.op(0));
        self.build_mut().ucvtf(inst.reg_a64, temp);
      }
      IrCmd::UintToFloat => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);
        let temp = self.temp_int(inst.op(0));
        self.build_mut().ucvtf(inst.reg_a64, temp);
      }
      IrCmd::NumToInt => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
        let temp = self.temp_double(inst.op(0));
        self.build_mut().fcvtzs(inst.reg_a64, temp);
      }
      IrCmd::NumToInt64 => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
        let temp = self.temp_double(inst.op(0));
        self.build_mut().fcvtzs(inst.reg_a64, temp);
      }
      IrCmd::NumToUint => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
          let temp = self.temp_double(inst.op(0));
          // 注：不用 fcvtzu，以与 C++ 代码保持一致
          self
            .build_mut()
            .fcvtzs(cast_reg(KindA64::X, inst.reg_a64), temp);
        }
      }
      IrCmd::FloatToNum => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);

        let hoist_76 = self.reg_op(inst.op(0));
        self.emit_fcvt(inst.reg_a64, hoist_76);
      }
      IrCmd::NumToFloat => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);

        let hoist_77 = self.reg_op(inst.op(0));
        self.emit_fcvt(inst.reg_a64, hoist_77);
      }
      IrCmd::FloatToVec => {
        inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);

        if (inst.op(0)).kind() == IrOpKind::Constant {
          let value = (self.double_op(inst.op(0))) as f32;
          let as_u32 = value.to_bits();

          if self.build_mut().is_fmov_supported_fp_32(value) {
            self.build_mut().fmov(inst.reg_a64, value);
          } else {
            let temp = self.regs.alloc_temp(KindA64::X);

            let vec = [as_u32, as_u32, as_u32, 0u32];
            self.build_mut().adr_register_a_64_void_usize(
              temp,
              vec.as_ptr().cast::<c_void>(),
              size_of_val(&vec),
            );
            self.build_mut().ldr(inst.reg_a64, mem(temp, 0));
          }
        } else {
          let temp = self.temp_float(inst.op(0));

          self
            .build_mut()
            .dup_4s(inst.reg_a64, cast_reg(KindA64::Q, temp), 0);
        }
      }
      IrCmd::TagVector => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::Q, index, &[(inst.op(0))]);

        let reg = self.reg_op(inst.op(0));
        let tempw = self.regs.alloc_temp(KindA64::W);

        if inst.reg_a64 != reg {
          self.build_mut().mov(inst.reg_a64, reg);
        }

        self.build_mut().mov(tempw, LUA_TVECTOR);
        self
          .build_mut()
          .ins_4_s_register_a_64_register_a_64_u8(inst.reg_a64, tempw, 3);
      }
      IrCmd::TruncateUint => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);

        let hoist_78 = self.reg_op(inst.op(0));
        self.emit_ubfx(
          cast_reg(KindA64::X, inst.reg_a64),
          cast_reg(KindA64::X, hoist_78),
          0,
          32,
        ); // explicit uxtw
      }
      IrCmd::AdjustStackToReg => {
        {
          let temp = self.regs.alloc_temp(KindA64::X);

          if (inst.op(1)).kind() == IrOpKind::Constant {
            self.emit_add(
              temp,
              r_base(),
              ((vm_reg_op(inst.op(0)) + self.int_op(inst.op(1))) * (size_of::<TValue>() as i32))
                as u16,
            );
            self
              .build_mut()
              .str(temp, mem(R_STATE, (offset_of!(LuaState, top) as i32)));
          } else if (inst.op(1)).kind() == IrOpKind::Inst {
            self.emit_vm_reg_addr(temp, inst.op(0));
            let hoist_79 = self.reg_op(inst.op(1));
            self.emit_add_register_a_64_register_a_64_register_a_64_i32(
              temp,
              temp,
              hoist_79,
              K_TVALUE_SIZE_LOG2,
            ); // implicit uxtw
            self
              .build_mut()
              .str(temp, mem(R_STATE, (offset_of!(LuaState, top) as i32)));
          } else {
            unsupported_instruction_form();
          }
        }
      }
      IrCmd::AdjustStackToTop => {
        let temp = self.regs.alloc_temp(KindA64::X);
        self
          .build_mut()
          .ldr(temp, mem(R_STATE, (offset_of!(LuaState, ci) as i32)));
        self
          .build_mut()
          .ldr(temp, mem(temp, (offset_of!(CallInfo, top) as i32)));
        self
          .build_mut()
          .str(temp, mem(R_STATE, (offset_of!(LuaState, top) as i32)));
      }
      IrCmd::FASTCALL => {
        self.spill_regs(index, &[]);

        let bfid = self.uint_op(inst.op(0)) as i32;
        let res = vm_reg_op(inst.op(1));
        let arg = vm_reg_op(inst.op(2));
        let nresults = self.int_op(inst.op(3));
        // Safety: 混窗元组门面（build↔function↔regs 共存别名，见 records 契约）；
        // 三视图即时消费于本调用，error 归并在其后，求值序与化前实参列一致。
        let (build, function, regs) = self.build_function_regs_mut();
        let ok = emit_builtin(build, function, regs, bfid, res, arg, nresults);
        self.error |= !ok;
      }
      IrCmd::InvokeFastcall => {
        {
          // 可能需要一个临时寄存器，且必须在 spill 期间保住它
          let temp = self.regs.alloc_temp(KindA64::Q);
          self.spill_regs(index, &[temp]);

          self.build_mut().mov(X0, R_STATE);
          self.emit_vm_reg_addr(X1, inst.op(1));
          self.emit_vm_reg_addr(X2, inst.op(2));
          self.emit_mov(W3, self.int_op(inst.op(6))); // nresults

          // 'E' 参数只可能由 LOP_FASTCALL3 lowering 产生
          if (inst.op(4)).kind() != IrOpKind::Undef {
            CODEGEN_ASSERT!(self.int_op(inst.op(5)) == 3);

            self
              .build_mut()
              .ldr(X4, mem(R_STATE, (offset_of!(LuaState, top) as i32)));

            self.emit_ldr(temp, vm_reg_addr(inst.op(3)));
            self.build_mut().str(temp, mem(X4, 0));

            self.emit_ldr(temp, vm_reg_addr(inst.op(4)));
            self
              .build_mut()
              .str(temp, mem(X4, size_of::<TValue>() as i32));
          } else {
            if (inst.op(3)).kind() == IrOpKind::VmReg {
              self.emit_vm_reg_addr(X4, inst.op(3));
            } else if (inst.op(3)).kind() == IrOpKind::VmConst {
              self.emit_vm_const_addr(X4, inst.op(3));
            } else {
              CODEGEN_ASSERT!((inst.op(3)).kind() == IrOpKind::Undef);
            }
          }

          // nparams
          if self.int_op(inst.op(5)) == LUA_MULTRET {
            // l->top - (ra + 1)
            self
              .build_mut()
              .ldr(X5, mem(R_STATE, (offset_of!(LuaState, top) as i32)));
            self.build_mut().sub(X5, X5, r_base());
            self.build_mut().sub(
              X5,
              X5,
              ((vm_reg_op(inst.op(1)) + 1) * (size_of::<TValue>() as i32)) as u16,
            );
            self.build_mut().lsr(X5, X5, K_TVALUE_SIZE_LOG2);
          } else {
            self.emit_mov(W5, self.int_op(inst.op(5)));
          }

          self.emit_ldr(
            X6,
            mem(
              R_NATIVE_CONTEXT,
              (offset_of!(NativeContext, luau_f_table) as i32)
                + (self.uint_op(inst.op(0)) as i32) * (size_of::<LuauFastFunction>() as i32),
            ),
          );
          self.build_mut().blr(X6);

          inst.reg_a64 = self.regs.take_reg(W0, index);
          // 跳过清高寄存器位；唯一消费者 CHECK_FASTCALL_RES 不读它们
        }
      }
      IrCmd::CheckFastcallRes => {
        let hoist_80 = self.reg_op(inst.op(0));
        self.emit_cmp(hoist_80, 0_u16);
        self.with_op_label(inst.op(1), |s, l| {
          s.build_mut().b_condition_a_64_label(ConditionA64::Less, l)
        });
      }
      IrCmd::DoArith => {
        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);
        self.emit_vm_reg_addr(X1, inst.op(0));

        if (inst.op(1)).kind() == IrOpKind::VmConst {
          self.emit_vm_const_addr(X2, inst.op(1));
        } else {
          self.emit_vm_reg_addr(X2, inst.op(1));
        }

        if (inst.op(2)).kind() == IrOpKind::VmConst {
          self.emit_vm_const_addr(X3, inst.op(2));
        } else {
          self.emit_vm_reg_addr(X3, inst.op(2));
        }

        match (self.int_op(inst.op(3)) as u32)
          .checked_sub(TMS::TmAdd as u32)
          .and_then(|tm| DOARITH_HELPER_OFFSETS.get(tm as usize))
        {
          Some(off) => self.emit_ldr(X4, native_ctx(*off)),
          _ => CODEGEN_ASSERT!(false, "Invalid doarith helper operation tag"),
        }

        self.build_mut().blr(X4);

        emit_update_base(self.build_mut());
      }
      IrCmd::DoLen => {
        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);
        self.emit_vm_reg_addr(X1, inst.op(0));
        self.emit_vm_reg_addr(X2, inst.op(1));
        self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_v_dolen)));
        self.build_mut().blr(X3);

        emit_update_base(self.build_mut());
      }
      IrCmd::GetTable => {
        self.lower_get_set_table(index, inst, offset_of!(NativeContext, lua_v_gettable));
      }
      IrCmd::SetTable => {
        self.lower_get_set_table(index, inst, offset_of!(NativeContext, lua_v_settable));
      }
      IrCmd::GetCachedImport => {
        {
          self.spill_regs(index, &[]);

          let mut skip = Label::default();
          let mut exit = Label::default();

          let temp_tag = self.regs.alloc_temp(KindA64::W);

          let addr_const_tag = self.temp_addr(inst.op(1), (offset_of!(TValue, tt) as i32));
          self.build_mut().ldr(temp_tag, addr_const_tag);

          // 若 import 的常量已设置就直接用，否则得调 import 路径查找函数
          CODEGEN_ASSERT!(LUA_TNIL == 0);
          self.build_mut().cbnz(temp_tag, &mut skip);

          {
            self.build_mut().mov(X0, R_STATE);
            self.emit_vm_reg_addr(X1, inst.op(0));
            self.emit_mov(W2, self.import_op(inst.op(2)));
            self.emit_mov(W3, self.uint_op(inst.op(3)));
            self.emit_ldr(X4, native_ctx(offset_of!(NativeContext, get_import)));
            self.build_mut().blr(X4);

            emit_update_base(self.build_mut());
            self.build_mut().b(&mut exit);
          }

          self.build_mut().set_label_label(&mut skip);

          let temp_tv = self.regs.alloc_temp(KindA64::Q);

          let addr_const = self.temp_addr(inst.op(1), 0);
          self.build_mut().ldr(temp_tv, addr_const);

          let addr_reg = self.temp_addr(inst.op(0), 0);
          self.build_mut().str(temp_tv, addr_reg);

          self.build_mut().set_label_label(&mut exit);
        }
      }
      IrCmd::CONCAT => {
        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);
        self.emit_mov(W1, self.uint_op(inst.op(1)));
        self.emit_mov(
          W2,
          vm_reg_op(inst.op(0)) + self.uint_op(inst.op(1)) as i32 - 1,
        );
        self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_v_concat)));
        self.build_mut().blr(X3);

        emit_update_base(self.build_mut());
      }
      IrCmd::GetUpvalue => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);

          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp2 = self.regs.alloc_temp(KindA64::W);

          self.build_mut().add(
            temp1,
            R_CLOSURE,
            (K_CLOSURE_L_UPREFS_OFFSET + (size_of::<TValue>() as i32) * vm_upvalue_op(inst.op(0)))
              as u16,
          );

          // uprefs[] 要么直接是值，要么指向持有值指针的 UpVal 对象
          let mut skip = Label::default();
          self
            .build_mut()
            .ldr(temp2, mem(temp1, (offset_of!(TValue, tt) as i32)));
          self.build_mut().cmp(temp2, (LUA_TUPVAL) as u16);
          self.emit_bcond(ConditionA64::NotEqual, &mut skip);

          // UpVal.v 指向值（在栈上或 UpVal 内的堆上，均可无条件解引用）
          self
            .build_mut()
            .ldr(temp1, mem(temp1, K_TVALUE_VALUE_GC_OFFSET));
          self
            .build_mut()
            .ldr(temp1, mem(temp1, (offset_of!(UpVal, v) as i32)));

          self.build_mut().set_label_label(&mut skip);

          self.build_mut().ldr(inst.reg_a64, mem(temp1, 0));
        }
      }
      IrCmd::SetUpvalue => {
        {
          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp2 = self.regs.alloc_temp(KindA64::X);

          // UpVal*
          self.build_mut().ldr(
            temp1,
            mem(
              R_CLOSURE,
              K_CLOSURE_L_UPREFS_OFFSET
                + (size_of::<TValue>() as i32) * vm_upvalue_op(inst.op(0))
                + K_TVALUE_VALUE_GC_OFFSET,
            ),
          );

          self
            .build_mut()
            .ldr(temp2, mem(temp1, (offset_of!(UpVal, v) as i32)));
          let hoist_81 = self.reg_op(inst.op(1));
          self.emit_str(hoist_81, mem(temp2, 0));

          if (inst.op(2)).kind() == IrOpKind::Undef || is_gco(self.tag_op(inst.op(2))) {
            let value = self.reg_op(inst.op(1));

            let mut skip = Label::default();
            self.check_object_barrier_conditions(
              temp1,
              temp2,
              value,
              inst.op(1),
              if (inst.op(2)).kind() == IrOpKind::Undef {
                -1
              } else {
                self.tag_op(inst.op(2)) as i32
              },
              &mut skip,
            );

            let spills = self.spill_regs(index, &[temp1, value]);

            self.build_mut().mov(X1, temp1);
            self.build_mut().mov(X0, R_STATE);
            self.build_mut().fmov(X2, cast_reg(KindA64::D, value));
            self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_c_barrierf)));
            self.build_mut().blr(X3);

            self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

            // 注：luaC_ barrier 不会重分配栈，故无需 emitUpdateBase
            self.build_mut().set_label_label(&mut skip);
          }
        }
      }
      IrCmd::CheckTag => {
        {
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let fail = self.ir_lowering_a_64_get_target_label(inst.op(2), index, &mut fresh);

          if self.tag_op(inst.op(1)) == 0 {
            let reg = self.reg_op(inst.op(0));
            self.with_target_label(fail, |s, l| s.build_mut().cbnz(reg, l));
          } else {
            let reg = self.reg_op(inst.op(0));
            self.emit_cmp(reg, (self.tag_op(inst.op(1))) as u16);
            self.with_target_label(fail, |s, l| {
              s.build_mut()
                .b_condition_a_64_label(ConditionA64::NotEqual, l)
            });
          }

          self.ir_lowering_a_64_finalize_target_label(inst.op(2), index, &mut fresh);
        }
      }
      IrCmd::CheckTruthy => {
        {
          // 无需检查 boolean 值的常量 tag 本应已被常量折叠移除
          CODEGEN_ASSERT!(
            (inst.op(0)).kind() != IrOpKind::Constant || self.tag_op(inst.op(0)) == LUA_TBOOLEAN
          );

          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let target = self.ir_lowering_a_64_get_target_label(inst.op(2), index, &mut fresh);

          let mut skip = Label::default();

          if (inst.op(0)).kind() != IrOpKind::Constant {
            // 'nil'（falsy）时 fallback
            CODEGEN_ASSERT!(LUA_TNIL == 0);
            let tag = self.reg_op(inst.op(0));
            self.with_target_label(target, |s, l| s.build_mut().cbz(tag, l));

            // 非 boolean（truthy）则跳过值测试
            let tag = self.reg_op(inst.op(0));
            self.build_mut().cmp(tag, (LUA_TBOOLEAN) as u16);
            self.emit_bcond(ConditionA64::NotEqual, &mut skip);
          }

          // 'false' 布尔值（falsy）时 fallback
          if (inst.op(1)).kind() != IrOpKind::Constant {
            let value = self.reg_op(inst.op(1));
            self.with_target_label(target, |s, l| s.build_mut().cbz(value, l));
          } else {
            if self.int_op(inst.op(1)) == 0 {
              self.with_target_label(target, |s, l| s.build_mut().b(l));
            }
          }

          if (inst.op(0)).kind() != IrOpKind::Constant {
            self.build_mut().set_label_label(&mut skip);
          }

          self.ir_lowering_a_64_finalize_target_label(inst.op(2), index, &mut fresh);
        }
      }
      IrCmd::CheckReadonly => {
        {
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let temp = self.regs.alloc_temp(KindA64::W);
          let hoist_82 = self.reg_op(inst.op(0));
          self.emit_ldrb(temp, mem(hoist_82, (offset_of!(LuaTable, readonly) as i32)));
          let target = self.ir_lowering_a_64_get_target_label(inst.op(1), index, &mut fresh);
          self.with_target_label(target, |s, l| s.build_mut().cbnz(temp, l));
          self.ir_lowering_a_64_finalize_target_label(inst.op(1), index, &mut fresh);
        }
      }
      IrCmd::CheckNoMetatable => {
        {
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let temp = self.regs.alloc_temp(KindA64::X);
          let hoist_83 = self.reg_op(inst.op(0));
          self.emit_ldr(
            temp,
            mem(hoist_83, (offset_of!(LuaTable, metatable) as i32)),
          );
          let target = self.ir_lowering_a_64_get_target_label(inst.op(1), index, &mut fresh);
          self.with_target_label(target, |s, l| s.build_mut().cbnz(temp, l));
          self.ir_lowering_a_64_finalize_target_label(inst.op(1), index, &mut fresh);
        }
      }
      IrCmd::CheckSafeEnv => {
        self.check_safe_env(inst.op(0), index, next);
      }
      IrCmd::CheckArraySize => {
        {
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let fail = self.ir_lowering_a_64_get_target_label(inst.op(2), index, &mut fresh);

          let temp = self.regs.alloc_temp(KindA64::W);
          let hoist_84 = self.reg_op(inst.op(0));
          self.emit_ldr(
            temp,
            mem(hoist_84, (offset_of!(LuaTable, sizearray) as i32)),
          );

          if (inst.op(1)).kind() == IrOpKind::Inst {
            let hoist_85 = self.reg_op(inst.op(1));
            self.emit_cmp(temp, hoist_85);
            self.with_target_label(fail, |s, l| {
              s.build_mut()
                .b_condition_a_64_label(ConditionA64::UnsignedLessEqual, l)
            });
          } else if (inst.op(1)).kind() == IrOpKind::Constant {
            if self.int_op(inst.op(1)) == 0 {
              self.with_target_label(fail, |s, l| s.build_mut().cbz(temp, l));
            } else if ((self.int_op(inst.op(1))) as usize) <= K_MAX_IMMEDIATE {
              self.emit_cmp(temp, (self.int_op(inst.op(1))) as u16);
              self.with_target_label(fail, |s, l| {
                s.build_mut()
                  .b_condition_a_64_label(ConditionA64::UnsignedLessEqual, l)
              });
            } else {
              let temp2 = self.regs.alloc_temp(KindA64::W);
              self.emit_mov(temp2, self.int_op(inst.op(1)));
              self.build_mut().cmp(temp, temp2);
              self.with_target_label(fail, |s, l| {
                s.build_mut()
                  .b_condition_a_64_label(ConditionA64::UnsignedLessEqual, l)
              });
            }
          } else {
            unsupported_instruction_form();
          }

          self.ir_lowering_a_64_finalize_target_label(inst.op(2), index, &mut fresh);
        }
      }
      IrCmd::JumpSlotMatch | IrCmd::CheckSlotMatch => {
        {
          let mut abort = Label::default(); // used when guard aborts execution
          let mismatch_op = if inst.cmd == IrCmd::JumpSlotMatch {
            inst.op(3)
          } else {
            inst.op(2)
          };
          let mismatch = if mismatch_op.kind() == IrOpKind::Undef {
            ptr::from_mut(&mut abort)
          } else {
            ptr::from_mut(self.label_op(mismatch_op))
          };

          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp1w = cast_reg(KindA64::W, temp1);
          let temp2 = self.regs.alloc_temp(KindA64::X);

          CODEGEN_ASSERT!(K_OFFSET_OF_TKEY_TAG_NEXT >= 8 && K_OFFSET_OF_TKEY_TAG_NEXT < 16);
          let hoist_86 = self.reg_op(inst.op(0));
          self.emit_ldp(
            temp1,
            temp2,
            mem(hoist_86, (offset_of!(LuaNode, key) as i32)),
          ); // load key.value into temp1 and key.tt (alongside other bits) into temp2
          self.build_mut().ubfx(
            temp2,
            temp2,
            ((K_OFFSET_OF_TKEY_TAG_NEXT - 8) * 8) as u8,
            K_TKEY_TAG_BITS as u8,
          ); // .tt is right before .next, and 8 bytes are skipped by ldp
          self.build_mut().cmp(temp2, (LUA_TSTRING) as u16);
          self.with_target_label(mismatch, |s, l| {
            s.build_mut()
              .b_condition_a_64_label(ConditionA64::NotEqual, l)
          });

          let addr = self.temp_addr(inst.op(1), (offset_of!(TValue, value) as i32));
          self.build_mut().ldr(temp2, addr);
          self.build_mut().cmp(temp1, temp2);
          self.with_target_label(mismatch, |s, l| {
            s.build_mut()
              .b_condition_a_64_label(ConditionA64::NotEqual, l)
          });

          let hoist_87 = self.reg_op(inst.op(0));
          self.emit_ldr(
            temp1w,
            mem(
              hoist_87,
              (offset_of!(LuaNode, val) + offset_of!(TValue, tt)) as i32,
            ),
          );
          CODEGEN_ASSERT!(LUA_TNIL == 0);
          self.with_target_label(mismatch, |s, l| s.build_mut().cbz(temp1w, l));

          if inst.cmd == IrCmd::JumpSlotMatch {
            self.jump_or_fallthrough_op(inst.op(2), next);
          } else if abort.id != 0 {
            emit_abort(self.build_mut(), &mut abort);
          }
        }
      }
      IrCmd::CheckNodeNoNext => {
        {
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let temp = self.regs.alloc_temp(KindA64::W);

          let hoist_88 = self.reg_op(inst.op(0));
          self.emit_ldr(
            temp,
            mem(
              hoist_88,
              (offset_of!(LuaNode, key) as i32) + K_OFFSET_OF_TKEY_TAG_NEXT,
            ),
          );
          self.build_mut().lsr(temp, temp, K_TKEY_TAG_BITS);
          let target = self.ir_lowering_a_64_get_target_label(inst.op(1), index, &mut fresh);
          self.with_target_label(target, |s, l| s.build_mut().cbnz(temp, l));
          self.ir_lowering_a_64_finalize_target_label(inst.op(1), index, &mut fresh);
        }
      }
      IrCmd::CheckNodeValue => {
        {
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let temp = self.regs.alloc_temp(KindA64::W);

          let hoist_89 = self.reg_op(inst.op(0));
          self.emit_ldr(
            temp,
            mem(
              hoist_89,
              (offset_of!(LuaNode, val) + offset_of!(TValue, tt)) as i32,
            ),
          );
          CODEGEN_ASSERT!(LUA_TNIL == 0);
          let target = self.ir_lowering_a_64_get_target_label(inst.op(1), index, &mut fresh);
          self.with_target_label(target, |s, l| s.build_mut().cbz(temp, l));
          self.ir_lowering_a_64_finalize_target_label(inst.op(1), index, &mut fresh);
        }
      }
      IrCmd::CheckBufferLen => {
        {
          let min_offset = self.int_op(inst.op(2));
          let max_offset = self.int_op(inst.op(3));
          CODEGEN_ASSERT!(min_offset < max_offset);
          CODEGEN_ASSERT!(
            min_offset >= -((K_MAX_IMMEDIATE) as i32) && min_offset <= ((K_MAX_IMMEDIATE) as i32)
          );

          let access_size = max_offset - min_offset;
          CODEGEN_ASSERT!(access_size > 0 && access_size <= ((K_MAX_IMMEDIATE) as i32));

          // 要让跳转到 exit sync block 成立，每个可能被取分支都需相同的寄存器分配状态
          let reg_a = if (inst.op(0)).kind() == IrOpKind::Inst {
            self.reg_op(inst.op(0))
          } else {
            NOREG
          };
          let reg_b = if (inst.op(1)).kind() == IrOpKind::Inst {
            self.reg_op(inst.op(1))
          } else {
            NOREG
          };
          let reg_e = if (inst.op(4)).kind() != IrOpKind::Undef {
            self.reg_op(inst.op(4))
          } else {
            NOREG
          };
          let temp_w1 = self.regs.alloc_temp(KindA64::W);
          let temp_w2 = self.regs.alloc_temp(KindA64::W);
          let temp_d = self.regs.alloc_temp(KindA64::D);

          // 校验本次多分支指令 lowering 不再分配其他寄存器
          self.exit_sync_inst_idx = index;
          self.exit_sync_alloc_token = self.regs.get_alloc_token();

          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let target = self.ir_lowering_a_64_get_target_label(inst.op(5), index, &mut fresh);

          // 检查它是否不仅是 size guard，还是 offset 恰为整数的 guard
          if (inst.op(4)).kind() != IrOpKind::Undef {
            CODEGEN_ASSERT!(
              get_cmd_value_kind(self.function_mut().inst_op(inst.op(1)).cmd) == IrValueKind::Int
            );
            CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
              self.function_mut().inst_op(inst.op(1)).cmd
            )); // Ensure that high register bits are cleared

            if (self.build_mut().features & FEATURE_JSCVT) != 0 {
              let temp = temp_w1;

              self.build_mut().fjcvtzs(temp, reg_e); // fjcvtzs sets PSTATE.Z (equal) iff conversion is exact
              self.with_target_label(target, |s, l| {
                s.build_mut()
                  .b_condition_a_64_label(ConditionA64::NotEqual, l)
              });
            } else {
              let temp = temp_d;

              self.build_mut().scvtf(temp, reg_b);
              self.build_mut().fcmp(reg_e, temp);
              self.with_target_label(target, |s, l| {
                s.build_mut()
                  .b_condition_a_64_label(ConditionA64::NotEqual, l)
              });
            }
          }

          let temp = temp_w1;
          self.build_mut().ldr(temp, mem(reg_a, K_BUFFER_LEN_OFFSET));

          if (inst.op(1)).kind() == IrOpKind::Inst {
            CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
              self.function_mut().inst_op(inst.op(1)).cmd
            )); // Ensure that high register bits are cleared

            if access_size == 1 && min_offset == 0 {
              // offset >= len 时失败
              self.build_mut().cmp(temp, reg_b);
              self.with_target_label(target, |s, l| {
                s.build_mut()
                  .b_condition_a_64_label(ConditionA64::UnsignedLessEqual, l)
              });
            } else if min_offset >= 0 && max_offset <= ((K_MAX_IMMEDIATE) as i32) {
              // offset + size > len 时失败；此处按 len - offset < size 计算
              let tempx = cast_reg(KindA64::X, temp);
              self.build_mut().sub(tempx, tempx, reg_b); // implicit uxtw
              self.build_mut().cmp(tempx, (max_offset) as u16);
              self.with_target_label(target, |s, l| {
                s.build_mut().b_condition_a_64_label(ConditionA64::Less, l)
              });
            // 注：这是有符号 64 位比较，越界 offset 会失败
            } else {
              let tempx = cast_reg(KindA64::X, temp);
              let temp2 = cast_reg(KindA64::X, temp_w2);

              // 以 32 位取 base offset
              if min_offset >= 0 {
                self
                  .build_mut()
                  .add(cast_reg(KindA64::W, temp2), reg_b, (min_offset) as u16);
              } else {
                self
                  .build_mut()
                  .sub(cast_reg(KindA64::W, temp2), reg_b, (-min_offset) as u16);
              }

              // fail if (((offset + min_offset) as u32)) as u64 { + access_size > length
              self.build_mut().add(temp2, temp2, (access_size) as u16);
              self.build_mut().cmp(temp2, tempx);
              self.with_target_label(target, |s, l| {
                s.build_mut()
                  .b_condition_a_64_label(ConditionA64::UnsignedGreater, l)
              });
            }
          } else if (inst.op(1)).kind() == IrOpKind::Constant {
            let offset = self.int_op(inst.op(1));
            // cpp IrLoweringA64.cpp:2701-2703: LuauCodegenFixBufferLenCheck 已定值
            let end_offset = max_offset;
            let fail_cond = ConditionA64::UNSIGNED_LESS;

            // 常量折叠本可处理，但这里仍为安全防溢出/下溢
            if offset < 0 || ((offset) as u32) + ((end_offset) as u32) >= ((INT_MAX) as u32) {
              self.with_target_label(target, |s, l| s.build_mut().b(l));
            } else if offset + end_offset <= ((K_MAX_IMMEDIATE) as i32) {
              self.build_mut().cmp(temp, (offset + end_offset) as u16);
              self.with_target_label(target, |s, l| {
                s.build_mut().b_condition_a_64_label(fail_cond, l)
              });
            } else {
              let temp2 = temp_w2;
              self.build_mut().mov(temp2, offset + end_offset);
              self.build_mut().cmp(temp, temp2);
              self.with_target_label(target, |s, l| {
                s.build_mut().b_condition_a_64_label(fail_cond, l)
              });
            }
          } else {
            unsupported_instruction_form();
          }
          self.ir_lowering_a_64_finalize_target_label(inst.op(5), index, &mut fresh);
        }
      }
      IrCmd::CheckUserdataTag => {
        {
          CODEGEN_ASSERT!(((self.int_op(inst.op(1))) as u32) <= K_MAX_IMMEDIATE as u32);

          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let fail = self.ir_lowering_a_64_get_target_label(inst.op(2), index, &mut fresh);
          let temp = self.regs.alloc_temp(KindA64::W);
          let hoist_90 = self.reg_op(inst.op(0));
          self.emit_ldrb(temp, mem(hoist_90, (offset_of!(Udata, tag) as i32)));
          self.emit_cmp(temp, (self.int_op(inst.op(1))) as u16);
          self.with_target_label(fail, |s, l| {
            s.build_mut()
              .b_condition_a_64_label(ConditionA64::NotEqual, l)
          });
          self.ir_lowering_a_64_finalize_target_label(inst.op(2), index, &mut fresh);
        }
      }
      IrCmd::CheckCmpNum => {
        {
          let cond = condition_op(inst.op(2));
          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let fail = self.ir_lowering_a_64_get_target_label(inst.op(3), index, &mut fresh);

          let temp_a = self.temp_double(inst.op(0));

          let hoist_91 = self.temp_double(inst.op(1));
          self.emit_fcmp(temp_a, hoist_91);
          self.with_target_label(fail, |s, l| {
            s.build_mut()
              .b_condition_a_64_label(get_condition_f_p(get_negated_condition(cond)), l)
          });

          self.ir_lowering_a_64_finalize_target_label(inst.op(3), index, &mut fresh);
        }
      }
      IrCmd::CheckCmpInt => {
        {
          let cond = condition_op(inst.op(2));

          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let fail = self.ir_lowering_a_64_get_target_label(inst.op(3), index, &mut fresh);

          if cond == IrCondition::Equal && self.int_op(inst.op(1)) == 0 {
            let reg = self.reg_op(inst.op(0));
            self.with_target_label(fail, |s, l| s.build_mut().cbnz(reg, l));
          } else if cond == IrCondition::NotEqual && self.int_op(inst.op(1)) == 0 {
            let reg = self.reg_op(inst.op(0));
            self.with_target_label(fail, |s, l| s.build_mut().cbz(reg, l));
          } else {
            let temp_a = self.temp_int(inst.op(0));

            if (inst.op(1)).kind() == IrOpKind::Constant
              && ((self.int_op(inst.op(1))) as u32) <= K_MAX_IMMEDIATE as u32
            {
              self.emit_cmp(temp_a, (self.int_op(inst.op(1))) as u16);
            } else {
              let hoist_92 = self.temp_int(inst.op(1));
              self.emit_cmp(temp_a, hoist_92);
            }

            self.with_target_label(fail, |s, l| {
              s.build_mut()
                .b_condition_a_64_label(get_condition_int(get_negated_condition(cond)), l)
            });
          }
          self.ir_lowering_a_64_finalize_target_label(inst.op(3), index, &mut fresh);
        }
      }
      IrCmd::CheckCmpInt64 => {
        {
          let cond = condition_op(inst.op(2));

          let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
          let fail = self.ir_lowering_a_64_get_target_label(inst.op(3), index, &mut fresh);

          if cond == IrCondition::Equal
            && (inst.op(1)).kind() == IrOpKind::Constant
            && self.int_64_op(inst.op(1)) == 0
          {
            let reg = self.reg_op(inst.op(0));
            self.with_target_label(fail, |s, l| s.build_mut().cbnz(reg, l));
          } else if cond == IrCondition::NotEqual
            && (inst.op(1)).kind() == IrOpKind::Constant
            && self.int_64_op(inst.op(1)) == 0
          {
            let reg = self.reg_op(inst.op(0));
            self.with_target_label(fail, |s, l| s.build_mut().cbz(reg, l));
          } else {
            let temp_a = self.temp_int64(inst.op(0));

            if (inst.op(1)).kind() == IrOpKind::Constant
              && ((self.int_64_op(inst.op(1))) as u64) <= K_MAX_IMMEDIATE as u64
            {
              let hoist_5 = self.int_64_op(inst.op(1));
              self.emit_cmp(temp_a, hoist_5 as u16);
            } else {
              let hoist_93 = self.temp_int64(inst.op(1));
              self.emit_cmp(temp_a, hoist_93);
            }

            self.with_target_label(fail, |s, l| {
              s.build_mut()
                .b_condition_a_64_label(get_condition_int64(get_negated_condition(cond)), l);
            });
          }
          self.ir_lowering_a_64_finalize_target_label(inst.op(3), index, &mut fresh);
        }
      }
      IrCmd::INTERRUPT => {
        self.spill_regs(index, &[]);

        let mut self_ = Label::default();

        self.build_mut().ldr(
          X0,
          mem(
            R_GLOBAL_STATE,
            (offset_of!(global_State, cb.interrupt) as i32),
          ),
        );
        self.build_mut().cbnz(X0, &mut self_);

        let next = self.build_mut().set_label();

        self.interrupt_handlers.push(InterruptHandler {
          self_,
          pcpos: self.uint_op(inst.op(0)),
          next,
        });
      }
      IrCmd::CheckGc => {
        {
          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp2 = self.regs.alloc_temp(KindA64::X);

          CODEGEN_ASSERT!(
            (offset_of!(global_State, totalbytes) as i32)
              == (offset_of!(global_State, gc_threshold) as i32) + (size_of::<usize>() as i32)
          );
          let mut skip = Label::default();
          self.build_mut().ldp(
            temp1,
            temp2,
            mem(
              R_GLOBAL_STATE,
              (offset_of!(global_State, gc_threshold) as i32),
            ),
          );
          self.build_mut().cmp(temp1, temp2);
          self.emit_bcond(ConditionA64::UnsignedGreater, &mut skip);

          let spills = self.spill_regs(index, &[]);

          self.build_mut().mov(X0, R_STATE);
          self.build_mut().mov(W1, 1);
          self.emit_ldr(X2, native_ctx(offset_of!(NativeContext, lua_c_step)));
          self.build_mut().blr(X2);

          emit_update_base(self.build_mut());

          self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

          self.build_mut().set_label_label(&mut skip);
        }
      }
      IrCmd::BarrierObj => {
        {
          let temp = self.regs.alloc_temp(KindA64::X);

          let mut skip = Label::default();
          let object = self.reg_op(inst.op(0));
          let ratag = if (inst.op(2)).kind() == IrOpKind::Undef {
            -1
          } else {
            self.tag_op(inst.op(2)) as i32
          };
          self.check_object_barrier_conditions(object, temp, NOREG, inst.op(1), ratag, &mut skip);

          let reg = self.reg_op(inst.op(0)); // note: we need to call regOp before spill so that we don't do redundant reloads
          let spills = self.spill_regs(index, &[reg]);
          self.build_mut().mov(X1, reg);
          self.build_mut().mov(X0, R_STATE);
          self.build_mut().ldr(
            X2,
            mem(
              r_base(),
              vm_reg_op(inst.op(1)) * (size_of::<TValue>() as i32)
                + (offset_of!(TValue, value) as i32),
            ),
          );
          self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_c_barrierf)));
          self.build_mut().blr(X3);

          self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

          // 注：luaC_ barrier 不会重分配栈，故无需 emitUpdateBase
          self.build_mut().set_label_label(&mut skip);
        }
      }
      IrCmd::BarrierTableBack => {
        {
          let mut skip = Label::default();
          let temp = self.regs.alloc_temp(KindA64::W);

          // isblack(obj2gco(t))
          let hoist_94 = self.reg_op(inst.op(0));
          self.emit_ldrb(temp, mem(hoist_94, (offset_of!(GCheader, marked) as i32)));
          self.build_mut().tbz(temp, BLACKBIT as u8, &mut skip);

          let reg = self.reg_op(inst.op(0)); // note: we need to call regOp before spill so that we don't do redundant reloads
          let spills = self.spill_regs(index, &[reg]);
          self.build_mut().mov(X1, reg);
          self.build_mut().mov(X0, R_STATE);
          self
            .build_mut()
            .add(X2, X1, (offset_of!(LuaTable, gclist) as i32) as u16);
          self.emit_ldr(X3, native_ctx(offset_of!(NativeContext, lua_c_barrierback)));
          self.build_mut().blr(X3);

          self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

          // 注：luaC_ barrier 不会重分配栈，故无需 emitUpdateBase
          self.build_mut().set_label_label(&mut skip);
        }
      }
      IrCmd::BarrierTableForward => {
        {
          let temp = self.regs.alloc_temp(KindA64::X);

          let mut skip = Label::default();
          let object = self.reg_op(inst.op(0));
          let ratag = if (inst.op(2)).kind() == IrOpKind::Undef {
            -1
          } else {
            self.tag_op(inst.op(2)) as i32
          };
          self.check_object_barrier_conditions(object, temp, NOREG, inst.op(1), ratag, &mut skip);

          let reg = self.reg_op(inst.op(0)); // note: we need to call regOp before spill so that we don't do redundant reloads
          let addr = self.temp_addr(inst.op(1), (offset_of!(TValue, value) as i32));
          let spills = self.spill_regs(index, &[reg]);
          self.build_mut().mov(X1, reg);
          self.build_mut().mov(X0, R_STATE);
          self.build_mut().ldr(X2, addr);
          self.emit_ldr(
            X3,
            native_ctx(offset_of!(NativeContext, lua_c_barriertable)),
          );
          self.build_mut().blr(X3);

          self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

          // 注：luaC_ barrier 不会重分配栈，故无需 emitUpdateBase
          self.build_mut().set_label_label(&mut skip);
        }
      }
      IrCmd::SetSavedpc => {
        let temp1 = self.regs.alloc_temp(KindA64::X);
        let temp2 = self.regs.alloc_temp(KindA64::X);

        let hoist_off_1 =
          (self.uint_op(inst.op(0)) as i32).wrapping_mul(size_of::<Instruction>() as i32);
        emit_add_offset(self.build_mut(), temp1, R_CODE, hoist_off_1);
        self
          .build_mut()
          .ldr(temp2, mem(R_STATE, (offset_of!(LuaState, ci) as i32)));
        self
          .build_mut()
          .str(temp1, mem(temp2, (offset_of!(CallInfo, savedpc) as i32)));
      }
      IrCmd::CloseUpvals => {
        {
          let mut skip = Label::default();
          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp2 = self.regs.alloc_temp(KindA64::X);

          // l->openupval != 0
          self.build_mut().ldr(
            temp1,
            mem(R_STATE, (offset_of!(LuaState, openupval) as i32)),
          );
          self.build_mut().cbz(temp1, &mut skip);

          // ra <= l->openupval->v
          self
            .build_mut()
            .ldr(temp1, mem(temp1, (offset_of!(UpVal, v) as i32)));
          self.emit_vm_reg_addr(temp2, inst.op(0));
          self.build_mut().cmp(temp2, temp1);
          self.emit_bcond(ConditionA64::UnsignedGreater, &mut skip);

          let spills = self.spill_regs(index, &[temp2]);
          self.build_mut().mov(X1, temp2);
          self.build_mut().mov(X0, R_STATE);
          self.emit_ldr(X2, native_ctx(offset_of!(NativeContext, lua_f_close)));
          self.build_mut().blr(X2);

          self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

          self.build_mut().set_label_label(&mut skip);
        }
      }
      IrCmd::CAPTURE => {
        // no-op
      }
      IrCmd::SETLIST => {
        self.spill_regs(index, &[]);
        let hoist_pcpos_1 = self.uint_op(inst.op(0));
        emit_fallback(
          self.build_mut(),
          (offset_of!(NativeContext, execute_setlist) as i32),
          hoist_pcpos_1,
        );
      }
      IrCmd::CALL => {
        self.spill_regs(index, &[]);
        // argtop = if (nparams == LUA_MULTRET) { l->top } else { ra + 1 + nparams };
        if self.int_op(inst.op(1)) == LUA_MULTRET {
          self
            .build_mut()
            .ldr(X2, mem(R_STATE, (offset_of!(LuaState, top) as i32)));
        } else {
          self.emit_add(
            X2,
            r_base(),
            ((vm_reg_op(inst.op(0)) + 1 + self.int_op(inst.op(1))) * (size_of::<TValue>() as i32))
              as u16,
          );
        }

        // call_fallback(l, ra, argtop, nresults)
        self.build_mut().mov(X0, R_STATE);
        self.emit_vm_reg_addr(X1, inst.op(0));
        self.emit_mov(W3, self.int_op(inst.op(2)));
        self.emit_ldr(X4, native_ctx(offset_of!(NativeContext, call_fallback)));
        self.build_mut().blr(X4);

        emit_update_base(self.build_mut());

        // 重入，x0=closure（NULL 表示 C 函数；CALL_FALLBACK_YIELD 会触发退出）
        self.with_helper_label(|h| &mut h.continue_call, |s, l| s.emit_cbnz(X0, l));
      }
      IrCmd::RETURN => {
        self.spill_regs(index, &[]);

        if self.function_ref().variadic {
          self
            .build_mut()
            .ldr(X1, mem(R_STATE, (offset_of!(LuaState, ci) as i32)));
          self
            .build_mut()
            .ldr(X1, mem(X1, (offset_of!(CallInfo, func) as i32)));
        } else if self.int_op(inst.op(1)) != 1 {
          self
            .build_mut()
            .sub(X1, r_base(), (size_of::<TValue>() as i32) as u16);
        } // invariant: ci->func + 1 == ci->base for non-variadic frames

        if self.int_op(inst.op(1)) == 0 {
          self.build_mut().mov(W2, 0);
          self.with_helper_label(|h| &mut h.return_, |s, l| s.emit_b(l));
        } else if self.int_op(inst.op(1)) == 1 && !self.function_ref().variadic {
          // fast path：尽量减少 x1 调整
          // 注意上面该情形已跳过 x1 计算
          self.emit_ldr(Q0, vm_reg_addr(inst.op(0)));
          self
            .build_mut()
            .str(Q0, mem(r_base(), -((size_of::<TValue>() as i32) as i32)));
          self.build_mut().mov(X1, r_base());
          self.build_mut().mov(W2, 1);
          self.with_helper_label(|h| &mut h.return_, |s, l| s.emit_b(l));
        } else if self.int_op(inst.op(1)) >= 1 && self.int_op(inst.op(1)) <= 3 {
          for r in 0..self.int_op(inst.op(1)) {
            self.build_mut().ldr(
              Q0,
              mem(
                r_base(),
                (vm_reg_op(inst.op(0)) + r) * (size_of::<TValue>() as i32),
              ),
            );
            self.build_mut().str(
              Q0,
              mem_kind(X1, size_of::<TValue>() as i32, AddressKindA64::Post),
            );
          }
          self.emit_mov(W2, self.int_op(inst.op(1)));
          self.with_helper_label(|h| &mut h.return_, |s, l| s.emit_b(l));
        } else {
          self.build_mut().mov(W2, 0);

          // vali = ra
          self.emit_vm_reg_addr(X3, inst.op(0));

          // valend = if (n == LUA_MULTRET) { l->top } else { ra + n
          if self.int_op(inst.op(1)) == LUA_MULTRET {
            self
              .build_mut()
              .ldr(X4, mem(R_STATE, (offset_of!(LuaState, top) as i32)));
          } else {
            self.emit_add(
              X4,
              r_base(),
              ((vm_reg_op(inst.op(0)) + self.int_op(inst.op(1))) * (size_of::<TValue>() as i32))
                as u16,
            );
          }

          let mut repeat_value_loop = Label::default();
          let mut exit_value_loop = Label::default();

          if self.int_op(inst.op(1)) == LUA_MULTRET {
            self.build_mut().cmp(X3, X4);
            self.emit_bcond(ConditionA64::CarrySet, &mut exit_value_loop);
            // CarrySet == UNSIGNED_GREATER_EQUAL
          }

          self.build_mut().set_label_label(&mut repeat_value_loop);
          self.build_mut().ldr(
            Q0,
            mem_kind(X3, size_of::<TValue>() as i32, AddressKindA64::Post),
          );
          self.build_mut().str(
            Q0,
            mem_kind(X1, size_of::<TValue>() as i32, AddressKindA64::Post),
          );
          self.build_mut().add(W2, W2, 1_u16);
          self.build_mut().cmp(X3, X4);
          self.emit_bcond(ConditionA64::CarryClear, &mut repeat_value_loop); // CarryClear == UNSIGNED_LESS

          self.build_mut().set_label_label(&mut exit_value_loop);
          self.with_helper_label(|h| &mut h.return_, |s, l| s.emit_b(l));
        }
      }
      IrCmd::FORGLOOP => {
        // 寄存器布局：ra + 1 = table，ra + 2 = 内部索引，ra + 3 .. ra + aux = 迭代变量
        self.spill_regs(index, &[]);
        // 清空额外变量，因为可能多于两个
        if self.int_op(inst.op(1)) > 2 {
          CODEGEN_ASSERT!(LUA_TNIL == 0);
          for i in 2..self.int_op(inst.op(1)) {
            self.build_mut().str(
              WZR,
              mem(
                r_base(),
                (vm_reg_op(inst.op(0)) + 3 + i) * (size_of::<TValue>() as i32)
                  + (offset_of!(TValue, tt) as i32),
              ),
            );
          }
        }
        // 暂用 full iter fallback；将来可考虑在此加速数组迭代
        self.build_mut().mov(X0, R_STATE);
        self.build_mut().ldr(
          X1,
          mem(
            r_base(),
            (vm_reg_op(inst.op(0)) + 1) * (size_of::<TValue>() as i32) + K_TVALUE_VALUE_GC_OFFSET,
          ),
        );
        self.build_mut().ldr(
          W2,
          mem(
            r_base(),
            (vm_reg_op(inst.op(0)) + 2) * (size_of::<TValue>() as i32) + K_TVALUE_VALUE_P_OFFSET,
          ),
        );
        self.emit_vm_reg_addr(X3, inst.op(0));
        self.emit_ldr(
          X4,
          native_ctx(offset_of!(NativeContext, forg_loop_table_iter)),
        );
        self.build_mut().blr(X4);
        // 注：forgLoopTableIter 不重分配栈，故无需 emitUpdateBase
        self.with_op_label(inst.op(2), |s, l| s.build_mut().cbnz(W0, l));
        self.jump_or_fallthrough_op(inst.op(3), next);
      }
      IrCmd::ForgloopFallback => {
        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);
        self.build_mut().mov(W1, vm_reg_op(inst.op(0)));
        self.emit_mov(W2, self.int_op(inst.op(1)));

        // cpp IrLoweringA64.cpp:3081-3088: LuauYieldIter2 已定值, 恒用三态 int 版 fallback
        self.emit_ldr(
          X3,
          native_ctx(offset_of!(NativeContext, forg_loop_non_table_fallback)),
        );
        self.build_mut().blr(X3);
        emit_update_base(self.build_mut());
        self.build_mut().cmp(W0, 0_u16);
        self.with_helper_label(
          |h| &mut h.exit_no_continue_vm,
          |s, l| {
            s.emit_b_condition_a_64_label(ConditionA64::Less, l);
          },
        );
        self.with_op_label(inst.op(2), |s, l| {
          s.build_mut()
            .b_condition_a_64_label(ConditionA64::Greater, l)
        });

        self.jump_or_fallthrough_op(inst.op(3), next);
      }
      IrCmd::ForgprepXnextFallback => {
        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);
        self.emit_vm_reg_addr(X1, inst.op(1));
        self.emit_mov(W2, self.uint_op(inst.op(0)) + 1);
        self.emit_ldr(
          X3,
          native_ctx(offset_of!(NativeContext, forg_prep_xnext_fallback)),
        );
        self.build_mut().blr(X3);
        // 注：forgPrepXnextFallback 不重分配栈，故无需 emitUpdateBase
        self.jump_or_fallthrough_op(inst.op(2), next);
      }
      IrCmd::COVERAGE => {
        {
          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp2 = self.regs.alloc_temp(KindA64::W);
          let temp3 = self.regs.alloc_temp(KindA64::W);

          self.emit_mov(
            temp1,
            (self.uint_op(inst.op(0)) as i32).wrapping_mul(size_of::<Instruction>() as i32),
          );
          self.build_mut().ldr(temp2, mem(R_CODE, temp1));

          // E（高 24 位）加 1；若 23 位计数器溢出，最高位变 1
          // 注：cmp 可用 adds 省掉，但为覆盖率不必在意代码尺寸
          self.build_mut().add(temp3, temp2, 256_u16);
          self.build_mut().cmp(temp3, 0_u16);
          self
            .build_mut()
            .csel(temp2, temp2, temp3, ConditionA64::Less);

          self.build_mut().str(temp2, mem(R_CODE, temp1));
        }

        // 完整指令 fallback
      }
      IrCmd::FallbackGetglobal => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(2)).kind() == IrOpKind::VmConst);
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_getglobal));
      }
      IrCmd::FallbackSetglobal => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(2)).kind() == IrOpKind::VmConst);
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_setglobal));
      }
      IrCmd::FallbackGettableks => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(2)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(3)).kind() == IrOpKind::VmConst);
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_gettableks));
      }
      IrCmd::FallbackSettableks => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(2)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(3)).kind() == IrOpKind::VmConst);
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_settableks));
      }
      IrCmd::FallbackNamecall => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(2)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(3)).kind() == IrOpKind::VmConst);
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_namecall));
      }
      IrCmd::FallbackPrepvarargs => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::Constant);
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_prepvarargs));
      }
      IrCmd::FallbackGetvarargs => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(2)).kind() == IrOpKind::Constant);

        self.spill_regs(index, &[]);
        self.build_mut().mov(X0, R_STATE);

        if self.int_op(inst.op(2)) == LUA_MULTRET {
          let hoist_off_2 =
            (self.uint_op(inst.op(0)) as i32).wrapping_mul(size_of::<Instruction>() as i32);
          emit_add_offset(self.build_mut(), X1, R_CODE, hoist_off_2);
          self.build_mut().mov(X2, r_base());
          self.build_mut().mov(W3, vm_reg_op(inst.op(1)));
          self.emit_ldr(
            X4,
            native_ctx(offset_of!(NativeContext, execute_getvarargsmult_ret)),
          );
          self.build_mut().blr(X4);

          emit_update_base(self.build_mut());
        } else {
          self.build_mut().mov(X1, r_base());
          self.build_mut().mov(W2, vm_reg_op(inst.op(1)));
          self.emit_mov(W3, self.int_op(inst.op(2)));
          self.emit_ldr(
            X4,
            native_ctx(offset_of!(NativeContext, execute_getvarargsconst)),
          );
          self.build_mut().blr(X4);

          // 注：executeGETVARARGSConst 不重分配栈，故无需 emitUpdateBase
        }
      }
      IrCmd::NEWCLOSURE => {
        {
          let reg = self.reg_op(inst.op(1)); // note: we need to call regOp before spill so that we don't do redundant reloads

          self.spill_regs(index, &[reg]);
          self.build_mut().mov(X2, reg);

          self.build_mut().mov(X0, R_STATE);
          self.emit_mov(W1, self.uint_op(inst.op(0)));

          self
            .build_mut()
            .ldr(X3, mem(R_CLOSURE, K_CLOSURE_L_P_OFFSET));
          self
            .build_mut()
            .ldr(X3, mem(X3, (offset_of!(Proto, p) as i32)));

          let proto_index = self.uint_op(inst.op(2)); // 0..32767
          let proto_offset = (K_NATIVE_PTR_SIZE as i32) * proto_index as i32;

          if proto_index <= AddressA64::K_MAX_OFFSET as u32 {
            self.build_mut().ldr(X3, mem(X3, proto_offset));
          } else {
            self.build_mut().mov(X4, proto_offset);
            self.build_mut().ldr(X3, mem(X3, X4));
          }

          self.emit_ldr(
            X4,
            native_ctx(offset_of!(NativeContext, lua_f_new_lclosure)),
          );
          self.build_mut().blr(X4);

          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
      }
      IrCmd::FallbackDupclosure => {
        CODEGEN_ASSERT!((inst.op(1)).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!((inst.op(2)).kind() == IrOpKind::VmConst);
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_dupclosure));
      }
      IrCmd::FallbackForgprep => {
        self.lower_fallback(index, inst, offset_of!(NativeContext, execute_forgprep));
        self.jump_or_fallthrough_op(inst.op(2), next);

        // 伪指令
      }
      IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MarkUsed | IrCmd::MarkDead => {
        CODEGEN_ASSERT!(false, "Pseudo instructions should not be lowered");
      }
      IrCmd::BitandInt64 | IrCmd::BitxorInt64 | IrCmd::BitorInt64 => {
        let emit = match inst.cmd {
          IrCmd::BitandInt64 => AssemblyBuilderA64::and_,
          IrCmd::BitxorInt64 => AssemblyBuilderA64::eor,
          IrCmd::BitorInt64 => AssemblyBuilderA64::orr,
          _ => unreachable!(),
        };
        self.lower_bit_logic_int_64(inst, index, emit);
      }
      IrCmd::BitnotInt64 => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
        let temp = self.temp_int64(inst.op(0));
        self.build_mut().mvn_(inst.reg_a64, temp);
      }
      IrCmd::BitlshiftInt64 => {
        self.lower_bit_shift_int_64(
          inst,
          index,
          AssemblyBuilderA64::lsl,
          AssemblyBuilderA64::lsr,
        );
      }
      IrCmd::BitrshiftInt64 => {
        self.lower_bit_shift_int_64(
          inst,
          index,
          AssemblyBuilderA64::lsr,
          AssemblyBuilderA64::lsl,
        );
      }
      IrCmd::BitarshiftInt64 => {
        {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::X, index, &[(inst.op(0)), (inst.op(1))]);
          let source = self.temp_int64(inst.op(0));
          let amount = self.temp_int64(inst.op(1));
          let temp = self.regs.alloc_temp(KindA64::X);

          let mut done = Label::default();
          let mut negative = Label::default();
          let mut out_of_range_positive = Label::default();
          let mut out_of_range_negative = Label::default();

          // amount > 63（算术右移按符号填充）
          self.build_mut().cmp(amount, 63_u16);
          self.emit_bcond(ConditionA64::Greater, &mut out_of_range_positive);

          // 加 63，若 < 0 则 amount < -63
          self.build_mut().add(temp, amount, 63_u16);
          self.build_mut().cmp(temp, 0_u16);
          self.emit_bcond(ConditionA64::Less, &mut out_of_range_negative);

          // 检查 amount 符号
          self.build_mut().cmp(amount, 0_u16);
          self.emit_bcond(ConditionA64::Less, &mut negative);

          // 符号扩展的算术右移
          self.build_mut().asr(inst.reg_a64, source, amount);
          self.build_mut().b(&mut done);

          // 修复（opt-r84）：此处必须先绑定 negative 标签再左移。此前 set_label_label
          // 调用被误并入注释行，导致 negative 只被上方 `b(Less, negative)` 分支引用
          // 却永不绑定，finalize 时命中「未分配标签」断言（SIGTRAP）。对齐 C++ 上游
          // IrLoweringA64.cpp BIT_ARSHIFT_INT64 臂的 `build.setLabel(negative);`。
          self.build_mut().set_label_label(&mut negative);
          // 按 -amount 左移
          self.build_mut().neg(temp, amount);
          self.build_mut().lsl(inst.reg_a64, source, temp);
          self.build_mut().b(&mut done);

          // amount > 63 = 符号填充（if n < 0 { -1 } else { 0 }）
          self.build_mut().set_label_label(&mut out_of_range_positive);
          self.build_mut().asr(inst.reg_a64, source, 63_u8);
          self.build_mut().b(&mut done);

          // amount < -63 = 结果为 0
          self.build_mut().set_label_label(&mut out_of_range_negative);
          self.build_mut().mov(inst.reg_a64, 0);

          self.build_mut().set_label_label(&mut done);
        }
      }
      IrCmd::BitlrotateInt64 => {
        {
          inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(1))]); // can't reuse A because it would be clobbered by neg
          let source = self.temp_int64(inst.op(0));
          let amount = self.temp_int64(inst.op(1));
          // 左旋转 = 按负值旋转
          self.build_mut().neg(inst.reg_a64, amount);
          self.build_mut().ror(inst.reg_a64, source, inst.reg_a64);
        }
      }
      IrCmd::BitrrotateInt64 => {
        inst.reg_a64 = self
          .regs
          .alloc_reuse(KindA64::X, index, &[(inst.op(0)), (inst.op(1))]);
        let source = self.temp_int64(inst.op(0));
        let amount = self.temp_int64(inst.op(1));
        self.build_mut().ror(inst.reg_a64, source, amount);
      }
      IrCmd::BitcountlzInt64 => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
        let temp = self.temp_int64(inst.op(0));
        self.build_mut().clz(inst.reg_a64, temp);
      }
      IrCmd::BitcountrzInt64 => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
        let temp = self.temp_int64(inst.op(0));
        self.build_mut().rbit(inst.reg_a64, temp);
        self.build_mut().clz(inst.reg_a64, inst.reg_a64);
      }
      IrCmd::ByteswapInt64 => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::X, index, &[(inst.op(0))]);
        let temp = self.temp_int64(inst.op(0));
        self.build_mut().rev(inst.reg_a64, temp);
      }
      IrCmd::BitandUint | IrCmd::BitxorUint | IrCmd::BitorUint => {
        self.lower_bit_logic_uint(inst, index);
      }
      IrCmd::BitnotUint => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);
        let temp = self.temp_uint(inst.op(0));
        self.build_mut().mvn_(inst.reg_a64, temp);
      }
      IrCmd::BitlshiftUint | IrCmd::BitrshiftUint | IrCmd::BitarshiftUint => {
        self.lower_bit_shift_uint(inst, index);
      }
      IrCmd::BitlrotateUint => {
        {
          if (inst.op(0)).kind() == IrOpKind::Inst && (inst.op(1)).kind() == IrOpKind::Constant {
            inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);
            let hoist_101 = self.reg_op(inst.op(0));
            self.emit_ror(
              inst.reg_a64,
              hoist_101,
              // cpp `uint8_t((32 - unsigned(intOp(OP_B))) & 31)`：unsigned 减法
              // 环绕语义（IrLoweringA64.cpp:3529），须用 wrapping 避免下溢 panic
              (32u32.wrapping_sub(self.int_op(inst.op(1)) as u32) & 31) as u8,
            );
          } else {
            inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(1))]); // can't reuse a because it would be clobbered by neg
            let temp1 = self.temp_uint(inst.op(0));
            let temp2 = self.temp_uint(inst.op(1));
            self.build_mut().neg(inst.reg_a64, temp2);
            self.build_mut().ror(inst.reg_a64, temp1, inst.reg_a64);
          }
        }
      }
      IrCmd::BitrrotateUint => {
        self.lower_bit_shift_uint(inst, index);
      }
      IrCmd::BitcountlzUint => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);
        let temp = self.temp_uint(inst.op(0));
        self.build_mut().clz(inst.reg_a64, temp);
      }
      IrCmd::BitcountrzUint => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);
        let temp = self.temp_uint(inst.op(0));
        self.build_mut().rbit(inst.reg_a64, temp);
        self.build_mut().clz(inst.reg_a64, inst.reg_a64);
      }
      IrCmd::ByteswapUint => {
        inst.reg_a64 = self.regs.alloc_reuse(KindA64::W, index, &[(inst.op(0))]);
        let temp = self.temp_uint(inst.op(0));
        self.build_mut().rev(inst.reg_a64, temp);
      }
      IrCmd::InvokeLibm => {
        {
          if HAS_OP_C!(inst) {
            let is_int = if (inst.op(2)).kind() == IrOpKind::Constant {
              matches!(self.const_op(inst.op(2)), IrConst::Int(_))
            } else {
              get_cmd_value_kind(self.function_mut().inst_op(inst.op(2)).cmd) == IrValueKind::Int
            };

            let temp1 = self.temp_double(inst.op(1));
            let temp2 = if is_int {
              self.temp_int(inst.op(2))
            } else {
              self.temp_double(inst.op(2))
            };
            let temp3 = if is_int {
              NOREG
            } else {
              self.regs.alloc_temp(KindA64::D)
            }; // note: spill() frees all registers so we need to avoid alloc after spill
            self.spill_regs(index, &[temp1, temp2]);

            if is_int {
              self.build_mut().fmov(D0, temp1);
              self.build_mut().mov(W0, temp2);
            } else if D0 != temp2 {
              self.build_mut().fmov(D0, temp1);
              self.build_mut().fmov(D1, temp2);
            } else {
              self.build_mut().fmov(temp3, D0);
              self.build_mut().fmov(D0, temp1);
              self.build_mut().fmov(D1, temp3);
            }
          } else {
            let temp1 = self.temp_double(inst.op(1));
            self.spill_regs(index, &[temp1]);
            self.build_mut().fmov(D0, temp1);
          }

          self.emit_ldr(
            X1,
            mem(
              R_NATIVE_CONTEXT,
              get_native_context_offset(self.uint_op(inst.op(0)) as i32) as i32,
            ),
          );
          self.build_mut().blr(X1);
          inst.reg_a64 = self.regs.take_reg(D0, index);
        }
      }
      IrCmd::GetType => {
        {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);

          CODEGEN_ASSERT!((K_NATIVE_PTR_SIZE as i32) == 8);

          if (inst.op(0)).kind() == IrOpKind::Inst {
            let hoist_103 = self.reg_op(inst.op(0));
            self.emit_add_register_a_64_register_a_64_register_a_64_i32(
              inst.reg_a64,
              R_GLOBAL_STATE,
              hoist_103,
              3,
            );
          }
          // 隐式 uxtw
          else if (inst.op(0)).kind() == IrOpKind::Constant {
            self.emit_add(
              inst.reg_a64,
              R_GLOBAL_STATE,
              (self.tag_op(inst.op(0)) * 8) as u16,
            );
          } else {
            unsupported_instruction_form();
          }

          self.build_mut().ldr(
            inst.reg_a64,
            mem(inst.reg_a64, (offset_of!(global_State, ttname) as i32)),
          );
        }
      }
      IrCmd::GetTypeof => {
        self.lower_state_reg_helper(index, inst, offset_of!(NativeContext, lua_t_objtypenamestr));
      }
      IrCmd::FINDUPVAL => {
        self.lower_state_reg_helper(index, inst, offset_of!(NativeContext, lua_f_findupval));
      }
      IrCmd::BufferReadi8 => {
        self.lower_buffer_read_int(inst, index, AssemblyBuilderA64::ldrsb);
      }
      IrCmd::BufferReadu8 => {
        self.lower_buffer_read_int(inst, index, AssemblyBuilderA64::ldrb);
      }
      IrCmd::BufferWritei8 => {
        self.lower_buffer_write_int(inst, AssemblyBuilderA64::strb);
      }
      IrCmd::BufferReadi16 => {
        self.lower_buffer_read_int(inst, index, AssemblyBuilderA64::ldrsh);
      }
      IrCmd::BufferReadu16 => {
        self.lower_buffer_read_int(inst, index, AssemblyBuilderA64::ldrh);
      }
      IrCmd::BufferWritei16 => {
        self.lower_buffer_write_int(inst, AssemblyBuilderA64::strh);
      }
      IrCmd::BufferReadi32 => {
        self.lower_buffer_read_int(inst, index, AssemblyBuilderA64::ldr);
      }
      IrCmd::BufferWritei32 => {
        self.lower_buffer_write_int(inst, AssemblyBuilderA64::str);
      }
      IrCmd::BufferReadf32 => {
        self.lower_buffer_read_reg(inst, index, KindA64::S);
      }
      IrCmd::BufferWritef32 => {
        let temp = self.temp_float(inst.op(2));
        self.lower_buffer_write_reg(inst, temp);
      }
      IrCmd::BufferReadf64 => {
        self.lower_buffer_read_reg(inst, index, KindA64::D);
      }
      IrCmd::BufferWritef64 => {
        let temp = self.temp_double(inst.op(2));
        self.lower_buffer_write_reg(inst, temp);
      }
      IrCmd::BufferReadi64 => {
        self.lower_buffer_read_reg(inst, index, KindA64::X);
      }
      IrCmd::BufferWritei64 => {
        let temp = self.temp_int64(inst.op(2));
        self.lower_buffer_write_reg(inst, temp);
      }
      IrCmd::JumpCmpProtoid => {
        {
          CODEGEN_ASSERT!(
            (inst.op(0)).kind() == IrOpKind::Inst && (inst.op(1)).kind() == IrOpKind::Constant
          );
          let temp = self.regs.alloc_temp(KindA64::X);
          let tempw = cast_reg(KindA64::W, temp);

          // 是 C closure 吗？
          let hoist_104 = self.reg_op(inst.op(0));
          self.emit_ldrb(tempw, mem(hoist_104, (offset_of!(Closure, is_c) as i32)));
          self.with_op_label(inst.op(3), |s, l| s.build_mut().cbnz(tempw, l));

          // 加载 Proto 并比较 funid
          let hoist_105 = self.reg_op(inst.op(0));
          self.emit_ldr(temp, mem(hoist_105, K_CLOSURE_L_P_OFFSET));
          self
            .build_mut()
            .ldr(tempw, mem(temp, (offset_of!(Proto, funid) as i32)));
          let proto_id = self.uint_op(inst.op(1));
          if proto_id <= K_MAX_IMMEDIATE as u32 {
            self.build_mut().cmp(tempw, proto_id as u16);
          } else {
            let temp2 = self.regs.alloc_temp(KindA64::W);
            self.build_mut().mov(temp2, proto_id);
            self.build_mut().cmp(tempw, temp2);
          }

          self.with_op_label(inst.op(3), |s, l| {
            s.build_mut()
              .b_condition_a_64_label(ConditionA64::NotEqual, l)
          });

          self.jump_or_fallthrough_op(inst.op(2), next);
        }
      }
    }
    self.value_tracker.after_inst_lowering(inst, index);

    self.regs.curr_inst_idx = K_INVALID_INST_IDX;

    self.regs.free_last_use_regs(inst, index);
    self.regs.free_temp_regs();
  }
}

impl IrLoweringA64 {
  // `self.build_mut().M(.., self.f(..))` 形在实参位再次借用 self，与 build_mut 的即时
  // 可变借用互斥（E0499/E0502）；改为 `self.emit_M(..)` 后，方法调用的双相借用（two-phase
  // calls）允许实参求值期短暂共享借用 self，语义与化前逐位一致（求值序不变、无新别名）。
  // 全部即时转调 `build_mut` 视图访问器，裸解引用仍收口在 records 唯一契约点。
  #[inline]
  fn emit_add<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    self.build_mut().add(dst, src1, src2);
  }
  /// `dst = r_base + vmReg(op) * sizeof(TValue)`：VmReg 实参槽位地址物化（C ABI 传参、
  /// fallback 入参等 25 处同形样板）。`AddSubArg for u16` 走 `addRegRegImm` 单条编码，
  /// 与逐点内联展开逐条等价。
  #[inline]
  fn emit_vm_reg_addr(&mut self, dst: RegisterA64, op: IrOp) {
    let off = (vm_reg_op(op) * (size_of::<TValue>() as i32)) as u16;
    self.build_mut().add(dst, r_base(), off);
  }
  /// `dst = r_constants + vmConst(op) * sizeof(TValue)`：常量槽位地址物化，
  /// 与 `emit_vm_reg_addr` 的区别只在基寄存器与 `emit_add_offset` 的移位编码路径。
  #[inline]
  fn emit_vm_const_addr(&mut self, dst: RegisterA64, op: IrOp) {
    emit_add_offset(
      self.build_mut(),
      dst,
      r_constants(),
      vm_const_op(op) * (size_of::<TValue>() as i32),
    );
  }
  /// 临时寄存器溢出登记（cpp `regs.spill(index, {...})`）：生成名 `spill_u32_...` 的
  /// 三段链式调用在本函数体内占 42 处、每处 3 行，单点化为 1 行；实参皆局部值，
  /// `&mut self` 整体可变借用不与实参求值冲突。
  #[inline]
  fn spill_regs(&mut self, index: u32, live: &[RegisterA64]) -> usize {
    self
      .regs
      .spill_u32_initializer_list_register_a_64(index, live)
  }
  /// 条件跳转（cpp `build->b(cond, label)`）：长生成名 `b_condition_a_64_label` 的三段
  /// 链式调用点单点化，转调与实参求值序不变。
  #[inline]
  fn emit_bcond(&mut self, cond: ConditionA64, label: &mut Label) {
    self.build_mut().b_condition_a_64_label(cond, label);
  }
  /// GetTable/SetTable 孪生 lowering（两臂仅 NativeContext helper 槽不同）：X0=state、
  /// X1=op(1)、X2=op(2)（VmReg 直址 / 常量经栈上 TValue 取址）、X3=op(0)，调完回填 base。
  fn lower_get_set_table(&mut self, index: u32, inst: &mut IrInst, helper: usize) {
    self.spill_regs(index, &[]);
    self.build_mut().mov(X0, R_STATE);
    self.emit_vm_reg_addr(X1, inst.op(1));

    if (inst.op(2)).kind() == IrOpKind::VmReg {
      self.emit_vm_reg_addr(X2, inst.op(2));
    } else if (inst.op(2)).kind() == IrOpKind::Constant {
      let n = nvalue(self.uint_op(inst.op(2)) as f64);
      self.build_mut().adr_register_a_64_void_usize_align(
        X2,
        ptr::from_ref(&n).cast::<c_void>(),
        size_of::<TValue>(),
        align_of::<TValue>(),
      );
    } else {
      unsupported_instruction_form();
    }

    self.emit_vm_reg_addr(X3, inst.op(0));
    self.emit_ldr(X4, native_ctx(helper));
    self.build_mut().blr(X4);

    emit_update_base(self.build_mut());
  }
  /// FallbackXxx 八站同构骨架：spill 空集后 pcpos 取 op(0) 共享 emit_fallback，
  /// 仅 NativeContext helper 槽 `off` 不同；断言与尾随动作（Forgprep 跳转）留调用臂。
  fn lower_fallback(&mut self, index: u32, inst: &mut IrInst, off: usize) {
    self.spill_regs(index, &[]);
    let pcpos = self.uint_op(inst.op(0));
    emit_fallback(self.build_mut(), off as i32, pcpos);
  }
  /// GetTypeof/FINDUPVAL 孪生：X0=state、X1=op(0) 栈地址，调 NativeContext helper
  /// 后取 X0；两臂仅 helper 槽 `off` 不同。
  fn lower_state_reg_helper(&mut self, index: u32, inst: &mut IrInst, off: usize) {
    self.spill_regs(index, &[]);
    self.build_mut().mov(X0, R_STATE);
    self.emit_vm_reg_addr(X1, inst.op(0));
    self.emit_ldr(X2, native_ctx(off));
    self.build_mut().blr(X2);

    inst.reg_a64 = self.regs.take_reg(X0, index);
  }
  /// MinNum/MinFloat 与 Max 同构四臂：fcmp 后按 `cond` fcsel 取小/取大，
  /// `kind`/`temp` 区分双精度 D（temp_double）与单精度 S（temp_float）形态。
  fn lower_min_max(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    kind: KindA64,
    temp: fn(&mut Self, IrOp) -> RegisterA64,
    cond: IrCondition,
  ) {
    inst.reg_a64 = self
      .regs
      .alloc_reuse(kind, index, &[(inst.op(0)), (inst.op(1))]);
    let temp1 = temp(self, inst.op(0));
    let temp2 = temp(self, inst.op(1));
    self.build_mut().fcmp(temp1, temp2);
    self
      .build_mut()
      .fcsel(inst.reg_a64, temp1, temp2, get_condition_f_p(cond));
  }
  /// MinVec/MaxVec 孪生：fcmgt 掩码方向互为镜像（a>b 取大 / b>a 即 a<b 取小），
  /// bif/bit 掩码选择序列共享。
  fn lower_min_max_vec(&mut self, inst: &mut IrInst, index: u32, max: bool) {
    inst.reg_a64 = self
      .regs
      .alloc_reuse(KindA64::Q, index, &[(inst.op(0)), (inst.op(1))]);

    let temp1 = self.reg_op(inst.op(0));
    let temp2 = self.reg_op(inst.op(1));

    let mask = self.regs.alloc_temp(KindA64::Q);

    let (s1, s2) = if max { (temp1, temp2) } else { (temp2, temp1) };
    self.build_mut().fcmgt_4s(mask, s1, s2);

    // 若 A 已在目标处，则 mask 为 0 时选 B
    if inst.reg_a64 == temp1 {
      self.build_mut().bif(inst.reg_a64, temp2, mask);
    } else {
      // 除非 B 已在目标处否则存入，并 mask 为 1 时选 A
      if inst.reg_a64 != temp2 {
        self.build_mut().mov(inst.reg_a64, temp2);
      }

      self.build_mut().bit(inst.reg_a64, temp1, mask);
    }
  }
  /// JumpIfTruthy/Falsy 孪生：nil→假侧、非 boolean→真侧、boolean 值三段判定。
  /// JumpIfTruthy 真侧=op(1)/假侧=op(2)，JumpIfFalsy 互换；两臂 fallthrough 恒为 op(2)。
  fn jump_if_branch(&mut self, truthy: bool, op0: IrOp, op1: IrOp, op2: IrOp, next: &IrBlock) {
    let (t, f) = if truthy { (op1, op2) } else { (op2, op1) };
    let temp = self.regs.alloc_temp(KindA64::W);
    self.build_mut().ldr(
      temp,
      mem(
        r_base(),
        vm_reg_op(op0) * (size_of::<TValue>() as i32) + (offset_of!(TValue, tt) as i32),
      ),
    );
    // nil => falsy
    CODEGEN_ASSERT!(LUA_TNIL == 0);
    self.with_op_label(f, |s, l| s.build_mut().cbz(temp, l));
    // 非 boolean => truthy
    self.build_mut().cmp(temp, (LUA_TBOOLEAN) as u16);
    self.with_op_label(t, |s, l| {
      s.build_mut()
        .b_condition_a_64_label(ConditionA64::NotEqual, l)
    });
    // 比较 boolean 值
    self.build_mut().ldr(
      temp,
      mem(
        r_base(),
        vm_reg_op(op0) * (size_of::<TValue>() as i32) + (offset_of!(TValue, value) as i32),
      ),
    );
    if truthy {
      self.with_op_label(t, |s, l| s.build_mut().cbnz(temp, l));
    } else {
      self.with_op_label(f, |s, l| s.build_mut().cbz(temp, l));
    }
    self.jump_or_fallthrough_op(op2, next);
  }
  #[inline]
  fn emit_add_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    self
      .build_mut()
      .add_register_a_64_register_a_64_register_a_64_i32(dst, src1, src2, shift);
  }
  #[inline]
  fn emit_b(&mut self, label: &mut Label) {
    self.build_mut().b(label);
  }
  #[inline]
  fn emit_b_condition_a_64_label(&mut self, cond: ConditionA64, label: &mut Label) {
    self.build_mut().b_condition_a_64_label(cond, label);
  }
  #[inline]
  fn emit_cbnz(&mut self, src: RegisterA64, label: &mut Label) {
    self.build_mut().cbnz(src, label);
  }
  #[inline]
  fn emit_cmp<T: CmpArg>(&mut self, src1: RegisterA64, src2: T) {
    self.build_mut().cmp(src1, src2);
  }
  #[inline]
  fn emit_dup_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
    self.build_mut().dup_4s(dst, src, index);
  }
  #[inline]
  fn emit_eor<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    self.build_mut().eor(dst, src1, src2);
  }
  #[inline]
  fn emit_fabs(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.build_mut().fabs(dst, src);
  }
  #[inline]
  fn emit_fadd(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    self.build_mut().fadd(dst, src1, src2);
  }
  #[inline]
  fn emit_fcmp(&mut self, src1: RegisterA64, src2: RegisterA64) {
    self.build_mut().fcmp(src1, src2);
  }
  #[inline]
  fn emit_fcvt(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.build_mut().fcvt(dst, src);
  }
  #[inline]
  fn emit_fdiv(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    self.build_mut().fdiv(dst, src1, src2);
  }
  #[inline]
  fn emit_fmov<T: FmovArg>(&mut self, dst: RegisterA64, src: T) {
    self.build_mut().fmov(dst, src);
  }
  #[inline]
  fn emit_fmul(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    self.build_mut().fmul(dst, src1, src2);
  }
  #[inline]
  fn emit_fneg(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.build_mut().fneg(dst, src);
  }
  #[inline]
  fn emit_frinta(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.build_mut().frinta(dst, src);
  }
  #[inline]
  fn emit_frintm(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.build_mut().frintm(dst, src);
  }
  #[inline]
  fn emit_frintp(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.build_mut().frintp(dst, src);
  }
  #[inline]
  fn emit_fsqrt(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.build_mut().fsqrt(dst, src);
  }
  #[inline]
  fn emit_fsub(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    self.build_mut().fsub(dst, src1, src2);
  }
  #[inline]
  fn emit_is_mask_supported(&mut self, mask: u32) -> bool {
    self.build_mut().is_mask_supported(mask)
  }
  #[inline]
  fn emit_ldp(&mut self, dst1: RegisterA64, dst2: RegisterA64, src: AddressA64) {
    self.build_mut().ldp(dst1, dst2, src);
  }
  #[inline]
  fn emit_ldr(&mut self, dst: RegisterA64, src: AddressA64) {
    self.build_mut().ldr(dst, src);
  }
  #[inline]
  fn emit_ldrb(&mut self, dst: RegisterA64, src: AddressA64) {
    self.build_mut().ldrb(dst, src);
  }
  #[inline]
  fn emit_mov<T: MovArg>(&mut self, dst: RegisterA64, src: T) {
    self.build_mut().mov(dst, src);
  }
  #[inline]
  fn emit_ror<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    self.build_mut().ror(dst, src1, src2);
  }
  #[inline]
  fn emit_sbfx(&mut self, dst: RegisterA64, src: RegisterA64, offset: u8, size: u8) {
    self.build_mut().sbfx(dst, src, offset, size);
  }
  #[inline]
  fn emit_str(&mut self, src: RegisterA64, dst: AddressA64) {
    self.build_mut().str(src, dst);
  }
  #[inline]
  fn emit_sub<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
    self.build_mut().sub(dst, src1, src2);
  }
  #[inline]
  fn emit_tst<T: TstArg>(&mut self, src1: RegisterA64, src2: T) {
    self.build_mut().tst(src1, src2);
  }
  #[inline]
  fn emit_ubfx(&mut self, dst: RegisterA64, src: RegisterA64, f: u8, w: u8) {
    self.build_mut().ubfx(dst, src, f, w);
  }
  #[inline]
  fn emit_umov_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
    self.build_mut().umov_4s(dst, src, index);
  }
}
