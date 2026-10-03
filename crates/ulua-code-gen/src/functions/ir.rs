use ulua_common::macros::luau_assert::LUAU_UNLIKELY;
use ulua_vm::enums::lua_type::{LUA_T_COUNT, LuaType};

use crate::{
  enums::{
    ir::{IrConstKind, IrValueKind},
    ir_cmd::IrCmd,
    ir_condition::IrCondition,
    ir_op_kind::IrOpKind,
  },
  functions::{
    get_cmd_value_kind::get_cmd_value_kind,
    kill_ir_utils::{kill_ir_function_ir_block_at, kill_ir_function_ir_inst_at},
    replace_ir_utils::replace_ir_function_ir_op_ir_op_at,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    ir_operand::{HAS_OP_C, op_c_ref},
  },
  records::{
    ir_builder::IrBuilder, ir_const::IrConst, ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp,
  },
};

pub fn condition_op(op: IrOp) -> IrCondition {
  debug_assert!(op.kind() == IrOpKind::Condition);
  // 越界判别值钳制为 Count（与原实现一致）；界内经类型化静态表转换，替代裸 transmute
  IrCondition::from_discriminant(op.index().try_into().unwrap_or(u8::MAX))
    .unwrap_or(IrCondition::Count)
}

#[inline]
pub fn has_result(cmd: IrCmd) -> bool {
  get_cmd_value_kind(cmd) != IrValueKind::None
}

pub fn has_side_effects(cmd: IrCmd) -> bool {
  if cmd == IrCmd::InvokeFastcall {
    return true;
  }

  if is_pseudo(cmd) {
    return false;
  }

  // 不产出结果的指令几乎必然带有别的副作用才有意义
  // 目前做完整 switch 只会镜像 'hasResult' 函数，故用此简单条件
  !has_result(cmd)
}

pub fn is_gco(tag: u8) -> bool {
  CODEGEN_ASSERT!(tag < LUA_T_COUNT as u8);

  // 对应 VM lobject.h 的 iscollectable(o)
  tag >= LuaType::String as u8
}

#[inline]
pub const fn is_pseudo(cmd: IrCmd) -> bool {
  // 仅供内部使用、不参与最终 lowering 的指令
  matches!(
    cmd,
    IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MarkUsed | IrCmd::MarkDead
  )
}

pub fn is_compatible_constant(build: &mut IrBuilder, arg: IrOp, expected: IrConstKind) -> bool {
  arg.kind() != IrOpKind::Constant || build.function.const_op(arg).kind() == expected
}

pub fn try_get_operand_tag(function: &IrFunction, op: IrOp) -> Option<u8> {
  let arg = function.as_inst_op_ref(op)?;
  if arg.cmd == IrCmd::TagVector {
    return Some(LuaType::Vector as u8);
  }

  if arg.cmd == IrCmd::LoadTvalue && HAS_OP_C!(arg) {
    let op_c = op_c_ref(arg);
    return Some(function.tag_op(op_c));
  }

  None
}

pub fn get_const_value_kind(constant: &IrConst) -> IrValueKind {
  match constant {
    IrConst::Int(_) | IrConst::Uint(_) => IrValueKind::Int,
    IrConst::Int64(_) => IrValueKind::Int64,
    IrConst::Double(_) => IrValueKind::Double,
    IrConst::Tag(_) => IrValueKind::Tag,
    IrConst::Import(_) => {
      CODEGEN_ASSERT!(false, "Import constants cannot be used as IR values");
      IrValueKind::Unknown
    }
  }
}

#[inline]
pub fn get_op_mut(inst: &mut IrInst, idx: u32) -> &mut IrOp {
  if LUAU_UNLIKELY!(idx >= inst.ops.size()) {
    inst.ops.resize(idx + 1);
  }
  &mut inst.ops[idx as usize]
}

#[inline]
pub fn can_invalidate_safe_env(cmd: IrCmd) -> bool {
  match cmd {
        IrCmd::CmpAny
        | IrCmd::DoArith
        | IrCmd::DoLen
        | IrCmd::GetTable
        | IrCmd::SetTable
        | IrCmd::CONCAT // TODO: if only strings and numbers are concatenated, there will be no user calls
        | IrCmd::CALL
        | IrCmd::ForgloopFallback
        | IrCmd::FallbackGetglobal
        | IrCmd::FallbackSetglobal
        | IrCmd::FallbackGettableks
        | IrCmd::FallbackSettableks
        | IrCmd::FallbackNamecall
        | IrCmd::FallbackForgprep => true,
        _ => false,
    }
}

pub fn any_argument_match<F>(inst: &IrInst, mut func: F) -> bool
where
  F: FnMut(&IrOp) -> bool,
{
  if is_pseudo(inst.cmd) {
    return false;
  }

  for op in &inst.ops {
    if func(op) {
      return true;
    }
  }
  false
}

pub fn visit_arguments<F>(inst: &mut IrInst, mut func: F)
where
  F: FnMut(IrOp),
{
  if is_pseudo(inst.cmd) {
    return;
  }

  for op in inst.ops.iter() {
    func(*op);
  }
}

#[inline]
pub fn produces_dirty_high_register_bits(cmd: IrCmd) -> bool {
  matches!(
    cmd,
    IrCmd::NumToUint | IrCmd::InvokeFastcall | IrCmd::CmpAny
  )
}

#[inline]
pub fn count_instructions_with_cmd(instructions: &[IrInst], cmd: IrCmd) -> u32 {
  instructions.iter().filter(|inst| inst.cmd == cmd).count() as u32
}

/// See also: ulua-vm `macros/vm_kv::VM_KV!`——形似义异：该宏是解释器运行时的常量表
/// 槽寻址（code-gen 慢路径 `VmFrame::kv` 已直接单源复用它），本函数仅提取 `IrOp`
/// 的 VmConst 操作数编号，勿合并。
pub fn vm_const_op(op: IrOp) -> i32 {
  // 本 crate 当前状态下 CODEGEN_ASSERT 宏会因其内部调用
  // ulua_common 与 core::arch 而产生编译问题。
  // 这里参照 vmUpvalueOp 的做法，用标准 debug_assert!
  // 在返回下标前确保 IR 操作数是预期种类。
  debug_assert!(op.kind() == IrOpKind::VmConst);
  op.index() as i32
}

/// See also: ulua-vm `macros/vm_reg::VM_REG!`——形似义异：该宏是解释器运行时的
/// 栈槽寻址（base+i 解引用窗口），本函数仅提取 `IrOp` 的 VmReg 操作数编号，勿合并。
pub fn vm_reg_op(op: IrOp) -> i32 {
  debug_assert!(op.kind() == IrOpKind::VmReg);
  op.index() as i32
}

/// See also: ulua-vm `macros/vm_uv::VM_UV!`——形似义异：该宏是解释器运行时的
/// upvalue 柔性数组寻址，本函数仅提取 `IrOp` 的 VmUpvalue 操作数编号，勿合并。
pub fn vm_upvalue_op(op: IrOp) -> u32 {
  // 与原 C++ helper 保持相同的运行期检查行为。
  // 注：本 crate 中 `CODEGEN_ASSERT` 预期可用；此前报告的
  // 编译错误源于宏在本文件内展开，故这里不依赖它，
  // 改用局部断言。
  debug_assert!(op.kind() == IrOpKind::VmUpvalue);
  op.index()
}

pub fn vm_exit_op(op: IrOp) -> u32 {
  debug_assert!(op.kind() == IrOpKind::VmExit);
  op.index()
}

pub fn safe_integer_constant(value: f64) -> bool {
  // 32 位范围内；既允许最大无符号数，也允许其负数对应值
  // double 实际可支持更大范围（但并非精确 2^53），不过本函数用于 32 位优化
  if value < -4294967295.0 || value > 4294967295.0 {
    return false;
  }

  (value as i64 as f64) == value
}

pub fn get_initialized_fallback(build: &mut IrBuilder, fallback: &mut IrOp, pcpos: i32) -> IrOp {
  if fallback.kind() == IrOpKind::None {
    *fallback = build.fallback_block(pcpos as u32);
  }

  *fallback
}

/// 与原版逐操作 1:1：`get_op_mut` 的按需补位（仅越界扩容）与 kill 后写入丢弃
/// 均由索引化替换变体内部等价处理（见其文档注释），无需裸指针绕借用。
pub fn replace_ir_function_ir_inst_operand(
  function: &mut IrFunction,
  inst_idx: u32,
  op_idx: u32,
  replacement: IrOp,
) {
  replace_ir_function_ir_op_ir_op_at(function, inst_idx, op_idx, replacement);
}

pub fn add_use(function: &mut IrFunction, op: IrOp) {
  if op.kind() == IrOpKind::Inst {
    if op.index() as usize >= function.instructions.len() {
      eprintln!("[add-use-dbg] OOB inst idx={} len={}", op.index(), function.instructions.len());
    }
    function.instructions[op.index() as usize].use_count += 1;
  } else if op.kind() == IrOpKind::Block {
    function.blocks[op.index() as usize].use_count += 1;
  }
}

pub fn remove_use(function: &mut IrFunction, op: IrOp) {
  if op.kind() == IrOpKind::Inst {
    remove_inst_use(function, op.index());
  } else if op.kind() == IrOpKind::Block {
    remove_block_use(function, op.index());
  }
}

pub fn remove_inst_use(function: &mut IrFunction, inst_idx: u32) {
  let inst = &mut function.instructions[inst_idx as usize];

  CODEGEN_ASSERT!(inst.use_count != 0);
  inst.use_count -= 1;

  // 借用止于末次读；置零后走索引化 kill，函数可变借用不再与指令借用重叠
  let drained = inst.use_count == 0;
  if drained {
    kill_ir_function_ir_inst_at(function, inst_idx);
  }
}

pub fn remove_block_use(function: &mut IrFunction, block_idx: u32) {
  let block = &mut function.blocks[block_idx as usize];

  CODEGEN_ASSERT!(block.use_count != 0);
  block.use_count -= 1;

  // entry block 有隐式 use，永远不会被移除
  let kill = block.use_count == 0 && block_idx != 0;
  if kill {
    kill_ir_function_ir_block_at(function, block_idx as usize);
  }
}

pub fn update_last_use_locations_in_block(function: &mut IrFunction, block_idx: u32) {
  let block = &function.blocks[block_idx as usize];
  let start = block.start;
  let finish = block.finish;

  for inst_idx in start..=finish {
    let ops = function.instructions[inst_idx as usize].ops.clone();

    for op in ops.iter() {
      let op: IrOp = *op;
      if op.kind() == IrOpKind::Inst {
        function.instructions[op.index() as usize].last_use = inst_idx;
      }
    }
  }
}
