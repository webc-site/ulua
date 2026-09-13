//! Node: `cxx:Method:Luau.CodeGen:CodeGen/src/IrLoweringA64.cpp:308:lowerInst`
//! Mechanically transpiled (translation/scripts/lowerinst_rewrite.py) + compiler-driven repair.
use core::{
  ffi::c_void,
  mem::{size_of, size_of_val},
};

use ulua_common::FFlag;
use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  macros::{blackbit::BLACKBIT, lua_multret::LUA_MULTRET, setnvalue::setnvalue},
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
    g_cheader::GCheader,
    global_state::global_State,
    lua_t_value::TValue,
    proto::Proto,
    t_string::tstring,
    udata::Udata,
    up_val::UpVal,
  },
  type_aliases::{
    instruction::Instruction, lua_node::LuaNode, lua_state::lua_State, lua_table::LuaTable,
    luau_fast_function::luau_FastFunction, value::Value,
  },
};

use crate::{
  enums::{
    address_kind_a_64::AddressKindA64, condition_a_64::ConditionA64, features_a_64::FeaturesA64,
    ir_cmd::IrCmd, ir_condition::IrCondition, ir_const_kind::IrConstKind, ir_op_kind::IrOpKind,
    ir_value_kind::IrValueKind, kind_a_64::KindA64,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64,
    interrupt_handler_ir_lowering_a_64::InterruptHandler, ir_block::IrBlock, ir_const::IrConst,
    ir_inst::IrInst, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp, label::Label,
    native_context::NativeContext, register_a_64::RegisterA64,
  },
};
// local register/address constants (mirrors EmitCommonA64.h)
const K_MAX_IMMEDIATE: u32 = 4095;
const INT_MAX: i32 = i32::MAX;
const K_TVALUE_SIZE_LOG2: i32 = 4;
const K_LUA_NODE_SIZE_LOG2: i32 = 5;
const K_OFFSET_OF_INSTRUCTION_C: i32 = 3;
const K_TSTRING_LEN_OFFSET: i32 = 36;
const K_BUFFER_LEN_OFFSET: i32 = 16;
const K_OFFSET_OF_TKEY_TAG_NEXT: i32 = 12;
const K_TKEY_TAG_BITS: i32 = 4;
const K_INVALID_INST_IDX: u32 = IrLoweringA64::K_INVALID_INST_IDX;
const FEATURE_JSCVT: u32 = FeaturesA64::FeatureJscvt as u32;
const FEATURE_ADV_SIMD: u32 = FeaturesA64::FeatureAdvSimd as u32;
const LUA_TNIL: u8 = LuaType::Nil as u8;
const LUA_TBOOLEAN: u8 = LuaType::Boolean as u8;
const LUA_TNUMBER: u8 = LuaType::Number as u8;
const LUA_TINTEGER: u8 = LuaType::Integer as u8;
const LUA_TVECTOR: u8 = LuaType::Vector as u8;
const LUA_TSTRING: u8 = LuaType::String as u8;
const LUA_TUPVAL: u8 = LuaType::Upval as u8;
const K_TVALUE_VALUE_GC_OFFSET: i32 =
  (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, gc)) as i32;
const K_TVALUE_VALUE_N_OFFSET: i32 =
  (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, n)) as i32;
const K_TVALUE_VALUE_L_OFFSET: i32 =
  (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, l)) as i32;
const K_TVALUE_VALUE_P_OFFSET: i32 =
  (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, p)) as i32;
const K_CLOSURE_L_P_OFFSET: i32 =
  (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32;
const K_CLOSURE_L_UPREFS_OFFSET: i32 =
  (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, uprefs)) as i32;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << RegisterA64::INDEX_SHIFT),
  }
}

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
  cast_reg::cast_reg, condition_op::condition_op, emit_abort::emit_abort,
  emit_add_offset::emit_add_offset as emit_add_offset_impl,
  emit_builtin_ir_lowering_a_64::emit_builtin_assembly_builder_a_64_ir_function_ir_reg_alloc_a_64_i32_i32_i32_i32 as emit_builtin,
  emit_fallback_ir_lowering_a_64::emit_fallback_assembly_builder_a_64_i32_i32 as emit_fallback_impl,
  emit_update_base_emit_common_a_64::emit_update_base, get_cmd_value_kind::get_cmd_value_kind,
  get_condition_fp::get_condition_fp, get_condition_int_64::get_condition_int_64,
  get_condition_int_ir_lowering_a_64::get_condition_int, get_double_bits::get_double_bits,
  get_float_bits::get_float_bits, get_inverse_condition_condition_a_64::get_inverse_condition,
  get_native_context_offset::get_native_context_offset,
  get_negated_condition_ir_utils::get_negated_condition_ir_condition as get_negated_condition,
  get_op_ir_data::get_op_mut, is_gco::is_gco,
  produces_dirty_high_register_bits::produces_dirty_high_register_bits, vm_const_op::vm_const_op,
  vm_reg_op::vm_reg_op, vm_upvalue_op::vm_upvalue_op as vm_upvalue_op_raw,
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

impl MovArg for u32 {
  fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.mov_register_a_64_i32(dst, self as i32);
  }
}

impl MovArg for u16 {
  fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.mov_register_a_64_i32(dst, self as i32);
  }
}

impl MovArg for u8 {
  fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
    build.mov_register_a_64_i32(dst, self as i32);
  }
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

impl AddSubArg for i32 {
  fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.add_register_a_64_register_a_64_u16(dst, src1, self as u16);
  }
  fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.sub_register_a_64_register_a_64_u16(dst, src1, self as u16);
  }
}

impl AddSubArg for u32 {
  fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.add_register_a_64_register_a_64_u16(dst, src1, self as u16);
  }
  fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
    build.sub_register_a_64_register_a_64_u16(dst, src1, self as u16);
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

impl CmpArg for i32 {
  fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
    build.cmp_register_a_64_u16(src1, self as u16);
  }
}

impl CmpArg for u32 {
  fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
    build.cmp_register_a_64_u16(src1, self as u16);
  }
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

fn has_op_b(inst: &IrInst) -> bool {
  1 < inst.ops.size() as usize && inst.ops[1].kind() != IrOpKind::None
}

fn has_op_c(inst: &IrInst) -> bool {
  2 < inst.ops.size() as usize && inst.ops[2].kind() != IrOpKind::None
}

fn has_op_d(inst: &IrInst) -> bool {
  3 < inst.ops.size() as usize && inst.ops[3].kind() != IrOpKind::None
}

fn has_op_e(inst: &IrInst) -> bool {
  4 < inst.ops.size() as usize && inst.ops[4].kind() != IrOpKind::None
}

fn get_condition_int64(cond: IrCondition) -> ConditionA64 {
  get_condition_int_64(cond)
}

fn get_condition_f_p(cond: IrCondition) -> ConditionA64 {
  get_condition_fp(cond)
}

impl IrLoweringA64 {
  fn int_op(&self, op: IrOp) -> i32 {
    unsafe { (*self.function).int_op(op) }
  }

  fn uint_op(&self, op: IrOp) -> u32 {
    unsafe { (*self.function).uint_op(op) }
  }

  fn int_64_op(&mut self, op: IrOp) -> i64 {
    unsafe { (*self.function).int64_op(op) }
  }

  fn double_op(&self, op: IrOp) -> f64 {
    unsafe { (*self.function).double_op(op) }
  }

  fn tag_op(&self, op: IrOp) -> u8 {
    unsafe { (*self.function).tag_op(op) }
  }

  fn const_op(&self, op: IrOp) -> IrConst {
    unsafe { (*self.function).const_op(op) }
  }

  fn import_op(&self, op: IrOp) -> u32 {
    unsafe { (*self.function).import_op(op) }
  }

  fn reg_op(&mut self, op: IrOp) -> RegisterA64 {
    self.ir_lowering_a_64_reg_op(op)
  }

  fn label_op(&mut self, op: IrOp) -> &mut Label {
    self.ir_lowering_a_64_label_op(op)
  }

  unsafe fn block_op(&self, op: IrOp) -> *mut IrBlock {
    unsafe { self.ir_lowering_a_64_block_op(op) }
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

  fn temp_addr_buffer(&mut self, buffer_op: IrOp, index_op: IrOp, tag: u8) -> AddressA64 {
    self.ir_lowering_a_64_temp_addr_buffer(buffer_op, index_op, tag)
  }

  fn temp_float(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let val = self.double_op(op) as f32;

      if unsafe { (*self.build).is_fmov_supported_fp_32(val) } {
        let temp = self.regs.alloc_temp(KindA64::S);
        unsafe { (*self.build).fmov_register_a_64_f32(temp, val) };
        temp
      } else {
        let temp = self.regs.alloc_temp(KindA64::S);
        let vali = get_float_bits(val);

        if (vali & 0xffff) == 0 {
          let temp2 = self.regs.alloc_temp(KindA64::W);
          unsafe {
            (*self.build).movz(temp2, (vali >> 16) as u16, 16);
            (*self.build).fmov_register_a_64_register_a_64(temp, temp2);
          }
        } else {
          let temp2 = self.regs.alloc_temp(KindA64::X);
          unsafe {
            (*self.build).adr_register_a_64_f32(temp2, val);
            (*self.build).ldr(temp, mem(temp2, 0));
          }
        }

        temp
      }
    } else {
      CODEGEN_ASSERT!(false, "Unsupported instruction form");
      RegisterA64::NOREG
    }
  }

  fn check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
    self.ir_lowering_a_64_check_safe_env(target, index, next)
  }

  fn is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
    self.ir_lowering_a_64_is_fallthrough_block(target, next)
  }

  fn jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
    self.ir_lowering_a_64_jump_or_fallthrough(target, next)
  }

  fn jump_or_fallthrough_op(&mut self, op: IrOp, next: &IrBlock) {
    let target = unsafe { self.block_op(op) };
    unsafe { self.jump_or_fallthrough(&mut *target, next) }
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

  pub fn ir_lowering_a_64_lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
    unsafe {
      self.regs.curr_inst_idx = index;

      self.value_tracker.before_inst_lowering(inst);
      match inst.cmd {
        IrCmd::LoadTag => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, tt) as i32),
          );
          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::LoadPointer => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
          let addr = self.temp_addr(*get_op_mut(inst, 0), K_TVALUE_VALUE_GC_OFFSET);
          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::LoadDouble => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
          let addr = self.temp_addr(*get_op_mut(inst, 0), K_TVALUE_VALUE_N_OFFSET);
          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::LoadInt => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, value) as i32),
          );
          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::LoadInt64 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
          let addr = self.temp_addr(*get_op_mut(inst, 0), K_TVALUE_VALUE_L_OFFSET);
          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::LoadFloat => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);
          let addr = self.temp_addr(*get_op_mut(inst, 0), self.int_op(*get_op_mut(inst, 1)));

          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::LoadTvalue => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);

          let addr_offset = if has_op_b(inst) {
            self.int_op(*get_op_mut(inst, 1))
          } else {
            0
          };
          let addr = self.temp_addr(*get_op_mut(inst, 0), addr_offset);
          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::LoadEnv => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
          (*self.build).ldr(
            inst.reg_a64,
            mem(R_CLOSURE, (core::mem::offset_of!(Closure, env) as i32)),
          );
        }
        IrCmd::GetArrAddr => {
          {
            inst.reg_a64 = self
              .regs
              .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
            (*self.build).ldr(
              inst.reg_a64,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, array) as i32),
              ),
            );

            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
              (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                inst.reg_a64,
                inst.reg_a64,
                self.reg_op(*get_op_mut(inst, 1)),
                K_TVALUE_SIZE_LOG2,
              ); // implicit uxtw
            } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
              if self.int_op(*get_op_mut(inst, 1)) == 0 {
                // no offset required
              } else if self.int_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)
                <= K_MAX_IMMEDIATE as i32
              {
                (*self.build).add(
                  inst.reg_a64,
                  inst.reg_a64,
                  (self.int_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
                );
              } else {
                let temp = self.regs.alloc_temp(KindA64::X);
                (*self.build).mov(
                  temp,
                  self.int_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32),
                );
                (*self.build).add(inst.reg_a64, inst.reg_a64, temp);
              }
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }
          }
        }
        IrCmd::GetSlotNodeAddr => {
          {
            inst.reg_a64 = self
              .regs
              .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp1w = cast_reg(KindA64::W, temp1);
            let temp2 = self.regs.alloc_temp(KindA64::W);
            let temp2x = cast_reg(KindA64::X, temp2);

            // note: since the stride of the load is the same as the destination register size, we can range check the array index, not the byte offset
            if self.uint_op(*get_op_mut(inst, 1)) <= AddressA64::K_MAX_OFFSET as u32 {
              (*self.build).ldr(
                temp1w,
                mem(
                  R_CODE,
                  (self.uint_op(*get_op_mut(inst, 1)) as i32) * (size_of::<Instruction>() as i32),
                ),
              );
            } else {
              (*self.build).mov(
                temp1,
                (self.uint_op(*get_op_mut(inst, 1)) as i32) * (size_of::<Instruction>() as i32),
              );
              (*self.build).ldr(temp1w, mem(R_CODE, temp1));
            }

            // C field can be shifted as long as it's at the most significant byte of the instruction word
            CODEGEN_ASSERT!(K_OFFSET_OF_INSTRUCTION_C == 3);
            (*self.build).ldrb(
              temp2,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, nodemask8) as i32),
              ),
            );
            (*self.build)
              .and_register_a_64_register_a_64_register_a_64_i32(temp2, temp2, temp1w, -24);

            // note: this may clobber (*get_op_mut(inst, 0)), so it's important that we don't use it after this
            (*self.build).ldr(
              inst.reg_a64,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, node) as i32),
              ),
            );
            (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
              inst.reg_a64,
              inst.reg_a64,
              temp2x,
              K_LUA_NODE_SIZE_LOG2,
            ); // "zero extend" temp2 to get a larger shift (top 32 bits are zero)
          }
        }
        IrCmd::GetHashNodeAddr => {
          {
            inst.reg_a64 = self
              .regs
              .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
            let temp1 = self.regs.alloc_temp(KindA64::W);
            let temp2 = self.regs.alloc_temp(KindA64::W);
            let temp2x = cast_reg(KindA64::X, temp2);

            // hash & ((1 << lsizenode) - 1) == hash & !(-1 << lsizenode)
            (*self.build).mov(temp1, -1);
            (*self.build).ldrb(
              temp2,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, lsizenode) as i32),
              ),
            );
            (*self.build).lsl(temp1, temp1, temp2);
            (*self.build).mov(temp2, self.uint_op(*get_op_mut(inst, 1)));
            (*self.build).bic(temp2, temp2, temp1, 0);

            // note: this may clobber (*get_op_mut(inst, 0)), so it's important that we don't use it after this
            (*self.build).ldr(
              inst.reg_a64,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, node) as i32),
              ),
            );
            (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
              inst.reg_a64,
              inst.reg_a64,
              temp2x,
              K_LUA_NODE_SIZE_LOG2,
            ); // "zero extend" temp2 to get a larger shift (top 32 bits are zero)
          }
        }
        IrCmd::GetClosureUpvalAddr => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
          let cl = if (*get_op_mut(inst, 0)).kind() == IrOpKind::Undef {
            R_CLOSURE
          } else {
            self.reg_op(*get_op_mut(inst, 0))
          };

          (*self.build).add(
            inst.reg_a64,
            cl,
            (K_CLOSURE_L_UPREFS_OFFSET
              + (size_of::<TValue>() as i32) * vm_upvalue_op(*get_op_mut(inst, 1)))
              as u16,
          );
        }
        IrCmd::StoreTag => {
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, tt) as i32),
          );
          if self.tag_op(*get_op_mut(inst, 1)) == 0 {
            (*self.build).str(WZR, addr);
          } else {
            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).mov(temp, self.tag_op(*get_op_mut(inst, 1)));
            (*self.build).str(temp, addr);
          }
        }
        IrCmd::StorePointer => {
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, value) as i32),
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
            CODEGEN_ASSERT!(self.int_op(*get_op_mut(inst, 1)) == 0);
            (*self.build).str(XZR, addr);
          } else {
            (*self.build).str(self.reg_op(*get_op_mut(inst, 1)), addr);
          }
        }
        IrCmd::StoreExtra => {
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, extra) as i32),
          );
          if self.int_op(*get_op_mut(inst, 1)) == 0 {
            (*self.build).str(WZR, addr);
          } else {
            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).mov(temp, self.int_op(*get_op_mut(inst, 1)));
            (*self.build).str(temp, addr);
          }
        }
        IrCmd::StoreDouble => {
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, value) as i32),
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && get_double_bits(self.double_op(*get_op_mut(inst, 1))) == 0
          {
            (*self.build).str(XZR, addr);
          } else {
            let temp = self.temp_double(*get_op_mut(inst, 1));
            (*self.build).str(temp, addr);
          }
        }
        IrCmd::StoreInt => {
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, value) as i32),
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && self.int_op(*get_op_mut(inst, 1)) == 0
          {
            (*self.build).str(WZR, addr);
          } else {
            let temp = self.temp_int(*get_op_mut(inst, 1));
            (*self.build).str(temp, addr);
          }
        }
        IrCmd::StoreInt64 => {
          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, value) as i32),
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && self.int_64_op(*get_op_mut(inst, 1)) == 0
          {
            (*self.build).str(XZR, addr);
          } else {
            let temp = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).str(temp, addr);
          }
        }
        IrCmd::StoreVector => {
          let temp1 = self.temp_float(*get_op_mut(inst, 1));
          let temp2 = self.temp_float(*get_op_mut(inst, 2));
          let temp3 = self.temp_float(*get_op_mut(inst, 3));

          let addr = self.temp_addr(
            *get_op_mut(inst, 0),
            (core::mem::offset_of!(TValue, value) as i32),
          );
          CODEGEN_ASSERT!(
            addr.kind == AddressKindA64::Imm
              && addr.data % 4 == 0
              && ((addr.data + 8) as u32) / 4 <= AddressA64::K_MAX_OFFSET as u32
          );

          (*self.build).str(temp1, mem(addr.base, addr.data));
          (*self.build).str(temp2, mem(addr.base, addr.data + 4));
          (*self.build).str(temp3, mem(addr.base, addr.data + 8));

          if has_op_e(inst) {
            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).mov(temp, self.tag_op(*get_op_mut(inst, 4)));
            (*self.build).str(
              temp,
              self.temp_addr(
                *get_op_mut(inst, 0),
                (core::mem::offset_of!(TValue, tt) as i32),
              ),
            );
          }
        }
        IrCmd::StoreTvalue => {
          let addr_offset = if has_op_c(inst) {
            self.int_op(*get_op_mut(inst, 2))
          } else {
            0
          };
          let addr = self.temp_addr(*get_op_mut(inst, 0), addr_offset);
          (*self.build).str(self.reg_op(*get_op_mut(inst, 1)), addr);
        }
        IrCmd::StoreSplitTvalue => {
          {
            let addr_offset = if has_op_d(inst) {
              self.int_op(*get_op_mut(inst, 3))
            } else {
              0
            };

            let tempt = self.regs.alloc_temp(KindA64::W);
            let addrt = self.temp_addr(
              *get_op_mut(inst, 0),
              (core::mem::offset_of!(TValue, tt) as i32) + addr_offset,
            );
            (*self.build).mov(tempt, self.tag_op(*get_op_mut(inst, 1)));
            (*self.build).str(tempt, addrt);

            let addr = self.temp_addr(
              *get_op_mut(inst, 0),
              (core::mem::offset_of!(TValue, value) as i32) + addr_offset,
            );

            if self.tag_op(*get_op_mut(inst, 1)) == LUA_TBOOLEAN {
              if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                // note: we reuse tag temp register as value for true booleans, and use built-in zero register for false values
                CODEGEN_ASSERT!(LUA_TBOOLEAN == 1);
                (*self.build).str(
                  if self.int_op(*get_op_mut(inst, 2)) != 0 {
                    tempt
                  } else {
                    WZR
                  },
                  addr,
                );
              } else {
                (*self.build).str(self.reg_op(*get_op_mut(inst, 2)), addr);
              }
            } else if self.tag_op(*get_op_mut(inst, 1)) == LUA_TNUMBER {
              let temp = self.temp_double(*get_op_mut(inst, 2));
              (*self.build).str(temp, addr);
            } else if self.tag_op(*get_op_mut(inst, 1)) == LUA_TINTEGER {
              let temp = self.temp_int64(*get_op_mut(inst, 2));
              (*self.build).str(temp, addr);
            } else if is_gco(self.tag_op(*get_op_mut(inst, 1))) {
              (*self.build).str(self.reg_op(*get_op_mut(inst, 2)), addr);
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }
          }
        }
        IrCmd::AddInt => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && ((self.int_op(*get_op_mut(inst, 1))) as u32) <= K_MAX_IMMEDIATE
          {
            (*self.build).add(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (self.int_op(*get_op_mut(inst, 1))) as u16,
            );
          } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant
            && ((self.int_op(*get_op_mut(inst, 0))) as u32) <= K_MAX_IMMEDIATE
          {
            (*self.build).add(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 1)),
              (self.int_op(*get_op_mut(inst, 0))) as u16,
            );
          } else {
            let temp1 = self.temp_int(*get_op_mut(inst, 0));
            let temp2 = self.temp_int(*get_op_mut(inst, 1));
            (*self.build).add(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::SubInt => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && ((self.int_op(*get_op_mut(inst, 1))) as u32) <= K_MAX_IMMEDIATE
          {
            (*self.build).sub(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (self.int_op(*get_op_mut(inst, 1))) as u16,
            );
          } else {
            let temp1 = self.temp_int(*get_op_mut(inst, 0));
            let temp2 = self.temp_int(*get_op_mut(inst, 1));
            (*self.build).sub(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::AddInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && ((self.int_64_op(*get_op_mut(inst, 1))) as u64) <= K_MAX_IMMEDIATE as u64
          {
            (*self.build).add(
              inst.reg_a64,
              self.temp_int64(*get_op_mut(inst, 0)),
              (self.int_64_op(*get_op_mut(inst, 1))) as u16,
            );
          } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant
            && ((self.int_64_op(*get_op_mut(inst, 0))) as u64) <= K_MAX_IMMEDIATE as u64
          {
            (*self.build).add(
              inst.reg_a64,
              self.temp_int64(*get_op_mut(inst, 1)),
              (self.int_64_op(*get_op_mut(inst, 0))) as u16,
            );
          } else {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).add(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::SubInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && ((self.int_64_op(*get_op_mut(inst, 1))) as u64) <= K_MAX_IMMEDIATE as u64
          {
            (*self.build).sub(
              inst.reg_a64,
              self.temp_int64(*get_op_mut(inst, 0)),
              (self.int_64_op(*get_op_mut(inst, 1))) as u16,
            );
          } else {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).sub(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::MulInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).mul(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::DivInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).sdiv(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::IdivInt64 => {
          // floored division: q = a / b, then if (q < 0 && a % b != 0) q -= 1
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index); // can't reuse: both operands needed for remainder
          {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            let temp_rem = self.regs.alloc_temp(KindA64::X);
            let temp_adj = self.regs.alloc_temp(KindA64::X);

            (*self.build).sdiv(inst.reg_a64, temp1, temp2); // result = a / b
            (*self.build).mov(temp_rem, inst.reg_a64); // copy quotient; rem requires dst to initially hold quotient
            (*self.build).rem(temp_rem, temp1, temp2);

            (*self.build).sub(temp_adj, inst.reg_a64, 1_u16); // adjusted = result - 1

            (*self.build).cmp(temp_rem, 0_u16);
            (*self.build).csel(temp_adj, temp_adj, inst.reg_a64, ConditionA64::NotEqual); // (remainder != 0) ? result-1 : result

            (*self.build).cmp(inst.reg_a64, 0_u16);
            (*self.build).csel(inst.reg_a64, temp_adj, inst.reg_a64, ConditionA64::Less);
            // (result < 0) ? temp_adj : result
          }
        }
        IrCmd::CheckDivInt64 => {
          {
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let fail =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 2), index, &mut fresh)
                as *mut Label;

            // guard against divide by zero
            let reg_b = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).cbz(reg_b, &mut *fail);

            // guard against if a is -2^63 and b is -1
            let reg_a = self.temp_int64(*get_op_mut(inst, 0));
            let temp_rotate = self.regs.alloc_temp(KindA64::X);

            // bit trick, if we are integer.minsigned (0x8000000000000000), then if we rotate by 63, we will get 1
            (*self.build).ror(temp_rotate, reg_a, 63);

            (*self.build).cmp(temp_rotate, 1_u16);

            // nzcv = 0000 EQ
            // nzcv = 0001 NE
            (*self.build).ccmn(reg_b, 1, get_condition_int64(IrCondition::Equal), 1);
            (*self.build)
              .b_condition_a_64_label(get_condition_int64(IrCondition::Equal), &mut *fail);

            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 2), index, &mut fresh);
          }
        }
        IrCmd::UdivInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).udiv(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::RemInt64 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
          {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).sdiv(inst.reg_a64, temp1, temp2);
            (*self.build).rem(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::ModInt64 => {
          // floored modulo: rem = a % b (C truncated); if (rem != 0 && sign(rem) != sign(b)) rem += b
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index); // can't reuse: dividend (temp1) needed after sdiv
          {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            let temp_rem = self.regs.alloc_temp(KindA64::X);
            let temp_adj = self.regs.alloc_temp(KindA64::X);

            (*self.build).sdiv(inst.reg_a64, temp1, temp2); // quotient = a / b
            (*self.build).mov(temp_rem, inst.reg_a64); // temp_rem = quotient
            (*self.build).rem(temp_rem, temp1, temp2); // temp_rem = C-style remainder

            (*self.build).add(temp_adj, temp_rem, temp2); // temp_adj = rem + b (floored candidate)
            (*self.build).eor(inst.reg_a64, temp_rem, temp2); // sign check: negative if signs differ

            (*self.build).cmp(inst.reg_a64, 0_u16);
            (*self.build).csel(temp_adj, temp_adj, temp_rem, ConditionA64::Less); // if signs differ then rem+b else rem

            (*self.build).cmp(temp_rem, 0_u16);
            (*self.build).csel(inst.reg_a64, temp_adj, temp_rem, ConditionA64::NotEqual);
            // if rem != 0 then adjusted else 0
          }
        }
        IrCmd::UremInt64 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
          {
            let temp1 = self.temp_int64(*get_op_mut(inst, 0));
            let temp2 = self.temp_int64(*get_op_mut(inst, 1));
            (*self.build).udiv(inst.reg_a64, temp1, temp2);
            (*self.build).rem(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::Sexti8Int => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);

          (*self.build).sbfx(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)), 0, 8);
          // sextb
        }
        IrCmd::Sexti16Int => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);

          (*self.build).sbfx(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)), 0, 16);
          // sexth
        }
        IrCmd::AddNum => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::D,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          (*self.build).fadd(inst.reg_a64, temp1, temp2);
        }
        IrCmd::SubNum => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::D,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          (*self.build).fsub(inst.reg_a64, temp1, temp2);
        }
        IrCmd::MulNum => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::D,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          (*self.build).fmul(inst.reg_a64, temp1, temp2);
        }
        IrCmd::DivNum => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::D,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          (*self.build).fdiv(inst.reg_a64, temp1, temp2);
        }
        IrCmd::IdivNum => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::D,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          (*self.build).fdiv(inst.reg_a64, temp1, temp2);
          (*self.build).frintm(inst.reg_a64, inst.reg_a64);
        }
        IrCmd::ModNum => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index); // can't allocReuse because both A and B are used twice
            let temp1 = self.temp_double(*get_op_mut(inst, 0));
            let temp2 = self.temp_double(*get_op_mut(inst, 1));
            (*self.build).fdiv(inst.reg_a64, temp1, temp2);
            (*self.build).frintm(inst.reg_a64, inst.reg_a64);
            (*self.build).fmul(inst.reg_a64, inst.reg_a64, temp2);
            (*self.build).fsub(inst.reg_a64, temp1, inst.reg_a64);
          }
        }
        IrCmd::MuladdNum => {
          let temp_a = self.temp_double(*get_op_mut(inst, 0));
          let temp_b = self.temp_double(*get_op_mut(inst, 1));
          let temp_c = self.temp_double(*get_op_mut(inst, 2));

          if ((*self.build).features & FEATURE_ADV_SIMD) != 0 {
            inst.reg_a64 = self
              .regs
              .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 2))]);
            if inst.reg_a64 != temp_c {
              (*self.build).fmov(inst.reg_a64, temp_c);
            }
            (*self.build).fmla(inst.reg_a64, temp_b, temp_a);
          } else {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
            (*self.build).fmul(inst.reg_a64, temp_b, temp_a);
            (*self.build).fadd(inst.reg_a64, inst.reg_a64, temp_c);
          }
        }
        IrCmd::MinNum => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::D,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          (*self.build).fcmp(temp1, temp2);
          (*self.build).fcsel(
            inst.reg_a64,
            temp1,
            temp2,
            get_condition_f_p(IrCondition::Less),
          );
        }
        IrCmd::MaxNum => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::D,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          (*self.build).fcmp(temp1, temp2);
          (*self.build).fcsel(
            inst.reg_a64,
            temp1,
            temp2,
            get_condition_f_p(IrCondition::Greater),
          );
        }
        IrCmd::UnmNum => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).fneg(inst.reg_a64, temp);
        }
        IrCmd::FloorNum => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).frintm(inst.reg_a64, temp);
        }
        IrCmd::CeilNum => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).frintp(inst.reg_a64, temp);
        }
        IrCmd::RoundNum => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).frinta(inst.reg_a64, temp);
        }
        IrCmd::SqrtNum => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).fsqrt(inst.reg_a64, temp);
        }
        IrCmd::AbsNum => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).fabs(inst.reg_a64, temp);
        }
        IrCmd::SignNum => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::D, index, &[(*get_op_mut(inst, 0))]);

          let temp = self.temp_double(*get_op_mut(inst, 0));
          let temp0 = self.regs.alloc_temp(KindA64::D);
          let temp1 = self.regs.alloc_temp(KindA64::D);

          (*self.build).fcmpz(temp);
          (*self.build).fmov(temp0, 0.0);
          (*self.build).fmov(temp1, 1.0);
          (*self.build).fcsel(
            inst.reg_a64,
            temp1,
            temp0,
            get_condition_f_p(IrCondition::Greater),
          );
          (*self.build).fmov(temp1, -1.0);
          (*self.build).fcsel(
            inst.reg_a64,
            temp1,
            inst.reg_a64,
            get_condition_f_p(IrCondition::Less),
          );
        }
        IrCmd::AddFloat => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::S,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_float(*get_op_mut(inst, 0));
          let temp2 = self.temp_float(*get_op_mut(inst, 1));
          (*self.build).fadd(inst.reg_a64, temp1, temp2);
        }
        IrCmd::SubFloat => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::S,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_float(*get_op_mut(inst, 0));
          let temp2 = self.temp_float(*get_op_mut(inst, 1));
          (*self.build).fsub(inst.reg_a64, temp1, temp2);
        }
        IrCmd::MulFloat => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::S,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_float(*get_op_mut(inst, 0));
          let temp2 = self.temp_float(*get_op_mut(inst, 1));
          (*self.build).fmul(inst.reg_a64, temp1, temp2);
        }
        IrCmd::DivFloat => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::S,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_float(*get_op_mut(inst, 0));
          let temp2 = self.temp_float(*get_op_mut(inst, 1));
          (*self.build).fdiv(inst.reg_a64, temp1, temp2);
        }
        IrCmd::MinFloat => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::S,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_float(*get_op_mut(inst, 0));
          let temp2 = self.temp_float(*get_op_mut(inst, 1));
          (*self.build).fcmp(temp1, temp2);
          (*self.build).fcsel(
            inst.reg_a64,
            temp1,
            temp2,
            get_condition_f_p(IrCondition::Less),
          );
        }
        IrCmd::MaxFloat => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::S,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_float(*get_op_mut(inst, 0));
          let temp2 = self.temp_float(*get_op_mut(inst, 1));
          (*self.build).fcmp(temp1, temp2);
          (*self.build).fcsel(
            inst.reg_a64,
            temp1,
            temp2,
            get_condition_f_p(IrCondition::Greater),
          );
        }
        IrCmd::UnmFloat => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::S, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_float(*get_op_mut(inst, 0));
          (*self.build).fneg(inst.reg_a64, temp);
        }
        IrCmd::FloorFloat => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::S, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_float(*get_op_mut(inst, 0));
          (*self.build).frintm(inst.reg_a64, temp);
        }
        IrCmd::CeilFloat => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::S, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_float(*get_op_mut(inst, 0));
          (*self.build).frintp(inst.reg_a64, temp);
        }
        IrCmd::SqrtFloat => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::S, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_float(*get_op_mut(inst, 0));
          (*self.build).fsqrt(inst.reg_a64, temp);
        }
        IrCmd::AbsFloat => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::S, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_float(*get_op_mut(inst, 0));
          (*self.build).fabs(inst.reg_a64, temp);
        }
        IrCmd::SignFloat => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::S, index, &[(*get_op_mut(inst, 0))]);

          let temp = self.temp_float(*get_op_mut(inst, 0));
          let temp0 = self.regs.alloc_temp(KindA64::S);
          let temp1 = self.regs.alloc_temp(KindA64::S);

          (*self.build).fcmpz(temp);
          (*self.build).fmov(temp0, 0.0);
          (*self.build).fmov(temp1, 1.0);
          (*self.build).fcsel(
            inst.reg_a64,
            temp1,
            temp0,
            get_condition_f_p(IrCondition::Greater),
          );
          (*self.build).fmov(temp1, -1.0);
          (*self.build).fcsel(
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
            &[
              (*get_op_mut(inst, 0)),
              (*get_op_mut(inst, 1)),
              (*get_op_mut(inst, 2)),
              (*get_op_mut(inst, 3)),
            ],
          );

          let temp1 = self.temp_double(*get_op_mut(inst, 0));
          let temp2 = self.temp_double(*get_op_mut(inst, 1));
          let temp3 = self.temp_double(*get_op_mut(inst, 2));
          let temp4 = self.temp_double(*get_op_mut(inst, 3));

          (*self.build).fcmp(temp3, temp4);
          (*self.build).fcsel(
            inst.reg_a64,
            temp2,
            temp1,
            get_condition_f_p(IrCondition::Equal),
          );
        }
        IrCmd::SelectInt64 => {
          let cond = condition_op(*get_op_mut(inst, 4));

          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[
              (*get_op_mut(inst, 0)),
              (*get_op_mut(inst, 1)),
              (*get_op_mut(inst, 2)),
              (*get_op_mut(inst, 3)),
            ],
          );

          let temp1 = self.temp_int64(*get_op_mut(inst, 0));
          let temp2 = self.temp_int64(*get_op_mut(inst, 1));
          let temp3 = self.temp_int64(*get_op_mut(inst, 2));
          let temp4 = self.temp_int64(*get_op_mut(inst, 3));

          (*self.build).cmp(temp3, temp4);
          (*self.build).csel(inst.reg_a64, temp2, temp1, get_condition_int64(cond));
        }
        IrCmd::SelectVec => {
          {
            // `(*get_op_mut(inst, 1))` cannot be reused for return value, because it can be overwritten with A before the first usage
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::Q,
              index,
              &[
                (*get_op_mut(inst, 0)),
                (*get_op_mut(inst, 2)),
                (*get_op_mut(inst, 3)),
              ],
            );

            let temp1 = self.reg_op(*get_op_mut(inst, 0));
            let temp2 = self.reg_op(*get_op_mut(inst, 1));
            let temp3 = self.reg_op(*get_op_mut(inst, 2));
            let temp4 = self.reg_op(*get_op_mut(inst, 3));

            let mask = self.regs.alloc_temp(KindA64::Q);

            // Evaluate predicate and calculate mask.
            (*self.build).fcmeq_4s(mask, temp3, temp4);
            // mov A to res register
            (*self.build).mov(inst.reg_a64, temp1);
            // If numbers are equal override A with B in res register.
            (*self.build).bit(inst.reg_a64, temp2, mask);
          }
        }
        IrCmd::SelectIfTruthy => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);

            // Place lhs as the result, we will overwrite it with rhs if 'A' is falsy later
            (*self.build).mov(inst.reg_a64, self.reg_op(*get_op_mut(inst, 1)));

            // Get rhs register early, so a potential restore happens on both sides of a conditional control flow
            let c = self.reg_op(*get_op_mut(inst, 2));

            let temp = self.regs.alloc_temp(KindA64::W);
            let mut save_rhs = Label::default();
            let mut exit = Label::default();

            // Check tag first
            (*self.build).umov_4s(temp, self.reg_op(*get_op_mut(inst, 0)), 3);
            (*self.build).cmp(temp, (LUA_TBOOLEAN) as u16);

            (*self.build).b_condition_a_64_label(ConditionA64::UNSIGNED_LESS, &mut save_rhs); // rhs if 'A' is nil
            (*self.build).b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut exit); // Keep lhs if 'A' is not a boolean

            // Check the boolean value
            (*self.build).umov_4s(temp, self.reg_op(*get_op_mut(inst, 0)), 0);
            (*self.build).cbnz(temp, &mut exit); // Keep lhs if 'A' is true

            (*self.build).set_label_label(&mut save_rhs);
            (*self.build).mov(inst.reg_a64, c);

            (*self.build).set_label_label(&mut exit);
          }
        }
        IrCmd::MuladdVec => {
          let temp_a = self.reg_op(*get_op_mut(inst, 0));
          let temp_b = self.reg_op(*get_op_mut(inst, 1));
          let temp_c = self.reg_op(*get_op_mut(inst, 2));

          if ((*self.build).features & FEATURE_ADV_SIMD) != 0 {
            inst.reg_a64 = self
              .regs
              .alloc_reuse(KindA64::Q, index, &[(*get_op_mut(inst, 2))]);
            if inst.reg_a64 != temp_c {
              (*self.build).mov(inst.reg_a64, temp_c);
            }
            (*self.build).fmla(inst.reg_a64, temp_b, temp_a);
          } else {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);
            (*self.build).fmul(inst.reg_a64, temp_b, temp_a);
            (*self.build).fadd(inst.reg_a64, inst.reg_a64, temp_c);
          }
        }
        IrCmd::AddVec => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::Q,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );

          (*self.build).fadd(
            inst.reg_a64,
            self.reg_op(*get_op_mut(inst, 0)),
            self.reg_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::SubVec => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::Q,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );

          (*self.build).fsub(
            inst.reg_a64,
            self.reg_op(*get_op_mut(inst, 0)),
            self.reg_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::MulVec => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::Q,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );

          (*self.build).fmul(
            inst.reg_a64,
            self.reg_op(*get_op_mut(inst, 0)),
            self.reg_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::DivVec => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::Q,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );

          (*self.build).fdiv(
            inst.reg_a64,
            self.reg_op(*get_op_mut(inst, 0)),
            self.reg_op(*get_op_mut(inst, 1)),
          );
        }
        IrCmd::IdivVec => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::Q,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );

          (*self.build).fdiv(
            inst.reg_a64,
            self.reg_op(*get_op_mut(inst, 0)),
            self.reg_op(*get_op_mut(inst, 1)),
          );
          (*self.build).frintm(inst.reg_a64, inst.reg_a64);
        }
        IrCmd::UnmVec => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::Q, index, &[(*get_op_mut(inst, 0))]);

          (*self.build).fneg(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)));
        }
        IrCmd::MinVec => {
          {
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::Q,
              index,
              &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
            );

            let temp1 = self.reg_op(*get_op_mut(inst, 0));
            let temp2 = self.reg_op(*get_op_mut(inst, 1));

            let mask = self.regs.alloc_temp(KindA64::Q);

            // b > a == a < b
            (*self.build).fcmgt_4s(mask, temp2, temp1);

            // If A is already at the target, select B where mask is 0
            if inst.reg_a64 == temp1 {
              (*self.build).bif(inst.reg_a64, temp2, mask);
            } else {
              // Store B at the target unless it's there, select A where mask is 1
              if inst.reg_a64 != temp2 {
                (*self.build).mov(inst.reg_a64, temp2);
              }

              (*self.build).bit(inst.reg_a64, temp1, mask);
            }
          }
        }
        IrCmd::MaxVec => {
          {
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::Q,
              index,
              &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
            );

            let temp1 = self.reg_op(*get_op_mut(inst, 0));
            let temp2 = self.reg_op(*get_op_mut(inst, 1));

            let mask = self.regs.alloc_temp(KindA64::Q);

            (*self.build).fcmgt_4s(mask, temp1, temp2);

            // If A is already at the target, select B where mask is 0
            if inst.reg_a64 == temp1 {
              (*self.build).bif(inst.reg_a64, temp2, mask);
            } else {
              // Store B at the target unless it's there, select A where mask is 1
              if inst.reg_a64 != temp2 {
                (*self.build).mov(inst.reg_a64, temp2);
              }

              (*self.build).bit(inst.reg_a64, temp1, mask);
            }
          }
        }
        IrCmd::FloorVec => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::Q, index, &[(*get_op_mut(inst, 0))]);

          (*self.build).frintm(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)));
        }
        IrCmd::CeilVec => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::Q, index, &[(*get_op_mut(inst, 0))]);

          (*self.build).frintp(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)));
        }
        IrCmd::AbsVec => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::Q, index, &[(*get_op_mut(inst, 0))]);
          (*self.build).fabs(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)));
        }
        IrCmd::DotVec => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);

            let temp = self.regs.alloc_temp(KindA64::Q);
            let temps = cast_reg(KindA64::S, temp);

            (*self.build).fmul(
              temp,
              self.reg_op(*get_op_mut(inst, 0)),
              self.reg_op(*get_op_mut(inst, 1)),
            );
            (*self.build).faddp(inst.reg_a64, temps); // x+y
            (*self.build).dup_4s(temp, temp, 2);
            (*self.build).fadd(inst.reg_a64, inst.reg_a64, temps); // +z
          }
        }
        IrCmd::ExtractVec => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);

            if self.int_op(*get_op_mut(inst, 1)) == 0 {
              // Lane vN.s[0] can just be read directly as sN
              (*self.build).fmov(
                inst.reg_a64,
                cast_reg(KindA64::S, self.reg_op(*get_op_mut(inst, 0))),
              );
            } else {
              (*self.build).dup_4s(
                inst.reg_a64,
                self.reg_op(*get_op_mut(inst, 0)),
                self.int_op(*get_op_mut(inst, 1)) as u8,
              );
            }
          }
        }
        IrCmd::NotAny => {
          {
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::W,
              index,
              &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
            );

            if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
              // other cases should've been constant folded
              CODEGEN_ASSERT!(self.tag_op(*get_op_mut(inst, 0)) == LUA_TBOOLEAN);
              (*self.build).eor(inst.reg_a64, self.reg_op(*get_op_mut(inst, 1)), 1);
            } else {
              let mut not_bool = Label::default();
              let mut exit = Label::default();

              // use the fact that NIL is the only value less than BOOLEAN to do two tag comparisons at once
              CODEGEN_ASSERT!(LUA_TNIL == 0 && LUA_TBOOLEAN == 1);
              (*self.build).cmp(self.reg_op(*get_op_mut(inst, 0)), (LUA_TBOOLEAN) as u16);
              (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut not_bool);

              if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                (*self.build).mov(
                  inst.reg_a64,
                  if self.int_op(*get_op_mut(inst, 1)) == 0 {
                    1
                  } else {
                    0
                  },
                );
              } else {
                (*self.build).eor(inst.reg_a64, self.reg_op(*get_op_mut(inst, 1)), 1);
              } // boolean => invert value

              (*self.build).b(&mut exit);

              // not boolean => result is true iff tag was nil
              (*self.build).set_label_label(&mut not_bool);
              (*self.build).cset(inst.reg_a64, ConditionA64::Less);

              (*self.build).set_label_label(&mut exit);
            }
          }
        }
        IrCmd::CmpInt => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );

          let cond = condition_op(*get_op_mut(inst, 2));

          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
            if ((self.int_op(*get_op_mut(inst, 0))) as u32) <= K_MAX_IMMEDIATE {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 1)),
                (self.int_op(*get_op_mut(inst, 0))) as u16,
              );
            } else {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 1)),
                self.temp_int(*get_op_mut(inst, 0)),
              );
            }

            (*self.build).cset(inst.reg_a64, get_inverse_condition(get_condition_int(cond)));
          } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
            if ((self.int_op(*get_op_mut(inst, 1))) as u32) <= K_MAX_IMMEDIATE {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 0)),
                (self.int_op(*get_op_mut(inst, 1))) as u16,
              );
            } else {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 0)),
                self.temp_int(*get_op_mut(inst, 1)),
              );
            }

            (*self.build).cset(inst.reg_a64, get_condition_int(cond));
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::CmpInt64 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);

          let cond = condition_op(*get_op_mut(inst, 2));

          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
            if ((self.int_64_op(*get_op_mut(inst, 0))) as u64) <= K_MAX_IMMEDIATE as u64 {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 1)),
                (self.int_64_op(*get_op_mut(inst, 0))) as u16,
              );
            } else {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 1)),
                self.temp_int64(*get_op_mut(inst, 0)),
              );
            }

            (*self.build).cset(
              inst.reg_a64,
              get_inverse_condition(get_condition_int64(cond)),
            );
          } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
              && ((self.int_64_op(*get_op_mut(inst, 1))) as u64) <= K_MAX_IMMEDIATE as u64
            {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 0)),
                (self.int_64_op(*get_op_mut(inst, 1))) as u16,
              );
            } else {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 0)),
                self.temp_int64(*get_op_mut(inst, 1)),
              );
            }

            (*self.build).cset(inst.reg_a64, get_condition_int64(cond));
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }
        }
        IrCmd::CmpAny => {
          {
            CODEGEN_ASSERT!(
              (*get_op_mut(inst, 0)).kind() == IrOpKind::VmReg
                && (*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg
            );
            let cond = condition_op(*get_op_mut(inst, 2));

            inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);

            let mut skip = Label::default();
            let mut exit = Label::default();

            // For equality comparison, 'luaV_equalval' expects tag to be equal before the call
            if cond == IrCondition::Equal {
              let tempa = self.regs.alloc_temp(KindA64::W);
              let tempb = self.regs.alloc_temp(KindA64::W);

              (*self.build).ldr(
                tempa,
                self.temp_addr(
                  *get_op_mut(inst, 0),
                  (core::mem::offset_of!(TValue, tt) as i32),
                ),
              );
              (*self.build).ldr(
                tempb,
                self.temp_addr(
                  *get_op_mut(inst, 1),
                  (core::mem::offset_of!(TValue, tt) as i32),
                ),
              );
              (*self.build).cmp(tempa, tempb);

              // If the tags are not equal, skip the call and set result to 0
              (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut skip);
            }

            // We have reserved the result register, so we can free it now so it is not recorded in the spill sequence
            self.regs.free_reg(inst.reg_a64);

            let spills = self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[]);

            (*self.build).mov(X0, R_STATE);
            (*self.build).add(
              X1,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
            );
            (*self.build).add(
              X2,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
            );

            if cond == IrCondition::LessEqual {
              (*self.build).ldr(
                X3,
                mem(
                  R_NATIVE_CONTEXT,
                  (core::mem::offset_of!(NativeContext, lua_v_lessequal) as i32),
                ),
              );
            } else if cond == IrCondition::Less {
              (*self.build).ldr(
                X3,
                mem(
                  R_NATIVE_CONTEXT,
                  (core::mem::offset_of!(NativeContext, lua_v_lessthan) as i32),
                ),
              );
            } else if cond == IrCondition::Equal {
              (*self.build).ldr(
                X3,
                mem(
                  R_NATIVE_CONTEXT,
                  (core::mem::offset_of!(NativeContext, lua_v_equalval) as i32),
                ),
              );
            } else {
              CODEGEN_ASSERT!(false, "Unsupported condition");
            }

            (*self.build).blr(X3);

            if inst.reg_a64 != W0 {
              (*self.build).mov(inst.reg_a64, W0);
            }

            inst.reg_a64 = self.regs.take_reg(inst.reg_a64, index);

            emit_update_base(&mut *self.build);

            self.regs.restore_usize(spills);

            if cond == IrCondition::Equal {
              (*self.build).b(&mut exit);
              (*self.build).set_label_label(&mut skip);

              (*self.build).mov(inst.reg_a64, 0);
              (*self.build).set_label_label(&mut exit);
            }

            // In case we made a call, skip high register bits clear, only consumer is JUMP_CMP_INT which doesn't read them
          }
        }
        IrCmd::CmpTag => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );

          let cond = condition_op(*get_op_mut(inst, 2));
          CODEGEN_ASSERT!(cond == IrCondition::Equal || cond == IrCondition::NotEqual);
          let mut a_reg = NOREG;
          let mut b_reg = NOREG;

          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
            a_reg = self.reg_op(*get_op_mut(inst, 0));
          } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::VmReg {
            a_reg = self.regs.alloc_temp(KindA64::W);
            let addr = self.temp_addr(
              *get_op_mut(inst, 0),
              (core::mem::offset_of!(TValue, tt) as i32),
            );
            (*self.build).ldr(a_reg, addr);
          } else {
            CODEGEN_ASSERT!((*get_op_mut(inst, 0)).kind() == IrOpKind::Constant);
          }

          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
            b_reg = self.reg_op(*get_op_mut(inst, 1));
          } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg {
            b_reg = self.regs.alloc_temp(KindA64::W);
            let addr = self.temp_addr(
              *get_op_mut(inst, 1),
              (core::mem::offset_of!(TValue, tt) as i32),
            );
            (*self.build).ldr(b_reg, addr);
          } else {
            CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);
          }

          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
            (*self.build).cmp(b_reg, (self.tag_op(*get_op_mut(inst, 0))) as u16);
            (*self.build).cset(inst.reg_a64, get_inverse_condition(get_condition_int(cond)));
          } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
            (*self.build).cmp(a_reg, (self.tag_op(*get_op_mut(inst, 1))) as u16);
            (*self.build).cset(inst.reg_a64, get_condition_int(cond));
          } else {
            (*self.build).cmp(a_reg, b_reg);
            (*self.build).cset(inst.reg_a64, get_condition_int(cond));
          }
        }
        IrCmd::CmpSplitTvalue => {
          {
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::W,
              index,
              &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
            );

            // Second operand of this instruction must be a constant
            // Without a constant type, we wouldn't know the correct way to compare the values at lowering time
            CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);

            let cond = condition_op(*get_op_mut(inst, 4));
            CODEGEN_ASSERT!(cond == IrCondition::Equal || cond == IrCondition::NotEqual);

            // Check tag equality first
            let temp = self.regs.alloc_temp(KindA64::W);

            if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 0)),
                (self.tag_op(*get_op_mut(inst, 1))) as u16,
              );
              (*self.build).cset(temp, get_condition_int(cond));
            } else {
              // Constant folding had to handle different constant tags
              CODEGEN_ASSERT!(
                self.tag_op(*get_op_mut(inst, 0)) == self.tag_op(*get_op_mut(inst, 1))
              );
            }

            if self.tag_op(*get_op_mut(inst, 1)) == LUA_TBOOLEAN {
              if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                CODEGEN_ASSERT!(
                  self.int_op(*get_op_mut(inst, 2)) == 0 || self.int_op(*get_op_mut(inst, 2)) == 1
                );
                (*self.build).cmp(
                  self.reg_op(*get_op_mut(inst, 3)),
                  (self.int_op(*get_op_mut(inst, 2))) as u16,
                ); // swapped arguments
              } else if (*get_op_mut(inst, 3)).kind() == IrOpKind::Constant {
                CODEGEN_ASSERT!(
                  self.int_op(*get_op_mut(inst, 3)) == 0 || self.int_op(*get_op_mut(inst, 3)) == 1
                );
                (*self.build).cmp(
                  self.reg_op(*get_op_mut(inst, 2)),
                  (self.int_op(*get_op_mut(inst, 3))) as u16,
                );
              } else {
                (*self.build).cmp(
                  self.reg_op(*get_op_mut(inst, 2)),
                  self.reg_op(*get_op_mut(inst, 3)),
                );
              }

              (*self.build).cset(inst.reg_a64, get_condition_int(cond));
            } else if self.tag_op(*get_op_mut(inst, 1)) == LUA_TSTRING {
              (*self.build).cmp(
                self.reg_op(*get_op_mut(inst, 2)),
                self.reg_op(*get_op_mut(inst, 3)),
              );
              (*self.build).cset(inst.reg_a64, get_condition_int(cond));
            } else if self.tag_op(*get_op_mut(inst, 1)) == LUA_TNUMBER {
              let temp1 = self.temp_double(*get_op_mut(inst, 2));
              let temp2 = self.temp_double(*get_op_mut(inst, 3));

              (*self.build).fcmp(temp1, temp2);
              (*self.build).cset(inst.reg_a64, get_condition_f_p(cond));
            } else if self.tag_op(*get_op_mut(inst, 1)) == LUA_TINTEGER {
              let temp1 = self.temp_int64(*get_op_mut(inst, 2));
              let temp2 = self.temp_int64(*get_op_mut(inst, 3));

              (*self.build).cmp(temp1, temp2);
              (*self.build).cset(inst.reg_a64, get_condition_int64(cond));
            } else {
              CODEGEN_ASSERT!(false, "unsupported type tag in CMP_SPLIT_TVALUE");
            }

            if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
              if cond == IrCondition::Equal {
                (*self.build).and_(inst.reg_a64, inst.reg_a64, temp);
              } else {
                (*self.build).orr(inst.reg_a64, inst.reg_a64, temp);
              }
            }
          }
        }
        IrCmd::JUMP => {
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Undef
            || (*get_op_mut(inst, 0)).kind() == IrOpKind::VmExit
          {
            let mut fresh = Label::default();
            let target =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 0), index, &mut fresh)
                as *mut Label;
            (*self.build).b(&mut *target);
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 0), index, &mut fresh);
          } else {
            self.jump_or_fallthrough_op(*get_op_mut(inst, 0), next);
          }
        }
        IrCmd::JumpIfTruthy => {
          {
            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).ldr(
              temp,
              mem(
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)
                  + (core::mem::offset_of!(TValue, tt) as i32),
              ),
            );
            // nil => falsy
            CODEGEN_ASSERT!(LUA_TNIL == 0);
            (*self.build).cbz(temp, self.label_op(*get_op_mut(inst, 2)));
            // not boolean => truthy
            (*self.build).cmp(temp, (LUA_TBOOLEAN) as u16);
            (*self.build)
              .b_condition_a_64_label(ConditionA64::NotEqual, self.label_op(*get_op_mut(inst, 1)));
            // compare boolean value
            (*self.build).ldr(
              temp,
              mem(
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)
                  + (core::mem::offset_of!(TValue, value) as i32),
              ),
            );
            (*self.build).cbnz(temp, self.label_op(*get_op_mut(inst, 1)));
            self.jump_or_fallthrough_op(*get_op_mut(inst, 2), next);
          }
        }
        IrCmd::JumpIfFalsy => {
          {
            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).ldr(
              temp,
              mem(
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)
                  + (core::mem::offset_of!(TValue, tt) as i32),
              ),
            );
            // nil => falsy
            CODEGEN_ASSERT!(LUA_TNIL == 0);
            (*self.build).cbz(temp, self.label_op(*get_op_mut(inst, 1)));
            // not boolean => truthy
            (*self.build).cmp(temp, (LUA_TBOOLEAN) as u16);
            (*self.build)
              .b_condition_a_64_label(ConditionA64::NotEqual, self.label_op(*get_op_mut(inst, 2)));
            // compare boolean value
            (*self.build).ldr(
              temp,
              mem(
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)
                  + (core::mem::offset_of!(TValue, value) as i32),
              ),
            );
            (*self.build).cbz(temp, self.label_op(*get_op_mut(inst, 1)));
            self.jump_or_fallthrough_op(*get_op_mut(inst, 2), next);
          }
        }
        IrCmd::JumpEqTag => {
          let mut zr = NOREG;
          let mut a_reg = NOREG;
          let mut b_reg = NOREG;

          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
            a_reg = self.reg_op(*get_op_mut(inst, 0));
          } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::VmReg {
            a_reg = self.regs.alloc_temp(KindA64::W);
            let addr = self.temp_addr(
              *get_op_mut(inst, 0),
              (core::mem::offset_of!(TValue, tt) as i32),
            );
            (*self.build).ldr(a_reg, addr);
          } else {
            CODEGEN_ASSERT!((*get_op_mut(inst, 0)).kind() == IrOpKind::Constant);
          }

          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
            b_reg = self.reg_op(*get_op_mut(inst, 1));
          } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg {
            b_reg = self.regs.alloc_temp(KindA64::W);
            let addr = self.temp_addr(
              *get_op_mut(inst, 1),
              (core::mem::offset_of!(TValue, tt) as i32),
            );
            (*self.build).ldr(b_reg, addr);
          } else {
            CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);
          }

          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant
            && self.tag_op(*get_op_mut(inst, 0)) == 0
          {
            zr = b_reg;
          } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && self.tag_op(*get_op_mut(inst, 1)) == 0
          {
            zr = a_reg;
          } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
            (*self.build).cmp(a_reg, (self.tag_op(*get_op_mut(inst, 1))) as u16);
          } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
            (*self.build).cmp(b_reg, (self.tag_op(*get_op_mut(inst, 0))) as u16);
          } else {
            (*self.build).cmp(a_reg, b_reg);
          }

          if self.is_fallthrough_block(&*self.block_op(*get_op_mut(inst, 3)), next) {
            if zr != NOREG {
              (*self.build).cbz(zr, self.label_op(*get_op_mut(inst, 2)));
            } else {
              (*self.build)
                .b_condition_a_64_label(ConditionA64::Equal, self.label_op(*get_op_mut(inst, 2)));
            }
            self.jump_or_fallthrough_op(*get_op_mut(inst, 3), next);
          } else {
            if zr != NOREG {
              (*self.build).cbnz(zr, self.label_op(*get_op_mut(inst, 3)));
            } else {
              (*self.build).b_condition_a_64_label(
                ConditionA64::NotEqual,
                self.label_op(*get_op_mut(inst, 3)),
              );
            }
            self.jump_or_fallthrough_op(*get_op_mut(inst, 2), next);
          }
        }
        IrCmd::JumpCmpInt => {
          let cond = condition_op(*get_op_mut(inst, 2));

          if cond == IrCondition::Equal && self.int_op(*get_op_mut(inst, 1)) == 0 {
            (*self.build).cbz(
              self.reg_op(*get_op_mut(inst, 0)),
              self.label_op(*get_op_mut(inst, 3)),
            );
          } else if cond == IrCondition::NotEqual && self.int_op(*get_op_mut(inst, 1)) == 0 {
            (*self.build).cbnz(
              self.reg_op(*get_op_mut(inst, 0)),
              self.label_op(*get_op_mut(inst, 3)),
            );
          } else {
            CODEGEN_ASSERT!(((self.int_op(*get_op_mut(inst, 1))) as u32) <= K_MAX_IMMEDIATE);
            (*self.build).cmp(
              self.reg_op(*get_op_mut(inst, 0)),
              (self.int_op(*get_op_mut(inst, 1))) as u16,
            );
            (*self.build)
              .b_condition_a_64_label(get_condition_int(cond), self.label_op(*get_op_mut(inst, 3)));
          }
          self.jump_or_fallthrough_op(*get_op_mut(inst, 4), next);
        }
        IrCmd::JumpEqPointer => {
          (*self.build).cmp(
            self.reg_op(*get_op_mut(inst, 0)),
            self.reg_op(*get_op_mut(inst, 1)),
          );
          (*self.build)
            .b_condition_a_64_label(ConditionA64::Equal, self.label_op(*get_op_mut(inst, 2)));
          self.jump_or_fallthrough_op(*get_op_mut(inst, 3), next);
        }
        IrCmd::JumpCmpNum => {
          let cond = condition_op(*get_op_mut(inst, 2));

          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && self.double_op(*get_op_mut(inst, 1)) == 0.0
          {
            let temp = self.temp_double(*get_op_mut(inst, 0));

            (*self.build).fcmpz(temp);
          } else {
            let temp1 = self.temp_double(*get_op_mut(inst, 0));
            let temp2 = self.temp_double(*get_op_mut(inst, 1));

            (*self.build).fcmp(temp1, temp2);
          }

          (*self.build)
            .b_condition_a_64_label(get_condition_f_p(cond), self.label_op(*get_op_mut(inst, 3)));
          self.jump_or_fallthrough_op(*get_op_mut(inst, 4), next);
        }
        IrCmd::JumpCmpFloat => {
          let cond = condition_op(*get_op_mut(inst, 2));

          if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && ((self.double_op(*get_op_mut(inst, 1))) as f32) == 0.0
          {
            let temp = self.temp_float(*get_op_mut(inst, 0));

            (*self.build).fcmpz(temp);
          } else {
            let temp1 = self.temp_float(*get_op_mut(inst, 0));
            let temp2 = self.temp_float(*get_op_mut(inst, 1));

            (*self.build).fcmp(temp1, temp2);
          }

          (*self.build)
            .b_condition_a_64_label(get_condition_f_p(cond), self.label_op(*get_op_mut(inst, 3)));
          self.jump_or_fallthrough_op(*get_op_mut(inst, 4), next);
        }
        IrCmd::JumpFornLoopCond => {
          {
            let index = self.temp_double(*get_op_mut(inst, 0));
            let limit = self.temp_double(*get_op_mut(inst, 1));
            let step = self.temp_double(*get_op_mut(inst, 2));

            let mut direct = Label::default();

            // step > 0
            (*self.build).fcmpz(step);
            (*self.build)
              .b_condition_a_64_label(get_condition_f_p(IrCondition::Greater), &mut direct);

            // !(limit <= index)
            (*self.build).fcmp(limit, index);
            (*self.build).b_condition_a_64_label(
              get_condition_f_p(IrCondition::NotLessEqual),
              self.label_op(*get_op_mut(inst, 4)),
            );
            (*self.build).b(self.label_op(*get_op_mut(inst, 3)));

            // !(index <= limit)
            (*self.build).set_label_label(&mut direct);

            (*self.build).fcmp(index, limit);
            (*self.build).b_condition_a_64_label(
              get_condition_f_p(IrCondition::NotLessEqual),
              self.label_op(*get_op_mut(inst, 4)),
            );
            self.jump_or_fallthrough_op(*get_op_mut(inst, 3), next);
          }
          // IrCmd::JUMP_SLOT_MATCH implemented below
        }
        IrCmd::TableLen => {
          {
            let reg = self.reg_op(*get_op_mut(inst, 0)); // note: we need to call regOp before spill so that we don't do redundant reloads
            self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[reg]);
            (*self.build).mov(X0, reg);
            (*self.build).ldr(
              X1,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_h_getn) as i32),
              ),
            );
            (*self.build).blr(X1);

            inst.reg_a64 = self.regs.take_reg(W0, index);

            (*self.build).ubfx(inst.reg_a64, inst.reg_a64, 0, 32); // Ensure high register bits are cleared
          }
        }
        IrCmd::StringLen => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);

          (*self.build).ldr(
            inst.reg_a64,
            mem(self.reg_op(*get_op_mut(inst, 0)), K_TSTRING_LEN_OFFSET),
          );
        }
        IrCmd::TableSetnum => {
          {
            // note: we need to call regOp before spill so that we don't do redundant reloads
            let table = self.reg_op(*get_op_mut(inst, 0));
            let key = self.reg_op(*get_op_mut(inst, 1));
            let temp = self.regs.alloc_temp(KindA64::W);

            self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[table, key]);

            if W1 != key {
              (*self.build).mov(X1, table);
              (*self.build).mov(W2, key);
            } else {
              (*self.build).mov(temp, W1);
              (*self.build).mov(X1, table);
              (*self.build).mov(W2, temp);
            }

            (*self.build).mov(X0, R_STATE);
            (*self.build).ldr(
              X3,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_h_setnum) as i32),
              ),
            );
            (*self.build).blr(X3);
            inst.reg_a64 = self.regs.take_reg(X0, index);
          }
        }
        IrCmd::NewTable => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).mov(X1, self.uint_op(*get_op_mut(inst, 0)));
          (*self.build).mov(X2, self.uint_op(*get_op_mut(inst, 1)));
          (*self.build).ldr(
            X3,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, lua_h_new) as i32),
            ),
          );
          (*self.build).blr(X3);
          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
        IrCmd::DupTable => {
          {
            let reg = self.reg_op(*get_op_mut(inst, 0)); // note: we need to call regOp before spill so that we don't do redundant reloads
            self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[reg]);
            (*self.build).mov(X1, reg);
            (*self.build).mov(X0, R_STATE);
            (*self.build).ldr(
              X2,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_h_clone) as i32),
              ),
            );
            (*self.build).blr(X2);
            inst.reg_a64 = self.regs.take_reg(X0, index);
          }
        }
        IrCmd::TryNumToIndex => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
            let temp1 = self.temp_double(*get_op_mut(inst, 0));

            if ((*self.build).features & FEATURE_JSCVT) != 0 {
              (*self.build).fjcvtzs(inst.reg_a64, temp1); // fjcvtzs sets PSTATE.Z (equal) iff conversion is exact
              (*self.build).b_condition_a_64_label(
                ConditionA64::NotEqual,
                self.label_op(*get_op_mut(inst, 1)),
              );
            } else {
              let temp2 = self.regs.alloc_temp(KindA64::D);

              (*self.build).fcvtzs(inst.reg_a64, temp1);
              (*self.build).scvtf(temp2, inst.reg_a64);
              (*self.build).fcmp(temp1, temp2);
              (*self.build).b_condition_a_64_label(
                ConditionA64::NotEqual,
                self.label_op(*get_op_mut(inst, 1)),
              );
            }
          }
        }
        IrCmd::TryCallFastgettm => {
          {
            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp2 = self.regs.alloc_temp(KindA64::W);

            (*self.build).ldr(
              temp1,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, metatable) as i32),
              ),
            );
            (*self.build).cbz(temp1, self.label_op(*get_op_mut(inst, 2))); // no metatable

            (*self.build).ldrb(
              temp2,
              mem(temp1, (core::mem::offset_of!(LuaTable, tmcache) as i32)),
            );
            (*self.build).tst(temp2, 1 << self.int_op(*get_op_mut(inst, 1))); // can't use tbz/tbnz because their jump offsets are too short
            (*self.build)
              .b_condition_a_64_label(ConditionA64::NotEqual, self.label_op(*get_op_mut(inst, 2))); // Equal = Zero after tst; tmcache caches *absence* of metamethods

            self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[temp1]);
            (*self.build).mov(X0, temp1);
            (*self.build).mov(W1, self.int_op(*get_op_mut(inst, 1)));
            (*self.build).ldr(
              X2,
              mem(
                R_GLOBAL_STATE,
                (core::mem::offset_of!(global_State, tmname) as i32)
                  + self.int_op(*get_op_mut(inst, 1)) * (size_of::<*mut tstring>() as i32),
              ),
            );
            (*self.build).ldr(
              X3,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_t_gettm) as i32),
              ),
            );
            (*self.build).blr(X3);

            (*self.build).cbz(X0, self.label_op(*get_op_mut(inst, 2))); // no tag method

            inst.reg_a64 = self.regs.take_reg(X0, index);
          }
        }
        IrCmd::NewUserdata => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).mov(X1, self.int_op(*get_op_mut(inst, 0)));
          (*self.build).mov(X2, self.int_op(*get_op_mut(inst, 1)));
          (*self.build).ldr(
            X3,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, new_userdata) as i32),
            ),
          );
          (*self.build).blr(X3);
          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
        IrCmd::Int64ToNum => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
          let temp = self.temp_int64(*get_op_mut(inst, 0));
          (*self.build).scvtf(inst.reg_a64, temp);
        }
        IrCmd::IntToNum => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
          let temp = self.temp_int(*get_op_mut(inst, 0));
          (*self.build).scvtf(inst.reg_a64, temp);
        }
        IrCmd::UintToNum => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
          let temp = self.temp_int(*get_op_mut(inst, 0));
          (*self.build).ucvtf(inst.reg_a64, temp);
        }
        IrCmd::UintToFloat => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);
          let temp = self.temp_int(*get_op_mut(inst, 0));
          (*self.build).ucvtf(inst.reg_a64, temp);
        }
        IrCmd::NumToInt => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).fcvtzs(inst.reg_a64, temp);
        }
        IrCmd::NumToInt64 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
          let temp = self.temp_double(*get_op_mut(inst, 0));
          (*self.build).fcvtzs(inst.reg_a64, temp);
        }
        IrCmd::NumToUint => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::W, index);
            let temp = self.temp_double(*get_op_mut(inst, 0));
            // note: we don't use fcvtzu for consistency with C++ code
            (*self.build).fcvtzs(cast_reg(KindA64::X, inst.reg_a64), temp);
          }
        }
        IrCmd::FloatToNum => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);

          (*self.build).fcvt(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)));
        }
        IrCmd::NumToFloat => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);

          (*self.build).fcvt(inst.reg_a64, self.reg_op(*get_op_mut(inst, 0)));
        }
        IrCmd::FloatToVec => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);

          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
            let value = (self.double_op(*get_op_mut(inst, 0))) as f32;
            let as_u32 = value.to_bits();

            if (*self.build).is_fmov_supported_fp_32(value) {
              (*self.build).fmov(inst.reg_a64, value);
            } else {
              let temp = self.regs.alloc_temp(KindA64::X);

              let vec = [as_u32, as_u32, as_u32, 0u32];
              (*self.build).adr_register_a_64_void_usize(
                temp,
                vec.as_ptr() as *const c_void,
                size_of_val(&vec),
              );
              (*self.build).ldr(inst.reg_a64, mem(temp, 0));
            }
          } else {
            let temp = self.temp_float(*get_op_mut(inst, 0));

            (*self.build).dup_4s(inst.reg_a64, cast_reg(KindA64::Q, temp), 0);
          }
        }
        IrCmd::TagVector => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::Q, index, &[(*get_op_mut(inst, 0))]);

          let reg = self.reg_op(*get_op_mut(inst, 0));
          let tempw = self.regs.alloc_temp(KindA64::W);

          if inst.reg_a64 != reg {
            (*self.build).mov(inst.reg_a64, reg);
          }

          (*self.build).mov(tempw, LUA_TVECTOR);
          (*self.build).ins_4_s_register_a_64_register_a_64_u8(inst.reg_a64, tempw, 3);
        }
        IrCmd::TruncateUint => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);

          (*self.build).ubfx(
            cast_reg(KindA64::X, inst.reg_a64),
            cast_reg(KindA64::X, self.reg_op(*get_op_mut(inst, 0))),
            0,
            32,
          ); // explicit uxtw
        }
        IrCmd::AdjustStackToReg => {
          {
            let temp = self.regs.alloc_temp(KindA64::X);

            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
              (*self.build).add(
                temp,
                r_base(),
                ((vm_reg_op(*get_op_mut(inst, 0)) + self.int_op(*get_op_mut(inst, 1)))
                  * (size_of::<TValue>() as i32)) as u16,
              );
              (*self.build).str(
                temp,
                mem(R_STATE, (core::mem::offset_of!(lua_State, top) as i32)),
              );
            } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
              (*self.build).add(
                temp,
                r_base(),
                (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
              );
              (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                temp,
                temp,
                self.reg_op(*get_op_mut(inst, 1)),
                K_TVALUE_SIZE_LOG2,
              ); // implicit uxtw
              (*self.build).str(
                temp,
                mem(R_STATE, (core::mem::offset_of!(lua_State, top) as i32)),
              );
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }
          }
        }
        IrCmd::AdjustStackToTop => {
          let temp = self.regs.alloc_temp(KindA64::X);
          (*self.build).ldr(
            temp,
            mem(R_STATE, (core::mem::offset_of!(lua_State, ci) as i32)),
          );
          (*self.build).ldr(
            temp,
            mem(temp, (core::mem::offset_of!(CallInfo, top) as i32)),
          );
          (*self.build).str(
            temp,
            mem(R_STATE, (core::mem::offset_of!(lua_State, top) as i32)),
          );
        }
        IrCmd::FASTCALL => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);

          let bfid = self.uint_op(*get_op_mut(inst, 0)) as i32;
          let res = vm_reg_op(*get_op_mut(inst, 1));
          let arg = vm_reg_op(*get_op_mut(inst, 2));
          let nresults = self.int_op(*get_op_mut(inst, 3));
          self.error |= !emit_builtin(
            &mut *self.build,
            &mut *self.function,
            &mut self.regs,
            bfid,
            res,
            arg,
            nresults,
          );
        }
        IrCmd::InvokeFastcall => {
          {
            // We might need a temporary and we have to preserve it over the spill
            let temp = self.regs.alloc_temp(KindA64::Q);
            self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[temp]);

            (*self.build).mov(X0, R_STATE);
            (*self.build).add(
              X1,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
            );
            (*self.build).add(
              X2,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 2)) * (size_of::<TValue>() as i32)) as u16,
            );
            (*self.build).mov(W3, self.int_op(*get_op_mut(inst, 6))); // nresults

            // 'E' argument can only be produced by LOP_FASTCALL3 lowering
            if (*get_op_mut(inst, 4)).kind() != IrOpKind::Undef {
              CODEGEN_ASSERT!(self.int_op(*get_op_mut(inst, 5)) == 3);

              (*self.build).ldr(
                X4,
                mem(R_STATE, (core::mem::offset_of!(lua_State, top) as i32)),
              );

              (*self.build).ldr(
                temp,
                mem(
                  r_base(),
                  vm_reg_op(*get_op_mut(inst, 3)) * (size_of::<TValue>() as i32),
                ),
              );
              (*self.build).str(temp, mem(X4, 0));

              (*self.build).ldr(
                temp,
                mem(
                  r_base(),
                  vm_reg_op(*get_op_mut(inst, 4)) * (size_of::<TValue>() as i32),
                ),
              );
              (*self.build).str(temp, mem(X4, size_of::<TValue>() as i32));
            } else {
              if (*get_op_mut(inst, 3)).kind() == IrOpKind::VmReg {
                (*self.build).add(
                  X4,
                  r_base(),
                  (vm_reg_op(*get_op_mut(inst, 3)) * (size_of::<TValue>() as i32)) as u16,
                );
              } else if (*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst {
                emit_add_offset(
                  &mut *self.build,
                  X4,
                  r_constants(),
                  vm_const_op(*get_op_mut(inst, 3)) * (size_of::<TValue>() as i32),
                );
              } else {
                CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::Undef);
              }
            }

            // nparams
            if self.int_op(*get_op_mut(inst, 5)) == LUA_MULTRET {
              // l->top - (ra + 1)
              (*self.build).ldr(
                X5,
                mem(R_STATE, (core::mem::offset_of!(lua_State, top) as i32)),
              );
              (*self.build).sub(X5, X5, r_base());
              (*self.build).sub(
                X5,
                X5,
                ((vm_reg_op(*get_op_mut(inst, 1)) + 1) * (size_of::<TValue>() as i32)) as u16,
              );
              (*self.build).lsr(X5, X5, K_TVALUE_SIZE_LOG2);
            } else {
              (*self.build).mov(W5, self.int_op(*get_op_mut(inst, 5)));
            }

            (*self.build).ldr(
              X6,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, luau_f_table) as i32)
                  + (self.uint_op(*get_op_mut(inst, 0)) as i32)
                    * (size_of::<luau_FastFunction>() as i32),
              ),
            );
            (*self.build).blr(X6);

            inst.reg_a64 = self.regs.take_reg(W0, index);
            // Skipping high register bits clear, only consumer is CHECK_FASTCALL_RES which doesn't read them
          }
        }
        IrCmd::CheckFastcallRes => {
          (*self.build).cmp(self.reg_op(*get_op_mut(inst, 0)), 0_u16);
          (*self.build)
            .b_condition_a_64_label(ConditionA64::Less, self.label_op(*get_op_mut(inst, 1)));
        }
        IrCmd::DoArith => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );

          if (*get_op_mut(inst, 1)).kind() == IrOpKind::VmConst {
            emit_add_offset(
              &mut *self.build,
              X2,
              r_constants(),
              vm_const_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32),
            );
          } else {
            (*self.build).add(
              X2,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
            );
          }

          if (*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst {
            emit_add_offset(
              &mut *self.build,
              X3,
              r_constants(),
              vm_const_op(*get_op_mut(inst, 2)) * (size_of::<TValue>() as i32),
            );
          } else {
            (*self.build).add(
              X3,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 2)) * (size_of::<TValue>() as i32)) as u16,
            );
          }

          match self.int_op(*get_op_mut(inst, 3)) as u32 {
            x if x == TMS::TmAdd as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithadd) as i32),
              ),
            ),
            x if x == TMS::TmSub as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithsub) as i32),
              ),
            ),
            x if x == TMS::TmMul as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithmul) as i32),
              ),
            ),
            x if x == TMS::TmDiv as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithdiv) as i32),
              ),
            ),
            x if x == TMS::TmIDiv as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithidiv) as i32),
              ),
            ),
            x if x == TMS::TmMod as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithmod) as i32),
              ),
            ),
            x if x == TMS::TmPow as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithpow) as i32),
              ),
            ),
            x if x == TMS::TmUnm as u32 => (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_v_doarithunm) as i32),
              ),
            ),
            _ => CODEGEN_ASSERT!(false, "Invalid doarith helper operation tag"),
          }

          (*self.build).blr(X4);

          emit_update_base(&mut *self.build);
        }
        IrCmd::DoLen => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).add(
            X2,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).ldr(
            X3,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, lua_v_dolen) as i32),
            ),
          );
          (*self.build).blr(X3);

          emit_update_base(&mut *self.build);
        }
        IrCmd::GetTable => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
          );

          if (*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg {
            (*self.build).add(
              X2,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 2)) * (size_of::<TValue>() as i32)) as u16,
            );
          } else if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
            let mut n = TValue::default();
            setnvalue!(
              &mut n as *mut TValue,
              self.uint_op(*get_op_mut(inst, 2)) as f64
            );
            (*self.build).adr_register_a_64_void_usize(
              X2,
              &n as *const TValue as *const c_void,
              size_of::<TValue>(),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }

          (*self.build).add(
            X3,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).ldr(
            X4,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, lua_v_gettable) as i32),
            ),
          );
          (*self.build).blr(X4);

          emit_update_base(&mut *self.build);
        }
        IrCmd::SetTable => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
          );

          if (*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg {
            (*self.build).add(
              X2,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 2)) * (size_of::<TValue>() as i32)) as u16,
            );
          } else if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
            let mut n = TValue::default();
            setnvalue!(
              &mut n as *mut TValue,
              self.uint_op(*get_op_mut(inst, 2)) as f64
            );
            (*self.build).adr_register_a_64_void_usize(
              X2,
              &n as *const TValue as *const c_void,
              size_of::<TValue>(),
            );
          } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
          }

          (*self.build).add(
            X3,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).ldr(
            X4,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, lua_v_settable) as i32),
            ),
          );
          (*self.build).blr(X4);

          emit_update_base(&mut *self.build);
        }
        IrCmd::GetCachedImport => {
          {
            self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[]);

            let mut skip = Label::default();
            let mut exit = Label::default();

            let temp_tag = self.regs.alloc_temp(KindA64::W);

            let addr_const_tag = self.temp_addr(
              *get_op_mut(inst, 1),
              (core::mem::offset_of!(TValue, tt) as i32),
            );
            (*self.build).ldr(temp_tag, addr_const_tag);

            // If the constant for the import is set, we will use it directly, otherwise we have to call an import path lookup function
            CODEGEN_ASSERT!(LUA_TNIL == 0);
            (*self.build).cbnz(temp_tag, &mut skip);

            {
              (*self.build).mov(X0, R_STATE);
              (*self.build).add(
                X1,
                r_base(),
                (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
              );
              (*self.build).mov(W2, self.import_op(*get_op_mut(inst, 2)));
              (*self.build).mov(W3, self.uint_op(*get_op_mut(inst, 3)));
              (*self.build).ldr(
                X4,
                mem(
                  R_NATIVE_CONTEXT,
                  (core::mem::offset_of!(NativeContext, get_import) as i32),
                ),
              );
              (*self.build).blr(X4);

              emit_update_base(&mut *self.build);
              (*self.build).b(&mut exit);
            }

            (*self.build).set_label_label(&mut skip);

            let temp_tv = self.regs.alloc_temp(KindA64::Q);

            let addr_const = self.temp_addr(*get_op_mut(inst, 1), 0);
            (*self.build).ldr(temp_tv, addr_const);

            let addr_reg = self.temp_addr(*get_op_mut(inst, 0), 0);
            (*self.build).str(temp_tv, addr_reg);

            (*self.build).set_label_label(&mut exit);
          }
        }
        IrCmd::CONCAT => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).mov(W1, self.uint_op(*get_op_mut(inst, 1)));
          (*self.build).mov(
            W2,
            vm_reg_op(*get_op_mut(inst, 0)) + self.uint_op(*get_op_mut(inst, 1)) as i32 - 1,
          );
          (*self.build).ldr(
            X3,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, lua_v_concat) as i32),
            ),
          );
          (*self.build).blr(X3);

          emit_update_base(&mut *self.build);
        }
        IrCmd::GetUpvalue => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::Q, index);

            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp2 = self.regs.alloc_temp(KindA64::W);

            (*self.build).add(
              temp1,
              R_CLOSURE,
              (K_CLOSURE_L_UPREFS_OFFSET
                + (size_of::<TValue>() as i32) * vm_upvalue_op(*get_op_mut(inst, 0)))
                as u16,
            );

            // uprefs[] is either an actual value, or it points to UpVal object which has a pointer to value
            let mut skip = Label::default();
            (*self.build).ldr(
              temp2,
              mem(temp1, (core::mem::offset_of!(TValue, tt) as i32)),
            );
            (*self.build).cmp(temp2, (LUA_TUPVAL) as u16);
            (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut skip);

            // UpVal.v points to the value (either on stack, or on heap inside each UpVal, but we can deref it unconditionally)
            (*self.build).ldr(temp1, mem(temp1, K_TVALUE_VALUE_GC_OFFSET));
            (*self.build).ldr(temp1, mem(temp1, (core::mem::offset_of!(UpVal, v) as i32)));

            (*self.build).set_label_label(&mut skip);

            (*self.build).ldr(inst.reg_a64, mem(temp1, 0));
          }
        }
        IrCmd::SetUpvalue => {
          {
            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp2 = self.regs.alloc_temp(KindA64::X);

            // UpVal*
            (*self.build).ldr(
              temp1,
              mem(
                R_CLOSURE,
                K_CLOSURE_L_UPREFS_OFFSET
                  + (size_of::<TValue>() as i32) * vm_upvalue_op(*get_op_mut(inst, 0))
                  + K_TVALUE_VALUE_GC_OFFSET,
              ),
            );

            (*self.build).ldr(temp2, mem(temp1, (core::mem::offset_of!(UpVal, v) as i32)));
            (*self.build).str(self.reg_op(*get_op_mut(inst, 1)), mem(temp2, 0));

            if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef
              || is_gco(self.tag_op(*get_op_mut(inst, 2)))
            {
              let value = self.reg_op(*get_op_mut(inst, 1));

              let mut skip = Label::default();
              self.check_object_barrier_conditions(
                temp1,
                temp2,
                value,
                *get_op_mut(inst, 1),
                if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
                  -1
                } else {
                  self.tag_op(*get_op_mut(inst, 2)) as i32
                },
                &mut skip,
              );

              let spills = self
                .regs
                .spill_u32_initializer_list_register_a_64(index, &[temp1, value]);

              (*self.build).mov(X1, temp1);
              (*self.build).mov(X0, R_STATE);
              (*self.build).fmov(X2, cast_reg(KindA64::D, value));
              (*self.build).ldr(
                X3,
                mem(
                  R_NATIVE_CONTEXT,
                  (core::mem::offset_of!(NativeContext, lua_c_barrierf) as i32),
                ),
              );
              (*self.build).blr(X3);

              self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

              // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
              (*self.build).set_label_label(&mut skip);
            }
          }
        }
        IrCmd::CheckTag => {
          {
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let fail =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 2), index, &mut fresh)
                as *mut Label;

            if self.tag_op(*get_op_mut(inst, 1)) == 0 {
              let reg = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cbnz(reg, &mut *fail);
            } else {
              let reg = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cmp(reg, (self.tag_op(*get_op_mut(inst, 1))) as u16);
              (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut *fail);
            }

            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 2), index, &mut fresh);
          }
        }
        IrCmd::CheckTruthy => {
          {
            // Constant tags which don't require boolean value check should've been removed in constant folding
            CODEGEN_ASSERT!(
              (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant
                || self.tag_op(*get_op_mut(inst, 0)) == LUA_TBOOLEAN
            );

            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let target =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 2), index, &mut fresh)
                as *mut Label;

            let mut skip = Label::default();

            if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
              // fail to fallback on 'nil' (falsy)
              CODEGEN_ASSERT!(LUA_TNIL == 0);
              let tag = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cbz(tag, &mut *target);

              // skip value test if it's not a boolean (truthy)
              let tag = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cmp(tag, (LUA_TBOOLEAN) as u16);
              (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut skip);
            }

            // fail to fallback on 'false' boolean value (falsy)
            if (*get_op_mut(inst, 1)).kind() != IrOpKind::Constant {
              let value = self.reg_op(*get_op_mut(inst, 1));
              (*self.build).cbz(value, &mut *target);
            } else {
              if self.int_op(*get_op_mut(inst, 1)) == 0 {
                (*self.build).b(&mut *target);
              }
            }

            if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
              (*self.build).set_label_label(&mut skip);
            }

            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 2), index, &mut fresh);
          }
        }
        IrCmd::CheckReadonly => {
          {
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).ldrb(
              temp,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, readonly) as i32),
              ),
            );
            let target =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 1), index, &mut fresh)
                as *mut Label;
            (*self.build).cbnz(temp, &mut *target);
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 1), index, &mut fresh);
          }
        }
        IrCmd::CheckNoMetatable => {
          {
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let temp = self.regs.alloc_temp(KindA64::X);
            (*self.build).ldr(
              temp,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, metatable) as i32),
              ),
            );
            let target =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 1), index, &mut fresh)
                as *mut Label;
            (*self.build).cbnz(temp, &mut *target);
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 1), index, &mut fresh);
          }
        }
        IrCmd::CheckSafeEnv => {
          self.check_safe_env(*get_op_mut(inst, 0), index, next);
        }
        IrCmd::CheckArraySize => {
          {
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let fail =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 2), index, &mut fresh)
                as *mut Label;

            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).ldr(
              temp,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaTable, sizearray) as i32),
              ),
            );

            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
              (*self.build).cmp(temp, self.reg_op(*get_op_mut(inst, 1)));
              (*self.build).b_condition_a_64_label(ConditionA64::UnsignedLessEqual, &mut *fail);
            } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
              if self.int_op(*get_op_mut(inst, 1)) == 0 {
                (*self.build).cbz(temp, &mut *fail);
              } else if ((self.int_op(*get_op_mut(inst, 1))) as usize) <= K_MAX_IMMEDIATE as usize {
                (*self.build).cmp(temp, (self.int_op(*get_op_mut(inst, 1))) as u16);
                (*self.build).b_condition_a_64_label(ConditionA64::UnsignedLessEqual, &mut *fail);
              } else {
                let temp2 = self.regs.alloc_temp(KindA64::W);
                (*self.build).mov(temp2, self.int_op(*get_op_mut(inst, 1)));
                (*self.build).cmp(temp, temp2);
                (*self.build).b_condition_a_64_label(ConditionA64::UnsignedLessEqual, &mut *fail);
              }
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }

            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 2), index, &mut fresh);
          }
        }
        IrCmd::JumpSlotMatch | IrCmd::CheckSlotMatch => {
          {
            let mut abort = Label::default(); // used when guard aborts execution
            let mismatch_op = if inst.cmd == IrCmd::JumpSlotMatch {
              *get_op_mut(inst, 3)
            } else {
              *get_op_mut(inst, 2)
            };
            let mismatch = if mismatch_op.kind() == IrOpKind::Undef {
              &mut abort as *mut Label
            } else {
              self.label_op(mismatch_op) as *mut Label
            };

            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp1w = cast_reg(KindA64::W, temp1);
            let temp2 = self.regs.alloc_temp(KindA64::X);

            CODEGEN_ASSERT!(K_OFFSET_OF_TKEY_TAG_NEXT >= 8 && K_OFFSET_OF_TKEY_TAG_NEXT < 16);
            (*self.build).ldp(
              temp1,
              temp2,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaNode, key) as i32),
              ),
            ); // load key.value into temp1 and key.tt (alongside other bits) into temp2
            (*self.build).ubfx(
              temp2,
              temp2,
              ((K_OFFSET_OF_TKEY_TAG_NEXT - 8) * 8) as u8,
              K_TKEY_TAG_BITS as u8,
            ); // .tt is right before .next, and 8 bytes are skipped by ldp
            (*self.build).cmp(temp2, (LUA_TSTRING) as u16);
            (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut *mismatch);

            let addr = self.temp_addr(
              *get_op_mut(inst, 1),
              (core::mem::offset_of!(TValue, value) as i32),
            );
            (*self.build).ldr(temp2, addr);
            (*self.build).cmp(temp1, temp2);
            (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut *mismatch);

            (*self.build).ldr(
              temp1w,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaNode, val) + core::mem::offset_of!(TValue, tt)) as i32,
              ),
            );
            CODEGEN_ASSERT!(LUA_TNIL == 0);
            (*self.build).cbz(temp1w, &mut *mismatch);

            if inst.cmd == IrCmd::JumpSlotMatch {
              self.jump_or_fallthrough_op(*get_op_mut(inst, 2), next);
            } else if abort.id != 0 {
              emit_abort(&mut *self.build, &mut abort);
            }
          }
        }
        IrCmd::CheckNodeNoNext => {
          {
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let temp = self.regs.alloc_temp(KindA64::W);

            (*self.build).ldr(
              temp,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaNode, key) as i32) + K_OFFSET_OF_TKEY_TAG_NEXT,
              ),
            );
            (*self.build).lsr(temp, temp, K_TKEY_TAG_BITS);
            let target =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 1), index, &mut fresh)
                as *mut Label;
            (*self.build).cbnz(temp, &mut *target);
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 1), index, &mut fresh);
          }
        }
        IrCmd::CheckNodeValue => {
          {
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let temp = self.regs.alloc_temp(KindA64::W);

            (*self.build).ldr(
              temp,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(LuaNode, val) + core::mem::offset_of!(TValue, tt)) as i32,
              ),
            );
            CODEGEN_ASSERT!(LUA_TNIL == 0);
            let target =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 1), index, &mut fresh)
                as *mut Label;
            (*self.build).cbz(temp, &mut *target);
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 1), index, &mut fresh);
          }
        }
        IrCmd::CheckBufferLen => {
          {
            let min_offset = self.int_op(*get_op_mut(inst, 2));
            let max_offset = self.int_op(*get_op_mut(inst, 3));
            CODEGEN_ASSERT!(min_offset < max_offset);
            CODEGEN_ASSERT!(
              min_offset >= -((K_MAX_IMMEDIATE) as i32) && min_offset <= ((K_MAX_IMMEDIATE) as i32)
            );

            let access_size = max_offset - min_offset;
            CODEGEN_ASSERT!(access_size > 0 && access_size <= ((K_MAX_IMMEDIATE) as i32));

            // For jumps to exit sync blocks to work, we need the same register allocation state at each potential taken branch
            let reg_a = if FFlag::LuauCodegenVmExitSync.get()
              && (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            {
              self.reg_op(*get_op_mut(inst, 0))
            } else {
              NOREG
            };
            let reg_b = if FFlag::LuauCodegenVmExitSync.get()
              && (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst
            {
              self.reg_op(*get_op_mut(inst, 1))
            } else {
              NOREG
            };
            let reg_e = if FFlag::LuauCodegenVmExitSync.get()
              && (*get_op_mut(inst, 4)).kind() != IrOpKind::Undef
            {
              self.reg_op(*get_op_mut(inst, 4))
            } else {
              NOREG
            };
            let temp_w1 = if FFlag::LuauCodegenVmExitSync.get() {
              self.regs.alloc_temp(KindA64::W)
            } else {
              NOREG
            };
            let temp_w2 = if FFlag::LuauCodegenVmExitSync.get() {
              self.regs.alloc_temp(KindA64::W)
            } else {
              NOREG
            };
            let temp_d = if FFlag::LuauCodegenVmExitSync.get() {
              self.regs.alloc_temp(KindA64::D)
            } else {
              NOREG
            };

            // Validate that we don't allocate anything else in this multi-branch instruction lowering
            if FFlag::LuauCodegenVmExitSync.get() {
              self.exit_sync_inst_idx = index;
              self.exit_sync_alloc_token = self.regs.get_alloc_token();
            }

            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let target =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 5), index, &mut fresh)
                as *mut Label;

            // Check if we are acting not only as a guard for the size, but as a guard that offset represents an exact integer
            if (*get_op_mut(inst, 4)).kind() != IrOpKind::Undef {
              CODEGEN_ASSERT!(
                get_cmd_value_kind((*self.function).inst_op(*get_op_mut(inst, 1)).cmd)
                  == IrValueKind::Int
              );
              CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                (*self.function).inst_op(*get_op_mut(inst, 1)).cmd
              )); // Ensure that high register bits are cleared

              if ((*self.build).features & FEATURE_JSCVT) != 0 {
                let temp = if FFlag::LuauCodegenVmExitSync.get() {
                  temp_w1
                } else {
                  self.regs.alloc_temp(KindA64::W)
                };

                (*self.build).fjcvtzs(
                  temp,
                  if FFlag::LuauCodegenVmExitSync.get() {
                    reg_e
                  } else {
                    self.reg_op(*get_op_mut(inst, 4))
                  },
                ); // fjcvtzs sets PSTATE.Z (equal) iff conversion is exact
                (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut *target);
              } else {
                let temp = if FFlag::LuauCodegenVmExitSync.get() {
                  temp_d
                } else {
                  self.regs.alloc_temp(KindA64::D)
                };

                (*self.build).scvtf(
                  temp,
                  if FFlag::LuauCodegenVmExitSync.get() {
                    reg_b
                  } else {
                    self.reg_op(*get_op_mut(inst, 1))
                  },
                );
                (*self.build).fcmp(
                  if FFlag::LuauCodegenVmExitSync.get() {
                    reg_e
                  } else {
                    self.reg_op(*get_op_mut(inst, 4))
                  },
                  temp,
                );
                (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut *target);
              }
            }

            let temp = if FFlag::LuauCodegenVmExitSync.get() {
              temp_w1
            } else {
              self.regs.alloc_temp(KindA64::W)
            };
            (*self.build).ldr(
              temp,
              mem(
                if FFlag::LuauCodegenVmExitSync.get() {
                  reg_a
                } else {
                  self.reg_op(*get_op_mut(inst, 0))
                },
                K_BUFFER_LEN_OFFSET,
              ),
            );

            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
              CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                (*self.function).inst_op(*get_op_mut(inst, 1)).cmd
              )); // Ensure that high register bits are cleared

              if access_size == 1 && min_offset == 0 {
                // fails if offset >= len
                (*self.build).cmp(
                  temp,
                  if FFlag::LuauCodegenVmExitSync.get() {
                    reg_b
                  } else {
                    self.reg_op(*get_op_mut(inst, 1))
                  },
                );
                (*self.build).b_condition_a_64_label(ConditionA64::UnsignedLessEqual, &mut *target);
              } else if min_offset >= 0 && max_offset <= ((K_MAX_IMMEDIATE) as i32) {
                // fails if offset + size > len; we compute it as len - offset < size
                let tempx = cast_reg(KindA64::X, temp);
                (*self.build).sub(
                  tempx,
                  tempx,
                  if FFlag::LuauCodegenVmExitSync.get() {
                    reg_b
                  } else {
                    self.reg_op(*get_op_mut(inst, 1))
                  },
                ); // implicit uxtw
                (*self.build).cmp(tempx, (max_offset) as u16);
                (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut *target);
              // note: this is a signed 64-bit comparison so that out of bounds offset fails
              } else {
                let tempx = cast_reg(KindA64::X, temp);
                let temp2 = if FFlag::LuauCodegenVmExitSync.get() {
                  cast_reg(KindA64::X, temp_w2)
                } else {
                  self.regs.alloc_temp(KindA64::X)
                };

                // Get the base offset in 32 bits
                if min_offset >= 0 {
                  (*self.build).add(
                    cast_reg(KindA64::W, temp2),
                    if FFlag::LuauCodegenVmExitSync.get() {
                      reg_b
                    } else {
                      self.reg_op(*get_op_mut(inst, 1))
                    },
                    (min_offset) as u16,
                  );
                } else {
                  (*self.build).sub(
                    cast_reg(KindA64::W, temp2),
                    if FFlag::LuauCodegenVmExitSync.get() {
                      reg_b
                    } else {
                      self.reg_op(*get_op_mut(inst, 1))
                    },
                    (-min_offset) as u16,
                  );
                }

                // fail if (((offset + min_offset) as u32)) as u64 { + access_size > length
                (*self.build).add(temp2, temp2, (access_size) as u16);
                (*self.build).cmp(temp2, tempx);
                (*self.build).b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut *target);
              }
            } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
              let offset = self.int_op(*get_op_mut(inst, 1));
              let end_offset = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                max_offset
              } else {
                access_size
              };
              let fail_cond = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                ConditionA64::UNSIGNED_LESS
              } else {
                ConditionA64::UnsignedLessEqual
              };

              // Constant folding can take care of it, but for safety we avoid overflow/underflow cases here
              if offset < 0 || ((offset) as u32) + ((end_offset) as u32) >= ((INT_MAX) as u32) {
                (*self.build).b(&mut *target);
              } else if offset + end_offset <= ((K_MAX_IMMEDIATE) as i32) {
                (*self.build).cmp(temp, (offset + end_offset) as u16);
                (*self.build).b_condition_a_64_label(fail_cond, &mut *target);
              } else {
                let temp2 = if FFlag::LuauCodegenVmExitSync.get() {
                  temp_w2
                } else {
                  self.regs.alloc_temp(KindA64::W)
                };
                (*self.build).mov(temp2, offset + end_offset);
                (*self.build).cmp(temp, temp2);
                (*self.build).b_condition_a_64_label(fail_cond, &mut *target);
              }
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 5), index, &mut fresh);
          }
        }
        IrCmd::CheckUserdataTag => {
          {
            CODEGEN_ASSERT!(((self.int_op(*get_op_mut(inst, 1))) as u32) <= K_MAX_IMMEDIATE);

            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let fail =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 2), index, &mut fresh)
                as *mut Label;
            let temp = self.regs.alloc_temp(KindA64::W);
            (*self.build).ldrb(
              temp,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(Udata, tag) as i32),
              ),
            );
            (*self.build).cmp(temp, (self.int_op(*get_op_mut(inst, 1))) as u16);
            (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut *fail);
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 2), index, &mut fresh);
          }
        }
        IrCmd::CheckCmpNum => {
          {
            let cond = condition_op(*get_op_mut(inst, 2));
            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let fail =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 3), index, &mut fresh)
                as *mut Label;

            let temp_a = self.temp_double(*get_op_mut(inst, 0));

            (*self.build).fcmp(temp_a, self.temp_double(*get_op_mut(inst, 1)));
            (*self.build)
              .b_condition_a_64_label(get_condition_f_p(get_negated_condition(cond)), &mut *fail);

            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 3), index, &mut fresh);
          }
        }
        IrCmd::CheckCmpInt => {
          {
            let cond = condition_op(*get_op_mut(inst, 2));

            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let fail =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 3), index, &mut fresh)
                as *mut Label;

            if cond == IrCondition::Equal && self.int_op(*get_op_mut(inst, 1)) == 0 {
              let reg = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cbnz(reg, &mut *fail);
            } else if cond == IrCondition::NotEqual && self.int_op(*get_op_mut(inst, 1)) == 0 {
              let reg = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cbz(reg, &mut *fail);
            } else {
              let temp_a = self.temp_int(*get_op_mut(inst, 0));

              if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                && ((self.int_op(*get_op_mut(inst, 1))) as u32) <= K_MAX_IMMEDIATE
              {
                (*self.build).cmp(temp_a, (self.int_op(*get_op_mut(inst, 1))) as u16);
              } else {
                (*self.build).cmp(temp_a, self.temp_int(*get_op_mut(inst, 1)));
              }

              (*self.build)
                .b_condition_a_64_label(get_condition_int(get_negated_condition(cond)), &mut *fail);
            }
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 3), index, &mut fresh);
          }
        }
        IrCmd::CheckCmpInt64 => {
          {
            let cond = condition_op(*get_op_mut(inst, 2));

            let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
            let fail =
              self.ir_lowering_a_64_get_target_label(*get_op_mut(inst, 3), index, &mut fresh)
                as *mut Label;

            if cond == IrCondition::Equal
              && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
              && self.int_64_op(*get_op_mut(inst, 1)) == 0
            {
              let reg = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cbnz(reg, &mut *fail);
            } else if cond == IrCondition::NotEqual
              && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
              && self.int_64_op(*get_op_mut(inst, 1)) == 0
            {
              let reg = self.reg_op(*get_op_mut(inst, 0));
              (*self.build).cbz(reg, &mut *fail);
            } else {
              let temp_a = self.temp_int64(*get_op_mut(inst, 0));

              if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                && ((self.int_64_op(*get_op_mut(inst, 1))) as u64) <= K_MAX_IMMEDIATE as u64
              {
                (*self.build).cmp(temp_a, (self.int_64_op(*get_op_mut(inst, 1))) as u16);
              } else {
                (*self.build).cmp(temp_a, self.temp_int64(*get_op_mut(inst, 1)));
              }

              (*self.build).b_condition_a_64_label(
                get_condition_int64(get_negated_condition(cond)),
                &mut *fail,
              );
            }
            self.ir_lowering_a_64_finalize_target_label(*get_op_mut(inst, 3), index, &mut fresh);
          }
        }
        IrCmd::INTERRUPT => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);

          let mut self_ = Label::default();

          (*self.build).ldr(
            X0,
            mem(
              R_GLOBAL_STATE,
              (core::mem::offset_of!(global_State, cb.interrupt) as i32),
            ),
          );
          (*self.build).cbnz(X0, &mut self_);

          let next = (*self.build).set_label();

          self.interrupt_handlers.push(InterruptHandler {
            self_,
            pcpos: self.uint_op(*get_op_mut(inst, 0)),
            next,
          });
        }
        IrCmd::CheckGc => {
          {
            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp2 = self.regs.alloc_temp(KindA64::X);

            CODEGEN_ASSERT!(
              (core::mem::offset_of!(global_State, totalbytes) as i32)
                == (core::mem::offset_of!(global_State, gc_threshold) as i32)
                  + (size_of::<usize>() as i32)
            );
            let mut skip = Label::default();
            (*self.build).ldp(
              temp1,
              temp2,
              mem(
                R_GLOBAL_STATE,
                (core::mem::offset_of!(global_State, gc_threshold) as i32),
              ),
            );
            (*self.build).cmp(temp1, temp2);
            (*self.build).b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut skip);

            let spills = self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[]);

            (*self.build).mov(X0, R_STATE);
            (*self.build).mov(W1, 1);
            (*self.build).ldr(
              X2,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_c_step) as i32),
              ),
            );
            (*self.build).blr(X2);

            emit_update_base(&mut *self.build);

            self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

            (*self.build).set_label_label(&mut skip);
          }
        }
        IrCmd::BarrierObj => {
          {
            let temp = self.regs.alloc_temp(KindA64::X);

            let mut skip = Label::default();
            let object = self.reg_op(*get_op_mut(inst, 0));
            let ratag = if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
              -1
            } else {
              self.tag_op(*get_op_mut(inst, 2)) as i32
            };
            self.check_object_barrier_conditions(
              object,
              temp,
              NOREG,
              *get_op_mut(inst, 1),
              ratag,
              &mut skip,
            );

            let reg = self.reg_op(*get_op_mut(inst, 0)); // note: we need to call regOp before spill so that we don't do redundant reloads
            let spills = self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[reg]);
            (*self.build).mov(X1, reg);
            (*self.build).mov(X0, R_STATE);
            (*self.build).ldr(
              X2,
              mem(
                r_base(),
                vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)
                  + (core::mem::offset_of!(TValue, value) as i32),
              ),
            );
            (*self.build).ldr(
              X3,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_c_barrierf) as i32),
              ),
            );
            (*self.build).blr(X3);

            self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

            // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
            (*self.build).set_label_label(&mut skip);
          }
        }
        IrCmd::BarrierTableBack => {
          {
            let mut skip = Label::default();
            let temp = self.regs.alloc_temp(KindA64::W);

            // isblack(obj2gco(t))
            (*self.build).ldrb(
              temp,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(GCheader, marked) as i32),
              ),
            );
            (*self.build).tbz(temp, BLACKBIT, &mut skip);

            let reg = self.reg_op(*get_op_mut(inst, 0)); // note: we need to call regOp before spill so that we don't do redundant reloads
            let spills = self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[reg]);
            (*self.build).mov(X1, reg);
            (*self.build).mov(X0, R_STATE);
            (*self.build).add(
              X2,
              X1,
              (core::mem::offset_of!(LuaTable, gclist) as i32) as u16,
            );
            (*self.build).ldr(
              X3,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_c_barrierback) as i32),
              ),
            );
            (*self.build).blr(X3);

            self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

            // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
            (*self.build).set_label_label(&mut skip);
          }
        }
        IrCmd::BarrierTableForward => {
          {
            let temp = self.regs.alloc_temp(KindA64::X);

            let mut skip = Label::default();
            let object = self.reg_op(*get_op_mut(inst, 0));
            let ratag = if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
              -1
            } else {
              self.tag_op(*get_op_mut(inst, 2)) as i32
            };
            self.check_object_barrier_conditions(
              object,
              temp,
              NOREG,
              *get_op_mut(inst, 1),
              ratag,
              &mut skip,
            );

            let reg = self.reg_op(*get_op_mut(inst, 0)); // note: we need to call regOp before spill so that we don't do redundant reloads
            let addr = self.temp_addr(
              *get_op_mut(inst, 1),
              (core::mem::offset_of!(TValue, value) as i32),
            );
            let spills = self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[reg]);
            (*self.build).mov(X1, reg);
            (*self.build).mov(X0, R_STATE);
            (*self.build).ldr(X2, addr);
            (*self.build).ldr(
              X3,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_c_barriertable) as i32),
              ),
            );
            (*self.build).blr(X3);

            self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

            // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
            (*self.build).set_label_label(&mut skip);
          }
        }
        IrCmd::SetSavedpc => {
          let temp1 = self.regs.alloc_temp(KindA64::X);
          let temp2 = self.regs.alloc_temp(KindA64::X);

          emit_add_offset(
            &mut *self.build,
            temp1,
            R_CODE,
            (self.uint_op(*get_op_mut(inst, 0)) as i32) * (size_of::<Instruction>() as i32),
          );
          (*self.build).ldr(
            temp2,
            mem(R_STATE, (core::mem::offset_of!(lua_State, ci) as i32)),
          );
          (*self.build).str(
            temp1,
            mem(temp2, (core::mem::offset_of!(CallInfo, savedpc) as i32)),
          );
        }
        IrCmd::CloseUpvals => {
          {
            let mut skip = Label::default();
            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp2 = self.regs.alloc_temp(KindA64::X);

            // l->openupval != 0
            (*self.build).ldr(
              temp1,
              mem(
                R_STATE,
                (core::mem::offset_of!(lua_State, openupval) as i32),
              ),
            );
            (*self.build).cbz(temp1, &mut skip);

            // ra <= l->openupval->v
            (*self.build).ldr(temp1, mem(temp1, (core::mem::offset_of!(UpVal, v) as i32)));
            (*self.build).add(
              temp2,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
            );
            (*self.build).cmp(temp2, temp1);
            (*self.build).b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut skip);

            let spills = self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[temp2]);
            (*self.build).mov(X1, temp2);
            (*self.build).mov(X0, R_STATE);
            (*self.build).ldr(
              X2,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_f_close) as i32),
              ),
            );
            (*self.build).blr(X2);

            self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

            (*self.build).set_label_label(&mut skip);
          }
        }
        IrCmd::CAPTURE => {
          // no-op
        }
        IrCmd::SETLIST => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_setlist) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::CALL => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          // argtop = if (nparams == LUA_MULTRET) { l->top } else { ra + 1 + nparams };
          if self.int_op(*get_op_mut(inst, 1)) == LUA_MULTRET {
            (*self.build).ldr(
              X2,
              mem(R_STATE, (core::mem::offset_of!(lua_State, top) as i32)),
            );
          } else {
            (*self.build).add(
              X2,
              r_base(),
              ((vm_reg_op(*get_op_mut(inst, 0)) + 1 + self.int_op(*get_op_mut(inst, 1)))
                * (size_of::<TValue>() as i32)) as u16,
            );
          }

          // call_fallback(l, ra, argtop, nresults)
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).mov(W3, self.int_op(*get_op_mut(inst, 2)));
          (*self.build).ldr(
            X4,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, call_fallback) as i32),
            ),
          );
          (*self.build).blr(X4);

          emit_update_base(&mut *self.build);

          // reentry with x0=closure (NULL implies C function; CALL_FALLBACK_YIELD will trigger exit)
          (*self.build).cbnz(X0, &mut (*self.helpers).continue_call);
        }
        IrCmd::RETURN => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);

          if (*self.function).variadic {
            (*self.build).ldr(
              X1,
              mem(R_STATE, (core::mem::offset_of!(lua_State, ci) as i32)),
            );
            (*self.build).ldr(X1, mem(X1, (core::mem::offset_of!(CallInfo, func) as i32)));
          } else if self.int_op(*get_op_mut(inst, 1)) != 1 {
            (*self.build).sub(X1, r_base(), (size_of::<TValue>() as i32) as u16);
          } // invariant: ci->func + 1 == ci->base for non-variadic frames

          if self.int_op(*get_op_mut(inst, 1)) == 0 {
            (*self.build).mov(W2, 0);
            (*self.build).b(&mut (*self.helpers).return_);
          } else if self.int_op(*get_op_mut(inst, 1)) == 1 && !(*self.function).variadic {
            // fast path: minimizes x1 adjustments
            // note that we skipped x1 computation for this specific case above
            (*self.build).ldr(
              Q0,
              mem(
                r_base(),
                vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32),
              ),
            );
            (*self.build).str(Q0, mem(r_base(), -((size_of::<TValue>() as i32) as i32)));
            (*self.build).mov(X1, r_base());
            (*self.build).mov(W2, 1);
            (*self.build).b(&mut (*self.helpers).return_);
          } else if self.int_op(*get_op_mut(inst, 1)) >= 1 && self.int_op(*get_op_mut(inst, 1)) <= 3
          {
            for r in 0..self.int_op(*get_op_mut(inst, 1)) {
              (*self.build).ldr(
                Q0,
                mem(
                  r_base(),
                  (vm_reg_op(*get_op_mut(inst, 0)) + r) * (size_of::<TValue>() as i32),
                ),
              );
              (*self.build).str(
                Q0,
                mem_kind(X1, size_of::<TValue>() as i32, AddressKindA64::Post),
              );
            }
            (*self.build).mov(W2, self.int_op(*get_op_mut(inst, 1)));
            (*self.build).b(&mut (*self.helpers).return_);
          } else {
            (*self.build).mov(W2, 0);

            // vali = ra
            (*self.build).add(
              X3,
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
            );

            // valend = if (n == LUA_MULTRET) { l->top } else { ra + n
            if self.int_op(*get_op_mut(inst, 1)) == LUA_MULTRET {
              (*self.build).ldr(
                X4,
                mem(R_STATE, (core::mem::offset_of!(lua_State, top) as i32)),
              );
            } else {
              (*self.build).add(
                X4,
                r_base(),
                ((vm_reg_op(*get_op_mut(inst, 0)) + self.int_op(*get_op_mut(inst, 1)))
                  * (size_of::<TValue>() as i32)) as u16,
              );
            }

            let mut repeat_value_loop = Label::default();
            let mut exit_value_loop = Label::default();

            if self.int_op(*get_op_mut(inst, 1)) == LUA_MULTRET {
              (*self.build).cmp(X3, X4);
              (*self.build).b_condition_a_64_label(ConditionA64::CarrySet, &mut exit_value_loop);
              // CarrySet == UNSIGNED_GREATER_EQUAL
            }

            (*self.build).set_label_label(&mut repeat_value_loop);
            (*self.build).ldr(
              Q0,
              mem_kind(X3, size_of::<TValue>() as i32, AddressKindA64::Post),
            );
            (*self.build).str(
              Q0,
              mem_kind(X1, size_of::<TValue>() as i32, AddressKindA64::Post),
            );
            (*self.build).add(W2, W2, 1_u16);
            (*self.build).cmp(X3, X4);
            (*self.build).b_condition_a_64_label(ConditionA64::CarryClear, &mut repeat_value_loop); // CarryClear == UNSIGNED_LESS

            (*self.build).set_label_label(&mut exit_value_loop);
            (*self.build).b(&mut (*self.helpers).return_);
          }
        }
        IrCmd::FORGLOOP => {
          // register layout: ra + 1 = table, ra + 2 = internal index, ra + 3 .. ra + aux = iteration variables
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          // clear extra variables since we might have more than two
          if self.int_op(*get_op_mut(inst, 1)) > 2 {
            CODEGEN_ASSERT!(LUA_TNIL == 0);
            for i in 2..self.int_op(*get_op_mut(inst, 1)) {
              (*self.build).str(
                WZR,
                mem(
                  r_base(),
                  (vm_reg_op(*get_op_mut(inst, 0)) + 3 + i) * (size_of::<TValue>() as i32)
                    + (core::mem::offset_of!(TValue, tt) as i32),
                ),
              );
            }
          }
          // we use full iter fallback for now; in the future it could be worthwhile to accelerate array iteration here
          (*self.build).mov(X0, R_STATE);
          (*self.build).ldr(
            X1,
            mem(
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 0)) + 1) * (size_of::<TValue>() as i32)
                + K_TVALUE_VALUE_GC_OFFSET,
            ),
          );
          (*self.build).ldr(
            W2,
            mem(
              r_base(),
              (vm_reg_op(*get_op_mut(inst, 0)) + 2) * (size_of::<TValue>() as i32)
                + K_TVALUE_VALUE_P_OFFSET,
            ),
          );
          (*self.build).add(
            X3,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).ldr(
            X4,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, forg_loop_table_iter) as i32),
            ),
          );
          (*self.build).blr(X4);
          // note: no emitUpdateBase necessary because forgLoopTableIter does not reallocate stack
          (*self.build).cbnz(W0, self.label_op(*get_op_mut(inst, 2)));
          self.jump_or_fallthrough_op(*get_op_mut(inst, 3), next);
        }
        IrCmd::ForgloopFallback => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).mov(W1, vm_reg_op(*get_op_mut(inst, 0)));
          (*self.build).mov(W2, self.int_op(*get_op_mut(inst, 1)));

          if FFlag::LuauYieldIter2.get() {
            (*self.build).ldr(
              X3,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, forg_loop_non_table_fallback) as i32),
              ),
            );
            (*self.build).blr(X3);
            emit_update_base(&mut *self.build);
            (*self.build).cmp(W0, 0_u16);
            (*self.build)
              .b_condition_a_64_label(ConditionA64::Less, &mut (*self.helpers).exit_no_continue_vm);
            (*self.build)
              .b_condition_a_64_label(ConditionA64::Greater, self.label_op(*get_op_mut(inst, 2)));
          } else {
            (*self.build).ldr(
              X3,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, forg_loop_non_table_fallback_deprecated)
                  as i32),
              ),
            );
            (*self.build).blr(X3);
            emit_update_base(&mut *self.build);
            (*self.build).cbnz(W0, self.label_op(*get_op_mut(inst, 2)));
          }

          self.jump_or_fallthrough_op(*get_op_mut(inst, 3), next);
        }
        IrCmd::ForgprepXnextFallback => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 1)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).mov(W2, self.uint_op(*get_op_mut(inst, 0)) + 1);
          (*self.build).ldr(
            X3,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, forg_prep_xnext_fallback) as i32),
            ),
          );
          (*self.build).blr(X3);
          // note: no emitUpdateBase necessary because forgPrepXnextFallback does not reallocate stack
          self.jump_or_fallthrough_op(*get_op_mut(inst, 2), next);
        }
        IrCmd::COVERAGE => {
          {
            let temp1 = self.regs.alloc_temp(KindA64::X);
            let temp2 = self.regs.alloc_temp(KindA64::W);
            let temp3 = self.regs.alloc_temp(KindA64::W);

            (*self.build).mov(
              temp1,
              (self.uint_op(*get_op_mut(inst, 0)) as i32) * (size_of::<Instruction>() as i32),
            );
            (*self.build).ldr(temp2, mem(R_CODE, temp1));

            // increments E (high 24 bits); if the result overflows a 23-bit counter, high bit becomes 1
            // note: cmp can be eliminated with adds but we aren't concerned with code size for coverage
            (*self.build).add(temp3, temp2, 256_u16);
            (*self.build).cmp(temp3, 0_u16);
            (*self.build).csel(temp2, temp2, temp3, ConditionA64::Less);

            (*self.build).str(temp2, mem(R_CODE, temp1));
          }

          // Full instruction fallbacks
        }
        IrCmd::FallbackGetglobal => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_getglobal) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FallbackSetglobal => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_setglobal) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FallbackGettableks => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_gettableks) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FallbackSettableks => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_settableks) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FallbackNamecall => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_namecall) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FallbackPrepvarargs => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_prepvarargs) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FallbackGetvarargs => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::Constant);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);

          if self.int_op(*get_op_mut(inst, 2)) == LUA_MULTRET {
            emit_add_offset(
              &mut *self.build,
              X1,
              R_CODE,
              (self.uint_op(*get_op_mut(inst, 0)) as i32) * (size_of::<Instruction>() as i32),
            );
            (*self.build).mov(X2, r_base());
            (*self.build).mov(W3, vm_reg_op(*get_op_mut(inst, 1)));
            (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, execute_getvarargsmult_ret) as i32),
              ),
            );
            (*self.build).blr(X4);

            emit_update_base(&mut *self.build);
          } else {
            (*self.build).mov(X1, r_base());
            (*self.build).mov(W2, vm_reg_op(*get_op_mut(inst, 1)));
            (*self.build).mov(W3, self.int_op(*get_op_mut(inst, 2)));
            (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, execute_getvarargsconst) as i32),
              ),
            );
            (*self.build).blr(X4);

            // note: no emitUpdateBase necessary because executeGETVARARGSConst does not reallocate stack
          }
        }
        IrCmd::NEWCLOSURE => {
          {
            let reg = self.reg_op(*get_op_mut(inst, 1)); // note: we need to call regOp before spill so that we don't do redundant reloads

            self
              .regs
              .spill_u32_initializer_list_register_a_64(index, &[reg]);
            (*self.build).mov(X2, reg);

            (*self.build).mov(X0, R_STATE);
            (*self.build).mov(W1, self.uint_op(*get_op_mut(inst, 0)));

            (*self.build).ldr(X3, mem(R_CLOSURE, K_CLOSURE_L_P_OFFSET));
            (*self.build).ldr(X3, mem(X3, (core::mem::offset_of!(Proto, p) as i32)));

            let proto_index = self.uint_op(*get_op_mut(inst, 2)); // 0..32767
            let proto_offset = ((size_of::<*mut Proto>() as i32) * proto_index as i32) as i32;

            if proto_index <= AddressA64::K_MAX_OFFSET as u32 {
              (*self.build).ldr(X3, mem(X3, proto_offset));
            } else {
              (*self.build).mov(X4, proto_offset);
              (*self.build).ldr(X3, mem(X3, X4));
            }

            (*self.build).ldr(
              X4,
              mem(
                R_NATIVE_CONTEXT,
                (core::mem::offset_of!(NativeContext, lua_f_new_lclosure) as i32),
              ),
            );
            (*self.build).blr(X4);

            inst.reg_a64 = self.regs.take_reg(X0, index);
          }
        }
        IrCmd::FallbackDupclosure => {
          CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst);

          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_dupclosure) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
        }
        IrCmd::FallbackForgprep => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          emit_fallback(
            &mut *self.build,
            (core::mem::offset_of!(NativeContext, execute_forgprep) as i32),
            self.uint_op(*get_op_mut(inst, 0)),
          );
          self.jump_or_fallthrough_op(*get_op_mut(inst, 2), next);

          // Pseudo instructions
        }
        IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MarkUsed | IrCmd::MarkDead => {
          CODEGEN_ASSERT!(false, "Pseudo instructions should not be lowered");
        }
        IrCmd::BitandInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_int64(*get_op_mut(inst, 0));
          let temp2 = self.temp_int64(*get_op_mut(inst, 1));
          (*self.build).and_(inst.reg_a64, temp1, temp2);
        }
        IrCmd::BitxorInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_int64(*get_op_mut(inst, 0));
          let temp2 = self.temp_int64(*get_op_mut(inst, 1));
          (*self.build).eor(inst.reg_a64, temp1, temp2);
        }
        IrCmd::BitorInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let temp1 = self.temp_int64(*get_op_mut(inst, 0));
          let temp2 = self.temp_int64(*get_op_mut(inst, 1));
          (*self.build).orr(inst.reg_a64, temp1, temp2);
        }
        IrCmd::BitnotInt64 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_int64(*get_op_mut(inst, 0));
          (*self.build).mvn_(inst.reg_a64, temp);
        }
        IrCmd::BitlshiftInt64 => {
          {
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::X,
              index,
              &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
            );
            let source = self.temp_int64(*get_op_mut(inst, 0));
            let amount = self.temp_int64(*get_op_mut(inst, 1));
            let temp = self.regs.alloc_temp(KindA64::X);

            let mut done = Label::default();
            let mut negative = Label::default();
            let mut out_of_range = Label::default();

            // (amount + 63) > 126 = |amount| > 63
            (*self.build).add(temp, amount, 63_u16);
            (*self.build).cmp(temp, 126_u16);
            (*self.build).b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut out_of_range);

            // check sign of amount
            (*self.build).cmp(amount, 0_u16);
            (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut negative);

            // left shift
            (*self.build).lsl(inst.reg_a64, source, amount);
            (*self.build).b(&mut done);

            // right shift by -amount
            (*self.build).set_label_label(&mut negative);
            (*self.build).neg(temp, amount);
            (*self.build).lsr(inst.reg_a64, source, temp);
            (*self.build).b(&mut done);

            (*self.build).set_label_label(&mut out_of_range);
            (*self.build).mov(inst.reg_a64, 0);

            (*self.build).set_label_label(&mut done);
          }
        }
        IrCmd::BitrshiftInt64 => {
          {
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::X,
              index,
              &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
            );
            let source = self.temp_int64(*get_op_mut(inst, 0));
            let amount = self.temp_int64(*get_op_mut(inst, 1));
            let temp = self.regs.alloc_temp(KindA64::X);

            let mut done = Label::default();
            let mut negative = Label::default();
            let mut out_of_range = Label::default();

            // (amount + 63) > 126 = |amount| > 63
            (*self.build).add(temp, amount, 63_u16);
            (*self.build).cmp(temp, 126_u16);
            (*self.build).b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut out_of_range);

            // check sign of amount
            (*self.build).cmp(amount, 0_u16);
            (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut negative);

            // unsigned right shift
            (*self.build).lsr(inst.reg_a64, source, amount);
            (*self.build).b(&mut done);

            // left shift by -amount
            (*self.build).set_label_label(&mut negative);
            (*self.build).neg(temp, amount);
            (*self.build).lsl(inst.reg_a64, source, temp);
            (*self.build).b(&mut done);

            (*self.build).set_label_label(&mut out_of_range);
            (*self.build).mov(inst.reg_a64, 0);

            (*self.build).set_label_label(&mut done);
          }
        }
        IrCmd::BitarshiftInt64 => {
          {
            inst.reg_a64 = self.regs.alloc_reuse(
              KindA64::X,
              index,
              &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
            );
            let source = self.temp_int64(*get_op_mut(inst, 0));
            let amount = self.temp_int64(*get_op_mut(inst, 1));
            let temp = self.regs.alloc_temp(KindA64::X);

            let mut done = Label::default();
            let mut negative = Label::default();
            let mut out_of_range_positive = Label::default();
            let mut out_of_range_negative = Label::default();

            // amount > 63 (arithmetic right shift fills with sign)
            (*self.build).cmp(amount, 63_u16);
            (*self.build).b_condition_a_64_label(ConditionA64::Greater, &mut out_of_range_positive);

            // add 63, if < 0 then amount < -63
            (*self.build).add(temp, amount, 63_u16);
            (*self.build).cmp(temp, 0_u16);
            (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut out_of_range_negative);

            // check sign of amount
            (*self.build).cmp(amount, 0_u16);
            (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut negative);

            // arithmetic right shift that sign extends
            (*self.build).asr(inst.reg_a64, source, amount);
            (*self.build).b(&mut done);

            // left shift by -amount (((*self.build).set_label_label(&mut negative)) as u32);
            (*self.build).neg(temp, amount);
            (*self.build).lsl(inst.reg_a64, source, temp);
            (*self.build).b(&mut done);

            // amount > 63 = sign-fill ( if n < 0 { -1 } else { 0 })
            (*self.build).set_label_label(&mut out_of_range_positive);
            (*self.build).asr(inst.reg_a64, source, 63_u8);
            (*self.build).b(&mut done);

            // amount < -63 = result is 0
            (*self.build).set_label_label(&mut out_of_range_negative);
            (*self.build).mov(inst.reg_a64, 0);

            (*self.build).set_label_label(&mut done);
          }
        }
        IrCmd::BitlrotateInt64 => {
          {
            inst.reg_a64 = self
              .regs
              .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 1))]); // can't reuse A because it would be clobbered by neg
            let source = self.temp_int64(*get_op_mut(inst, 0));
            let amount = self.temp_int64(*get_op_mut(inst, 1));
            // left rotate = rotate by negative
            (*self.build).neg(inst.reg_a64, amount);
            (*self.build).ror(inst.reg_a64, source, inst.reg_a64);
          }
        }
        IrCmd::BitrrotateInt64 => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::X,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          let source = self.temp_int64(*get_op_mut(inst, 0));
          let amount = self.temp_int64(*get_op_mut(inst, 1));
          (*self.build).ror(inst.reg_a64, source, amount);
        }
        IrCmd::BitcountlzInt64 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_int64(*get_op_mut(inst, 0));
          (*self.build).clz(inst.reg_a64, temp);
        }
        IrCmd::BitcountrzInt64 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_int64(*get_op_mut(inst, 0));
          (*self.build).rbit(inst.reg_a64, temp);
          (*self.build).clz(inst.reg_a64, inst.reg_a64);
        }
        IrCmd::ByteswapInt64 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::X, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_int64(*get_op_mut(inst, 0));
          (*self.build).rev(inst.reg_a64, temp);
        }
        IrCmd::BitandUint => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && (*self.build).is_mask_supported((self.int_op(*get_op_mut(inst, 1))) as u32)
          {
            (*self.build).and_(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (self.int_op(*get_op_mut(inst, 1))) as u32,
            );
          } else {
            let temp1 = self.temp_uint(*get_op_mut(inst, 0));
            let temp2 = self.temp_uint(*get_op_mut(inst, 1));
            (*self.build).and_(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::BitxorUint => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && (*self.build).is_mask_supported((self.int_op(*get_op_mut(inst, 1))) as u32)
          {
            (*self.build).eor(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (self.int_op(*get_op_mut(inst, 1))) as u32,
            );
          } else {
            let temp1 = self.temp_uint(*get_op_mut(inst, 0));
            let temp2 = self.temp_uint(*get_op_mut(inst, 1));
            (*self.build).eor(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::BitorUint => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            && (*self.build).is_mask_supported((self.int_op(*get_op_mut(inst, 1))) as u32)
          {
            (*self.build).orr(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (self.int_op(*get_op_mut(inst, 1))) as u32,
            );
          } else {
            let temp1 = self.temp_uint(*get_op_mut(inst, 0));
            let temp2 = self.temp_uint(*get_op_mut(inst, 1));
            (*self.build).orr(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::BitnotUint => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_uint(*get_op_mut(inst, 0));
          (*self.build).mvn_(inst.reg_a64, temp);
        }
        IrCmd::BitlshiftUint => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
          {
            (*self.build).lsl(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (((self.int_op(*get_op_mut(inst, 1))) as u32) & 31) as u8,
            );
          } else {
            let temp1 = self.temp_uint(*get_op_mut(inst, 0));
            let temp2 = self.temp_uint(*get_op_mut(inst, 1));
            (*self.build).lsl(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::BitrshiftUint => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
          {
            (*self.build).lsr(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (((self.int_op(*get_op_mut(inst, 1))) as u32) & 31) as u8,
            );
          } else {
            let temp1 = self.temp_uint(*get_op_mut(inst, 0));
            let temp2 = self.temp_uint(*get_op_mut(inst, 1));
            (*self.build).lsr(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::BitarshiftUint => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
          {
            (*self.build).asr(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (((self.int_op(*get_op_mut(inst, 1))) as u32) & 31) as u8,
            );
          } else {
            let temp1 = self.temp_uint(*get_op_mut(inst, 0));
            let temp2 = self.temp_uint(*get_op_mut(inst, 1));
            (*self.build).asr(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::BitlrotateUint => {
          {
            if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
              && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            {
              inst.reg_a64 = self
                .regs
                .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);
              (*self.build).ror(
                inst.reg_a64,
                self.reg_op(*get_op_mut(inst, 0)),
                ((32 - ((self.int_op(*get_op_mut(inst, 1))) as u32)) & 31) as u8,
              );
            } else {
              inst.reg_a64 = self
                .regs
                .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 1))]); // can't reuse a because it would be clobbered by neg
              let temp1 = self.temp_uint(*get_op_mut(inst, 0));
              let temp2 = self.temp_uint(*get_op_mut(inst, 1));
              (*self.build).neg(inst.reg_a64, temp2);
              (*self.build).ror(inst.reg_a64, temp1, inst.reg_a64);
            }
          }
        }
        IrCmd::BitrrotateUint => {
          inst.reg_a64 = self.regs.alloc_reuse(
            KindA64::W,
            index,
            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
          );
          if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
          {
            (*self.build).ror(
              inst.reg_a64,
              self.reg_op(*get_op_mut(inst, 0)),
              (((self.int_op(*get_op_mut(inst, 1))) as u32) & 31) as u8,
            );
          } else {
            let temp1 = self.temp_uint(*get_op_mut(inst, 0));
            let temp2 = self.temp_uint(*get_op_mut(inst, 1));
            (*self.build).ror(inst.reg_a64, temp1, temp2);
          }
        }
        IrCmd::BitcountlzUint => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_uint(*get_op_mut(inst, 0));
          (*self.build).clz(inst.reg_a64, temp);
        }
        IrCmd::BitcountrzUint => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_uint(*get_op_mut(inst, 0));
          (*self.build).rbit(inst.reg_a64, temp);
          (*self.build).clz(inst.reg_a64, inst.reg_a64);
        }
        IrCmd::ByteswapUint => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 0))]);
          let temp = self.temp_uint(*get_op_mut(inst, 0));
          (*self.build).rev(inst.reg_a64, temp);
        }
        IrCmd::InvokeLibm => {
          {
            if has_op_c(inst) {
              let is_int = if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                self.const_op(*get_op_mut(inst, 2)).kind == IrConstKind::Int
              } else {
                get_cmd_value_kind((*self.function).inst_op(*get_op_mut(inst, 2)).cmd)
                  == IrValueKind::Int
              };

              let temp1 = self.temp_double(*get_op_mut(inst, 1));
              let temp2 = if is_int {
                self.temp_int(*get_op_mut(inst, 2))
              } else {
                self.temp_double(*get_op_mut(inst, 2))
              };
              let temp3 = if is_int {
                NOREG
              } else {
                self.regs.alloc_temp(KindA64::D)
              }; // note: spill() frees all registers so we need to avoid alloc after spill
              self
                .regs
                .spill_u32_initializer_list_register_a_64(index, &[temp1, temp2]);

              if is_int {
                (*self.build).fmov(D0, temp1);
                (*self.build).mov(W0, temp2);
              } else if D0 != temp2 {
                (*self.build).fmov(D0, temp1);
                (*self.build).fmov(D1, temp2);
              } else {
                (*self.build).fmov(temp3, D0);
                (*self.build).fmov(D0, temp1);
                (*self.build).fmov(D1, temp3);
              }
            } else {
              let temp1 = self.temp_double(*get_op_mut(inst, 1));
              self
                .regs
                .spill_u32_initializer_list_register_a_64(index, &[temp1]);
              (*self.build).fmov(D0, temp1);
            }

            (*self.build).ldr(
              X1,
              mem(
                R_NATIVE_CONTEXT,
                get_native_context_offset(self.uint_op(*get_op_mut(inst, 0)) as i32) as i32,
              ),
            );
            (*self.build).blr(X1);
            inst.reg_a64 = self.regs.take_reg(D0, index);
          }
        }
        IrCmd::GetType => {
          {
            inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);

            CODEGEN_ASSERT!((size_of::<*mut tstring>() as i32) == 8);

            if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
              (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                inst.reg_a64,
                R_GLOBAL_STATE,
                self.reg_op(*get_op_mut(inst, 0)),
                3,
              );
            }
            // implicit uxtw
            else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
              (*self.build).add(
                inst.reg_a64,
                R_GLOBAL_STATE,
                (self.tag_op(*get_op_mut(inst, 0)) * 8) as u16,
              );
            } else {
              CODEGEN_ASSERT!(false, "Unsupported instruction form");
            }

            (*self.build).ldr(
              inst.reg_a64,
              mem(
                inst.reg_a64,
                (core::mem::offset_of!(global_State, ttname) as i32),
              ),
            );
          }
        }
        IrCmd::GetTypeof => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).ldr(
            X2,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, lua_t_objtypenamestr) as i32),
            ),
          );
          (*self.build).blr(X2);

          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
        IrCmd::FINDUPVAL => {
          self
            .regs
            .spill_u32_initializer_list_register_a_64(index, &[]);
          (*self.build).mov(X0, R_STATE);
          (*self.build).add(
            X1,
            r_base(),
            (vm_reg_op(*get_op_mut(inst, 0)) * (size_of::<TValue>() as i32)) as u16,
          );
          (*self.build).ldr(
            X2,
            mem(
              R_NATIVE_CONTEXT,
              (core::mem::offset_of!(NativeContext, lua_f_findupval) as i32),
            ),
          );
          (*self.build).blr(X2);

          inst.reg_a64 = self.regs.take_reg(X0, index);
        }
        IrCmd::BufferReadi8 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 1))]);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldrsb(inst.reg_a64, addr);
        }
        IrCmd::BufferReadu8 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 1))]);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldrb(inst.reg_a64, addr);
        }
        IrCmd::BufferWritei8 => {
          let temp = self.temp_int(*get_op_mut(inst, 2));
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 3)),
          );

          (*self.build).strb(temp, addr);
        }
        IrCmd::BufferReadi16 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 1))]);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldrsh(inst.reg_a64, addr);
        }
        IrCmd::BufferReadu16 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 1))]);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldrh(inst.reg_a64, addr);
        }
        IrCmd::BufferWritei16 => {
          let temp = self.temp_int(*get_op_mut(inst, 2));
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 3)),
          );

          (*self.build).strh(temp, addr);
        }
        IrCmd::BufferReadi32 => {
          inst.reg_a64 = self
            .regs
            .alloc_reuse(KindA64::W, index, &[(*get_op_mut(inst, 1))]);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::BufferWritei32 => {
          let temp = self.temp_int(*get_op_mut(inst, 2));
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 3)),
          );

          (*self.build).str(temp, addr);
        }
        IrCmd::BufferReadf32 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::S, index);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::BufferWritef32 => {
          let temp = self.temp_float(*get_op_mut(inst, 2));
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 3)),
          );

          (*self.build).str(temp, addr);
        }
        IrCmd::BufferReadf64 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::D, index);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::BufferWritef64 => {
          let temp = self.temp_double(*get_op_mut(inst, 2));
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 3)),
          );

          (*self.build).str(temp, addr);
        }
        IrCmd::BufferReadi64 => {
          inst.reg_a64 = self.regs.alloc_reg(KindA64::X, index);
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 2)),
          );

          (*self.build).ldr(inst.reg_a64, addr);
        }
        IrCmd::BufferWritei64 => {
          let temp = self.temp_int64(*get_op_mut(inst, 2));
          let addr = self.temp_addr_buffer(
            *get_op_mut(inst, 0),
            *get_op_mut(inst, 1),
            self.tag_op(*get_op_mut(inst, 3)),
          );

          (*self.build).str(temp, addr);
        }
        IrCmd::JumpCmpProtoid => {
          {
            CODEGEN_ASSERT!(
              (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
            );
            let temp = self.regs.alloc_temp(KindA64::X);
            let tempw = cast_reg(KindA64::W, temp);

            // Is it a C closure?
            (*self.build).ldrb(
              tempw,
              mem(
                self.reg_op(*get_op_mut(inst, 0)),
                (core::mem::offset_of!(Closure, is_c) as i32),
              ),
            );
            (*self.build).cbnz(tempw, self.label_op(*get_op_mut(inst, 3)));

            // Load Proto and compare funid
            (*self.build).ldr(
              temp,
              mem(self.reg_op(*get_op_mut(inst, 0)), K_CLOSURE_L_P_OFFSET),
            );
            (*self.build).ldr(
              tempw,
              mem(temp, (core::mem::offset_of!(Proto, funid) as i32)),
            );
            let proto_id = self.uint_op(*get_op_mut(inst, 1));
            if proto_id <= K_MAX_IMMEDIATE {
              (*self.build).cmp(tempw, proto_id as u16);
            } else {
              let temp2 = self.regs.alloc_temp(KindA64::W);
              (*self.build).mov(temp2, proto_id);
              (*self.build).cmp(tempw, temp2);
            }

            (*self.build)
              .b_condition_a_64_label(ConditionA64::NotEqual, self.label_op(*get_op_mut(inst, 3)));

            self.jump_or_fallthrough_op(*get_op_mut(inst, 2), next);
          }
        }
      }
      self.value_tracker.after_inst_lowering(inst, index);

      self.regs.curr_inst_idx = K_INVALID_INST_IDX;

      self.regs.free_last_use_regs(inst, index);
      self.regs.free_temp_regs();
    }
  }
}
