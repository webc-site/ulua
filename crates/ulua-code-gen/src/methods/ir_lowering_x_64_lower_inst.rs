//! Source: `CodeGen/src/IrLoweringX64.cpp:50`
use core::{
  ffi::c_void,
  mem::{offset_of, size_of},
  ptr,
};

use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  macros::lua_multret::LUA_MULTRET,
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
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
  type_aliases::{instruction::Instruction, luau_fast_function::LuauFastFunction},
};

// 本地寄存器常量助手（对应 EmitCommonX64.h）
use crate::{
  enums::{
    condition_x_64::ConditionX64, features_x_64::FeaturesX64, ir_cmd::IrCmd,
    ir_condition::IrCondition, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind,
    rounding_mode_x_64::RoundingModeX64, size_x_64::SizeX64,
  },
  macros::{
    codegen_assert::{CODEGEN_ASSERT, unsupported_instruction_form},
    ir_operand::{HAS_OP_B, HAS_OP_C, HAS_OP_D, HAS_OP_E},
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{R_BASE, R_CONSTANTS, R_NATIVE_CONTEXT, R_STATE},
    interrupt_handler_ir_lowering_x_64::InterruptHandler,
    ir_block::IrBlock,
    ir_const::IrConst,
    ir_data::{K_INVALID_INST_IDX, K_NATIVE_PTR_SIZE},
    ir_inst::IrInst,
    ir_lowering_x_64::IrLoweringX64,
    ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64,
    label::Label,
    native_context::NativeContext,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
    scoped_spills::ScopedSpills,
  },
};
const K_TVALUE_SIZE_LOG2: i32 = 4;
const K_LUA_NODE_SIZE_LOG2: i32 = 5;
const K_OFFSET_OF_TKEY_TAG_NEXT: i32 = 12;
const K_TKEY_TAG_BITS: i32 = 4;
const K_TKEY_TAG_MASK: i32 = (1 << K_TKEY_TAG_BITS) - 1;
const INT_MAX: i32 = i32::MAX;
// 结构体偏移由编译器按 Rust 布局计算, 对齐 cpp 的 offsetof(TString, len) / offsetof(Buffer, len)
const K_TSTRING_LEN_OFFSET: i32 = offset_of!(tstring, len) as i32;
const K_BUFFER_LEN_OFFSET: i32 = offset_of!(LuauBuffer, len) as i32;
const K_CLOSURE_LUPREFS_OFFSET: i32 =
  (offset_of!(Closure, inner) + offset_of!(LClosure, uprefs)) as i32;
const K_CLOSURE_LPOFFSET: i32 = (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32;
const fn xmm0() -> RegisterX64 {
  RegisterX64::XMM0
}
fn sized_mem(mut op: OperandX64, size: SizeX64) -> OperandX64 {
  op.mem_size = size;
  op
}
/// dword/qword[base + ri*sizeof(TValue) + off]：栈槽(rBase=R14)/常量槽
/// (rConstants=R12) 中 TValue 字段地址，与 luau_reg_tag/luau_constant_* 族逐位同构。
const fn tv_slot(size: SizeX64, base: RegisterX64, ri: i32, off: i32) -> OperandX64 {
  OperandX64::mem(
    size,
    RegisterX64::NOREG,
    1,
    base,
    ri * (size_of::<TValue>() as i32) + off,
  )
}
use crate::functions::{
  byte_reg::byte_reg, call_arith_helper::call_arith_helper,
  call_barrier_object::call_barrier_object, call_barrier_table_fast::call_barrier_table_fast,
  call_get_table::call_get_table, call_length_helper::call_length_helper,
  call_set_table::call_set_table, call_step_gc::call_step_gc,
  check_object_barrier_conditions::check_object_barrier_conditions, condition_op::condition_op,
  convert_number_to_index_or_jump::convert_number_to_index_or_jump, dword_reg::dword_reg,
  emit_builtin_emit_builtins_x_64::emit_builtin_ir_reg_alloc_x_64_assembly_builder_x_64_i32_i32_i32_i32 as emit_builtin,
  emit_fallback_emit_common_x_64::emit_fallback, emit_inst_call::emit_inst_call,
  emit_inst_for_g_loop::emit_inst_for_g_loop, emit_inst_return::emit_inst_return,
  emit_inst_set_list::emit_inst_set_list, emit_update_base_emit_common_x_64::emit_update_base,
  float_bits::get_float_bits, get_cmd_value_kind::get_cmd_value_kind,
  get_condition_int_emit_common_x_64::get_condition_int,
  get_inverse_condition_condition_x_64::get_inverse_condition,
  get_native_context_offset::get_native_context_offset,
  get_negated_condition_ir_utils::get_negated_condition_ir_condition,
  get_table_node_at_cached_slot::get_table_node_at_cached_slot, is_gco::is_gco,
  jump_on_number_cmp::jump_on_number_cmp, luau_constant::luau_constant,
  luau_constant_address::luau_constant_address, luau_constant_tag::luau_constant_tag,
  luau_constant_value::luau_constant_value, luau_node_key_tag::luau_node_key_tag,
  luau_node_key_value::luau_node_key_value, luau_reg::luau_reg, luau_reg_address::luau_reg_address,
  luau_reg_tag::luau_reg_tag, luau_reg_value::luau_reg_value,
  luau_reg_value_int::luau_reg_value_int, luau_reg_value_int_64::luau_reg_value_int_64,
  luau_reg_value_vector::luau_reg_value_vector, nvalue::nvalue,
  produces_dirty_high_register_bits::produces_dirty_high_register_bits, qword_reg::qword_reg,
  s_closure::s_closure, s_code::s_code, vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  vm_upvalue_op::vm_upvalue_op, word_reg::word_reg,
};

impl IrLoweringX64 {
  /// 双分支跳转门面（与 a64 `jump_or_fallthrough_op` 同款）：『按操作数取块可变视图 +
  /// self 方法』混窗收口于 records 的 `with_block_mut`，本文件零 unsafe。
  fn jump_or_fallthrough_op(&mut self, op: IrOp, next: &IrBlock) {
    self.with_block_mut(op, |s, t| s.jump_or_fallthrough(t, next));
  }

  /// XMM 双精度四则同构 lowering 骨架（AddNum/SubNum/MulNum/DivNum/IdivNum 共享）：
  /// 结果寄存器可复用任一源；op0 为常量时先装载临时寄存器，再发射 `emit` 指令。
  /// `load`/`mov`/`emit` 显式承载 sd/ss 宽度与运算种类差异，临时 ScopedReg 在发射点前
  /// 保持存活，寄存器分配顺序与展开版逐位一致。
  fn lower_scalar_arith(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    load: fn(&mut Self, IrOp) -> OperandX64,
    mov: fn(&mut Self, OperandX64, OperandX64),
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64, OperandX64),
  ) {
    inst.reg_x64 = self
      .regs
      .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0), inst.op(1)]);

    if inst.op(0).kind() == IrOpKind::Constant {
      let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);
      let cst = load(self, inst.op(0));
      mov(self, OperandX64::reg(tmp.reg), cst);
      let src2 = load(self, inst.op(1));
      emit(
        self.build_mut(),
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(tmp.reg),
        src2,
      );
    } else {
      let src1 = OperandX64::reg(self.reg_op(inst.op(0)));
      let src2 = load(self, inst.op(1));
      emit(self.build_mut(), OperandX64::reg(inst.reg_x64), src1, src2);
    }
  }

  /// 标量双精度（sd）四则/极值骨架：Num 族八臂共享 lower_scalar_arith 的 sd 形态。
  fn lower_num_arith(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64, OperandX64),
  ) {
    self.lower_scalar_arith(
      inst,
      index,
      Self::mem_reg_double_op,
      Self::emit_vmovsd_operand_x_64_operand_x_64,
      emit,
    );
  }

  /// 标量单精度（ss）四则/极值骨架：Float 族八臂共享 lower_scalar_arith 的 ss 形态。
  fn lower_float_arith(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64, OperandX64),
  ) {
    self.lower_scalar_arith(
      inst,
      index,
      Self::mem_reg_float_op,
      Self::emit_vmovss_operand_x_64_operand_x_64,
      emit,
    );
  }

  /// XMM 一元同构 lowering 骨架（Sqrt 的 Num/Float 侧与 FloatToNum/NumToFloat 共享）：
  /// 结果寄存器复用 op0，源经 `load` 装载后发射 `emit(reg, reg, src)` 三目指令；
  /// 装载与发射顺序与展开版逐位一致。
  fn lower_unary_xmm(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    load: fn(&mut Self, IrOp) -> OperandX64,
    emit: fn(&mut Self, OperandX64, OperandX64, OperandX64),
  ) {
    inst.reg_x64 = self
      .regs
      .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

    let src = load(self, inst.op(0));
    emit(
      self,
      OperandX64::reg(inst.reg_x64),
      OperandX64::reg(inst.reg_x64),
      src,
    );
  }

  /// XMM packed 向量四则同构 lowering 骨架（AddVec/SubVec/MulVec/DivVec/IdivVec 共享）：
  /// 两源操作数经 vec_op 装载（相同操作数只装载一份），再经 `emit` 发射对应 packed 指令。
  fn lower_vec_arith(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64, OperandX64),
  ) {
    inst.reg_x64 = self
      .regs
      .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0), inst.op(1)]);

    let mut tmp1 = self.scoped_reg();
    let mut tmp2 = self.scoped_reg();

    let op0 = inst.op(0);
    let op1 = inst.op(1);
    let tmpa = self.vec_op(op0, &mut tmp1);
    let tmpb = if op0 == op1 {
      tmpa
    } else {
      self.vec_op(op1, &mut tmp2)
    };

    emit(
      self.build_mut(),
      OperandX64::reg(inst.reg_x64),
      OperandX64::reg(tmpa),
      OperandX64::reg(tmpb),
    );
  }

  /// Uint 位运算 op0 装载骨架（Bitand/Bitxor/Bitor/Bitnot 与各移位臂共享）：
  /// 结果寄存器可复用 op0，否则先从内存搬运；发射顺序与展开版逐位一致。
  fn lower_uint_load(&mut self, inst: &mut IrInst, index: u32) {
    inst.reg_x64 = self
      .regs
      .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0)]);
    let op0 = inst.op(0);

    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
      let src = self.mem_reg_uint_op(op0);
      self.emit_mov(OperandX64::reg(inst.reg_x64), src);
    }
  }

  /// GetTable/SetTable 孪生 lowering（两臂仅被调 VM helper 不同）：op(2) 为 VmReg 时
  /// 直取栈槽 TValue 地址；为常量时把 nvalue 化的 TValue 落入 data 池取址；否则不支持。
  fn lower_get_set_table(
    &mut self,
    inst: &mut IrInst,
    call: fn(&mut IrRegAllocX64, &mut AssemblyBuilderX64, i32, OperandX64, i32),
  ) {
    if inst.op(2).kind() == IrOpKind::VmReg {
      // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
      // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
      let (build, regs) = self.build_regs_mut();
      call(
        regs,
        build,
        vm_reg_op(inst.op(1)),
        luau_reg_address(vm_reg_op(inst.op(2))),
        vm_reg_op(inst.op(0)),
      );
    } else if inst.op(2).kind() == IrOpKind::Constant {
      let n = nvalue(self.uint_op(inst.op(2)) as f64);
      let key = self
        .build_mut()
        .bytes(ptr::from_ref(&n).cast::<c_void>(), size_of::<TValue>(), 8);
      let (build, regs) = self.build_regs_mut();
      call(
        regs,
        build,
        vm_reg_op(inst.op(1)),
        key,
        vm_reg_op(inst.op(0)),
      );
    } else {
      unsupported_instruction_form();
    }
  }

  /// FallbackXxx 八站同构骨架：pcpos 取 op(0)，混窗实参列共享，仅 NativeContext
  /// helper 槽位 `off` 不同；各臂断言与尾随动作（如 Forgprep 的跳转）留在调用臂。
  fn lower_fallback(&mut self, inst: &mut IrInst, off: usize) {
    let pcpos = self.uint_op(inst.op(0)) as i32;
    // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
    // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
    let (build, regs) = self.build_regs_mut();
    emit_fallback(regs, build, off as i32, pcpos);
  }

  /// GetTypeof/FINDUPVAL 孪生：state 加 op(0) 栈地址单实参调 NativeContext helper，
  /// 结果取 RAX；两臂仅 helper 槽位 `off` 不同。
  fn lower_state_reg_helper(&mut self, inst: &mut IrInst, index: u32, off: usize) {
    let mut call_wrap = self.call_wrap_state(index);
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      luau_reg_address(vm_reg_op(inst.op(0))),
      IrOp::default(),
    );
    call_wrap.call(&OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      R_NATIVE_CONTEXT,
      off as i32,
    ));

    inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
  }

  /// LoadTag/LoadPointer 孪生：装载 TValue 的 tt(Dword)/value(Qword) 字段；VmReg/
  /// VmConst 走 rBase/rConstants 槽地址，Inst 视为 TValue 指针按 `off` 取字段内存；
  /// 语句序列与展开版逐条一致。
  fn lower_load_tv_field(&mut self, inst: &mut IrInst, index: u32, tag: bool) {
    let (size, off) = if tag {
      (SizeX64::Dword, offset_of!(TValue, tt) as i32)
    } else {
      (SizeX64::Qword, offset_of!(TValue, value) as i32)
    };
    inst.reg_x64 = self.regs.alloc_reg(size, index);

    if inst.op(0).kind() == IrOpKind::VmReg {
      self.build_mut().mov(
        OperandX64::reg(inst.reg_x64),
        tv_slot(size, R_BASE, vm_reg_op(inst.op(0)), off),
      );
    } else if inst.op(0).kind() == IrOpKind::VmConst {
      self.build_mut().mov(
        OperandX64::reg(inst.reg_x64),
        tv_slot(size, R_CONSTANTS, vm_const_op(inst.op(0)), off),
      );
    }
    // 若拿到的是寄存器，则假定它是指向 TValue 的指针
    // 将来或引入显式操作数类型以增强稳健性
    else if inst.op(0).kind() == IrOpKind::Inst {
      let src = self.reg_op(inst.op(0));
      self.emit_mov(
        OperandX64::reg(inst.reg_x64),
        OperandX64::mem(size, RegisterX64::NOREG, 1, src, off),
      );
    } else {
      unsupported_instruction_form();
    }
  }

  /// LoadDouble/LoadInt64/LoadFloat 三胞胎：按臂选寄存器尺度/搬移指令/内存尺度与
  /// 栈(rBase)/常量(rConstants)槽内 value 域偏移（Int64 常量路径按 cpp 原样取 value
  /// 首址；Float 偏移为装载偏移 op(1)）；tv_slot 与 luau_reg_value 族逐位同构，
  /// 发射序与展开版逐条一致。
  fn lower_load_scalar(&mut self, inst: &mut IrInst, index: u32) {
    // 搬移指令按臂：Double=vmovsd、Int64=mov、Float=vmovss
    let mv: fn(&mut Self, OperandX64, OperandX64) = match inst.cmd {
      IrCmd::LoadInt64 => Self::emit_mov,
      IrCmd::LoadFloat => Self::emit_vmovss_operand_x_64_operand_x_64,
      _ => Self::emit_vmovsd_operand_x_64_operand_x_64,
    };
    // (寄存器尺度, 内存尺度, 栈槽/常量槽的 value 域偏移)
    let (size, msize, roff, coff): (SizeX64, SizeX64, i32, i32) = match inst.cmd {
      IrCmd::LoadInt64 => (SizeX64::Qword, SizeX64::Qword, 8, 0),
      IrCmd::LoadFloat => {
        let off = self.int_op(inst.op(1));
        (SizeX64::Xmmword, SizeX64::Dword, off, off)
      }
      _ => (SizeX64::Xmmword, SizeX64::Qword, 0, 0),
    };
    inst.reg_x64 = self.regs.alloc_reg(size, index);

    if inst.op(0).kind() == IrOpKind::VmReg {
      mv(
        self,
        OperandX64::reg(inst.reg_x64),
        tv_slot(msize, R_BASE, vm_reg_op(inst.op(0)), roff),
      );
    } else if inst.op(0).kind() == IrOpKind::VmConst {
      mv(
        self,
        OperandX64::reg(inst.reg_x64),
        tv_slot(msize, R_CONSTANTS, vm_const_op(inst.op(0)), coff),
      );
    } else {
      unsupported_instruction_form();
    }
  }

  /// StoreTag/StoreExtra 孪生：把常量写进 TValue 的 tt/extra(Dword) 字段；Inst 视为
  /// TValue 指针按 `off` 取字段内存，否则走 rBase 栈槽地址；立即数读取器按臂分派。
  fn lower_store_tv_field(&mut self, inst: &mut IrInst, tag: bool) {
    let off = if tag {
      offset_of!(TValue, tt) as i32
    } else {
      offset_of!(TValue, extra) as i32
    };
    let imm = |s: &Self| {
      if tag {
        s.tag_op(inst.op(1)) as i32
      } else {
        s.int_op(inst.op(1))
      }
    };

    if inst.op(1).kind() == IrOpKind::Constant {
      if inst.op(0).kind() == IrOpKind::Inst {
        let src = self.reg_op(inst.op(0));
        self.emit_mov(
          OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 1, src, off),
          OperandX64::imm(imm(self)),
        );
      } else {
        self.emit_mov(
          tv_slot(SizeX64::Dword, R_BASE, vm_reg_op(inst.op(0)), off),
          OperandX64::imm(imm(self)),
        );
      }
    } else {
      unsupported_instruction_form();
    }
  }

  /// JumpCmpNum/JumpCmpFloat 孪生：浮点比较条件跳转，两臂仅源装载器与
  /// floatPrecision 旗标不同；求值序与展开版逐条一致。
  fn lower_jump_cmp_fp(&mut self, inst: &mut IrInst, next: &IrBlock, float_precision: bool) {
    let load = if float_precision {
      Self::mem_reg_float_op
    } else {
      Self::mem_reg_double_op
    };

    let cond = condition_op(inst.op(2));

    let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);

    let lhs = load(self, inst.op(0));
    let rhs = load(self, inst.op(1));
    self.with_op_label(inst.op(3), |s, l| {
      jump_on_number_cmp(s.build_mut(), tmp.reg, lhs, rhs, cond, l, float_precision);
    });
    self.jump_or_fallthrough_op(inst.op(4), next);
  }

  /// CheckReadonly/CheckNoMetatable 孪生：测 LuaTable 的 readonly(Byte)/
  /// metatable(Qword) 字段非零则按条件跳转退出，两臂仅字段尺度与槽位偏移不同。
  fn lower_check_table_field(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    next: &IrBlock,
    readonly: bool,
  ) {
    let (size, off) = if readonly {
      (SizeX64::Byte, offset_of!(LuaTable, readonly) as i32)
    } else {
      (SizeX64::Qword, offset_of!(LuaTable, metatable) as i32)
    };
    let src = self.reg_op(inst.op(0));
    self.emit_cmp(
      OperandX64::mem(size, RegisterX64::NOREG, 1, src, off),
      OperandX64::imm(0_i32),
    );
    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
      ConditionX64::NotEqual,
      inst.op(1),
      index,
      next,
    );
  }

  /// UintToNum/UintToFloat 孪生：AVX 无 uint->fp 转换；源必来自 UINT op，它们都清了
  /// 高 32 位，故通常可用 64 位寄存器直接转；唯一例外 NumToUint 不清高位，须先经
  /// Dword 临时寄存器截断。发射器按臂在 vcvtsi2sd/vcvtsi2ss 间分派。
  fn lower_uint_to_fp(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut Self, OperandX64, OperandX64, OperandX64),
  ) {
    inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

    let source = self.function_mut().inst_op(inst.op(0)).cmd;
    if source == IrCmd::NumToUint {
      let tmp = self.alloc_scoped_reg(SizeX64::Dword);
      let src = self.reg_op(inst.op(0));
      self.emit_mov(OperandX64::reg(tmp.reg), OperandX64::reg(src));
      emit(
        self,
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(qword_reg(tmp.reg)),
      );
    } else {
      CODEGEN_ASSERT!(source != IrCmd::SUBSTITUTE); // we don't process substitutions
      let src = self.reg_op(inst.op(0));
      emit(
        self,
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(qword_reg(src)),
      );
    }
  }

  /// BufferReadf32/f64/i64 三胞胎：按缓冲地址取标量装载到新寄存器；寄存器尺度、
  /// 内存尺度与装载指令按臂分派，求值序与展开版逐条一致。
  fn lower_buffer_read_scalar(&mut self, inst: &mut IrInst, index: u32) {
    let (rsize, msize, load): (SizeX64, SizeX64, fn(&mut Self, OperandX64, OperandX64)) =
      match inst.cmd {
        IrCmd::BufferReadf32 => (
          SizeX64::Xmmword,
          SizeX64::Dword,
          Self::emit_vmovss_operand_x_64_operand_x_64,
        ),
        IrCmd::BufferReadf64 => (
          SizeX64::Xmmword,
          SizeX64::Qword,
          Self::emit_vmovsd_operand_x_64_operand_x_64,
        ),
        _ => (SizeX64::Qword, SizeX64::Qword, Self::emit_mov),
      };
    inst.reg_x64 = self.regs.alloc_reg(rsize, index);

    let addr = self.buffer_addr_op(inst.op(0), inst.op(1), self.tag_op(inst.op(2)));
    load(self, OperandX64::reg(inst.reg_x64), sized_mem(addr, msize));
  }

  /// Bitcountlz/Bitcountrz 四臂共享（Uint/Int64 孪生对）：零值测试分支与零路径
  /// 置 32/64 共享；非零路径 bsr+xor(31/63)（前导零）与 bsf（尾随零）差异按 `lz`
  /// 分派，寄存器尺度与位宽常量按 `w64` 分派。
  fn lower_bit_count(&mut self, inst: &mut IrInst, index: u32, lz: bool, w64: bool) {
    // Dword/31/32 与 Qword/63/64 的尺度三元组
    let (size, top_mask, width) = if w64 {
      (SizeX64::Qword, 0x3f, 64)
    } else {
      (SizeX64::Dword, 0x1f, 32)
    };
    inst.reg_x64 = self.regs.alloc_reg_or_reuse(size, index, &[inst.op(0)]);

    let mut zero = Label::default();
    let mut exit = Label::default();

    let a = self.reg_op(inst.op(0));
    let b = self.reg_op(inst.op(0));
    self.emit_test(OperandX64::reg(a), OperandX64::reg(b));
    self.build_mut().jcc(ConditionX64::Equal, &mut zero);

    let src = self.reg_op(inst.op(0));
    if lz {
      self.emit_bsr(inst.reg_x64, OperandX64::reg(src));
      self
        .build_mut()
        .xor_(OperandX64::reg(inst.reg_x64), OperandX64::imm(top_mask));
    } else {
      self.emit_bsf(inst.reg_x64, OperandX64::reg(src));
    }
    self.build_mut().jmp_label(&mut exit);

    self.build_mut().set_label(&mut zero);
    self
      .build_mut()
      .mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(width));

    self.build_mut().set_label(&mut exit);
  }

  /// Uint 位逻辑二元同构骨架（Bitand/Bitxor/Bitor 共享）：op0 装载后
  /// op1 经 mem_reg_uint_op 装载，再经 `emit` 发射对应指令。
  fn lower_bit_logic_uint(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64),
  ) {
    self.lower_uint_load(inst, index);

    let src = self.mem_reg_uint_op(inst.op(1));
    emit(self.build_mut(), OperandX64::reg(inst.reg_x64), src);
  }

  /// Uint 移位/轮转同构骨架（Bitlshift/Bitrshift/Bitarshift/Bitlrotate/Bitrrotate 共享）：
  /// 移位量为常量时发射立即数形式；否则先占用 RCX 经 CL 发射，顺序与展开版逐位一致。
  fn lower_bit_shift_uint(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64),
  ) {
    let mut shift_tmp = self.scoped_reg();

    // 自定义移位量只能放进 RegisterX64::CL
    // 但当移位量不是存在 b 中的常量时就用它
    if inst.op(1).kind() != IrOpKind::Constant {
      shift_tmp.take(dword_reg(RegisterX64::RCX));
    }

    self.lower_uint_load(inst, index);

    if inst.op(1).kind() == IrOpKind::Constant {
      // 移位量是常量时，取出字节大小的移位量
      let shift = ((self.int_op(inst.op(1))) as u32) as i8;
      emit(
        self.build_mut(),
        OperandX64::reg(inst.reg_x64),
        OperandX64::imm(shift as i32),
      );
    } else {
      let src = self.mem_reg_uint_op(inst.op(1));
      self.emit_mov(OperandX64::reg(shift_tmp.reg), src);
      emit(
        self.build_mut(),
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(byte_reg(shift_tmp.reg)),
      );
    }
  }

  /// Int64 移位对偶骨架（Bitlshift/Bitrshift 共享，两臂互为精确镜像）：移位量为常量时
  /// 按符号发射——负移位即按 -amount 发射 `emit_reverse`（>63 结果为 0），主方向发射
  /// `emit_main`（>63 结果为 0）；非常量时先占用 RCX，经 `lea+cmp` 查 |amount| > 63
  /// 与符号测试分支，分别发射主/反向移位。寄存器分配与发射顺序同展开版逐位一致，
  /// (emit_main, emit_reverse) 取 (shl, shr) / (shr, shl) 即得左右移两臂。
  fn lower_bit_shift_int_64(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit_main: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64),
    emit_reverse: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64),
  ) {
    let mut shift_tmp = self.scoped_reg();

    if inst.op(1).kind() != IrOpKind::Constant {
      shift_tmp.take(RegisterX64::RCX);
    }

    inst.reg_x64 = self
      .regs
      .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);
    let op0 = inst.op(0);

    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
      let src = self.mem_reg_int_64_op(op0);
      self.emit_mov(OperandX64::reg(inst.reg_x64), src);
    }

    if inst.op(1).kind() == IrOpKind::Constant {
      let shift = self.int64_op(inst.op(1));

      if shift < 0 {
        // 负移位 = 按 -amount 反向移位
        let amount = (-shift) as u8;
        if amount > 63 {
          self
            .build_mut()
            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
        } else {
          emit_reverse(
            self.build_mut(),
            OperandX64::reg(inst.reg_x64),
            OperandX64::imm(((amount) as i8) as i32),
          );
        }
      } else if shift > 63 {
        self
          .build_mut()
          .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
      } else {
        emit_main(
          self.build_mut(),
          OperandX64::reg(inst.reg_x64),
          OperandX64::imm(((shift) as i8) as i32),
        );
      }
    } else {
      let tmp = self.alloc_scoped_reg(SizeX64::Qword);

      let mut negative = Label::default();
      let mut out_of_range = Label::default();
      let mut done = Label::default();

      let src = self.mem_reg_int_64_op(inst.op(1));
      self.emit_mov(OperandX64::reg(shift_tmp.reg), src);

      // 检查 |amount| > 63：(amount + 63) 无符号 > 126
      self.build_mut().lea_operand_x_64_operand_x_64(
        OperandX64::reg(tmp.reg),
        OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, shift_tmp.reg, 63),
      );
      self
        .build_mut()
        .cmp(OperandX64::reg(tmp.reg), OperandX64::imm(126_i32));
      self.build_mut().jcc(ConditionX64::Above, &mut out_of_range);

      // 检查 amount 符号
      self.build_mut().test(
        OperandX64::reg(shift_tmp.reg),
        OperandX64::reg(shift_tmp.reg),
      );
      self.build_mut().jcc(ConditionX64::Less, &mut negative);

      // 主方向移位
      emit_main(
        self.build_mut(),
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(byte_reg(shift_tmp.reg)),
      );
      self.build_mut().jmp_label(&mut done);

      // 按 -amount 反向移位
      self.build_mut().set_label(&mut negative);
      self.build_mut().neg(OperandX64::reg(shift_tmp.reg));
      emit_reverse(
        self.build_mut(),
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(byte_reg(shift_tmp.reg)),
      );
      self.build_mut().jmp_label(&mut done);

      self.build_mut().set_label(&mut out_of_range);
      self
        .build_mut()
        .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

      self.build_mut().set_label(&mut done);
    }
  }

  fn lower_int_64_load(&mut self, inst: &mut IrInst, index: u32) {
    inst.reg_x64 = self
      .regs
      .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);
    let op0 = inst.op(0);

    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
      let src = self.mem_reg_int_64_op(op0);
      self.emit_mov(OperandX64::reg(inst.reg_x64), src);
    }
  }

  fn lower_bit_logic_int_64(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64),
  ) {
    self.lower_int_64_load(inst, index);

    let src = self.mem_reg_int_64_op(inst.op(1));
    emit(self.build_mut(), OperandX64::reg(inst.reg_x64), src);
  }

  fn lower_bit_rotate_int_64(
    &mut self,
    inst: &mut IrInst,
    index: u32,
    emit: fn(&mut AssemblyBuilderX64, OperandX64, OperandX64),
  ) {
    let mut shift_tmp = self.scoped_reg();

    if inst.op(1).kind() != IrOpKind::Constant {
      shift_tmp.take(RegisterX64::RCX);
    }

    self.lower_int_64_load(inst, index);

    if inst.op(1).kind() == IrOpKind::Constant {
      let shift = ((self.int64_op(inst.op(1))) as u32) as i8;
      emit(
        self.build_mut(),
        OperandX64::reg(inst.reg_x64),
        OperandX64::imm(shift as i32),
      );
    } else {
      let src = self.mem_reg_int_64_op(inst.op(1));
      self.emit_mov(OperandX64::reg(shift_tmp.reg), src);
      emit(
        self.build_mut(),
        OperandX64::reg(inst.reg_x64),
        OperandX64::reg(byte_reg(shift_tmp.reg)),
      );
    }
  }

  /// CheckCmpInt/CheckCmpInt64 孪生：尺度（Dword/Qword）、零常量测试取值（int/int64）
  /// 与装载器（mem_reg_int_op/mem_reg_int_64_op）差异按 `w64` 分派；三分支
  /// （零测试 / 常量装载比较 / 通用比较）语句序列与展开版逐位一致。
  fn lower_check_cmp_int(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock, w64: bool) {
    let ld: fn(&mut Self, IrOp) -> OperandX64 = if w64 {
      Self::mem_reg_int_64_op
    } else {
      Self::mem_reg_int_op
    };
    let cond = condition_op(inst.op(2));

    if (matches!(cond, IrCondition::Equal | IrCondition::NotEqual))
      && inst.op(1).kind() == IrOpKind::Constant
      && (if w64 {
        self.int64_op(inst.op(1)) == 0
      } else {
        self.int_op(inst.op(1)) == 0
      })
    {
      let hoist_a = self.reg_op(inst.op(0));
      let hoist_b = self.reg_op(inst.op(0));
      self.emit_test(OperandX64::reg(hoist_a), OperandX64::reg(hoist_b));
      self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
        if cond == IrCondition::Equal {
          ConditionX64::NotZero
        } else {
          ConditionX64::Zero
        },
        inst.op(3),
        index,
        next,
      );
    } else if inst.op(0).kind() == IrOpKind::Constant {
      let tmp = self.alloc_scoped_reg(if w64 { SizeX64::Qword } else { SizeX64::Dword });
      let hoist_0 = ld(self, inst.op(0));
      self.emit_mov(OperandX64::reg(tmp.reg), hoist_0);
      let hoist_1 = ld(self, inst.op(1));
      self.emit_cmp(OperandX64::reg(tmp.reg), hoist_1);
      self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
        get_condition_int(get_negated_condition_ir_condition(cond)),
        inst.op(3),
        index,
        next,
      );
    } else {
      let hoist_2 = self.reg_op(inst.op(0));
      let hoist_3 = ld(self, inst.op(1));
      self.emit_cmp(OperandX64::reg(hoist_2), hoist_3);
      self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
        get_condition_int(get_negated_condition_ir_condition(cond)),
        inst.op(3),
        index,
        next,
      );
    }
  }

  fn lower_buffer_read_int(&mut self, inst: &mut IrInst, index: u32, size: SizeX64, signed: bool) {
    inst.reg_x64 = self
      .regs
      .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0), inst.op(1)]);

    let hoist = self.buffer_addr_op(inst.op(0), inst.op(1), self.tag_op(inst.op(2)));
    let mem = sized_mem(hoist, size);
    match size {
      SizeX64::Dword => self.emit_mov(OperandX64::reg(inst.reg_x64), mem),
      _ if signed => self.emit_movsx(inst.reg_x64, mem),
      _ => self.emit_movzx(inst.reg_x64, mem),
    }
  }

  fn lower_buffer_write_int(&mut self, inst: &mut IrInst, size: SizeX64) {
    let value = if inst.op(2).kind() == IrOpKind::Inst {
      let reg = self.reg_op(inst.op(2));
      OperandX64::reg(match size {
        SizeX64::Byte => byte_reg(reg),
        SizeX64::Word => word_reg(reg),
        _ => reg,
      })
    } else {
      let int_val = self.int_op(inst.op(2));
      OperandX64::imm(match size {
        SizeX64::Byte => int_val as i8 as i32,
        SizeX64::Word => int_val as i16 as i32,
        _ => int_val,
      })
    };

    let hoist = self.buffer_addr_op(inst.op(0), inst.op(1), self.tag_op(inst.op(3)));
    self.emit_mov(sized_mem(hoist, size), value);
  }

  /// BufferWritef64/i64 孪生：常量先落对应尺度临时寄存器再 str 入缓冲，寄存器
  /// 实例直存；临时尺度/常量发射/搬移指令按臂分派，发射序与展开版逐条一致。
  fn lower_buffer_write_scalar(&mut self, inst: &mut IrInst) {
    let f64_form = inst.cmd == IrCmd::BufferWritef64;
    let mv: fn(&mut Self, OperandX64, OperandX64) = if f64_form {
      Self::emit_vmovsd_operand_x_64_operand_x_64
    } else {
      Self::emit_mov
    };

    if inst.op(2).kind() == IrOpKind::Constant {
      let tmp = self.alloc_scoped_reg(if f64_form {
        SizeX64::Xmmword
      } else {
        SizeX64::Qword
      });
      let hoist = if f64_form {
        self.emit_f64(self.double_op(inst.op(2)))
      } else {
        self.emit_i64(self.int64_op(inst.op(2)))
      };
      mv(self, OperandX64::reg(tmp.reg), hoist);

      let addr = self.buffer_addr_op(inst.op(0), inst.op(1), self.tag_op(inst.op(3)));
      mv(
        self,
        sized_mem(addr, SizeX64::Qword),
        OperandX64::reg(tmp.reg),
      );
    } else if inst.op(2).kind() == IrOpKind::Inst {
      let addr = self.buffer_addr_op(inst.op(0), inst.op(1), self.tag_op(inst.op(3)));
      let src = self.reg_op(inst.op(2));
      mv(self, sized_mem(addr, SizeX64::Qword), OperandX64::reg(src));
    } else {
      unsupported_instruction_form();
    }
  }

  pub fn lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
    // 本函数是安全的：对 build/function/helpers/label 裸指针的每次解引用都收在
    // `records/ir_lowering_x_64.rs` 的视图/闭包窗门面（`build_mut`/`with_op_label`/
    // `with_target_label`/`with_helper_label` 等，各带 Safety 契约）内，
    // 本巨文件内不再出现 unsafe 与裸指针解引用。
    self.regs.curr_inst_idx = index;

    self.value_tracker.before_inst_lowering(inst);
    match inst.cmd {
      IrCmd::LoadTag => self.lower_load_tv_field(inst, index, true),
      IrCmd::LoadPointer => self.lower_load_tv_field(inst, index, false),
      IrCmd::LoadDouble => self.lower_load_scalar(inst, index),
      IrCmd::LoadInt => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

        self.build_mut().mov(
          OperandX64::reg(inst.reg_x64),
          luau_reg_value_int(vm_reg_op(inst.op(0))),
        );
      }
      IrCmd::LoadInt64 => self.lower_load_scalar(inst, index),
      IrCmd::LoadFloat => self.lower_load_scalar(inst, index),
      IrCmd::LoadTvalue => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

        let addr_offset = if HAS_OP_B!(inst) {
          self.int_op(inst.op(1))
        } else {
          0
        };

        if inst.op(0).kind() == IrOpKind::VmReg {
          self.build_mut().vmovups(
            OperandX64::reg(inst.reg_x64),
            luau_reg(vm_reg_op(inst.op(0))),
          );
        } else if inst.op(0).kind() == IrOpKind::VmConst {
          self.build_mut().vmovups(
            OperandX64::reg(inst.reg_x64),
            luau_constant(vm_const_op(inst.op(0))),
          );
        } else if inst.op(0).kind() == IrOpKind::Inst {
          let hoist_2 = self.reg_op(inst.op(0));
          self.emit_vmovups(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Xmmword,
              RegisterX64::NOREG,
              1,
              hoist_2,
              addr_offset,
            ),
          );
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::LoadEnv => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

        self
          .build_mut()
          .mov(OperandX64::reg(inst.reg_x64), s_closure());
        self.build_mut().mov(
          OperandX64::reg(inst.reg_x64),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            inst.reg_x64,
            (offset_of!(Closure, env) as i32),
          ),
        );
      }
      IrCmd::GetArrAddr => {
        if inst.op(1).kind() == IrOpKind::Inst {
          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(1)]);

          if dword_reg(inst.reg_x64) != self.reg_op(inst.op(1)) {
            let hoist_3 = self.reg_op(inst.op(1));
            self.emit_mov(
              OperandX64::reg(dword_reg(inst.reg_x64)),
              OperandX64::reg(hoist_3),
            );
          }

          self.build_mut().shl(
            OperandX64::reg(dword_reg(inst.reg_x64)),
            OperandX64::imm(K_TVALUE_SIZE_LOG2),
          );
          let hoist_4 = self.reg_op(inst.op(0));
          self.emit_add(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              hoist_4,
              (offset_of!(LuaTable, array) as i32),
            ),
          );
        } else if inst.op(1).kind() == IrOpKind::Constant {
          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);

          let hoist_5 = self.reg_op(inst.op(0));
          self.emit_mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              hoist_5,
              (offset_of!(LuaTable, array) as i32),
            ),
          );

          if self.int_op(inst.op(1)) != 0 {
            self.emit_lea_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::None,
                RegisterX64::NOREG,
                1,
                inst.reg_x64,
                // cpp `intOp(OP_B(inst)) * sizeof(TValue)`：int 提升为 size_t 无符号环绕，
                // 位移截断到 32 位，与 i32 wrapping 乘法低 32 位一致
                self
                  .int_op(inst.op(1))
                  .wrapping_mul(size_of::<TValue>() as i32),
              ),
            );
          }
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::GetSlotNodeAddr => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

        let tmp = self.alloc_scoped_reg(SizeX64::Qword);

        // 操作数取值均为 function 只读视图（reg_op/uint_op），先拷取即释，再经
        // `build_mut` 门面单点派生唯一可变借用，与化前实参求值序效应等价。
        let hoist_node_reg = self.reg_op(inst.op(0));
        let hoist_node_hash = (self.uint_op(inst.op(1))) as i32;
        get_table_node_at_cached_slot(
          self.build_mut(),
          tmp.reg,
          inst.reg_x64,
          hoist_node_reg,
          hoist_node_hash,
        );
      }
      IrCmd::GetHashNodeAddr => {
        {
          // 自定义移位量只能放进 RegisterX64::CL
          // cpp: `ScopedRegX64 shiftTmp{regs, regs.takeReg(rcx, kInvalidInstIdx)};`（IrLoweringX64.cpp:179）
          let mut shift_tmp = self.scoped_reg();
          shift_tmp.take(RegisterX64::RCX);

          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          let tmp = self.alloc_scoped_reg(SizeX64::Qword);

          let hoist_6 = self.reg_op(inst.op(0));
          self.emit_mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              hoist_6,
              (offset_of!(LuaTable, node) as i32),
            ),
          );
          self
            .build_mut()
            .mov(OperandX64::reg(dword_reg(tmp.reg)), OperandX64::imm(1_i32));
          let hoist_7 = self.reg_op(inst.op(0));
          self.emit_mov(
            OperandX64::reg(byte_reg(shift_tmp.reg)),
            OperandX64::mem(
              SizeX64::Byte,
              RegisterX64::NOREG,
              1,
              hoist_7,
              (offset_of!(LuaTable, lsizenode) as i32),
            ),
          );
          self.build_mut().shl(
            OperandX64::reg(dword_reg(tmp.reg)),
            OperandX64::reg(byte_reg(shift_tmp.reg)),
          );
          let imm = self.uint_op(inst.op(1)) as i32;
          self.build_mut().dec(OperandX64::reg(dword_reg(tmp.reg)));
          self
            .build_mut()
            .and_(OperandX64::reg(dword_reg(tmp.reg)), OperandX64::imm(imm));
          self.build_mut().shl(
            OperandX64::reg(tmp.reg),
            OperandX64::imm(K_LUA_NODE_SIZE_LOG2),
          );
          self
            .build_mut()
            .add(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
        };
      }
      IrCmd::GetClosureUpvalAddr => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);

        if inst.op(0).kind() == IrOpKind::Undef {
          self
            .build_mut()
            .mov(OperandX64::reg(inst.reg_x64), s_closure());
        } else {
          let cl = self.reg_op(inst.op(0));
          if inst.reg_x64 != cl {
            self
              .build_mut()
              .mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(cl));
          }
        }

        self.build_mut().add(
          OperandX64::reg(inst.reg_x64),
          OperandX64::imm(
            (offset_of!(Closure, inner) as i32)
              + (offset_of!(LClosure, uprefs) as i32)
              + (size_of::<TValue>() as i32) * vm_upvalue_op(inst.op(1)) as i32,
          ),
        );
      }
      IrCmd::StoreTag => self.lower_store_tv_field(inst, true),
      IrCmd::StorePointer => {
        let value_lhs = if inst.op(0).kind() == IrOpKind::Inst {
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            self.reg_op(inst.op(0)),
            (offset_of!(TValue, value) as i32),
          )
        } else {
          luau_reg_value(vm_reg_op(inst.op(0)))
        };

        if inst.op(1).kind() == IrOpKind::Constant {
          CODEGEN_ASSERT!(self.int_op(inst.op(1)) == 0);
          self.build_mut().mov(value_lhs, OperandX64::imm(0_i32));
        } else if inst.op(1).kind() == IrOpKind::Inst {
          let hoist_9 = self.reg_op(inst.op(1));
          self.emit_mov(value_lhs, OperandX64::reg(hoist_9));
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::StoreExtra => self.lower_store_tv_field(inst, false),
      IrCmd::StoreDouble => {
        let value_lhs = if inst.op(0).kind() == IrOpKind::Inst {
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            self.reg_op(inst.op(0)),
            (offset_of!(TValue, value) as i32),
          )
        } else {
          luau_reg_value(vm_reg_op(inst.op(0)))
        };

        if inst.op(1).kind() == IrOpKind::Constant {
          let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);

          let hoist_0 = self.emit_f64(self.double_op(inst.op(1)));
          self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp.reg), hoist_0);
          self
            .build_mut()
            .vmovsd_operand_x_64_operand_x_64(value_lhs, OperandX64::reg(tmp.reg));
        } else if inst.op(1).kind() == IrOpKind::Inst {
          let hoist_11 = self.reg_op(inst.op(1));
          self.emit_vmovsd_operand_x_64_operand_x_64(value_lhs, OperandX64::reg(hoist_11));
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::StoreInt => {
        if inst.op(1).kind() == IrOpKind::Constant {
          self.emit_mov(
            luau_reg_value_int(vm_reg_op(inst.op(0))),
            OperandX64::imm(self.int_op(inst.op(1))),
          );
        } else if inst.op(1).kind() == IrOpKind::Inst {
          let hoist_12 = self.reg_op(inst.op(1));
          self.emit_mov(
            luau_reg_value_int(vm_reg_op(inst.op(0))),
            OperandX64::reg(hoist_12),
          );
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::StoreInt64 => {
        if inst.op(1).kind() == IrOpKind::Constant {
          let value = self.int64_op(inst.op(1));

          // x64 的 mov r/m64, imm32 会符号扩展
          // 范围外的值改用寄存器
          if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            self.build_mut().mov(
              luau_reg_value_int_64(vm_reg_op(inst.op(0))),
              OperandX64::imm((value) as i32),
            );
          } else {
            let tmp = self.alloc_scoped_reg(SizeX64::Qword);
            self.build_mut().mov64(tmp.reg, value);
            self.build_mut().mov(
              luau_reg_value_int_64(vm_reg_op(inst.op(0))),
              OperandX64::reg(tmp.reg),
            );
          }
        } else if inst.op(1).kind() == IrOpKind::Inst {
          let hoist_13 = self.reg_op(inst.op(1));
          self.emit_mov(
            luau_reg_value_int_64(vm_reg_op(inst.op(0))),
            OperandX64::reg(hoist_13),
          );
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::StoreVector => {
        self.store_float(luau_reg_value_vector(vm_reg_op(inst.op(0)), 0), inst.op(1));
        self.store_float(luau_reg_value_vector(vm_reg_op(inst.op(0)), 1), inst.op(2));
        self.store_float(luau_reg_value_vector(vm_reg_op(inst.op(0)), 2), inst.op(3));

        if HAS_OP_E!(inst) {
          self.emit_mov(
            luau_reg_tag(vm_reg_op(inst.op(0))),
            OperandX64::imm((self.tag_op(inst.op(4))) as i32),
          );
        }
      }
      IrCmd::StoreTvalue => {
        let addr_offset = if HAS_OP_C!(inst) {
          self.int_op(inst.op(2))
        } else {
          0
        };

        if inst.op(0).kind() == IrOpKind::VmReg {
          let hoist_14 = self.reg_op(inst.op(1));
          self.emit_vmovups(luau_reg(vm_reg_op(inst.op(0))), OperandX64::reg(hoist_14));
        } else if inst.op(0).kind() == IrOpKind::Inst {
          let hoist_15 = self.reg_op(inst.op(0));
          let hoist_16 = self.reg_op(inst.op(1));
          self.emit_vmovups(
            OperandX64::mem(
              SizeX64::Xmmword,
              RegisterX64::NOREG,
              1,
              hoist_15,
              addr_offset,
            ),
            OperandX64::reg(hoist_16),
          );
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::StoreSplitTvalue => {
        {
          let addr_offset = if HAS_OP_D!(inst) {
            self.int_op(inst.op(3))
          } else {
            0
          };

          let tag_lhs = if inst.op(0).kind() == IrOpKind::Inst {
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              self.reg_op(inst.op(0)),
              (offset_of!(TValue, tt) as i32) + addr_offset,
            )
          } else {
            luau_reg_tag(vm_reg_op(inst.op(0)))
          };
          self.emit_mov(tag_lhs, OperandX64::imm((self.tag_op(inst.op(1))) as i32));

          if self.tag_op(inst.op(1)) == LuaType::Boolean as u8 {
            let value_lhs = if inst.op(0).kind() == IrOpKind::Inst {
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                self.reg_op(inst.op(0)),
                (offset_of!(TValue, value) as i32) + addr_offset,
              )
            } else {
              luau_reg_value_int(vm_reg_op(inst.op(0)))
            };
            // 操作数 2 是常量时只能走立即数，此时不可 regOp：它对非 Inst 操作数直接断言，
            // 故取值必须留在分支内惰性进行（先前无条件 hoist 的写法在 COVERAGE 常量填充下即炸）。
            let op2 = inst.op(2);
            let rhs = if op2.kind() == IrOpKind::Constant {
              OperandX64::imm(self.int_op(op2))
            } else {
              OperandX64::reg(self.reg_op(op2))
            };
            self.emit_mov(value_lhs, rhs);
          } else if self.tag_op(inst.op(1)) == LuaType::Number as u8 {
            let value_lhs = if inst.op(0).kind() == IrOpKind::Inst {
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(inst.op(0)),
                (offset_of!(TValue, value) as i32) + addr_offset,
              )
            } else {
              luau_reg_value(vm_reg_op(inst.op(0)))
            };

            if inst.op(2).kind() == IrOpKind::Constant {
              let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);

              let hoist_1 = self.emit_f64(self.double_op(inst.op(2)));
              self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp.reg), hoist_1);
              self
                .build_mut()
                .vmovsd_operand_x_64_operand_x_64(value_lhs, OperandX64::reg(tmp.reg));
            } else {
              let hoist_18 = self.reg_op(inst.op(2));
              self.emit_vmovsd_operand_x_64_operand_x_64(value_lhs, OperandX64::reg(hoist_18));
            }
          } else if self.tag_op(inst.op(1)) == LuaType::Integer as u8 {
            let value_lhs = if inst.op(0).kind() == IrOpKind::Inst {
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(inst.op(0)),
                (offset_of!(TValue, value) as i32) + addr_offset,
              )
            } else {
              luau_reg_value_int_64(vm_reg_op(inst.op(0)))
            };

            if inst.op(2).kind() == IrOpKind::Constant {
              let value = self.int64_op(inst.op(2));

              // x64 的 mov r/m64, imm32 会符号扩展
              if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                self
                  .build_mut()
                  .mov(value_lhs, OperandX64::imm((value) as i32));
              } else {
                let tmp = self.alloc_scoped_reg(SizeX64::Qword);
                self.build_mut().mov64(tmp.reg, value);
                self.build_mut().mov(value_lhs, OperandX64::reg(tmp.reg));
              }
            } else {
              let hoist_19 = self.reg_op(inst.op(2));
              self.emit_mov(value_lhs, OperandX64::reg(hoist_19));
            }
          } else if is_gco(self.tag_op(inst.op(1))) {
            let value_lhs = if inst.op(0).kind() == IrOpKind::Inst {
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                self.reg_op(inst.op(0)),
                (offset_of!(TValue, value) as i32) + addr_offset,
              )
            } else {
              luau_reg_value(vm_reg_op(inst.op(0)))
            };
            let hoist_20 = self.reg_op(inst.op(2));
            self.emit_mov(value_lhs, OperandX64::reg(hoist_20));
          } else {
            unsupported_instruction_form();
          }
        }
      }
      IrCmd::AddInt => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0)]);
        let op0 = inst.op(0);
        let op1 = inst.op(1);

        if op0.kind() == IrOpKind::Constant {
          let hoist_21 = self.reg_op(op1);
          self.emit_lea_operand_x_64_operand_x_64(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::None,
              RegisterX64::NOREG,
              1,
              hoist_21,
              self.int_op(op0),
            ),
          );
        } else if op0.kind() == IrOpKind::Inst {
          if inst.reg_x64 == self.reg_op(op0) {
            if op1.kind() == IrOpKind::Inst {
              let hoist_22 = self.reg_op(op1);
              self.emit_add(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_22));
            } else if self.int_op(op1) == 1 {
              self.build_mut().inc(OperandX64::reg(inst.reg_x64));
            } else {
              self.emit_add(
                OperandX64::reg(inst.reg_x64),
                OperandX64::imm(self.int_op(op1)),
              );
            }
          } else {
            if op1.kind() == IrOpKind::Inst {
              let hoist_23 = self.reg_op(op1);
              let hoist_24 = self.reg_op(op0);
              self.emit_lea_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                OperandX64::mem(SizeX64::None, hoist_23, 1, hoist_24, 0),
              );
            } else {
              let hoist_25 = self.reg_op(op0);
              self.emit_lea_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                OperandX64::mem(
                  SizeX64::None,
                  RegisterX64::NOREG,
                  1,
                  hoist_25,
                  self.int_op(op1),
                ),
              );
            }
          }
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::AddInt64 => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);
        let op0 = inst.op(0);
        let op1 = inst.op(1);

        if op0.kind() == IrOpKind::Constant {
          let value = self.int64_op(op0);

          if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            let hoist_26 = self.reg_op(op1);
            self.emit_lea_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::mem(
                SizeX64::None,
                RegisterX64::NOREG,
                1,
                hoist_26,
                (value) as i32,
              ),
            );
          } else {
            self.build_mut().mov64(inst.reg_x64, value);
            let hoist_27 = self.reg_op(op1);
            self.emit_add(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_27));
          }
        } else if op0.kind() == IrOpKind::Inst {
          if inst.reg_x64 == self.reg_op(op0) {
            if op1.kind() == IrOpKind::Inst {
              let hoist_28 = self.reg_op(op1);
              self.emit_add(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_28));
            } else if self.int64_op(op1) == 1 {
              self.build_mut().inc(OperandX64::reg(inst.reg_x64));
            } else {
              let value = self.int64_op(op1);

              if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                self.build_mut().add(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm((value) as i32),
                );
              } else {
                let tmp = self.alloc_scoped_reg(SizeX64::Qword);
                self.build_mut().mov64(tmp.reg, value);
                self
                  .build_mut()
                  .add(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
              }
            }
          } else {
            if op1.kind() == IrOpKind::Inst {
              let hoist_29 = self.reg_op(op1);
              let hoist_30 = self.reg_op(op0);
              self.emit_lea_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                OperandX64::mem(SizeX64::None, hoist_29, 1, hoist_30, 0),
              );
            } else {
              let value = self.int64_op(op1);

              if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                let hoist_31 = self.reg_op(op0);
                self.emit_lea_operand_x_64_operand_x_64(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::mem(
                    SizeX64::None,
                    RegisterX64::NOREG,
                    1,
                    hoist_31,
                    (value) as i32,
                  ),
                );
              } else {
                self.build_mut().mov64(inst.reg_x64, value);
                let hoist_32 = self.reg_op(op0);
                self.emit_add(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_32));
              }
            }
          }
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::SubInt => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0)]);
        let op0 = inst.op(0);
        let op1 = inst.op(1);

        if op0.kind() == IrOpKind::Inst {
          if op1.kind() == IrOpKind::Constant {
            if inst.reg_x64 != self.reg_op(op0) {
              let hoist_33 = self.reg_op(op0);
              self.emit_lea_operand_x_64_operand_x_64(
                OperandX64::reg(inst.reg_x64),
                OperandX64::mem(
                  SizeX64::None,
                  RegisterX64::NOREG,
                  1,
                  hoist_33,
                  -self.int_op(op1),
                ),
              );
            } else {
              self.emit_sub(
                OperandX64::reg(inst.reg_x64),
                OperandX64::imm(self.int_op(op1)),
              );
            }
          } else {
            // 若结果复用源，可原地减法，否则需先设置初值
            if inst.reg_x64 != self.reg_op(op0) {
              let hoist_34 = self.reg_op(op0);
              self.emit_mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_34));
            }

            let hoist_35 = self.reg_op(op1);
            self.emit_sub(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_35));
          }
        } else if op1.kind() == IrOpKind::Inst {
          self.emit_mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::imm(self.int_op(op0)),
          );
          let hoist_36 = self.reg_op(op1);
          self.emit_sub(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_36));
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::SubInt64 => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);
        let op0 = inst.op(0);
        let op1 = inst.op(1);

        if op0.kind() == IrOpKind::Inst {
          if op1.kind() == IrOpKind::Constant {
            let value = self.int64_op(op1);

            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
              if inst.reg_x64 != self.reg_op(op0) {
                let hoist_37 = self.reg_op(op0);
                self.emit_lea_operand_x_64_operand_x_64(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::mem(
                    SizeX64::None,
                    RegisterX64::NOREG,
                    1,
                    hoist_37,
                    -((value) as i32),
                  ),
                );
              } else {
                self.build_mut().sub(
                  OperandX64::reg(inst.reg_x64),
                  OperandX64::imm((value) as i32),
                );
              }
            } else {
              let tmp = self.alloc_scoped_reg(SizeX64::Qword);
              self.build_mut().mov64(tmp.reg, value);

              if inst.reg_x64 != self.reg_op(op0) {
                let hoist_38 = self.reg_op(op0);
                self.emit_mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_38));
              }

              self
                .build_mut()
                .sub(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
            }
          } else {
            // 若结果复用源，可原地减法，否则需先设置初值
            if inst.reg_x64 != self.reg_op(op0) {
              let hoist_39 = self.reg_op(op0);
              self.emit_mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_39));
            }

            let hoist_40 = self.reg_op(op1);
            self.emit_sub(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_40));
          }
        } else if op1.kind() == IrOpKind::Inst {
          self.emit_mov64(inst.reg_x64, self.int64_op(op0));
          let hoist_41 = self.reg_op(op1);
          self.emit_sub(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_41));
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::Sexti8Int => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0)]);

        let hoist_42 = self.reg_op(inst.op(0));
        self.emit_movsx(inst.reg_x64, OperandX64::reg(byte_reg(hoist_42)));
      }
      IrCmd::Sexti16Int => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0)]);

        let hoist_43 = self.reg_op(inst.op(0));
        self.emit_movsx(inst.reg_x64, OperandX64::reg(word_reg(hoist_43)));
      }
      IrCmd::AddNum | IrCmd::SubNum | IrCmd::MulNum | IrCmd::DivNum => {
        let emit = match inst.cmd {
          IrCmd::AddNum => AssemblyBuilderX64::vaddsd,
          IrCmd::SubNum => AssemblyBuilderX64::vsubsd,
          IrCmd::MulNum => AssemblyBuilderX64::vmulsd,
          IrCmd::DivNum => AssemblyBuilderX64::vdivsd,
          _ => unreachable!(),
        };
        self.lower_num_arith(inst, index, emit);
      }
      IrCmd::MulInt64 => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);

        let hoist_56 = self.mem_reg_int_64_op(inst.op(0));
        self.emit_mov(OperandX64::reg(inst.reg_x64), hoist_56);
        let hoist_57 = self.mem_reg_int_64_op(inst.op(1));
        self.emit_imul_operand_x_64_operand_x_64(OperandX64::reg(inst.reg_x64), hoist_57);
      }
      IrCmd::DivInt64 => {
        {
          // idiv 会破坏 RegisterX64::RAX（商）与 RegisterX64::RDX（余数）
          let mut div_rax = self.alloc_scoped_reg(SizeX64::Dword);
          let mut div_rdx = self.alloc_scoped_reg(SizeX64::Dword);
          div_rax.take(RegisterX64::RAX);
          div_rdx.take(RegisterX64::RDX);

          let hoist_62 = self.mem_reg_int_64_op(inst.op(0));
          self.emit_mov(OperandX64::reg(RegisterX64::RAX), hoist_62);
          self.build_mut().cqo(); // sign-extend RAX into RDX:RAX
          let hoist_63 = self.mem_reg_int_64_op(inst.op(1));
          self.emit_idiv(hoist_63);

          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0), inst.op(1)]);
          self.build_mut().mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(RegisterX64::RAX),
          );
        }
      }
      IrCmd::IdivNum => {
        self.lower_num_arith(inst, index, AssemblyBuilderX64::vdivsd);
        self.build_mut().vroundsd(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          RoundingModeX64::RoundToNegativeInfinity,
        );
      }
      IrCmd::IdivInt64 => {
        {
          // idiv 会破坏 RegisterX64::RAX（商）与 RegisterX64::RDX（余数）
          let mut div_rax = self.alloc_scoped_reg(SizeX64::Dword);
          div_rax.take(RegisterX64::RAX);
          let mut div_rdx = self.alloc_scoped_reg(SizeX64::Dword);
          div_rdx.take(RegisterX64::RDX);
          let temp_b = self.alloc_scoped_reg(SizeX64::Qword);

          let hoist_68 = self.mem_reg_int_64_op(inst.op(1));
          self.emit_mov(OperandX64::reg(temp_b.reg), hoist_68);

          // idiv 以 RDX:RAX 除以操作数；商在 RAX，余数在 RDX
          let hoist_69 = self.mem_reg_int_64_op(inst.op(0));
          self.emit_mov(OperandX64::reg(RegisterX64::RAX), hoist_69);
          self.build_mut().cqo(); // sign-extend RAX into RDX:RAX
          self.build_mut().idiv(OperandX64::reg(temp_b.reg));

          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0), inst.op(1)]);
          self.build_mut().mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(RegisterX64::RAX),
          ); // start with truncated quotient

          let mut done = Label::default();
          self.build_mut().test(
            OperandX64::reg(RegisterX64::RDX),
            OperandX64::reg(RegisterX64::RDX),
          );
          self.build_mut().jcc(ConditionX64::Equal, &mut done); // remainder == 0, no adjustment needed

          self.build_mut().xor_(
            OperandX64::reg(RegisterX64::RDX),
            OperandX64::reg(temp_b.reg),
          );
          self.build_mut().jcc(ConditionX64::GreaterEqual, &mut done); // same sign, no adjustment

          self
            .build_mut()
            .sub(OperandX64::reg(inst.reg_x64), OperandX64::imm(1_i32)); // floor adjustment
          self.build_mut().set_label(&mut done);
        }
      }
      IrCmd::UdivInt64 | IrCmd::UremInt64 => {
        let is_rem = inst.cmd == IrCmd::UremInt64;
        let mut div_rax = self.alloc_scoped_reg(SizeX64::Dword);
        let mut div_rdx = self.alloc_scoped_reg(SizeX64::Dword);
        div_rax.take(RegisterX64::RAX);
        div_rdx.take(RegisterX64::RDX);

        let hoist_0 = self.mem_reg_int_64_op(inst.op(0));
        self.emit_mov(OperandX64::reg(RegisterX64::RAX), hoist_0);
        self.build_mut().xor_(
          OperandX64::reg(RegisterX64::RDX),
          OperandX64::reg(RegisterX64::RDX),
        ); // zero-extend RAX into RDX:RAX
        let hoist_1 = self.mem_reg_int_64_op(inst.op(1));
        self.emit_div(hoist_1);

        inst.reg_x64 =
          self
            .regs
            .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0), inst.op(1)]);
        let src_reg = if is_rem {
          RegisterX64::RDX
        } else {
          RegisterX64::RAX
        };
        self
          .build_mut()
          .mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(src_reg));
      }
      IrCmd::RemInt64 => {
        {
          // idiv 会破坏 RegisterX64::RAX（商）与 RegisterX64::RDX（余数）
          let mut div_rax = self.alloc_scoped_reg(SizeX64::Dword);
          let mut div_rdx = self.alloc_scoped_reg(SizeX64::Dword);
          div_rax.take(RegisterX64::RAX);
          div_rdx.take(RegisterX64::RDX);
          let temp_b = self.alloc_scoped_reg(SizeX64::Qword);
          let temp_a = self.alloc_scoped_reg(SizeX64::Qword);
          let hoist_72 = self.mem_reg_int_64_op(inst.op(0));
          self.emit_mov(OperandX64::reg(temp_a.reg), hoist_72);
          let hoist_73 = self.mem_reg_int_64_op(inst.op(1));
          self.emit_mov(OperandX64::reg(temp_b.reg), hoist_73);

          // 防护 dividend == i64::MIN && divisor == -1（有符号溢出）
          // 若发生则必须返回 0
          let mut skip = Label::default();
          let mut done = Label::default();

          self
            .build_mut()
            .cmp(OperandX64::reg(temp_b.reg), OperandX64::imm(-1_i32));
          self.build_mut().jcc(ConditionX64::NotEqual, &mut skip);

          let tmp_min = self.alloc_scoped_reg(SizeX64::Qword);
          self
            .build_mut()
            .mov(OperandX64::reg(RegisterX64::RDX), OperandX64::imm(0_i32));
          self.build_mut().mov64(tmp_min.reg, i64::MIN);
          self
            .build_mut()
            .cmp(OperandX64::reg(temp_a.reg), OperandX64::reg(tmp_min.reg));
          self.build_mut().jcc(ConditionX64::Equal, &mut done);

          self.build_mut().set_label(&mut skip);

          self.build_mut().mov(
            OperandX64::reg(RegisterX64::RAX),
            OperandX64::reg(temp_a.reg),
          );
          self.build_mut().cqo(); // sign-extend RAX into RDX:RAX
          self.build_mut().idiv(OperandX64::reg(temp_b.reg));

          self.build_mut().set_label(&mut done);
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0), inst.op(1)]);
          self.build_mut().mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(RegisterX64::RDX),
          );
        }
      }

      IrCmd::MuladdNum => {
        if FeaturesX64::FeatureFma3.is_set(self.build_mut().features) {
          if inst.op(0).kind() != IrOpKind::Inst {
            inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);
            let hoist_76 = self.mem_reg_double_op(inst.op(0));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(inst.reg_x64), hoist_76);
          } else {
            inst.reg_x64 = self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);
            let a_reg = self.reg_op(inst.op(0));
            if inst.reg_x64 != a_reg {
              self
                .build_mut()
                .vmovupd(OperandX64::reg(inst.reg_x64), OperandX64::reg(a_reg));
            }
          }

          let mut opt_btmp = self.alloc_scoped_reg(SizeX64::Dword);

          let b_reg: RegisterX64 = if inst.op(1).kind() == IrOpKind::Constant {
            opt_btmp.alloc(SizeX64::Xmmword);

            let hoist_77 = self.mem_reg_double_op(inst.op(1));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(opt_btmp.reg), hoist_77);
            opt_btmp.reg
          } else {
            self.reg_op(inst.op(1))
          };

          let hoist_78 = self.mem_reg_double_op(inst.op(2));
          self.emit_vfmadd213pd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(b_reg),
            hoist_78,
          );
        } else {
          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

          if inst.op(0).kind() != IrOpKind::Inst && inst.op(1).kind() != IrOpKind::Inst {
            let hoist_79 = self.mem_reg_double_op(inst.op(0));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(inst.reg_x64), hoist_79);
            let hoist_80 = self.mem_reg_double_op(inst.op(1));
            self.emit_vmulsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              hoist_80,
            );
          } else if inst.op(0).kind() == IrOpKind::Inst {
            let hoist_81 = self.reg_op(inst.op(0));
            let hoist_82 = self.mem_reg_double_op(inst.op(1));
            self.emit_vmulsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(hoist_81),
              hoist_82,
            );
          } else {
            CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::Inst);
            let hoist_83 = self.reg_op(inst.op(1));
            let hoist_84 = self.mem_reg_double_op(inst.op(0));
            self.emit_vmulsd(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(hoist_83),
              hoist_84,
            );
          }

          let hoist_85 = self.mem_reg_double_op(inst.op(2));
          self.emit_vaddsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            hoist_85,
          );
        }
      }
      IrCmd::ModNum => {
        inst.reg_x64 =
          self
            .regs
            .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0), inst.op(1)]);

        // cpp 仅在常量分支 alloc（IrLoweringX64.cpp:850-863），此处不预先分配
        let mut opt_lhs_tmp = self.scoped_reg();

        let lhs: RegisterX64 = if inst.op(0).kind() == IrOpKind::Constant {
          opt_lhs_tmp.alloc(SizeX64::Xmmword);

          let hoist_86 = self.mem_reg_double_op(inst.op(0));
          self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(opt_lhs_tmp.reg), hoist_86);
          opt_lhs_tmp.reg
        } else {
          self.reg_op(inst.op(0))
        };

        if inst.op(1).kind() == IrOpKind::Inst {
          let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);

          let hoist_87 = self.mem_reg_double_op(inst.op(1));
          self.emit_vdivsd(OperandX64::reg(tmp.reg), OperandX64::reg(lhs), hoist_87);
          self.build_mut().vroundsd(
            OperandX64::reg(tmp.reg),
            OperandX64::reg(tmp.reg),
            OperandX64::reg(tmp.reg),
            RoundingModeX64::RoundToNegativeInfinity,
          );
          let hoist_88 = self.mem_reg_double_op(inst.op(1));
          self.emit_vmulsd(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg), hoist_88);
          self.build_mut().vsubsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(lhs),
            OperandX64::reg(tmp.reg),
          );
        } else {
          let tmp1 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp2 = self.alloc_scoped_reg(SizeX64::Xmmword);

          let hoist_89 = self.mem_reg_double_op(inst.op(1));
          self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp1.reg), hoist_89);
          self.build_mut().vdivsd(
            OperandX64::reg(tmp2.reg),
            OperandX64::reg(lhs),
            OperandX64::reg(tmp1.reg),
          );
          self.build_mut().vroundsd(
            OperandX64::reg(tmp2.reg),
            OperandX64::reg(tmp2.reg),
            OperandX64::reg(tmp2.reg),
            RoundingModeX64::RoundToNegativeInfinity,
          );
          self.build_mut().vmulsd(
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(tmp2.reg),
            OperandX64::reg(tmp1.reg),
          );
          self.build_mut().vsubsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(lhs),
            OperandX64::reg(tmp1.reg),
          );
        }
      }
      IrCmd::ModInt64 => {
        {
          // idiv 会破坏 RegisterX64::RAX（商）与 RegisterX64::RDX（余数）
          let mut div_rax = self.alloc_scoped_reg(SizeX64::Dword);
          div_rax.take(RegisterX64::RAX);
          let mut div_rdx = self.alloc_scoped_reg(SizeX64::Dword);
          div_rdx.take(RegisterX64::RDX);
          let temp_b = self.alloc_scoped_reg(SizeX64::Qword);
          let temp_a = self.alloc_scoped_reg(SizeX64::Qword);
          let hoist_90 = self.mem_reg_int_64_op(inst.op(0));
          self.emit_mov(OperandX64::reg(temp_a.reg), hoist_90);
          let hoist_91 = self.mem_reg_int_64_op(inst.op(1));
          self.emit_mov(OperandX64::reg(temp_b.reg), hoist_91);

          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0), inst.op(1)]);

          // 防护 dividend == i64::MIN && divisor == -1（有符号溢出）
          // 若发生则必须返回 0
          let mut skip = Label::default();
          let mut done = Label::default();

          self
            .build_mut()
            .cmp(OperandX64::reg(temp_b.reg), OperandX64::imm(-1_i32));
          self.build_mut().jcc(ConditionX64::NotEqual, &mut skip);

          let tmp_min = self.alloc_scoped_reg(SizeX64::Qword);
          self
            .build_mut()
            .mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(0_i32));
          self.build_mut().mov64(tmp_min.reg, i64::MIN);
          self
            .build_mut()
            .cmp(OperandX64::reg(temp_a.reg), OperandX64::reg(tmp_min.reg));
          self.build_mut().jcc(ConditionX64::Equal, &mut done);

          self.build_mut().set_label(&mut skip);

          self.build_mut().mov(
            OperandX64::reg(RegisterX64::RAX),
            OperandX64::reg(temp_a.reg),
          );
          self.build_mut().cqo();
          self.build_mut().idiv(OperandX64::reg(temp_b.reg));

          self.build_mut().mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(RegisterX64::RDX),
          );

          self.build_mut().test(
            OperandX64::reg(RegisterX64::RDX),
            OperandX64::reg(RegisterX64::RDX),
          );
          self.build_mut().jcc(ConditionX64::Equal, &mut done);

          self.build_mut().xor_(
            OperandX64::reg(RegisterX64::RDX),
            OperandX64::reg(temp_b.reg),
          );
          self.build_mut().jcc(ConditionX64::GreaterEqual, &mut done);

          self
            .build_mut()
            .add(OperandX64::reg(inst.reg_x64), OperandX64::reg(temp_b.reg));
          self.build_mut().set_label(&mut done);
        }
      }
      IrCmd::MinNum | IrCmd::MaxNum => {
        let emit = match inst.cmd {
          IrCmd::MinNum => AssemblyBuilderX64::vminsd,
          IrCmd::MaxNum => AssemblyBuilderX64::vmaxsd,
          _ => unreachable!(),
        };
        self.lower_num_arith(inst, index, emit);
      }
      IrCmd::UnmNum => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let hoist_100 = self.reg_op(inst.op(0));
        let hoist_101 = self.emit_f64(-0.0);
        self.emit_vxorpd(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(hoist_100),
          hoist_101,
        );
      }
      IrCmd::FloorNum | IrCmd::CeilNum => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let hoist_102 = self.mem_reg_double_op(inst.op(0));
        let mode = if inst.cmd == IrCmd::FloorNum {
          RoundingModeX64::RoundToNegativeInfinity
        } else {
          RoundingModeX64::RoundToPositiveInfinity
        };
        self.emit_vroundsd(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          hoist_102,
          mode,
        );
      }
      IrCmd::RoundNum => {
        {
          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

          let tmp1 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp2 = self.alloc_scoped_reg(SizeX64::Xmmword);

          if inst.op(0).kind() != IrOpKind::Inst {
            let hoist_104 = self.mem_reg_double_op(inst.op(0));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(inst.reg_x64), hoist_104);
          } else if self.reg_op(inst.op(0)) != inst.reg_x64 {
            let hoist_105 = self.reg_op(inst.op(0));
            self.emit_vmovsd_operand_x_64_operand_x_64_operand_x_64(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(hoist_105),
            );
          }

          let hoist_2 = self.emit_f64x2(-0.0, -0.0);
          self.emit_vandpd(
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(inst.reg_x64),
            hoist_2,
          );
          let hoist_3 = self.emit_i64(0x3fdfffffffffffff);
          self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp2.reg), hoist_3); // 0.49999999999999994
          self.build_mut().vorpd(
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(tmp2.reg),
          );
          self.build_mut().vaddsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmp1.reg),
          );
          self.build_mut().vroundsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            RoundingModeX64::RoundToZero,
          );
        }
      }
      IrCmd::SqrtNum => {
        self.lower_unary_xmm(inst, index, Self::mem_reg_double_op, Self::emit_vsqrtsd);
      }
      IrCmd::AbsNum => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        if inst.op(0).kind() != IrOpKind::Inst {
          let hoist_107 = self.mem_reg_double_op(inst.op(0));
          self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(inst.reg_x64), hoist_107);
        } else if self.reg_op(inst.op(0)) != inst.reg_x64 {
          let hoist_108 = self.reg_op(inst.op(0));
          self.emit_vmovsd_operand_x_64_operand_x_64_operand_x_64(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(hoist_108),
          );
        }

        let hoist_4 = self.emit_i64(!(1 << 63));
        self.emit_vandpd(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          hoist_4,
        );
      }
      IrCmd::SignNum => {
        {
          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

          let tmp0 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp1 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp2 = self.alloc_scoped_reg(SizeX64::Xmmword);

          self.build_mut().vxorpd(
            OperandX64::reg(tmp0.reg),
            OperandX64::reg(tmp0.reg),
            OperandX64::reg(tmp0.reg),
          );

          // arg < 0 时把 tmp1 置 -1，否则 0
          let hoist_109 = self.reg_op(inst.op(0));
          self.emit_vcmpltsd(
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(hoist_109),
            OperandX64::reg(tmp0.reg),
          );
          let hoist_5 = self.emit_f64(-1.0);
          self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp2.reg), hoist_5);
          self.build_mut().vandpd(
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(tmp2.reg),
          );

          // 0 < arg 时把 mask 位置 1，否则 0
          let hoist_110 = self.reg_op(inst.op(0));
          self.emit_vcmpltsd(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmp0.reg),
            OperandX64::reg(hoist_110),
          );

          // Result = if (mask-bit == 1) { 1.0 } else { tmp1
          // arg < 0 时 tmp1 为 -1、mask 位为 0 }，结果为 -1
          // arg == 0 时 tmp1 为 0、mask 位为 0，结果为 0
          // arg > 0 时 tmp1 为 0、mask 位为 1，结果为 1
          let hoist_6 = self.emit_f64x2(1.0, 1.0);
          self.emit_vblendvpd(inst.reg_x64, tmp1.reg, hoist_6, inst.reg_x64);
        }
      }
      IrCmd::AddFloat
      | IrCmd::SubFloat
      | IrCmd::MulFloat
      | IrCmd::DivFloat
      | IrCmd::MinFloat
      | IrCmd::MaxFloat => {
        let emit = match inst.cmd {
          IrCmd::AddFloat => AssemblyBuilderX64::vaddss,
          IrCmd::SubFloat => AssemblyBuilderX64::vsubss,
          IrCmd::MulFloat => AssemblyBuilderX64::vmulss,
          IrCmd::DivFloat => AssemblyBuilderX64::vdivss,
          IrCmd::MinFloat => AssemblyBuilderX64::vminss,
          IrCmd::MaxFloat => AssemblyBuilderX64::vmaxss,
          _ => unreachable!(),
        };
        self.lower_float_arith(inst, index, emit);
      }
      IrCmd::UnmFloat => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let hoist_135 = self.reg_op(inst.op(0));
        let hoist_136 = self.emit_f32(-0.0);
        self.emit_vxorps(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(hoist_135),
          hoist_136,
        );
      }
      IrCmd::FloorFloat | IrCmd::CeilFloat => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let hoist = self.mem_reg_float_op(inst.op(0));
        let mode = if inst.cmd == IrCmd::FloorFloat {
          RoundingModeX64::RoundToNegativeInfinity
        } else {
          RoundingModeX64::RoundToPositiveInfinity
        };
        self.emit_vroundss(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          hoist,
          mode,
        );
      }
      IrCmd::SqrtFloat => {
        self.lower_unary_xmm(inst, index, Self::mem_reg_float_op, Self::emit_vsqrtss);
      }
      IrCmd::AbsFloat => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        if inst.op(0).kind() != IrOpKind::Inst {
          let hoist_140 = self.mem_reg_float_op(inst.op(0));
          self.emit_vmovss_operand_x_64_operand_x_64(OperandX64::reg(inst.reg_x64), hoist_140);
        } else if self.reg_op(inst.op(0)) != inst.reg_x64 {
          let hoist_141 = self.reg_op(inst.op(0));
          self.emit_vmovss_operand_x_64_operand_x_64_operand_x_64(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(hoist_141),
          );
        }

        let hoist_7 = self.emit_i32(0x7fffffff);
        self.emit_vandps(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          hoist_7,
        );
      }
      IrCmd::SignFloat => {
        {
          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

          let tmp0 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp1 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp2 = self.alloc_scoped_reg(SizeX64::Xmmword);

          self.build_mut().vxorps(
            OperandX64::reg(tmp0.reg),
            OperandX64::reg(tmp0.reg),
            OperandX64::reg(tmp0.reg),
          );

          // arg < 0 时把 tmp1 置 -1，否则 0
          let hoist_142 = self.reg_op(inst.op(0));
          self.emit_vcmpltss(
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(hoist_142),
            OperandX64::reg(tmp0.reg),
          );
          let hoist_8 = self.emit_f32(-1.0);
          self.emit_vmovss_operand_x_64_operand_x_64(OperandX64::reg(tmp2.reg), hoist_8);
          self.build_mut().vandps(
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(tmp1.reg),
            OperandX64::reg(tmp2.reg),
          );

          // 0 < arg 时把 mask 位置 1，否则 0
          let hoist_143 = self.reg_op(inst.op(0));
          self.emit_vcmpltss(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmp0.reg),
            OperandX64::reg(hoist_143),
          );

          // Result = if (mask-bit == 1) { 1.0 } else { tmp1
          // arg < 0 时 tmp1 为 -1、mask 位为 0 }，结果为 -1
          // arg == 0 时 tmp1 为 0、mask 位为 0，结果为 0
          // arg > 0 时 tmp1 为 0、mask 位为 1，结果为 1
          let hoist_9 = self.emit_f32x4(1.0, 1.0, 1.0, 1.0);
          self.emit_vblendvps(inst.reg_x64, tmp1.reg, hoist_9, inst.reg_x64);
        }
      }
      IrCmd::SelectNum => {
        {
          inst.reg_x64 = self.regs.alloc_reg_or_reuse(
            SizeX64::Xmmword,
            index,
            &[(inst.op(0)), inst.op(2), inst.op(3)],
          ); // can't reuse b if a is a memory operand

          let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);

          if inst.op(2).kind() == IrOpKind::Inst {
            let hoist_144 = self.reg_op(inst.op(2));
            let hoist_145 = self.mem_reg_double_op(inst.op(3));
            self.emit_vcmpeqsd(
              OperandX64::reg(tmp.reg),
              OperandX64::reg(hoist_144),
              hoist_145,
            );
          } else {
            let hoist_146 = self.mem_reg_double_op(inst.op(2));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp.reg), hoist_146);
            let hoist_147 = self.mem_reg_double_op(inst.op(3));
            self.emit_vcmpeqsd(
              OperandX64::reg(tmp.reg),
              OperandX64::reg(tmp.reg),
              hoist_147,
            );
          }

          if inst.op(0).kind() == IrOpKind::Inst {
            let hoist_148 = self.reg_op(inst.op(0));
            let hoist_149 = self.mem_reg_double_op(inst.op(1));
            self.emit_vblendvpd(inst.reg_x64, hoist_148, hoist_149, tmp.reg);
          } else {
            let hoist_150 = self.mem_reg_double_op(inst.op(0));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(inst.reg_x64), hoist_150);
            let hoist_151 = self.mem_reg_double_op(inst.op(1));
            self.emit_vblendvpd(inst.reg_x64, inst.reg_x64, hoist_151, tmp.reg);
          }
        }
      }
      IrCmd::SelectInt64 => {
        {
          // C cond D 成立则选 B，否则选 A
          // A、B：int64（端点），C、D：int64（比较参数），E：条件
          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);

          let cond = condition_op(inst.op(4));

          // 先放 falseVal（A），条件成立时换成 trueVal（B）
          let hoist_152 = self.mem_reg_int_64_op(inst.op(0));
          self.emit_mov(OperandX64::reg(inst.reg_x64), hoist_152);

          let tmp = self.alloc_scoped_reg(SizeX64::Qword);
          // 比较 C 与 D
          if inst.op(2).kind() == IrOpKind::Inst {
            let hoist_153 = self.reg_op(inst.op(2));
            let hoist_154 = self.mem_reg_int_64_op(inst.op(3));
            self.emit_cmp(OperandX64::reg(hoist_153), hoist_154);
          } else {
            let hoist_155 = self.mem_reg_int_64_op(inst.op(2));
            self.emit_mov(OperandX64::reg(tmp.reg), hoist_155);
            let hoist_156 = self.mem_reg_int_64_op(inst.op(3));
            self.emit_cmp(OperandX64::reg(tmp.reg), hoist_156);
          }

          // 条件为真时改选 B
          let hoist_157 = self.mem_reg_int_64_op(inst.op(1));
          self.emit_cmov(get_condition_int(cond), inst.reg_x64, hoist_157);
        }
      }
      IrCmd::SelectVec => {
        inst.reg_x64 =
          self
            .regs
            .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(2), inst.op(3)]);

        let mut tmp1 = self.scoped_reg();
        let mut tmp2 = self.scoped_reg();
        let tmpc = self.vec_op(inst.op(2), &mut tmp1);
        let tmpd = self.vec_op(inst.op(3), &mut tmp2);

        self.build_mut().vcmpeqps(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(tmpc),
          OperandX64::reg(tmpd),
        );
        let o_a = self.vec_op(inst.op(0), &mut tmp1);
        let o_b = self.vec_op(inst.op(1), &mut tmp2);
        self
          .build_mut()
          .vblendvps(inst.reg_x64, o_a, OperandX64::reg(o_b), inst.reg_x64);
      }
      IrCmd::SelectIfTruthy => {
        {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index); // No reuse since multiple inputs can be shared

          // 先放 lhs 作为结果，稍后若 'A' 为 falsy 再用 rhs 覆盖
          let hoist_158 = self.reg_op(inst.op(1));
          self.emit_vmovaps(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_158));

          // 尽早取 rhs 寄存器，让可能的 restore 发生在条件控制流两侧之外
          let c = self.reg_op(inst.op(2));

          let tmp = self.alloc_scoped_reg(SizeX64::Dword);
          let mut save_rhs = Label::default();
          let mut exit = Label::default();

          // 先检查 tag
          let hoist_159 = self.reg_op(inst.op(0));
          self.emit_vpextrd(tmp.reg, hoist_159, 3_u8);
          self.build_mut().cmp(
            OperandX64::reg(tmp.reg),
            OperandX64::imm((LuaType::Boolean as u8) as i32),
          );

          self.build_mut().jcc(ConditionX64::Below, &mut save_rhs); // rhs if 'A' is nil
          self.build_mut().jcc(ConditionX64::Above, &mut exit); // Keep lhs if 'A' is not a boolean

          // 检查 boolean 值
          let hoist_160 = self.reg_op(inst.op(0));
          self.emit_vpextrd(tmp.reg, hoist_160, 0_u8);
          self
            .build_mut()
            .test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
          self.build_mut().jcc(ConditionX64::NotZero, &mut exit); // Keep lhs if 'A' is true

          self.build_mut().set_label(&mut save_rhs);
          self
            .build_mut()
            .vmovaps(OperandX64::reg(inst.reg_x64), OperandX64::reg(c));

          self.build_mut().set_label(&mut exit);
        }
      }
      IrCmd::AddVec | IrCmd::SubVec | IrCmd::MulVec | IrCmd::DivVec => {
        let emit = match inst.cmd {
          IrCmd::AddVec => AssemblyBuilderX64::vaddps,
          IrCmd::SubVec => AssemblyBuilderX64::vsubps,
          IrCmd::MulVec => AssemblyBuilderX64::vmulps,
          IrCmd::DivVec => AssemblyBuilderX64::vdivps,
          _ => unreachable!(),
        };
        self.lower_vec_arith(inst, index, emit);
      }
      IrCmd::IdivVec => {
        self.lower_vec_arith(inst, index, AssemblyBuilderX64::vdivps);
        self.build_mut().vroundps(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          RoundingModeX64::RoundToNegativeInfinity,
        );
      }
      IrCmd::MuladdVec => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);
        let mut tmp1 = self.scoped_reg();
        let mut tmp2 = self.scoped_reg();
        let mut tmp3 = self.scoped_reg();

        let tmpa = self.vec_op(inst.op(0), &mut tmp1);
        let tmpb = self.vec_op(inst.op(1), &mut tmp2);
        let tmpc = self.vec_op(inst.op(2), &mut tmp3);

        if FeaturesX64::FeatureFma3.is_set(self.build_mut().features) {
          if inst.reg_x64 != tmpa {
            self
              .build_mut()
              .vmovups(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmpa));
          }

          self.build_mut().vfmadd213ps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpb),
            OperandX64::reg(tmpc),
          );
        } else {
          self.build_mut().vmulps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
          );
          self.build_mut().vaddps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpc),
          );
        }
      }
      IrCmd::UnmVec => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let hoist_161 = self.reg_op(inst.op(0));
        let hoist_162 = self.emit_f32x4(-0.0, -0.0, -0.0, -0.0);
        self.emit_vxorpd(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(hoist_161),
          hoist_162,
        );
      }
      IrCmd::MinVec | IrCmd::MaxVec => {
        let emit = match inst.cmd {
          IrCmd::MinVec => AssemblyBuilderX64::vminps,
          IrCmd::MaxVec => AssemblyBuilderX64::vmaxps,
          _ => unreachable!(),
        };
        self.lower_vec_arith(inst, index, emit);
      }
      IrCmd::FloorVec | IrCmd::CeilVec => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let mut tmp1 = self.scoped_reg();
        let tmpa = self.vec_op(inst.op(0), &mut tmp1);

        let mode = if inst.cmd == IrCmd::FloorVec {
          RoundingModeX64::RoundToNegativeInfinity
        } else {
          RoundingModeX64::RoundToPositiveInfinity
        };
        self
          .build_mut()
          .vroundps(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmpa), mode);
      }
      IrCmd::AbsVec => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let mut tmp1 = self.scoped_reg();
        let tmpa = self.vec_op(inst.op(0), &mut tmp1);

        let hoist_10 = self.emit_u32x4(0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff);
        self.emit_vandps(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(tmpa),
          hoist_10,
        );
      }
      IrCmd::DotVec => {
        {
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0), inst.op(1)]);

          let mut tmp1 = self.scoped_reg();
          let mut tmp2 = self.scoped_reg();

          let op0 = inst.op(0);
          let op1 = inst.op(1);
          let tmpa = self.vec_op(op0, &mut tmp1);
          let tmpb = if op0 == op1 {
            tmpa
          } else {
            self.vec_op(op1, &mut tmp2)
          };

          self.build_mut().vdpps(
            OperandX64::reg(inst.reg_x64),
            OperandX64::reg(tmpa),
            OperandX64::reg(tmpb),
            0x71,
          ); // 7 = 0b0111, sum first 3 products into first float
        }
      }
      IrCmd::ExtractVec => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let hoist_163 = self.reg_op(inst.op(0));
        let hoist_164 = self.reg_op(inst.op(0));
        self.emit_vpshufps(
          inst.reg_x64,
          hoist_163,
          OperandX64::reg(hoist_164),
          (self.int_op(inst.op(1))) as u8,
        );
      }
      IrCmd::NotAny => {
        {
          // TODO: 若唯一 user 是 STORE_INT，我们就错过了直写目标的机会
          inst.reg_x64 =
            self
              .regs
              .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0), inst.op(1)]);

          let mut save_one = Label::default();
          let mut save_zero = Label::default();
          let mut exit = Label::default();

          if inst.op(0).kind() == IrOpKind::Constant {
            // 其他情形应已被常量折叠
            CODEGEN_ASSERT!(self.tag_op(inst.op(0)) == LuaType::Boolean as u8);
          } else {
            let hoist_165 = self.reg_op(inst.op(0));
            self.emit_cmp(
              OperandX64::reg(hoist_165),
              OperandX64::imm((LuaType::Nil as u8) as i32),
            );
            self.build_mut().jcc(ConditionX64::Equal, &mut save_one);

            let hoist_166 = self.reg_op(inst.op(0));
            self.emit_cmp(
              OperandX64::reg(hoist_166),
              OperandX64::imm((LuaType::Boolean as u8) as i32),
            );
            self.build_mut().jcc(ConditionX64::NotEqual, &mut save_zero);
          }

          if inst.op(1).kind() == IrOpKind::Constant {
            // 值为 1 时 fallthrough 去存 0
            if self.int_op(inst.op(1)) == 0 {
              self.build_mut().jmp_label(&mut save_one);
            }
          } else {
            let hoist_167 = self.reg_op(inst.op(1));
            self.emit_cmp(OperandX64::reg(hoist_167), OperandX64::imm(0_i32));
            self.build_mut().jcc(ConditionX64::Equal, &mut save_one);
          }

          self.build_mut().set_label(&mut save_zero);
          self
            .build_mut()
            .mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(0_i32));
          self.build_mut().jmp_label(&mut exit);

          self.build_mut().set_label(&mut save_one);
          self
            .build_mut()
            .mov(OperandX64::reg(inst.reg_x64), OperandX64::imm(1_i32));

          self.build_mut().set_label(&mut exit);
        }
      }
      IrCmd::CmpInt => {
        {
          // 不能复用操作数寄存器作目标，因为比较前必须修改它
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          // 将在字节寄存器上运算，它们写入时不清高位
          self
            .build_mut()
            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

          let cond = condition_op(inst.op(2));

          if inst.op(0).kind() == IrOpKind::Constant {
            let hoist_168 = self.reg_op(inst.op(1));
            self.emit_cmp(
              OperandX64::reg(hoist_168),
              OperandX64::imm(self.int_op(inst.op(0))),
            );
            self.build_mut().setcc(
              get_inverse_condition(get_condition_int(cond)),
              OperandX64::reg(byte_reg(inst.reg_x64)),
            );
          } else if inst.op(0).kind() == IrOpKind::Inst {
            let hoist_169 = self.reg_op(inst.op(0));
            self.emit_cmp(
              OperandX64::reg(hoist_169),
              OperandX64::imm(self.int_op(inst.op(1))),
            );
            self.build_mut().setcc(
              get_condition_int(cond),
              OperandX64::reg(byte_reg(inst.reg_x64)),
            );
          } else {
            unsupported_instruction_form();
          }
        }
      }
      IrCmd::CmpAny => {
        {
          CODEGEN_ASSERT!(
            inst.op(0).kind() == IrOpKind::VmReg && inst.op(1).kind() == IrOpKind::VmReg
          );
          let cond = condition_op(inst.op(2));

          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          let mut skip = Label::default();
          let mut exit = Label::default();

          // 等值比较时，'luaV_equalval' 要求调用前 tag 相等
          if cond == IrCondition::Equal {
            let tmp = self.alloc_scoped_reg(SizeX64::Dword);

            let hoist_170 = self.mem_reg_tag_op(inst.op(0));
            self.emit_mov(OperandX64::reg(tmp.reg), hoist_170);
            let hoist_171 = self.mem_reg_tag_op(inst.op(1));
            self.emit_cmp(hoist_171, OperandX64::reg(tmp.reg));

            // tag 不相等则跳过调用并把结果置 0
            self.build_mut().jcc(ConditionX64::NotEqual, &mut skip);
          }

          {
            let _spill_guard = ScopedSpills::new(&mut self.regs);

            let mut call_wrap = self.call_wrap_state(index);
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              luau_reg_address(vm_reg_op(inst.op(0))),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              luau_reg_address(vm_reg_op(inst.op(1))),
              IrOp::default(),
            );
            call_wrap.set_result_register(inst.reg_x64, index);

            if cond == IrCondition::LessEqual {
              call_wrap.call(&OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                R_NATIVE_CONTEXT,
                (offset_of!(NativeContext, lua_v_lessequal) as i32),
              ));
            } else if cond == IrCondition::Less {
              call_wrap.call(&OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                R_NATIVE_CONTEXT,
                (offset_of!(NativeContext, lua_v_lessthan) as i32),
              ));
            } else if cond == IrCondition::Equal {
              call_wrap.call(&OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                R_NATIVE_CONTEXT,
                (offset_of!(NativeContext, lua_v_equalval) as i32),
              ));
            } else {
              CODEGEN_ASSERT!(false, "Unsupported condition");
            }

            emit_update_base(self.build_mut());
          }

          if cond == IrCondition::Equal {
            self.build_mut().jmp_label(&mut exit);
            self.build_mut().set_label(&mut skip);

            self
              .build_mut()
              .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
            self.build_mut().set_label(&mut exit);
          }

          // 若发生过调用，跳过清高寄存器位；唯一消费者 JUMP_CMP_INT 不读它们
        }
      }
      IrCmd::CmpTag => {
        {
          // 不能复用操作数寄存器作目标，因为比较前必须修改它
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          // 将在字节寄存器上运算，它们写入时不清高位
          self
            .build_mut()
            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

          let cond = condition_op(inst.op(2));
          CODEGEN_ASSERT!(matches!(cond, IrCondition::Equal | IrCondition::NotEqual));
          let cond_x64 = get_condition_int(cond);

          if self.tag_op(inst.op(1)) == LuaType::Nil as u8 && inst.op(0).kind() == IrOpKind::Inst {
            let hoist_172 = self.reg_op(inst.op(0));
            let hoist_173 = self.reg_op(inst.op(0));
            self.emit_test(OperandX64::reg(hoist_172), OperandX64::reg(hoist_173));
          } else {
            let hoist_174 = self.mem_reg_tag_op(inst.op(0));
            self.emit_cmp(hoist_174, OperandX64::imm((self.tag_op(inst.op(1))) as i32));
          }

          self
            .build_mut()
            .setcc(cond_x64, OperandX64::reg(byte_reg(inst.reg_x64)));
        }
      }
      IrCmd::CmpSplitTvalue => {
        {
          // 不能复用操作数寄存器作目标，因为比较前必须修改它
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          // 本指令的第二个操作数必须是常量
          // 没有常量类型，lowering 时就不知道正确的值比较方式
          CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::Constant);

          // 将在字节寄存器上运算，它们写入时不清高位
          self
            .build_mut()
            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

          let cond = condition_op(inst.op(4));
          CODEGEN_ASSERT!(matches!(cond, IrCondition::Equal | IrCondition::NotEqual));

          // 先检查 tag 相等
          let tmp1 = self.alloc_scoped_reg(SizeX64::Byte);

          if inst.op(0).kind() != IrOpKind::Constant {
            let hoist_175 = self.reg_op(inst.op(0));
            self.emit_cmp(
              OperandX64::reg(hoist_175),
              OperandX64::imm((self.tag_op(inst.op(1))) as i32),
            );
            self
              .build_mut()
              .setcc(get_condition_int(cond), OperandX64::reg(byte_reg(tmp1.reg)));
          } else {
            // 不同常量 tag 本应由常量折叠处理掉
            CODEGEN_ASSERT!(self.tag_op(inst.op(0)) == self.tag_op(inst.op(1)));
          }

          if self.tag_op(inst.op(1)) == LuaType::Boolean as u8 {
            if inst.op(2).kind() == IrOpKind::Constant {
              let hoist_176 = self.reg_op(inst.op(3));
              self.emit_cmp(
                OperandX64::reg(hoist_176),
                OperandX64::imm(self.int_op(inst.op(2))),
              );
            }
            // 参数已交换
            else if inst.op(3).kind() == IrOpKind::Constant {
              let hoist_177 = self.reg_op(inst.op(2));
              self.emit_cmp(
                OperandX64::reg(hoist_177),
                OperandX64::imm(self.int_op(inst.op(3))),
              );
            } else {
              let hoist_178 = self.reg_op(inst.op(2));
              let hoist_179 = self.reg_op(inst.op(3));
              self.emit_cmp(OperandX64::reg(hoist_178), OperandX64::reg(hoist_179));
            }

            self.build_mut().setcc(
              get_condition_int(cond),
              OperandX64::reg(byte_reg(inst.reg_x64)),
            );
          } else if self.tag_op(inst.op(1)) == LuaType::String as u8 {
            let hoist_180 = self.reg_op(inst.op(2));
            let hoist_181 = self.reg_op(inst.op(3));
            self.emit_cmp(OperandX64::reg(hoist_180), OperandX64::reg(hoist_181));
            self.build_mut().setcc(
              get_condition_int(cond),
              OperandX64::reg(byte_reg(inst.reg_x64)),
            );
          } else if self.tag_op(inst.op(1)) == LuaType::Number as u8 {
            if inst.op(2).kind() == IrOpKind::Constant {
              let hoist_182 = self.reg_op(inst.op(3));
              let hoist_183 = self.mem_reg_double_op(inst.op(2));
              self.emit_vucomisd(OperandX64::reg(hoist_182), hoist_183);
            }
            // 参数已交换
            else if inst.op(3).kind() == IrOpKind::Constant {
              let hoist_184 = self.reg_op(inst.op(2));
              let hoist_185 = self.mem_reg_double_op(inst.op(3));
              self.emit_vucomisd(OperandX64::reg(hoist_184), hoist_185);
            } else {
              let hoist_186 = self.reg_op(inst.op(2));
              let hoist_187 = self.reg_op(inst.op(3));
              self.emit_vucomisd(OperandX64::reg(hoist_186), OperandX64::reg(hoist_187));
            }

            let op2 = inst.op(2);
            let op3 = inst.op(3);
            if op2 == op3 {
              // 两数相等时只需检查奇偶即可探测 NaN
              if cond == IrCondition::Equal {
                self.build_mut().setcc(
                  ConditionX64::NotParity,
                  OperandX64::reg(byte_reg(inst.reg_x64)),
                );
              } else {
                self.build_mut().setcc(
                  ConditionX64::Parity,
                  OperandX64::reg(byte_reg(inst.reg_x64)),
                );
              }
            } else {
              let tmp2 = self.alloc_scoped_reg(SizeX64::Dword);

              if cond == IrCondition::Equal {
                self
                  .build_mut()
                  .mov(OperandX64::reg(tmp2.reg), OperandX64::imm(0_i32));
                self.build_mut().setcc(
                  ConditionX64::NotParity,
                  OperandX64::reg(byte_reg(inst.reg_x64)),
                );
                self.build_mut().cmov(
                  ConditionX64::NotEqual,
                  inst.reg_x64,
                  OperandX64::reg(tmp2.reg),
                );
              } else {
                self
                  .build_mut()
                  .mov(OperandX64::reg(tmp2.reg), OperandX64::imm(1_i32));
                self.build_mut().setcc(
                  ConditionX64::Parity,
                  OperandX64::reg(byte_reg(inst.reg_x64)),
                );
                self.build_mut().cmov(
                  ConditionX64::NotEqual,
                  inst.reg_x64,
                  OperandX64::reg(tmp2.reg),
                );
              }
            }
          } else if self.tag_op(inst.op(1)) == LuaType::Integer as u8 {
            if inst.op(2).kind() == IrOpKind::Constant {
              let hoist_188 = self.reg_op(inst.op(3));
              let hoist_189 = self.mem_reg_int_64_op(inst.op(2));
              self.emit_cmp(OperandX64::reg(hoist_188), hoist_189);
            }
            // 参数已交换
            else if inst.op(3).kind() == IrOpKind::Constant {
              let hoist_190 = self.reg_op(inst.op(2));
              let hoist_191 = self.mem_reg_int_64_op(inst.op(3));
              self.emit_cmp(OperandX64::reg(hoist_190), hoist_191);
            } else {
              let hoist_192 = self.reg_op(inst.op(2));
              let hoist_193 = self.reg_op(inst.op(3));
              self.emit_cmp(OperandX64::reg(hoist_192), OperandX64::reg(hoist_193));
            }

            self.build_mut().setcc(
              get_condition_int(cond),
              OperandX64::reg(byte_reg(inst.reg_x64)),
            );
          } else {
            CODEGEN_ASSERT!(false, "unsupported type tag in CMP_SPLIT_TVALUE");
          }

          if inst.op(0).kind() != IrOpKind::Constant {
            if cond == IrCondition::Equal {
              self.build_mut().and_(
                OperandX64::reg(byte_reg(inst.reg_x64)),
                OperandX64::reg(byte_reg(tmp1.reg)),
              );
            } else {
              self.build_mut().or_(
                OperandX64::reg(byte_reg(inst.reg_x64)),
                OperandX64::reg(byte_reg(tmp1.reg)),
              );
            }
          }
        }
      }
      IrCmd::JUMP => {
        self.jump_or_abort_on_undef_ir_op_u32_ir_block(inst.op(0), index, next);
      }
      IrCmd::JumpIfTruthy => {
        self.jump_if_branch_op(true, inst.op(0), inst.op(1), inst.op(2));
        self.jump_or_fallthrough_op(inst.op(2), next);
      }
      IrCmd::JumpIfFalsy => {
        self.jump_if_branch_op(false, inst.op(0), inst.op(1), inst.op(2));
        self.jump_or_fallthrough_op(inst.op(2), next);
      }
      IrCmd::JumpEqTag => {
        CODEGEN_ASSERT!(
          inst.op(1).kind() == IrOpKind::Inst || inst.op(1).kind() == IrOpKind::Constant
        );
        let opb = if inst.op(1).kind() == IrOpKind::Inst {
          OperandX64::reg(self.reg_op(inst.op(1)))
        } else {
          OperandX64::imm(self.tag_op(inst.op(1)) as i32)
        };

        if inst.op(0).kind() == IrOpKind::Constant {
          self.emit_cmp(opb, OperandX64::imm((self.tag_op(inst.op(0))) as i32));
        } else {
          let hoist_194 = self.mem_reg_tag_op(inst.op(0));
          self.emit_cmp(hoist_194, opb);
        }

        if self.is_fallthrough_block(self.block_op_ref(inst.op(3)), next) {
          self.with_op_label(inst.op(2), |s, l| s.build_mut().jcc(ConditionX64::Equal, l));
          self.jump_or_fallthrough_op(inst.op(3), next);
        } else {
          self.with_op_label(inst.op(3), |s, l| {
            s.build_mut().jcc(ConditionX64::NotEqual, l)
          });
          self.jump_or_fallthrough_op(inst.op(2), next);
        }
      }
      IrCmd::JumpCmpInt => {
        let cond = condition_op(inst.op(2));

        if (matches!(cond, IrCondition::Equal | IrCondition::NotEqual))
          && self.int_op(inst.op(1)) == 0
        {
          let invert = cond == IrCondition::NotEqual;

          let hoist_195 = self.reg_op(inst.op(0));
          let hoist_196 = self.reg_op(inst.op(0));
          self.emit_test(OperandX64::reg(hoist_195), OperandX64::reg(hoist_196));

          if self.is_fallthrough_block(self.block_op_ref(inst.op(3)), next) {
            self.with_op_label(inst.op(4), |s, l| {
              s.build_mut().jcc(
                if invert {
                  ConditionX64::Zero
                } else {
                  ConditionX64::NotZero
                },
                l,
              )
            });
            self.jump_or_fallthrough_op(inst.op(3), next);
          } else {
            self.with_op_label(inst.op(3), |s, l| {
              s.build_mut().jcc(
                if invert {
                  ConditionX64::NotZero
                } else {
                  ConditionX64::Zero
                },
                l,
              )
            });
            self.jump_or_fallthrough_op(inst.op(4), next);
          }
        } else {
          let hoist_197 = self.reg_op(inst.op(0));
          self.emit_cmp(
            OperandX64::reg(hoist_197),
            OperandX64::imm(self.int_op(inst.op(1))),
          );

          self.with_op_label(inst.op(3), |s, l| {
            s.build_mut().jcc(get_condition_int(cond), l)
          });
          self.jump_or_fallthrough_op(inst.op(4), next);
        }
      }
      IrCmd::JumpCmpInt64 => {
        // cpp IrLoweringX64.cpp:1767
        let cond = condition_op(inst.op(2));
        let mut cc = get_condition_int(cond);

        // 常量传播可能把常量放在任一侧；没有立即数与寄存器比较的 64 位形式，
        // 因此交换操作数并反转条件（与 CMP_INT64 相同）
        if inst.op(0).kind() == IrOpKind::Constant {
          let hoist_198 = self.reg_op(inst.op(1));
          let hoist_199 = self.mem_reg_int_64_op(inst.op(0));
          self.emit_cmp(OperandX64::reg(hoist_198), hoist_199);
          cc = get_inverse_condition(cc);
        } else {
          let hoist_200 = self.reg_op(inst.op(0));
          let hoist_201 = self.mem_reg_int_64_op(inst.op(1));
          self.emit_cmp(OperandX64::reg(hoist_200), hoist_201);
        }

        self.with_op_label(inst.op(3), |s, l| s.build_mut().jcc(cc, l));
        self.jump_or_fallthrough_op(inst.op(4), next);
      }
      IrCmd::JumpEqPointer => {
        let hoist_202 = self.reg_op(inst.op(0));
        let hoist_203 = self.reg_op(inst.op(1));
        self.emit_cmp(OperandX64::reg(hoist_202), OperandX64::reg(hoist_203));

        self.with_op_label(inst.op(2), |s, l| s.build_mut().jcc(ConditionX64::Equal, l));
        self.jump_or_fallthrough_op(inst.op(3), next);
      }
      IrCmd::JumpCmpNum => self.lower_jump_cmp_fp(inst, next, false),
      IrCmd::JumpCmpFloat => self.lower_jump_cmp_fp(inst, next, true),
      IrCmd::JumpFornLoopCond => {
        {
          let tmp1 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp2 = self.alloc_scoped_reg(SizeX64::Xmmword);
          let tmp3 = self.alloc_scoped_reg(SizeX64::Xmmword);

          let index = if inst.op(0).kind() == IrOpKind::Inst {
            self.reg_op(inst.op(0))
          } else {
            tmp1.reg
          };
          let limit = if inst.op(1).kind() == IrOpKind::Inst {
            self.reg_op(inst.op(1))
          } else {
            tmp2.reg
          };

          if inst.op(0).kind() != IrOpKind::Inst {
            let hoist_204 = self.mem_reg_double_op(inst.op(0));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp1.reg), hoist_204);
          }

          if inst.op(1).kind() != IrOpKind::Inst {
            let hoist_205 = self.mem_reg_double_op(inst.op(1));
            self.emit_vmovsd_operand_x_64_operand_x_64(OperandX64::reg(tmp2.reg), hoist_205);
          }

          let mut direct = Label::default();

          // step > 0
          let step = self.mem_reg_double_op(inst.op(2));
          let zero = self.build_mut().f64(0.0);
          jump_on_number_cmp(
            self.build_mut(),
            tmp3.reg,
            step,
            zero,
            IrCondition::Greater,
            &mut direct,
            /* floatPrecision */ false,
          );

          // !(limit <= index)
          self.with_op_label(inst.op(4), |s, l| {
            jump_on_number_cmp(
              s.build_mut(),
              RegisterX64::NOREG,
              OperandX64::reg(limit),
              OperandX64::reg(index),
              IrCondition::NotLessEqual,
              l,
              /* floatPrecision */ false,
            );
          });
          self.with_op_label(inst.op(3), |s, l| s.build_mut().jmp_label(l));

          // !(index <= limit)
          self.build_mut().set_label(&mut direct);
          // !(index <= limit)
          self.with_op_label(inst.op(4), |s, l| {
            jump_on_number_cmp(
              s.build_mut(),
              RegisterX64::NOREG,
              OperandX64::reg(index),
              OperandX64::reg(limit),
              IrCondition::NotLessEqual,
              l,
              /* floatPrecision */ false,
            );
          });
          self.jump_or_fallthrough_op(inst.op(3), next);
        }
      }
      IrCmd::TableLen => {
        {
          let mut call_wrap = self.call_wrap(index);
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(self.reg_op(inst.op(0))),
            inst.op(0),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_NATIVE_CONTEXT,
            (offset_of!(NativeContext, lua_h_getn) as i32),
          ));

          inst.reg_x64 = self.regs.take_reg(dword_reg(RegisterX64::RAX), index);

          self
            .build_mut()
            .mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
          // 确保清掉寄存器高位
        }
      }
      IrCmd::TableSetnum => {
        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Qword,
          OperandX64::reg(self.reg_op(inst.op(0))),
          inst.op(0),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::reg(self.reg_op(inst.op(1))),
          inst.op(1),
        );
        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, lua_h_setnum) as i32),
        ));
        inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
      }
      IrCmd::StringLen => {
        let ptr = self.reg_op(inst.op(0));
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);
        self.build_mut().mov(
          OperandX64::reg(inst.reg_x64),
          OperandX64::mem(
            SizeX64::Dword,
            RegisterX64::NOREG,
            1,
            ptr,
            K_TSTRING_LEN_OFFSET,
          ),
        );
      }
      IrCmd::NewTable => {
        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(self.uint_op(inst.op(0)) as i32),
          IrOp::default(),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(self.uint_op(inst.op(1)) as i32),
          IrOp::default(),
        );
        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, lua_h_new) as i32),
        ));
        inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
      }
      IrCmd::DupTable => {
        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Qword,
          OperandX64::reg(self.reg_op(inst.op(0))),
          inst.op(0),
        );
        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, lua_h_clone) as i32),
        ));
        inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
      }
      IrCmd::TryNumToIndex => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

        let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);

        let src = self.reg_op(inst.op(0));
        let dst = inst.reg_x64;
        self.with_op_label(inst.op(1), |s, l| {
          convert_number_to_index_or_jump(s.build_mut(), tmp.reg, src, dst, l);
        });
      }
      IrCmd::TryCallFastgettm => {
        {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

          let mut tmp = self.alloc_scoped_reg(SizeX64::Qword);

          let hoist_206 = self.reg_op(inst.op(0));
          self.emit_mov(
            OperandX64::reg(tmp.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              hoist_206,
              (offset_of!(LuaTable, metatable) as i32),
            ),
          );
          // Safety: 混窗元组门面（function↔regs 共存别名，见 records 契约）；两视图
          // 即时消费于本调用，与化前 `(&mut self.regs).free_last_use_reg((*self.function)...)`
          // 实参列逐位等价。
          let (function, regs) = self.function_regs_mut();
          regs.free_last_use_reg(function.inst_op(inst.op(0)), index); // Release before the call if it's the last use

          self
            .build_mut()
            .test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
          self.with_op_label(inst.op(2), |s, l| s.build_mut().jcc(ConditionX64::Zero, l)); // No metatable

          self.emit_test(
            OperandX64::mem(
              SizeX64::Byte,
              RegisterX64::NOREG,
              1,
              tmp.reg,
              (offset_of!(LuaTable, tmcache) as i32),
            ),
            OperandX64::imm(1 << self.int_op(inst.op(1))),
          );
          self.with_op_label(inst.op(2), |s, l| {
            s.build_mut().jcc(ConditionX64::NotZero, l)
          }); // No tag method

          let mut tmp2 = self.alloc_scoped_reg(SizeX64::Qword);
          self.build_mut().mov(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_STATE,
              (offset_of!(LuaState, global) as i32),
            ),
          );

          {
            let _spill_guard = ScopedSpills::new(&mut self.regs);

            let mut call_wrap = self.call_wrap(index);
            call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp);
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::imm(self.int_op(inst.op(1))),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                tmp2.release(),
                (offset_of!(global_State, tmname) as i32)
                    // cpp `intOp * sizeof(TString*)`：无符号环绕语义，用 wrapping 对齐
                    + self
                        .int_op(inst.op(1))
                        .wrapping_mul(K_NATIVE_PTR_SIZE as i32),
              ),
              IrOp::default(),
            );
            call_wrap.set_result_register(inst.reg_x64, index);
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_NATIVE_CONTEXT,
              (offset_of!(NativeContext, lua_t_gettm) as i32),
            ));
          }

          self
            .build_mut()
            .test(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
          self.with_op_label(inst.op(2), |s, l| s.build_mut().jcc(ConditionX64::Zero, l));
          // 没有 tag method
        }
      }
      IrCmd::NewUserdata => {
        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Qword,
          OperandX64::imm(self.int_op(inst.op(0))),
          IrOp::default(),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(self.int_op(inst.op(1))),
          IrOp::default(),
        );
        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, new_userdata) as i32),
        ));
        inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
      }
      IrCmd::IntToNum | IrCmd::Int64ToNum => {
        // 两臂语句序列逐位相同（源寄存器宽度由分配器决定），合并孪生臂
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

        let hoist_207 = self.reg_op(inst.op(0));
        self.emit_vcvtsi2sd(
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(inst.reg_x64),
          OperandX64::reg(hoist_207),
        );
      }
      IrCmd::UintToNum => self.lower_uint_to_fp(inst, index, Self::emit_vcvtsi2sd),
      IrCmd::UintToFloat => self.lower_uint_to_fp(inst, index, Self::emit_vcvtsi2ss),
      IrCmd::NumToInt => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

        let hoist_212 = self.mem_reg_double_op(inst.op(0));
        self.emit_vcvttsd2si(OperandX64::reg(inst.reg_x64), hoist_212);
      }
      IrCmd::NumToUint => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

        // 注：为与 C++ 一致，执行 'uint64_t = (long long)double'
        let hoist_213 = self.mem_reg_double_op(inst.op(0));
        self.emit_vcvttsd2si(OperandX64::reg(qword_reg(inst.reg_x64)), hoist_213);
      }
      IrCmd::FloatToNum => {
        self.lower_unary_xmm(inst, index, Self::mem_reg_double_op, Self::emit_vcvtss2sd);
      }
      IrCmd::NumToFloat => {
        self.lower_unary_xmm(inst, index, Self::mem_reg_double_op, Self::emit_vcvtsd2ss);
      }
      IrCmd::FloatToVec => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

        if inst.op(0).kind() == IrOpKind::Constant {
          let as_u32 = get_float_bits((self.double_op(inst.op(0))) as f32);

          let hoist_11 = self.emit_u32x4(as_u32, as_u32, as_u32, 0);
          self.emit_vmovaps(OperandX64::reg(inst.reg_x64), hoist_11);
        } else {
          let hoist_216 = self.reg_op(inst.op(0));
          let hoist_217 = self.reg_op(inst.op(0));
          self.emit_vpshufps(
            inst.reg_x64,
            hoist_216,
            OperandX64::reg(hoist_217),
            0b00_00_00_00,
          );
        }
      }
      IrCmd::TagVector => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Xmmword, index, &[inst.op(0)]);

        let hoist_218 = self.reg_op(inst.op(0));
        let hoist_219 = self.emit_i32(LuaType::Vector as i32);
        self.emit_vpinsrd(inst.reg_x64, hoist_218, hoist_219, 3_u8);
      }
      IrCmd::TruncateUint => {
        inst.reg_x64 = self
          .regs
          .alloc_reg_or_reuse(SizeX64::Dword, index, &[inst.op(0)]);

        // 可能生成源与目的相同的 mov，而它并非空操作
        let hoist_220 = self.reg_op(inst.op(0));
        self.emit_mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(hoist_220));
      }
      IrCmd::AdjustStackToReg => {
        let tmp = self.alloc_scoped_reg(SizeX64::Qword);

        if inst.op(1).kind() == IrOpKind::Constant {
          self.emit_lea_operand_x_64_operand_x_64(
            OperandX64::reg(tmp.reg),
            OperandX64::mem(
              SizeX64::None,
              RegisterX64::NOREG,
              1,
              R_BASE,
              (vm_reg_op(inst.op(0)) + self.int_op(inst.op(1))) * (size_of::<TValue>() as i32),
            ),
          );
          self.build_mut().mov(
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_STATE,
              (offset_of!(LuaState, top) as i32),
            ),
            OperandX64::reg(tmp.reg),
          );
        } else if inst.op(1).kind() == IrOpKind::Inst {
          let hoist_221 = self.reg_op(inst.op(1));
          self.emit_mov(
            OperandX64::reg(dword_reg(tmp.reg)),
            OperandX64::reg(hoist_221),
          );
          self.build_mut().shl(
            OperandX64::reg(tmp.reg),
            OperandX64::imm(K_TVALUE_SIZE_LOG2),
          );
          self.build_mut().lea_operand_x_64_operand_x_64(
            OperandX64::reg(tmp.reg),
            OperandX64::mem(
              SizeX64::None,
              tmp.reg,
              1,
              R_BASE,
              vm_reg_op(inst.op(0)) * (size_of::<TValue>() as i32),
            ),
          );
          self.build_mut().mov(
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_STATE,
              (offset_of!(LuaState, top) as i32),
            ),
            OperandX64::reg(tmp.reg),
          );
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::AdjustStackToTop => {
        let tmp = self.alloc_scoped_reg(SizeX64::Qword);
        self.build_mut().mov(
          OperandX64::reg(tmp.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_STATE,
            (offset_of!(LuaState, ci) as i32),
          ),
        );
        self.build_mut().mov(
          OperandX64::reg(tmp.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            tmp.reg,
            (offset_of!(CallInfo, top) as i32),
          ),
        );
        self.build_mut().mov(
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_STATE,
            (offset_of!(LuaState, top) as i32),
          ),
          OperandX64::reg(tmp.reg),
        );
      }
      IrCmd::FASTCALL => {
        let bfid = self.uint_op(inst.op(0)) as i32;
        let ra = vm_reg_op(inst.op(1));
        let arg = vm_reg_op(inst.op(2));
        let nparams = self.int_op(inst.op(3));
        // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
        // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
        let (build, regs) = self.build_regs_mut();
        emit_builtin(regs, build, bfid, ra, arg, nparams);
      }
      IrCmd::InvokeFastcall => {
        {
          let bfid = self.uint_op(inst.op(0));

          let mut args = OperandX64::imm(0);
          let mut args_alt = self.scoped_reg();

          // 'E' 参数只可能由 LOP_FASTCALL3 产生
          if inst.op(4).kind() != IrOpKind::Undef {
            CODEGEN_ASSERT!(self.int_op(inst.op(5)) == 3);

            let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);
            args_alt.alloc(SizeX64::Qword);

            self.build_mut().mov(
              OperandX64::reg(args_alt.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                R_STATE,
                (offset_of!(LuaState, top) as i32),
              ),
            );

            self
              .build_mut()
              .vmovups(OperandX64::reg(tmp.reg), luau_reg(vm_reg_op(inst.op(3))));
            self.build_mut().vmovups(
              OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, args_alt.reg, 0),
              OperandX64::reg(tmp.reg),
            );

            self
              .build_mut()
              .vmovups(OperandX64::reg(tmp.reg), luau_reg(vm_reg_op(inst.op(4))));
            self.build_mut().vmovups(
              OperandX64::mem(
                SizeX64::Xmmword,
                RegisterX64::NOREG,
                1,
                args_alt.reg,
                size_of::<TValue>() as i32,
              ),
              OperandX64::reg(tmp.reg),
            );
          } else {
            if inst.op(3).kind() == IrOpKind::VmReg {
              args = luau_reg_address(vm_reg_op(inst.op(3)));
            } else if inst.op(3).kind() == IrOpKind::VmConst {
              args = luau_constant_address(vm_const_op(inst.op(3)));
            } else {
              CODEGEN_ASSERT!(inst.op(3).kind() == IrOpKind::Undef);
            }
          }

          let ra = vm_reg_op(inst.op(1));
          let arg = vm_reg_op(inst.op(2));
          let nparams = self.int_op(inst.op(5));
          let nresults = self.int_op(inst.op(6));

          let mut call_wrap = self.call_wrap_state(index);
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            luau_reg_address(ra),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            luau_reg_address(arg),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(nresults),
            IrOp::default(),
          );

          if inst.op(4).kind() != IrOpKind::Undef {
            call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut args_alt);
          } else {
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              args,
              IrOp::default(),
            );
          }

          if nparams == LUA_MULTRET {
            let reg = call_wrap.suggest_next_argument_register(SizeX64::Qword);
            let tmp = self.alloc_scoped_reg(SizeX64::Qword);

            // l->top - (ra + 1)
            self.build_mut().mov(
              OperandX64::reg(reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                R_STATE,
                (offset_of!(LuaState, top) as i32),
              ),
            );
            self.build_mut().lea_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              OperandX64::mem(
                SizeX64::None,
                RegisterX64::NOREG,
                1,
                R_BASE,
                (ra + 1) * (size_of::<TValue>() as i32),
              ),
            );
            self
              .build_mut()
              .sub(OperandX64::reg(reg), OperandX64::reg(tmp.reg));
            self
              .build_mut()
              .shr(OperandX64::reg(reg), OperandX64::imm(K_TVALUE_SIZE_LOG2));

            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::reg(dword_reg(reg)),
              IrOp::default(),
            );
          } else {
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::imm(nparams),
              IrOp::default(),
            );
          }

          let mut func = self.alloc_scoped_reg(SizeX64::Qword);
          self.build_mut().mov(
            OperandX64::reg(func.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_NATIVE_CONTEXT,
              (offset_of!(NativeContext, luau_f_table) as i32)
                + (bfid as i32) * (size_of::<LuauFastFunction>() as i32),
            ),
          );

          call_wrap.call(&OperandX64::reg(func.release()));
          inst.reg_x64 = self.regs.take_reg(dword_reg(RegisterX64::RAX), index);
          // builtin 调用结果在 eax 返回
          // 跳过清高寄存器位；唯一消费者 CHECK_FASTCALL_RES 不读它们
        }
      }
      IrCmd::CheckFastcallRes => {
        {
          let res = self.reg_op(inst.op(0));

          self
            .build_mut()
            .test(OperandX64::reg(res), OperandX64::reg(res)); // test here will set SF=1 for a negative number and it always sets OF to 0
          self.with_op_label(inst.op(1), |s, l| s.build_mut().jcc(ConditionX64::Less, l));
          // SF != OF 时 jl 跳转
        }
      }
      IrCmd::DoArith => {
        let opb = if inst.op(1).kind() == IrOpKind::VmReg {
          luau_reg_address(vm_reg_op(inst.op(1)))
        } else {
          luau_constant_address(vm_const_op(inst.op(1)))
        };
        let opc = if inst.op(2).kind() == IrOpKind::VmReg {
          luau_reg_address(vm_reg_op(inst.op(2)))
        } else {
          luau_constant_address(vm_const_op(inst.op(2)))
        };
        let ra = vm_reg_op(inst.op(0));
        // tm 由编译器写入 IR，必为合法 TMS 判别值；经 TMS::from_u32 校验转换，
        // 越界值归入 TmIndex 走 call_arith_helper 的断言兜底（消除裸 transmute UB）
        let tm = TMS::from_u32(self.int_op(inst.op(3)) as u32).unwrap_or(TMS::TmIndex);
        // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
        // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
        let (build, regs) = self.build_regs_mut();
        call_arith_helper(regs, build, ra, opb, opc, tm);
      }
      IrCmd::DoLen => {
        // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
        // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
        let (build, regs) = self.build_regs_mut();
        call_length_helper(regs, build, vm_reg_op(inst.op(0)), vm_reg_op(inst.op(1)));
      }
      IrCmd::GetTable => self.lower_get_set_table(inst, call_get_table),
      IrCmd::SetTable => self.lower_get_set_table(inst, call_set_table),
      IrCmd::GetCachedImport => {
        {
          self.regs.assert_all_free();
          self.regs.assert_no_spills();

          let mut skip = Label::default();
          let mut exit = Label::default();

          // 若 import 的常量已设置就直接用，否则得调 import 路径查找函数
          self.build_mut().cmp(
            luau_constant_tag(vm_const_op(inst.op(1))),
            OperandX64::imm((LuaType::Nil as u8) as i32),
          );
          self.build_mut().jcc(ConditionX64::NotEqual, &mut skip);

          {
            let _spill_guard = ScopedSpills::new(&mut self.regs);

            let mut call_wrap = self.call_wrap_state(index);
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Qword,
              luau_reg_address(vm_reg_op(inst.op(0))),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::imm(self.import_op(inst.op(2)) as i32),
              IrOp::default(),
            );
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              OperandX64::imm(self.uint_op(inst.op(3)) as i32),
              IrOp::default(),
            );
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_NATIVE_CONTEXT,
              (offset_of!(NativeContext, get_import) as i32),
            ));

            emit_update_base(self.build_mut());
          }

          self.build_mut().jmp_label(&mut exit);

          self.build_mut().set_label(&mut skip);

          let tmp1 = self.alloc_scoped_reg(SizeX64::Xmmword);

          self.build_mut().vmovups(
            OperandX64::reg(tmp1.reg),
            luau_constant(vm_const_op(inst.op(1))),
          );
          self
            .build_mut()
            .vmovups(luau_reg(vm_reg_op(inst.op(0))), OperandX64::reg(tmp1.reg));
          self.build_mut().set_label(&mut exit);
        }
      }
      IrCmd::CONCAT => {
        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(self.uint_op(inst.op(1)) as i32),
          IrOp::default(),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(vm_reg_op(inst.op(0)) + self.uint_op(inst.op(1)) as i32 - 1),
          IrOp::default(),
        );
        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, lua_v_concat) as i32),
        ));

        emit_update_base(self.build_mut());
      }
      IrCmd::GetUpvalue => {
        {
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Xmmword, index);

          let tmp1 = self.alloc_scoped_reg(SizeX64::Qword);

          self.build_mut().mov(OperandX64::reg(tmp1.reg), s_closure());
          self.build_mut().add(
            OperandX64::reg(tmp1.reg),
            OperandX64::imm(
              K_CLOSURE_LUPREFS_OFFSET
                + (size_of::<TValue>() as i32) * vm_upvalue_op(inst.op(0)) as i32,
            ),
          );

          // uprefs[] 要么直接是值，要么指向持有值指针的 UpVal 对象
          let mut skip = Label::default();
          self.build_mut().cmp(
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              tmp1.reg,
              (offset_of!(TValue, tt) as i32),
            ),
            OperandX64::imm((LuaType::Upval as u8) as i32),
          );
          self.build_mut().jcc(ConditionX64::NotEqual, &mut skip);

          // UpVal.v 指向值（在栈上或 UpVal 内的堆上，均可无条件解引用）
          self.build_mut().mov(
            OperandX64::reg(tmp1.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp1.reg,
              (offset_of!(TValue, value.gc) as i32),
            ),
          );
          self.build_mut().mov(
            OperandX64::reg(tmp1.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp1.reg,
              (offset_of!(UpVal, v) as i32),
            ),
          );

          self.build_mut().set_label(&mut skip);

          self.build_mut().vmovups(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, tmp1.reg, 0),
          );
        }
      }
      IrCmd::SetUpvalue => {
        let mut tmp1 = self.alloc_scoped_reg(SizeX64::Qword);
        let mut tmp2 = self.alloc_scoped_reg(SizeX64::Qword);

        self.build_mut().mov(OperandX64::reg(tmp1.reg), s_closure());
        self.build_mut().mov(
          OperandX64::reg(tmp2.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            tmp1.reg,
            K_CLOSURE_LUPREFS_OFFSET
              + (size_of::<TValue>() as i32) * vm_upvalue_op(inst.op(0)) as i32
              + (offset_of!(TValue, value.gc) as i32),
          ),
        );

        self.build_mut().mov(
          OperandX64::reg(tmp1.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            tmp2.reg,
            (offset_of!(UpVal, v) as i32),
          ),
        );
        let hoist_222 = self.reg_op(inst.op(1));
        self.emit_vmovups(
          OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, tmp1.reg, 0),
          OperandX64::reg(hoist_222),
        );

        tmp1.free();

        if inst.op(2).kind() == IrOpKind::Undef || is_gco(self.tag_op(inst.op(2))) {
          let object = tmp2.release();
          let value_op = inst.op(1);
          let value = self.reg_op(value_op);
          let tag_op = inst.op(2);
          let ratag = if tag_op.kind() == IrOpKind::Undef {
            -1
          } else {
            self.tag_op(tag_op) as i32
          };
          // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
          // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
          let (build, regs) = self.build_regs_mut();
          call_barrier_object(regs, build, object, IrOp::default(), value, value_op, ratag);
        }
      }
      IrCmd::CheckTag => {
        let hoist_223 = self.mem_reg_tag_op(inst.op(0));
        self.emit_cmp(hoist_223, OperandX64::imm((self.tag_op(inst.op(1))) as i32));
        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
          ConditionX64::NotEqual,
          inst.op(2),
          index,
          next,
        );
      }
      IrCmd::CheckTruthy => {
        {
          // 无需检查 boolean 值的常量 tag 本应已被常量折叠移除
          CODEGEN_ASSERT!(
            inst.op(0).kind() != IrOpKind::Constant
              || self.tag_op(inst.op(0)) == LuaType::Boolean as u8
          );

          let mut skip = Label::default();

          if inst.op(0).kind() != IrOpKind::Constant {
            // 'nil'（falsy）时 fallback
            let hoist_224 = self.mem_reg_tag_op(inst.op(0));
            self.emit_cmp(hoist_224, OperandX64::imm((LuaType::Nil as u8) as i32));
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              ConditionX64::Equal,
              inst.op(2),
              index,
              next,
            );

            // 非 boolean（truthy）则跳过值测试
            let hoist_225 = self.mem_reg_tag_op(inst.op(0));
            self.emit_cmp(hoist_225, OperandX64::imm((LuaType::Boolean as u8) as i32));
            self.build_mut().jcc(ConditionX64::NotEqual, &mut skip);
          }

          // 'false' 布尔值（falsy）时 fallback
          if inst.op(1).kind() != IrOpKind::Constant {
            let hoist_226 = self.mem_reg_uint_op(inst.op(1));
            self.emit_cmp(hoist_226, OperandX64::imm(0_i32));
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              ConditionX64::Equal,
              inst.op(2),
              index,
              next,
            );
          } else {
            if self.int_op(inst.op(1)) == 0 {
              self.jump_or_abort_on_undef_ir_op_u32_ir_block(inst.op(2), index, next);
            }
          }

          if inst.op(0).kind() != IrOpKind::Constant {
            self.build_mut().set_label(&mut skip);
          }
        }
      }
      IrCmd::CheckReadonly => self.lower_check_table_field(inst, index, next, true),
      IrCmd::CheckNoMetatable => self.lower_check_table_field(inst, index, next, false),
      IrCmd::CheckSafeEnv => {
        self.check_safe_env(inst.op(0), index, next);
      }
      IrCmd::CheckArraySize => {
        if inst.op(1).kind() == IrOpKind::Inst {
          let hoist_229 = self.reg_op(inst.op(0));
          let hoist_230 = self.reg_op(inst.op(1));
          self.emit_cmp(
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              hoist_229,
              (offset_of!(LuaTable, sizearray) as i32),
            ),
            OperandX64::reg(hoist_230),
          );
        } else if inst.op(1).kind() == IrOpKind::Constant {
          let hoist_231 = self.reg_op(inst.op(0));
          self.emit_cmp(
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              hoist_231,
              (offset_of!(LuaTable, sizearray) as i32),
            ),
            OperandX64::imm(self.int_op(inst.op(1))),
          );
        } else {
          unsupported_instruction_form();
        }

        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
          ConditionX64::BelowEqual,
          inst.op(2),
          index,
          next,
        );
      }
      IrCmd::JumpSlotMatch | IrCmd::CheckSlotMatch => {
        {
          // 未绑定哨兵: location = !0 (对齐 cpp `Label fresh;`), 由 place_label 走 fixup
          let mut abort = Label::default(); // Used when guard aborts execution
          let mismatch_op = if inst.cmd == IrCmd::JumpSlotMatch {
            inst.op(3)
          } else {
            inst.op(2)
          };
          // 保留：mismatch 需在后续 alloc_scoped_reg/reg_op 等 `&mut self` 调用间持有
          // label（跨调用别名窗口），借用检查无法表达，走 `label_op` 裸指针边界。
          let mismatch = if mismatch_op.kind() == IrOpKind::Undef {
            ptr::from_mut(&mut abort)
          } else {
            self.op_label_ptr(mismatch_op)
          };

          let tmp = self.alloc_scoped_reg(SizeX64::Qword);

          // 检查 node key tag 是否为 string
          let hoist_232 = self.reg_op(inst.op(0));
          self.emit_mov(
            OperandX64::reg(dword_reg(tmp.reg)),
            luau_node_key_tag(hoist_232),
          );
          self.build_mut().and_(
            OperandX64::reg(dword_reg(tmp.reg)),
            OperandX64::imm(K_TKEY_TAG_MASK),
          );
          self.build_mut().cmp(
            OperandX64::reg(dword_reg(tmp.reg)),
            OperandX64::imm((LuaType::String as u8) as i32),
          );
          self.with_target_label(mismatch, |s, l| {
            s.build_mut().jcc(ConditionX64::NotEqual, l)
          });

          // 校验 node key value 与期望值一致
          self.build_mut().mov(
            OperandX64::reg(tmp.reg),
            luau_constant_value(vm_const_op(inst.op(1))),
          );
          let hoist_233 = self.reg_op(inst.op(0));
          self.emit_cmp(OperandX64::reg(tmp.reg), luau_node_key_value(hoist_233));
          self.with_target_label(mismatch, |s, l| {
            s.build_mut().jcc(ConditionX64::NotEqual, l)
          });

          // 校验 node value 不为 nil
          let hoist_234 = self.reg_op(inst.op(0));
          self.emit_cmp(
            OperandX64::mem(
              SizeX64::Dword,
              RegisterX64::NOREG,
              1,
              hoist_234,
              (offset_of!(LuaNode, val) as i32) + (offset_of!(TValue, tt) as i32),
            ),
            OperandX64::imm((LuaType::Nil as u8) as i32),
          );
          self.with_target_label(mismatch, |s, l| s.build_mut().jcc(ConditionX64::Equal, l));

          if inst.cmd == IrCmd::JumpSlotMatch {
            self.jump_or_fallthrough_op(inst.op(2), next);
          } else if mismatch_op.kind() == IrOpKind::Undef {
            let mut skip = Label::default();
            self.build_mut().jmp_label(&mut skip);
            self.build_mut().set_label(&mut abort);
            self.build_mut().ud_2();
            self.build_mut().set_label(&mut skip);
          }
        }
      }
      IrCmd::CheckNodeNoNext => {
        let tmp = self.alloc_scoped_reg(SizeX64::Dword);

        let hoist_235 = self.reg_op(inst.op(0));
        self.emit_mov(
          OperandX64::reg(tmp.reg),
          OperandX64::mem(
            SizeX64::Dword,
            RegisterX64::NOREG,
            1,
            hoist_235,
            (offset_of!(LuaNode, key) as i32) + K_OFFSET_OF_TKEY_TAG_NEXT,
          ),
        );
        self
          .build_mut()
          .shr(OperandX64::reg(tmp.reg), OperandX64::imm(K_TKEY_TAG_BITS));
        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
          ConditionX64::NotZero,
          inst.op(1),
          index,
          next,
        );
      }
      IrCmd::CheckNodeValue => {
        let hoist_236 = self.reg_op(inst.op(0));
        self.emit_cmp(
          OperandX64::mem(
            SizeX64::Dword,
            RegisterX64::NOREG,
            1,
            hoist_236,
            (offset_of!(LuaNode, val) as i32) + (offset_of!(TValue, tt) as i32),
          ),
          OperandX64::imm((LuaType::Nil as u8) as i32),
        );
        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
          ConditionX64::Equal,
          inst.op(1),
          index,
          next,
        );
      }
      IrCmd::CheckBufferLen => {
        {
          let min_offset = self.int_op(inst.op(2));
          let max_offset = self.int_op(inst.op(3));
          CODEGEN_ASSERT!(min_offset < max_offset);

          let access_size = max_offset - min_offset;
          CODEGEN_ASSERT!(access_size > 0);

          // 确定将需要的寄存器
          let has_integer_check = inst.op(4).kind() != IrOpKind::Undef;
          let needs_extended_bounds_regs =
            inst.op(1).kind() == IrOpKind::Inst && !(access_size == 1 && min_offset == 0);

          // 要让跳转到 exit sync block 成立，每个可能被取分支都需相同的寄存器分配状态
          let reg_a = if inst.op(0).kind() == IrOpKind::Inst {
            self.reg_op(inst.op(0))
          } else {
            RegisterX64::NOREG
          };
          let reg_b = if inst.op(1).kind() == IrOpKind::Inst {
            self.reg_op(inst.op(1))
          } else {
            RegisterX64::NOREG
          };
          let reg_e = if has_integer_check {
            self.reg_op(inst.op(4))
          } else {
            RegisterX64::NOREG
          };

          let mut tmp_xmm = self.scoped_reg();
          let mut tmp1 = self.scoped_reg();
          let mut tmp2 = self.scoped_reg();

          if has_integer_check {
            tmp_xmm.alloc(SizeX64::Xmmword);
          }

          if needs_extended_bounds_regs {
            tmp1.alloc(SizeX64::Qword);
            tmp2.alloc(SizeX64::Dword);
          }

          let mut fresh = Label::default();

          // 检查它是否不仅是 size guard，还是 offset 恰为整数的 guard
          if has_integer_check {
            CODEGEN_ASSERT!(
              get_cmd_value_kind(self.function_mut().inst_op(inst.op(1)).cmd) == IrValueKind::Int
            );
            CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
              self.function_mut().inst_op(inst.op(1)).cmd
            )); // Ensure that high register bits are cleared

            // 把整数转回 double
            self.build_mut().vcvtsi2sd(
              OperandX64::reg(tmp_xmm.reg),
              OperandX64::reg(tmp_xmm.reg),
              OperandX64::reg(reg_b),
            );

            self
              .build_mut()
              .vucomisd(OperandX64::reg(tmp_xmm.reg), OperandX64::reg(reg_e)); // Sets ZF=1 if equal or NaN, PF=1 on NaN

            // 不允许非整数值
            self.jump_or_abort_on_undef_no_finalize(
              ConditionX64::NotZero,
              inst.op(5),
              index,
              next,
              &mut fresh,
            ); // exit on ZF=0
            self.jump_or_abort_on_undef_no_finalize(
              ConditionX64::Parity,
              inst.op(5),
              index,
              next,
              &mut fresh,
            ); // exit on PF=1
          }

          if inst.op(1).kind() == IrOpKind::Inst {
            CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
              self.function_mut().inst_op(inst.op(1)).cmd
            )); // Ensure that high register bits are cleared

            if access_size == 1 && min_offset == 0 {
              // 单字节访问的更简单检查
              self.build_mut().cmp(
                OperandX64::mem(
                  SizeX64::Dword,
                  RegisterX64::NOREG,
                  1,
                  reg_a,
                  K_BUFFER_LEN_OFFSET,
                ),
                OperandX64::reg(reg_b),
              );
              self.jump_or_abort_on_undef_no_finalize(
                ConditionX64::BelowEqual,
                inst.op(5),
                index,
                next,
                &mut fresh,
              );
            } else {
              // 为用单分支完成边界检查，取限制在 32 位内的 index
              // 最大偏移再用 64 位加法累加
              // 这能保证 0xffffffff 之类的值加法不回绕

              if min_offset >= 0 {
                self.build_mut().lea_operand_x_64_operand_x_64(
                  OperandX64::reg(tmp1.reg),
                  OperandX64::mem(
                    SizeX64::None,
                    RegisterX64::NOREG,
                    1,
                    qword_reg(reg_b),
                    max_offset,
                  ),
                );
              } else {
                // min offset 为负时，先在 32 位内把它从 offset 减去
                self.build_mut().lea_operand_x_64_operand_x_64(
                  OperandX64::reg(dword_reg(tmp1.reg)),
                  OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, reg_b, min_offset),
                );

                // 然后像之前一样加上完整访问尺寸
                self.build_mut().lea_operand_x_64_operand_x_64(
                  OperandX64::reg(tmp1.reg),
                  OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, tmp1.reg, access_size),
                );
              }

              self.build_mut().mov(
                OperandX64::reg(tmp2.reg),
                OperandX64::mem(
                  SizeX64::Dword,
                  RegisterX64::NOREG,
                  1,
                  reg_a,
                  K_BUFFER_LEN_OFFSET,
                ),
              );
              self.build_mut().cmp(
                OperandX64::reg(qword_reg(tmp2.reg)),
                OperandX64::reg(tmp1.reg),
              );
              self.jump_or_abort_on_undef_no_finalize(
                ConditionX64::Below,
                inst.op(5),
                index,
                next,
                &mut fresh,
              );
            }
          } else if inst.op(1).kind() == IrOpKind::Constant {
            let offset = self.int_op(inst.op(1));

            // cpp IrLoweringX64.cpp:2537: LuauCodegenFixBufferLenCheck 已定值, endOffset = maxOffset
            let end_offset = max_offset;

            // 常量折叠本可处理，但这里仍为安全防溢出/下溢
            if offset < 0 || ((offset) as u32) + ((end_offset) as u32) >= ((INT_MAX) as u32) {
              self.jump_or_abort_on_undef_no_finalize(
                ConditionX64::Count,
                inst.op(5),
                index,
                next,
                &mut fresh,
              );
            } else {
              self.build_mut().cmp(
                OperandX64::mem(
                  SizeX64::Dword,
                  RegisterX64::NOREG,
                  1,
                  reg_a,
                  K_BUFFER_LEN_OFFSET,
                ),
                OperandX64::imm(offset + end_offset),
              );
            }

            self.jump_or_abort_on_undef_no_finalize(
              ConditionX64::Below,
              inst.op(5),
              index,
              next,
              &mut fresh,
            );
          } else {
            unsupported_instruction_form();
          }

          self.finalize_target_label(inst.op(5), index, &mut fresh);
        }
      }
      IrCmd::CheckUserdataTag => {
        let hoist_237 = self.reg_op(inst.op(0));
        self.emit_cmp(
          OperandX64::mem(
            SizeX64::Byte,
            RegisterX64::NOREG,
            1,
            hoist_237,
            (offset_of!(Udata, tag) as i32),
          ),
          OperandX64::imm(self.int_op(inst.op(1))),
        );
        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
          ConditionX64::NotEqual,
          inst.op(2),
          index,
          next,
        );
      }
      IrCmd::CheckCmpNum => {
        let cond = condition_op(inst.op(2));

        let mut fresh = Label::default();
        let fail = self.get_target_label(inst.op(3), index, &mut fresh);

        let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);

        // 两侧操作数先经 mem_reg_double_op 取值（常量侧的 f64 立即数发射发生在借派生
        // 之前，与原实参求值序一致），再走 `build_mut` 门面单点可变借用；fail 为局部标签
        // 可变借用，与 build 不相互别名。
        let hoist_lhs = self.mem_reg_double_op(inst.op(0));
        let hoist_rhs = self.mem_reg_double_op(inst.op(1));
        self.with_target_label(fail, |s, l| {
          jump_on_number_cmp(
            s.build_mut(),
            tmp.reg,
            hoist_lhs,
            hoist_rhs,
            get_negated_condition_ir_condition(cond),
            l,
            false,
          );
        });

        self.finalize_target_label(inst.op(3), index, &mut fresh);
      }
      IrCmd::CheckCmpInt => {
        self.lower_check_cmp_int(inst, index, next, false);
      }
      IrCmd::INTERRUPT => {
        {
          let pcpos = self.uint_op(inst.op(0));

          // 这里无条件 spill 所有值，从而合成 interrupt handler 代码时可忽略寄存器状态
          // 若能以某种方式单独记录 interrupt handler 代码，将来可改
          // 反正中断是循环边或 call/ret，此处没有明显的寄存器复用机会
          self.regs.preserve_and_free_inst_values();

          let tmp = self.alloc_scoped_reg(SizeX64::Qword);

          let mut self_lbl = Label::default();

          self.build_mut().mov(
            OperandX64::reg(tmp.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_STATE,
              (offset_of!(LuaState, global) as i32),
            ),
          );
          self.build_mut().cmp(
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp.reg,
              (offset_of!(global_State, cb.interrupt) as i32),
            ),
            OperandX64::imm(0_i32),
          );
          self.build_mut().jcc(ConditionX64::NotEqual, &mut self_lbl);

          let mut next = Label::default();
          self.build_mut().set_label(&mut next);

          self.interrupt_handlers.push(InterruptHandler {
            self_: self_lbl,
            pcpos,
            next,
          });
        }
      }
      IrCmd::CheckGc => {
        // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
        // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
        let (build, regs) = self.build_regs_mut();
        call_step_gc(regs, build);
      }
      IrCmd::BarrierObj => {
        let object_op = inst.op(0);
        let object = self.reg_op(object_op);
        let value_op = inst.op(1);
        let tag_op = inst.op(2);
        let ratag = if tag_op.kind() == IrOpKind::Undef {
          -1
        } else {
          self.tag_op(tag_op) as i32
        };
        // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
        // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
        let (build, regs) = self.build_regs_mut();
        call_barrier_object(
          regs,
          build,
          object,
          object_op,
          RegisterX64::NOREG,
          value_op,
          ratag,
        );
      }
      IrCmd::BarrierTableBack => {
        let table_op = inst.op(0);
        let table = self.reg_op(table_op);
        // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
        // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
        let (build, regs) = self.build_regs_mut();
        call_barrier_table_fast(regs, build, table, table_op);
      }
      IrCmd::BarrierTableForward => {
        let mut skip = Label::default();

        let mut tmp = self.alloc_scoped_reg(SizeX64::Qword);

        // 操作数均为 function 只读取值，先拷即释；再经 `build_mut` 门面单点派生唯一
        // 可变借用，skip 为局部标签借用，与化前实参求值序效应等价。
        let hoist_bar_obj = self.reg_op(inst.op(0));
        let hoist_bar_value_op = inst.op(1);
        let hoist_bar_tm = if inst.op(2).kind() == IrOpKind::Undef {
          -1
        } else {
          self.tag_op(inst.op(2)) as i32
        };
        check_object_barrier_conditions(
          self.build_mut(),
          tmp.reg,
          hoist_bar_obj,
          RegisterX64::NOREG,
          hoist_bar_value_op,
          hoist_bar_tm,
          &mut skip,
        );

        {
          let _spill_guard = ScopedSpills::new(&mut self.regs);

          let mut call_wrap = self.call_wrap_state(index);
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(self.reg_op(inst.op(0))),
            inst.op(0),
          );
          call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp);
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_NATIVE_CONTEXT,
            (offset_of!(NativeContext, lua_c_barriertable) as i32),
          ));
        }

        self.build_mut().set_label(&mut skip);
      }
      IrCmd::SetSavedpc => {
        let tmp1 = self.alloc_scoped_reg(SizeX64::Qword);
        let tmp2 = self.alloc_scoped_reg(SizeX64::Qword);

        self.build_mut().mov(OperandX64::reg(tmp2.reg), s_code());
        self.emit_add(
          OperandX64::reg(tmp2.reg),
          OperandX64::imm(
            // cpp `uintOp * sizeof(Instruction)`：无符号环绕语义，用 wrapping 对齐
            (self.uint_op(inst.op(0)) as i32).wrapping_mul(size_of::<Instruction>() as i32),
          ),
        );
        self.build_mut().mov(
          OperandX64::reg(tmp1.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_STATE,
            (offset_of!(LuaState, ci) as i32),
          ),
        );
        self.build_mut().mov(
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            tmp1.reg,
            (offset_of!(CallInfo, savedpc) as i32),
          ),
          OperandX64::reg(tmp2.reg),
        );
      }
      IrCmd::CloseUpvals => {
        {
          let mut next = Label::default();
          let mut tmp1 = self.alloc_scoped_reg(SizeX64::Qword);
          let mut tmp2 = self.alloc_scoped_reg(SizeX64::Qword);

          // l->openupval != 0
          self.build_mut().mov(
            OperandX64::reg(tmp1.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_STATE,
              (offset_of!(LuaState, openupval) as i32),
            ),
          );
          self
            .build_mut()
            .test(OperandX64::reg(tmp1.reg), OperandX64::reg(tmp1.reg));
          self.build_mut().jcc(ConditionX64::Zero, &mut next);

          // ra <= l->openupval->v
          self.build_mut().lea_operand_x_64_operand_x_64(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(
              SizeX64::None,
              RegisterX64::NOREG,
              1,
              R_BASE,
              vm_reg_op(inst.op(0)) * (size_of::<TValue>() as i32),
            ),
          );
          self.build_mut().cmp(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              tmp1.reg,
              (offset_of!(UpVal, v) as i32),
            ),
          );
          self.build_mut().jcc(ConditionX64::Above, &mut next);

          tmp1.free();

          {
            let _spill_guard = ScopedSpills::new(&mut self.regs);

            let mut call_wrap = self.call_wrap_state(index);
            call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp2);
            call_wrap.call(&OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              R_NATIVE_CONTEXT,
              (offset_of!(NativeContext, lua_f_close) as i32),
            ));
          }

          self.build_mut().set_label(&mut next);
        }
      }
      IrCmd::CAPTURE => {
        // 目前是空操作

        // 回退到非 IR 指令实现
      }
      IrCmd::SETLIST => {
        self.regs.assert_all_free();
        let ra = vm_reg_op(inst.op(1));
        let rb = vm_reg_op(inst.op(2));
        let count = self.int_op(inst.op(3));
        let index = self.uint_op(inst.op(4));
        let aux = if inst.op(5).kind() == IrOpKind::Undef {
          -1
        } else {
          self.uint_op(inst.op(5)) as i32
        };
        // Safety: 混窗元组门面（build↔regs 共存别名，见 records 契约）；两视图即时
        // 消费于本调用，与化前 `(&mut self.regs, &mut *self.build)` 实参列逐位等价。
        let (build, regs) = self.build_regs_mut();
        emit_inst_set_list(regs, build, ra, rb, count, index, aux);
      }
      IrCmd::CALL => {
        self.regs.assert_all_free();
        self.regs.assert_no_spills();
        let ra = vm_reg_op(inst.op(0));
        let nparams = self.int_op(inst.op(1));
        let nresults = self.int_op(inst.op(2));
        self.with_build_regs_helpers(|b, r, h| emit_inst_call(r, b, h, ra, nparams, nresults));
      }
      IrCmd::RETURN => {
        self.regs.assert_all_free();
        self.regs.assert_no_spills();
        let reg = vm_reg_op(inst.op(0));
        let n = self.int_op(inst.op(1));
        let variadic = self.function_ref().variadic;
        self.with_build_helpers(|b, h| emit_inst_return(b, h, reg, n, variadic));
      }
      IrCmd::FORGLOOP => {
        self.regs.assert_all_free();
        let ra = vm_reg_op(inst.op(0));
        let aux = self.int_op(inst.op(1));
        let target = self.op_label_ptr(inst.op(2));
        self.with_target_label(target, |s, l| {
          let (build, regs) = s.build_regs_mut();
          emit_inst_for_g_loop(regs, build, ra, aux, l);
        });
        self.jump_or_fallthrough_op(inst.op(3), next);
      }
      IrCmd::ForgloopFallback => {
        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(vm_reg_op(inst.op(0))),
          IrOp::default(),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(self.int_op(inst.op(1))),
          IrOp::default(),
        );

        // cpp IrLoweringX64.cpp:2725-2741: LuauYieldIter2 已定值, 恒用三态 int 版 fallback
        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, forg_loop_non_table_fallback) as i32),
        ));

        emit_update_base(self.build_mut());

        self.build_mut().test(
          OperandX64::reg(dword_reg(RegisterX64::RAX)),
          OperandX64::reg(dword_reg(RegisterX64::RAX)),
        );
        self.with_helper_label(
          |h| &mut h.exit_no_continue_vm,
          |s, l| {
            s.emit_jcc(ConditionX64::Less, l);
          },
        );
        self.with_op_label(inst.op(2), |s, l| {
          s.build_mut().jcc(ConditionX64::Greater, l)
        });

        self.jump_or_fallthrough_op(inst.op(3), next);
      }
      IrCmd::ForgprepXnextFallback => {
        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Qword,
          luau_reg_address(vm_reg_op(inst.op(1))),
          IrOp::default(),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(self.uint_op(inst.op(0)) as i32 + 1),
          IrOp::default(),
        );
        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, forg_prep_xnext_fallback) as i32),
        ));
        self.jump_or_fallthrough_op(inst.op(2), next);
      }
      IrCmd::COVERAGE => {
        {
          let tmp1 = self.alloc_scoped_reg(SizeX64::Qword);
          let tmp2 = self.alloc_scoped_reg(SizeX64::Dword);
          let tmp3 = self.alloc_scoped_reg(SizeX64::Dword);

          self.build_mut().mov(OperandX64::reg(tmp1.reg), s_code());
          self.emit_add(
            OperandX64::reg(tmp1.reg),
            OperandX64::imm(
              // cpp `uintOp * sizeof(Instruction)`：无符号环绕语义，用 wrapping 对齐
              (self.uint_op(inst.op(0)) as i32).wrapping_mul(size_of::<Instruction>() as i32),
            ),
          );

          // hits = LUAU_INSN_E(*pc)
          self.build_mut().mov(
            OperandX64::reg(tmp2.reg),
            OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 1, tmp1.reg, 0),
          );
          self
            .build_mut()
            .sar(OperandX64::reg(tmp2.reg), OperandX64::imm(8_i32));

          // hits = if (hits < (1 << 23) - 1) { hits + 1 } else { hits };
          self
            .build_mut()
            .xor_(OperandX64::reg(tmp3.reg), OperandX64::reg(tmp3.reg));
          self
            .build_mut()
            .cmp(OperandX64::reg(tmp2.reg), OperandX64::imm((1 << 23) - 1));
          self
            .build_mut()
            .setcc(ConditionX64::NotEqual, OperandX64::reg(byte_reg(tmp3.reg)));
          self
            .build_mut()
            .add(OperandX64::reg(tmp2.reg), OperandX64::reg(tmp3.reg));

          // vm_patch_e(pc, hits);
          self
            .build_mut()
            .sal(OperandX64::reg(tmp2.reg), OperandX64::imm(8_i32));
          self.build_mut().movzx(
            tmp3.reg,
            OperandX64::mem(SizeX64::Byte, RegisterX64::NOREG, 1, tmp1.reg, 0),
          );
          self
            .build_mut()
            .or_(OperandX64::reg(tmp3.reg), OperandX64::reg(tmp2.reg));
          self.build_mut().mov(
            OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 1, tmp1.reg, 0),
            OperandX64::reg(tmp3.reg),
          );
        }

        // 完整指令 fallback
      }
      IrCmd::FallbackGetglobal => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(2).kind() == IrOpKind::VmConst);
        self.lower_fallback(inst, offset_of!(NativeContext, execute_getglobal));
      }
      IrCmd::FallbackSetglobal => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(2).kind() == IrOpKind::VmConst);
        self.lower_fallback(inst, offset_of!(NativeContext, execute_setglobal));
      }
      IrCmd::FallbackGettableks => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(2).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(3).kind() == IrOpKind::VmConst);
        self.lower_fallback(inst, offset_of!(NativeContext, execute_gettableks));
      }
      IrCmd::FallbackSettableks => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(2).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(3).kind() == IrOpKind::VmConst);
        self.lower_fallback(inst, offset_of!(NativeContext, execute_settableks));
      }
      IrCmd::FallbackNamecall => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(2).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(3).kind() == IrOpKind::VmConst);
        self.lower_fallback(inst, offset_of!(NativeContext, execute_namecall));
      }
      IrCmd::FallbackPrepvarargs => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::Constant);
        self.lower_fallback(inst, offset_of!(NativeContext, execute_prepvarargs));
      }
      IrCmd::FallbackGetvarargs => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(2).kind() == IrOpKind::Constant);

        if self.int_op(inst.op(2)) == LUA_MULTRET {
          let mut call_wrap = self.call_wrap_state(index);

          let reg = call_wrap.suggest_next_argument_register(SizeX64::Qword);
          self.build_mut().mov(OperandX64::reg(reg), s_code());
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              reg,
              // cpp `uintOp * sizeof(Instruction)`：无符号环绕语义，用 wrapping 对齐
              (self.uint_op(inst.op(0)) as i32).wrapping_mul(size_of::<Instruction>() as i32),
            ),
            IrOp::default(),
          );

          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(R_BASE),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(vm_reg_op(inst.op(1))),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_NATIVE_CONTEXT,
            (offset_of!(NativeContext, execute_getvarargsmult_ret) as i32),
          ));

          emit_update_base(self.build_mut());
        } else {
          let mut call_wrap = self.call_wrap_state(index);
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Qword,
            OperandX64::reg(R_BASE),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(vm_reg_op(inst.op(1))),
            IrOp::default(),
          );
          call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::Dword,
            OperandX64::imm(self.int_op(inst.op(2))),
            IrOp::default(),
          );
          call_wrap.call(&OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_NATIVE_CONTEXT,
            (offset_of!(NativeContext, execute_getvarargsconst) as i32),
          ));
        }
      }
      IrCmd::NEWCLOSURE => {
        let mut tmp2 = self.alloc_scoped_reg(SizeX64::Qword);
        self.build_mut().mov(OperandX64::reg(tmp2.reg), s_closure());
        self.build_mut().mov(
          OperandX64::reg(tmp2.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            tmp2.reg,
            K_CLOSURE_LPOFFSET,
          ),
        );
        self.build_mut().mov(
          OperandX64::reg(tmp2.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            tmp2.reg,
            (offset_of!(Proto, p) as i32),
          ),
        );
        self.emit_mov(
          OperandX64::reg(tmp2.reg),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            tmp2.reg,
            // cpp `uintOp * sizeof(Proto)`：无符号环绕语义，用 wrapping 对齐
            (self.uint_op(inst.op(2)) as i32).wrapping_mul(K_NATIVE_PTR_SIZE as i32),
          ),
        );

        let mut call_wrap = self.call_wrap_state(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Dword,
          OperandX64::imm(self.uint_op(inst.op(0)) as i32),
          inst.op(0),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Qword,
          OperandX64::reg(self.reg_op(inst.op(1))),
          inst.op(1),
        );
        call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::Qword, &mut tmp2);

        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          (offset_of!(NativeContext, lua_f_new_lclosure) as i32),
        ));

        inst.reg_x64 = self.regs.take_reg(RegisterX64::RAX, index);
      }
      IrCmd::FallbackDupclosure => {
        CODEGEN_ASSERT!(inst.op(1).kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(inst.op(2).kind() == IrOpKind::VmConst);
        self.lower_fallback(inst, offset_of!(NativeContext, execute_dupclosure));
      }
      IrCmd::FallbackForgprep => {
        self.lower_fallback(inst, offset_of!(NativeContext, execute_forgprep));
        self.jump_or_fallthrough_op(inst.op(2), next);
      }
      IrCmd::BitandUint | IrCmd::BitxorUint | IrCmd::BitorUint => {
        let emit = match inst.cmd {
          IrCmd::BitandUint => AssemblyBuilderX64::and_,
          IrCmd::BitxorUint => AssemblyBuilderX64::xor_,
          IrCmd::BitorUint => AssemblyBuilderX64::or_,
          _ => unreachable!(),
        };
        self.lower_bit_logic_uint(inst, index, emit);
      }
      IrCmd::BitnotUint => {
        self.lower_uint_load(inst, index);
        self.build_mut().not_(OperandX64::reg(inst.reg_x64));
      }
      IrCmd::BitlshiftUint
      | IrCmd::BitrshiftUint
      | IrCmd::BitarshiftUint
      | IrCmd::BitlrotateUint
      | IrCmd::BitrrotateUint => {
        let emit = match inst.cmd {
          IrCmd::BitlshiftUint => AssemblyBuilderX64::shl,
          IrCmd::BitrshiftUint => AssemblyBuilderX64::shr,
          IrCmd::BitarshiftUint => AssemblyBuilderX64::sar,
          IrCmd::BitlrotateUint => AssemblyBuilderX64::rol,
          IrCmd::BitrrotateUint => AssemblyBuilderX64::ror,
          _ => unreachable!(),
        };
        self.lower_bit_shift_uint(inst, index, emit);
      }
      IrCmd::BitcountlzUint | IrCmd::BitcountrzUint => {
        self.lower_bit_count(inst, index, inst.cmd == IrCmd::BitcountlzUint, false);
      }
      IrCmd::ByteswapUint => {
        self.lower_uint_load(inst, index);
        self.build_mut().bswap(inst.reg_x64);
      }
      IrCmd::InvokeLibm => {
        let mut call_wrap = self.call_wrap(index);
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Xmmword,
          self.mem_reg_double_op(inst.op(1)),
          inst.op(1),
        );

        if HAS_OP_C!(inst) {
          let is_int = if inst.op(2).kind() == IrOpKind::Constant {
            matches!(self.ir_lowering_x_64_const_op(inst.op(2)), IrConst::Int(_))
          } else {
            get_cmd_value_kind(self.function_mut().inst_op(inst.op(2)).cmd) == IrValueKind::Int
          };

          if is_int {
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Dword,
              self.mem_reg_uint_op(inst.op(2)),
              inst.op(2),
            );
          } else {
            call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
              SizeX64::Xmmword,
              self.mem_reg_double_op(inst.op(2)),
              inst.op(2),
            );
          }
        }

        call_wrap.call(&OperandX64::mem(
          SizeX64::Qword,
          RegisterX64::NOREG,
          1,
          R_NATIVE_CONTEXT,
          get_native_context_offset(self.uint_op(inst.op(0)) as i32) as i32,
        ));
        inst.reg_x64 = self.regs.take_reg(xmm0(), index);
      }
      IrCmd::GetType => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

        self.build_mut().mov(
          OperandX64::reg(inst.reg_x64),
          OperandX64::mem(
            SizeX64::Qword,
            RegisterX64::NOREG,
            1,
            R_STATE,
            (offset_of!(LuaState, global) as i32),
          ),
        );

        if inst.op(0).kind() == IrOpKind::Inst {
          let hoist_268 = self.reg_op(inst.op(0));
          self.emit_mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Qword,
              qword_reg(hoist_268),
              K_NATIVE_PTR_SIZE as u8,
              inst.reg_x64,
              (offset_of!(global_State, ttname) as i32),
            ),
          );
        } else if inst.op(0).kind() == IrOpKind::Constant {
          self.emit_mov(
            OperandX64::reg(inst.reg_x64),
            OperandX64::mem(
              SizeX64::Qword,
              RegisterX64::NOREG,
              1,
              inst.reg_x64,
              (offset_of!(global_State, ttname) as i32)
                + (self.tag_op(inst.op(0)) as i32) * (K_NATIVE_PTR_SIZE as i32),
            ),
          );
        } else {
          unsupported_instruction_form();
        }
      }
      IrCmd::GetTypeof => {
        self.lower_state_reg_helper(inst, index, offset_of!(NativeContext, lua_t_objtypenamestr));
      }
      IrCmd::FINDUPVAL => {
        self.lower_state_reg_helper(inst, index, offset_of!(NativeContext, lua_f_findupval));
      }
      IrCmd::BufferReadi8 => self.lower_buffer_read_int(inst, index, SizeX64::Byte, true),
      IrCmd::BufferReadu8 => self.lower_buffer_read_int(inst, index, SizeX64::Byte, false),
      IrCmd::BufferWritei8 => self.lower_buffer_write_int(inst, SizeX64::Byte),
      IrCmd::BufferReadi16 => self.lower_buffer_read_int(inst, index, SizeX64::Word, true),
      IrCmd::BufferReadu16 => self.lower_buffer_read_int(inst, index, SizeX64::Word, false),
      IrCmd::BufferWritei16 => self.lower_buffer_write_int(inst, SizeX64::Word),
      IrCmd::BufferReadi32 => self.lower_buffer_read_int(inst, index, SizeX64::Dword, false),
      IrCmd::BufferWritei32 => self.lower_buffer_write_int(inst, SizeX64::Dword),
      IrCmd::BufferReadf32 | IrCmd::BufferReadf64 | IrCmd::BufferReadi64 => {
        self.lower_buffer_read_scalar(inst, index);
      }
      IrCmd::BufferWritef32 => {
        let dst = sized_mem(
          self.buffer_addr_op(inst.op(0), inst.op(1), self.tag_op(inst.op(3))),
          SizeX64::Dword,
        );
        let src = inst.op(2);
        self.store_float(dst, src);
      }
      IrCmd::BufferWritef64 | IrCmd::BufferWritei64 => self.lower_buffer_write_scalar(inst),
      IrCmd::CheckDivInt64 => {
        {
          let tmp_a = self.alloc_scoped_reg(SizeX64::Qword);
          let tmp_b = self.alloc_scoped_reg(SizeX64::Qword);
          let hoist_286 = self.mem_reg_int_64_op(inst.op(0));
          self.emit_mov(OperandX64::reg(tmp_a.reg), hoist_286);
          let hoist_287 = self.mem_reg_int_64_op(inst.op(1));
          self.emit_mov(OperandX64::reg(tmp_b.reg), hoist_287);

          // 防护除零
          self
            .build_mut()
            .test(OperandX64::reg(tmp_b.reg), OperandX64::reg(tmp_b.reg));
          self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
            ConditionX64::Equal,
            inst.op(2),
            index,
            next,
          );

          // 防护 dividend == i64::MIN && divisor == -1（有符号溢出）
          {
            let mut skip = Label::default();

            self
              .build_mut()
              .cmp(OperandX64::reg(tmp_b.reg), OperandX64::imm(-1_i32));
            self.build_mut().jcc(ConditionX64::NotEqual, &mut skip);

            let tmp_min = self.alloc_scoped_reg(SizeX64::Qword);
            self.build_mut().mov64(tmp_min.reg, i64::MIN);
            self
              .build_mut()
              .cmp(OperandX64::reg(tmp_a.reg), OperandX64::reg(tmp_min.reg));
            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
              ConditionX64::Equal,
              inst.op(2),
              index,
              next,
            );

            self.build_mut().set_label(&mut skip);
          }
        }
      }
      IrCmd::CheckCmpInt64 => {
        self.lower_check_cmp_int(inst, index, next, true);
      }
      IrCmd::CmpInt64 => {
        {
          // 不能复用操作数寄存器作目标，因为比较前必须修改它
          inst.reg_x64 = self.regs.alloc_reg(SizeX64::Dword, index);

          // 将在字节寄存器上运算，它们写入时不清高位
          self
            .build_mut()
            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

          let cond = condition_op(inst.op(2));

          if inst.op(0).kind() == IrOpKind::Constant {
            let hoist_294 = self.reg_op(inst.op(1));
            let hoist_295 = self.mem_reg_int_64_op(inst.op(0));
            self.emit_cmp(OperandX64::reg(hoist_294), hoist_295);
            self.build_mut().setcc(
              get_inverse_condition(get_condition_int(cond)),
              OperandX64::reg(byte_reg(inst.reg_x64)),
            );
          } else if inst.op(0).kind() == IrOpKind::Inst {
            let hoist_296 = self.reg_op(inst.op(0));
            let hoist_297 = self.mem_reg_int_64_op(inst.op(1));
            self.emit_cmp(OperandX64::reg(hoist_296), hoist_297);
            self.build_mut().setcc(
              get_condition_int(cond),
              OperandX64::reg(byte_reg(inst.reg_x64)),
            );
          } else {
            unsupported_instruction_form();
          }
        }
      }
      IrCmd::NumToInt64 => {
        inst.reg_x64 = self.regs.alloc_reg(SizeX64::Qword, index);

        let hoist_299 = self.mem_reg_double_op(inst.op(0));
        self.emit_vcvttsd2si(OperandX64::reg(inst.reg_x64), hoist_299);
      }
      IrCmd::BitandInt64 | IrCmd::BitxorInt64 | IrCmd::BitorInt64 => {
        let emit = match inst.cmd {
          IrCmd::BitandInt64 => AssemblyBuilderX64::and_,
          IrCmd::BitxorInt64 => AssemblyBuilderX64::xor_,
          IrCmd::BitorInt64 => AssemblyBuilderX64::or_,
          _ => unreachable!(),
        };
        self.lower_bit_logic_int_64(inst, index, emit);
      }
      IrCmd::BitnotInt64 => {
        self.lower_int_64_load(inst, index);
        self.build_mut().not_(OperandX64::reg(inst.reg_x64));
      }
      IrCmd::BitlshiftInt64 => {
        self.lower_bit_shift_int_64(
          inst,
          index,
          AssemblyBuilderX64::shl,
          AssemblyBuilderX64::shr,
        );
      }
      IrCmd::BitrshiftInt64 => {
        self.lower_bit_shift_int_64(
          inst,
          index,
          AssemblyBuilderX64::shr,
          AssemblyBuilderX64::shl,
        );
      }
      IrCmd::BitarshiftInt64 => {
        {
          let mut shift_tmp = self.scoped_reg();

          if inst.op(1).kind() != IrOpKind::Constant {
            shift_tmp.take(RegisterX64::RCX);
          }

          inst.reg_x64 = self
            .regs
            .alloc_reg_or_reuse(SizeX64::Qword, index, &[inst.op(0)]);
          let op0 = inst.op(0);

          if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
            let hoist_311 = self.mem_reg_int_64_op(op0);
            self.emit_mov(OperandX64::reg(inst.reg_x64), hoist_311);
          }

          if inst.op(1).kind() == IrOpKind::Constant {
            let shift = self.int64_op(inst.op(1));

            if shift < -63 {
              // 左移 > 63 = 0
              self
                .build_mut()
                .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
            } else if shift < 0 {
              // 负 arshift = 按 -amount 左移
              self.build_mut().shl(
                OperandX64::reg(inst.reg_x64),
                OperandX64::imm(((-shift) as i8) as i32),
              );
            } else if shift > 63 {
              // 算术右移 > 63 = 符号填充
              self
                .build_mut()
                .sar(OperandX64::reg(inst.reg_x64), OperandX64::imm(63_i8 as i32));
            } else {
              self.build_mut().sar(
                OperandX64::reg(inst.reg_x64),
                OperandX64::imm(((shift) as i8) as i32),
              );
            }
          } else {
            let tmp = self.alloc_scoped_reg(SizeX64::Qword);

            let mut negative = Label::default();
            let mut out_of_range_positive = Label::default();
            let mut out_of_range_negative = Label::default();
            let mut done = Label::default();

            let hoist_312 = self.mem_reg_int_64_op(inst.op(1));
            self.emit_mov(OperandX64::reg(shift_tmp.reg), hoist_312);

            // amount > 63：符号填充
            self
              .build_mut()
              .cmp(OperandX64::reg(shift_tmp.reg), OperandX64::imm(63_i32));
            self
              .build_mut()
              .jcc(ConditionX64::Greater, &mut out_of_range_positive);

            // 检查 amount < -63：(amount + 63) < 0
            self.build_mut().lea_operand_x_64_operand_x_64(
              OperandX64::reg(tmp.reg),
              OperandX64::mem(SizeX64::None, RegisterX64::NOREG, 1, shift_tmp.reg, 63),
            );
            self
              .build_mut()
              .test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
            self
              .build_mut()
              .jcc(ConditionX64::Less, &mut out_of_range_negative);

            // 检查 amount 符号
            self.build_mut().test(
              OperandX64::reg(shift_tmp.reg),
              OperandX64::reg(shift_tmp.reg),
            );
            self.build_mut().jcc(ConditionX64::Less, &mut negative);

            // 算术右移
            self.build_mut().sar(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(byte_reg(shift_tmp.reg)),
            );
            self.build_mut().jmp_label(&mut done);

            // 按 -amount 左移
            self.build_mut().set_label(&mut negative);
            self.build_mut().neg(OperandX64::reg(shift_tmp.reg));
            self.build_mut().shl(
              OperandX64::reg(inst.reg_x64),
              OperandX64::reg(byte_reg(shift_tmp.reg)),
            );
            self.build_mut().jmp_label(&mut done);

            // amount > 63：符号填充（if n < 0 { -1 } else { 0 }）
            self.build_mut().set_label(&mut out_of_range_positive);
            self
              .build_mut()
              .sar(OperandX64::reg(inst.reg_x64), OperandX64::imm(63_i8 as i32));
            self.build_mut().jmp_label(&mut done);

            // amount < -63：结果为 0
            self.build_mut().set_label(&mut out_of_range_negative);
            self
              .build_mut()
              .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

            self.build_mut().set_label(&mut done);
          }
        }
      }
      IrCmd::BitlrotateInt64 | IrCmd::BitrrotateInt64 => {
        let emit = match inst.cmd {
          IrCmd::BitlrotateInt64 => AssemblyBuilderX64::rol,
          IrCmd::BitrrotateInt64 => AssemblyBuilderX64::ror,
          _ => unreachable!(),
        };
        self.lower_bit_rotate_int_64(inst, index, emit);
      }
      IrCmd::BitcountlzInt64 | IrCmd::BitcountrzInt64 => {
        self.lower_bit_count(inst, index, inst.cmd == IrCmd::BitcountlzInt64, true);
      }
      IrCmd::ByteswapInt64 => {
        self.lower_int_64_load(inst, index);
        self.build_mut().bswap(inst.reg_x64);
      }
      IrCmd::JumpCmpProtoid => {
        {
          CODEGEN_ASSERT!(inst.op(0).kind() == IrOpKind::Inst);
          let hoist_324 = self.reg_op(inst.op(0));
          self.emit_cmp(
            OperandX64::mem(
              SizeX64::Byte,
              RegisterX64::NOREG,
              1,
              hoist_324,
              (offset_of!(Closure, is_c) as i32),
            ),
            OperandX64::imm(1_i32),
          );
          self.with_op_label(inst.op(3), |s, l| s.build_mut().jcc(ConditionX64::Equal, l));
          {
            let tmp = self.alloc_scoped_reg(SizeX64::Qword);
            let hoist_325 = self.reg_op(inst.op(0));
            self.emit_mov(
              OperandX64::reg(tmp.reg),
              OperandX64::mem(
                SizeX64::Qword,
                RegisterX64::NOREG,
                1,
                hoist_325,
                K_CLOSURE_LPOFFSET,
              ),
            );
            self.emit_cmp(
              OperandX64::mem(
                SizeX64::Dword,
                RegisterX64::NOREG,
                1,
                tmp.reg,
                (offset_of!(Proto, funid) as i32),
              ),
              OperandX64::imm((self.uint_op(inst.op(1))) as i32),
            );
            self.with_op_label(inst.op(3), |s, l| {
              s.build_mut().jcc(ConditionX64::NotEqual, l)
            });
          }
          self.jump_or_fallthrough_op(inst.op(2), next);
        }

        // 伪指令
      }
      IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MarkUsed | IrCmd::MarkDead => {
        CODEGEN_ASSERT!(false, "Pseudo instructions should not be lowered");
      }
    }
    self.value_tracker.after_inst_lowering(inst, index);

    self.regs.curr_inst_idx = K_INVALID_INST_IDX;

    self.regs.free_last_use_regs(inst, index);
  }
}

impl IrLoweringX64 {
  // `self.build_mut().M(.., self.f(..))` 形在实参位再次借用 self，与 build_mut 的即时
  // 可变借用互斥（E0499/E0502）；改为 `self.emit_M(..)` 后，方法调用的双相借用（two-phase
  // calls）允许实参求值期短暂共享借用 self，语义与化前逐位一致（求值序不变、无新别名）。
  // 全部即时转调 `build_mut` 视图访问器，裸解引用仍收口在 records 唯一契约点。
  #[inline]
  fn emit_add(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.build_mut().add(lhs, rhs);
  }
  #[inline]
  fn emit_bsf(&mut self, dst: RegisterX64, src: OperandX64) {
    self.build_mut().bsf(dst, src);
  }
  #[inline]
  fn emit_bsr(&mut self, dst: RegisterX64, src: OperandX64) {
    self.build_mut().bsr(dst, src);
  }
  #[inline]
  fn emit_cmov(&mut self, cond: ConditionX64, lhs: RegisterX64, rhs: OperandX64) {
    self.build_mut().cmov(cond, lhs, rhs);
  }
  #[inline]
  fn emit_cmp(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.build_mut().cmp(lhs, rhs);
  }
  #[inline]
  fn emit_div(&mut self, op: OperandX64) {
    self.build_mut().div(op);
  }
  #[inline]
  fn emit_f32(&mut self, value: f32) -> OperandX64 {
    self.build_mut().f32(value)
  }
  #[inline]
  fn emit_f32x4(&mut self, x: f32, y: f32, z: f32, w: f32) -> OperandX64 {
    self.build_mut().f32x4(x, y, z, w)
  }
  #[inline]
  fn emit_f64(&mut self, value: f64) -> OperandX64 {
    self.build_mut().f64(value)
  }
  #[inline]
  fn emit_f64x2(&mut self, x: f64, y: f64) -> OperandX64 {
    self.build_mut().f64x2(x, y)
  }
  #[inline]
  fn emit_i32(&mut self, value: i32) -> OperandX64 {
    self.build_mut().i32(value)
  }
  #[inline]
  fn emit_i64(&mut self, value: i64) -> OperandX64 {
    self.build_mut().i64(value)
  }
  #[inline]
  fn emit_idiv(&mut self, op: OperandX64) {
    self.build_mut().idiv(op);
  }
  #[inline]
  fn emit_imul_operand_x_64_operand_x_64(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.build_mut().imul_operand_x_64_operand_x_64(lhs, rhs);
  }
  #[inline]
  fn emit_jcc(&mut self, cond: ConditionX64, label: &mut Label) {
    self.build_mut().jcc(cond, label);
  }
  #[inline]
  fn emit_lea_operand_x_64_operand_x_64(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.build_mut().lea_operand_x_64_operand_x_64(lhs, rhs);
  }
  #[inline]
  fn emit_mov(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.build_mut().mov(lhs, rhs);
  }
  #[inline]
  fn emit_mov64(&mut self, lhs: RegisterX64, imm: i64) {
    self.build_mut().mov64(lhs, imm);
  }
  #[inline]
  fn emit_movsx(&mut self, lhs: RegisterX64, rhs: OperandX64) {
    self.build_mut().movsx(lhs, rhs);
  }
  #[inline]
  fn emit_movzx(&mut self, lhs: RegisterX64, rhs: OperandX64) {
    self.build_mut().movzx(lhs, rhs);
  }
  #[inline]
  fn emit_sub(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.build_mut().sub(lhs, rhs);
  }
  #[inline]
  fn emit_test(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.build_mut().test(lhs, rhs);
  }
  #[inline]
  fn emit_u32x4(&mut self, x: u32, y: u32, z: u32, w: u32) -> OperandX64 {
    self.build_mut().u32x4(x, y, z, w)
  }
  #[inline]
  fn emit_vaddsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vaddsd(dst, src1, src2);
  }
  #[inline]
  fn emit_vandpd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vandpd(dst, src1, src2);
  }
  #[inline]
  fn emit_vandps(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vandps(dst, src1, src2);
  }
  #[inline]
  fn emit_vblendvpd(
    &mut self,
    dst: RegisterX64,
    src1: RegisterX64,
    src2: OperandX64,
    mask: RegisterX64,
  ) {
    self.build_mut().vblendvpd(dst, src1, src2, mask);
  }
  #[inline]
  fn emit_vblendvps(
    &mut self,
    dst: RegisterX64,
    src1: RegisterX64,
    src2: OperandX64,
    mask: RegisterX64,
  ) {
    self.build_mut().vblendvps(dst, src1, src2, mask);
  }
  #[inline]
  fn emit_vcmpeqsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vcmpeqsd(dst, src1, src2);
  }
  #[inline]
  fn emit_vcmpltsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vcmpltsd(dst, src1, src2);
  }
  #[inline]
  fn emit_vcmpltss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vcmpltss(dst, src1, src2);
  }
  #[inline]
  fn emit_vcvtsd2ss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vcvtsd2ss(dst, src1, src2);
  }
  #[inline]
  fn emit_vcvtsi2sd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vcvtsi2sd(dst, src1, src2);
  }
  #[inline]
  fn emit_vcvtsi2ss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vcvtsi2ss(dst, src1, src2);
  }
  #[inline]
  fn emit_vcvtss2sd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vcvtss2sd(dst, src1, src2);
  }
  #[inline]
  fn emit_vcvttsd2si(&mut self, dst: OperandX64, src: OperandX64) {
    self.build_mut().vcvttsd2si(dst, src);
  }
  #[inline]
  fn emit_vdivsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vdivsd(dst, src1, src2);
  }
  #[inline]
  fn emit_vfmadd213pd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vfmadd213pd(dst, src1, src2);
  }
  #[inline]
  fn emit_vmovaps(&mut self, dst: OperandX64, src: OperandX64) {
    self.build_mut().vmovaps(dst, src);
  }
  #[inline]
  fn emit_vmovsd_operand_x_64_operand_x_64(&mut self, dst: OperandX64, src: OperandX64) {
    self.build_mut().vmovsd_operand_x_64_operand_x_64(dst, src);
  }
  #[inline]
  fn emit_vmovsd_operand_x_64_operand_x_64_operand_x_64(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
  ) {
    self
      .build_mut()
      .vmovsd_operand_x_64_operand_x_64_operand_x_64(dst, src1, src2);
  }
  #[inline]
  fn emit_vmovss_operand_x_64_operand_x_64(&mut self, dst: OperandX64, src: OperandX64) {
    self.build_mut().vmovss_operand_x_64_operand_x_64(dst, src);
  }
  #[inline]
  fn emit_vmovss_operand_x_64_operand_x_64_operand_x_64(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
  ) {
    self
      .build_mut()
      .vmovss_operand_x_64_operand_x_64_operand_x_64(dst, src1, src2);
  }
  #[inline]
  fn emit_vmovups(&mut self, dst: OperandX64, src: OperandX64) {
    self.build_mut().vmovups(dst, src);
  }
  #[inline]
  fn emit_vmulsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vmulsd(dst, src1, src2);
  }
  #[inline]
  fn emit_vpextrd(&mut self, dst: RegisterX64, src: RegisterX64, offset: u8) {
    self.build_mut().vpextrd(dst, src, offset);
  }
  #[inline]
  fn emit_vpinsrd(&mut self, dst: RegisterX64, src1: RegisterX64, src2: OperandX64, offset: u8) {
    self.build_mut().vpinsrd(dst, src1, src2, offset);
  }
  #[inline]
  fn emit_vpshufps(&mut self, dst: RegisterX64, src1: RegisterX64, src2: OperandX64, shuffle: u8) {
    self.build_mut().vpshufps(dst, src1, src2, shuffle);
  }
  #[inline]
  fn emit_vroundsd(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    rounding_mode: RoundingModeX64,
  ) {
    self.build_mut().vroundsd(dst, src1, src2, rounding_mode);
  }
  #[inline]
  fn emit_vroundss(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    rounding_mode: RoundingModeX64,
  ) {
    self.build_mut().vroundss(dst, src1, src2, rounding_mode);
  }
  #[inline]
  fn emit_vsqrtsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vsqrtsd(dst, src1, src2);
  }
  #[inline]
  fn emit_vsqrtss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vsqrtss(dst, src1, src2);
  }
  #[inline]
  fn emit_vucomisd(&mut self, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vucomisd(src1, src2);
  }
  #[inline]
  fn emit_vxorpd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vxorpd(dst, src1, src2);
  }
  #[inline]
  fn emit_vxorps(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.build_mut().vxorps(dst, src1, src2);
  }
}
