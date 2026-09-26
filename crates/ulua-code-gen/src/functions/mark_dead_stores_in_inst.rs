use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    is_gco::is_gco, is_non_terminating_jump::is_non_terminating_jump, reg_bitset::reg_bit_test,
    try_get_operand_tag::try_get_operand_tag,
    try_replace_tag_with_full_store::try_replace_tag_with_full_store,
    try_replace_value_with_full_store::try_replace_value_with_full_store,
    try_replace_vector_value_with_full_store::try_replace_vector_value_with_full_store,
    update_remaining_uses::update_remaining_uses, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_builder::ConstantMap,
    ir_data::K_UNKNOWN_TAG,
    ir_function::IrFunction,
    ir_op::IrOp,
    remove_dead_store_state::{
      RemoveDeadStoreState, kill_t_value_store_core, kill_tag_and_value_store_pair_core,
      kill_value_store_core, tag_value_pair_established,
    },
  },
  type_aliases::ir_ops::IrOps,
};

#[inline]
fn reg_captured(function: &IrFunction, reg: i32) -> bool {
  reg_bit_test(&function.cfg.captured.regs, reg as usize)
}

/// 部分（value）store 的公共登记：旧 tag 已知时其 value store 可被新 store 移除，
/// 本指令记为新的 value store，tag/value 成对后完整 store 记录作废。
/// `maybe_gco` 由 store 种类给出（指针按已知 tag 判定，标量/向量恒为假）。
fn note_value_store(
  state: &mut RemoveDeadStoreState,
  function: &mut IrFunction,
  reg: usize,
  index: u32,
  maybe_gco: bool,
) {
  // tag 已知时，部分 value store 可被新 store 移除
  if state.info[reg].known_tag != K_UNKNOWN_TAG {
    kill_value_store_core(function, &mut state.info[reg]);
  }

  let reg_info = &mut state.info[reg];
  reg_info.value_inst_idx = index;

  if tag_value_pair_established(reg_info) {
    reg_info.tvalue_inst_idx = !0u32;
  }

  reg_info.maybe_gco = maybe_gco;
  state.has_gco_to_clear |= maybe_gco;
}

/// 完整 TValue store 的公共登记：旧 tag/value/完整 store 全部回收，本指令成为唯一 tvalue store
fn note_tvalue_store(
  state: &mut RemoveDeadStoreState,
  function: &mut IrFunction,
  reg: usize,
  index: u32,
) {
  let reg_info = &mut state.info[reg];
  reg_info.ignore_at_exit = false;

  kill_tag_and_value_store_pair_core(function, reg_info);
  kill_t_value_store_core(function, reg_info);

  reg_info.tag_inst_idx = !0u32;
  reg_info.value_inst_idx = !0u32;

  reg_info.tvalue_inst_idx = index;
}

/// store 臂公共前奏：`op` 为未被闭包捕获的 VmReg 时返回其下标并清 `ignore_at_exit`。
/// 非 VmReg / 已捕获两种情形返回 `None`：cpp 对已捕获直接 `return`、对非常量槽位落入
/// 尾部 switch——store 类 cmd 恒不在尾部 GC-assist 列表内，两条路径的尾部均为 no-op，
/// 故此处统一折叠为跳过臂体，逐位等价。
fn store_arm_reg(
  state: &mut RemoveDeadStoreState,
  function: &IrFunction,
  op: IrOp,
) -> Option<usize> {
  if op.kind() != IrOpKind::VmReg {
    return None;
  }

  let reg = vm_reg_op(op) as usize;
  if reg_captured(function, reg as i32) {
    return None;
  }

  state.info[reg].ignore_at_exit = false;
  Some(reg)
}

// 移植自 IrVisitUseDef.h 的 `visitVmRegDefsUses<T>`，特化为 RemoveDeadStoreState 访问器。
// 指令以 (cmd, ops) 快照传入：函数视图独立可变借用，def 链经索引 kill，
// 不再持有函数数组槽位的长生命周期 `&mut IrInst`。
fn visit_vm_reg_defs_uses(
  state: &mut RemoveDeadStoreState,
  function: &mut IrFunction,
  cmd: IrCmd,
  ops: &IrOps,
) {
  // 为正确分析，必须先处理指令的全部 use，再处理定义
  match cmd {
    IrCmd::LoadTag
    | IrCmd::LoadPointer
    | IrCmd::LoadDouble
    | IrCmd::LoadInt
    | IrCmd::LoadInt64
    | IrCmd::LoadFloat
    | IrCmd::LoadTvalue => {
      state.maybe_use(ops[0]);
    }
    IrCmd::StoreTag
    | IrCmd::StoreExtra
    | IrCmd::StorePointer
    | IrCmd::StoreDouble
    | IrCmd::StoreInt
    | IrCmd::StoreInt64
    | IrCmd::StoreVector
    | IrCmd::StoreTvalue
    | IrCmd::StoreSplitTvalue => {
      state.maybe_def(function, ops[0]);
    }
    IrCmd::CmpAny => {
      state.use_(ops[0], 0);
      state.use_(ops[1], 0);
    }
    IrCmd::CmpTag => {
      state.maybe_use(ops[0]);
    }
    IrCmd::JumpIfTruthy | IrCmd::JumpIfFalsy => {
      state.use_(ops[0], 0);
    }
    IrCmd::JumpEqTag => {
      state.maybe_use(ops[0]);
    }
    IrCmd::DoArith => {
      state.maybe_use(ops[1]);
      state.maybe_use(ops[2]);
      state.def(function, ops[0], 0);
    }
    IrCmd::GetTable => {
      state.use_(ops[1], 0);
      state.maybe_use(ops[2]);
      state.def(function, ops[0], 0);
    }
    IrCmd::SetTable => {
      state.use_(ops[0], 0);
      state.use_(ops[1], 0);
      state.maybe_use(ops[2]);
    }
    IrCmd::DoLen => {
      state.use_(ops[1], 0);
      state.def(function, ops[0], 0);
    }
    IrCmd::GetCachedImport => {
      state.def(function, ops[0], 0);
    }
    IrCmd::CONCAT => {
      state.use_range(vm_reg_op(ops[0]), function.uint_op(ops[1]) as i32);
      state.def_range(function, vm_reg_op(ops[0]), function.uint_op(ops[1]) as i32);
    }
    IrCmd::GetUpvalue => {}
    IrCmd::SetUpvalue => {}
    IrCmd::INTERRUPT => {}
    IrCmd::BarrierObj | IrCmd::BarrierTableForward => {
      state.maybe_use(ops[1]);
    }
    IrCmd::CloseUpvals => {}
    IrCmd::CAPTURE => {
      state.maybe_use(ops[0]);

      if function.uint_op(ops[1]) == 1 {
        state.capture(vm_reg_op(ops[0]));
      }
    }
    IrCmd::SETLIST => {
      state.use_(ops[1], 0);
      state.use_range(vm_reg_op(ops[2]), function.int_op(ops[3]));
    }
    IrCmd::CALL => {
      state.use_(ops[0], 0);
      state.use_range(vm_reg_op(ops[0]) + 1, function.int_op(ops[1]));
      state.def_range(function, vm_reg_op(ops[0]), function.int_op(ops[2]));
    }
    IrCmd::RETURN => {
      state.use_range(vm_reg_op(ops[0]), function.int_op(ops[1]));
    }
    IrCmd::FASTCALL => {
      state.use_(ops[2], 0);
      state.def_range(function, vm_reg_op(ops[1]), function.int_op(ops[3]));
    }
    IrCmd::InvokeFastcall => {
      let count = function.int_op(ops[5]);
      if count != -1 {
        // 只有 LOP_FASTCALL3 的 lowering 允许第三个可选参数
        if count >= 3 && ops[4].kind() == IrOpKind::Undef {
          CODEGEN_ASSERT!(
            ops[3].kind() == IrOpKind::VmReg && vm_reg_op(ops[3]) == vm_reg_op(ops[2]) + 1
          );

          state.use_range(vm_reg_op(ops[2]), count);
        } else {
          if count >= 1 {
            state.use_(ops[2], 0);
          }

          if count >= 2 {
            state.maybe_use(ops[3]);
          }

          if count >= 3 {
            state.maybe_use(ops[4]);
          }
        }
      } else {
        state.use_varargs(vm_reg_op(ops[2]) as u8);
      }

      state.def_range(function, vm_reg_op(ops[1]), function.int_op(ops[6]));
    }
    IrCmd::FORGLOOP => {
      // 第一个寄存器未被指令使用；用 CHECK_TAG 检查它仍是 'nil'
      state.use_(ops[0], 1);
      state.use_(ops[0], 2);

      state.def(function, ops[0], 2);
      state.def_range(function, vm_reg_op(ops[0]) + 3, function.int_op(ops[1]));
    }
    IrCmd::ForgloopFallback => {
      state.use_range(vm_reg_op(ops[0]), 3);

      state.def(function, ops[0], 2);
      state.def_range(
        function,
        vm_reg_op(ops[0]) + 3,
        (function.int_op(ops[1]) as u8) as i32,
      );
    }
    IrCmd::ForgprepXnextFallback => {
      state.use_(ops[1], 0);
    }
    IrCmd::FallbackGetglobal => {
      state.def(function, ops[1], 0);
    }
    IrCmd::FallbackSetglobal => {
      state.use_(ops[1], 0);
    }
    IrCmd::FallbackGettableks => {
      state.use_(ops[2], 0);
      state.def(function, ops[1], 0);
    }
    IrCmd::FallbackSettableks => {
      state.use_(ops[1], 0);
      state.use_(ops[2], 0);
    }
    IrCmd::FallbackNamecall => {
      state.use_(ops[2], 0);
      state.def_range(function, vm_reg_op(ops[1]), 2);
    }
    IrCmd::FallbackPrepvarargs => {}
    IrCmd::FallbackGetvarargs => {
      state.def_range(function, vm_reg_op(ops[1]), function.int_op(ops[2]));
    }
    IrCmd::FallbackDupclosure => {
      state.def(function, ops[1], 0);
    }
    IrCmd::FallbackForgprep => {
      // 本指令不总是重定义 Rn、Rn+1、Rn+2，故须标为隐式 use
      state.use_range(vm_reg_op(ops[1]), 3);
      state.def_range(function, vm_reg_op(ops[1]), 3);
    }
    IrCmd::AdjustStackToReg => {
      state.def_range(function, vm_reg_op(ops[0]), -1);
    }
    IrCmd::AdjustStackToTop => {}
    IrCmd::GetTypeof => {
      state.use_(ops[0], 0);
    }
    IrCmd::FINDUPVAL => {
      state.use_(ops[0], 0);
    }
    IrCmd::MarkUsed => {
      state.use_range(vm_reg_op(ops[0]), function.int_op(ops[1]));
    }
    IrCmd::MarkDead => {}
    _ => {
      // 所有引用寄存器的指令都必须被显式处理
      for op in ops.as_slice() {
        CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
      }
    }
  }
}

/// 当前指令与所在块以 (block_idx, index) 索引化传入：入口一次性快照 cmd 与 ops
/// （Copy 向量），体中所有函数/state 借用均为独立短借用，
/// 原「块与指令槽位 `addr_of_mut!` 取址交割」整体消失。
pub fn mark_dead_stores_in_inst(
  state: &mut RemoveDeadStoreState,
  constant_map: &mut ConstantMap,
  function: &mut IrFunction,
  block_idx: u32,
  index: u32,
) {
  let (cmd, ops, use_count) = {
    let inst = &function.instructions[index as usize];
    (inst.cmd, inst.ops.clone(), inst.use_count)
  };

  update_remaining_uses(state, index, cmd, use_count, &ops);

  let nil = LuaType::Nil as u8;

  match cmd {
    IrCmd::StoreTag => {
      if let Some(reg) = store_arm_reg(state, function, ops[0])
        && !try_replace_tag_with_full_store(state, function, block_idx, index, ops[0], ops[1], reg)
      {
        let tag = function.tag_op(ops[1]);

        let reg_info = &mut state.info[reg];
        reg_info.tag_inst_idx = index;

        if tag_value_pair_established(reg_info) {
          if tag == nil {
            reg_info.value_inst_idx = !0u32;
          }

          reg_info.tvalue_inst_idx = !0u32;
        }

        reg_info.maybe_gco = is_gco(tag);
        reg_info.known_tag = tag;
        state.has_gco_to_clear |= reg_info.maybe_gco;
      }
    }
    IrCmd::StoreExtra => {
      // 为简化，extra 字段的 store 与此前所有 store 一起保留
      if ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(ops[0]);
        state.use_reg(reg as u8);
        state.info[reg as usize].ignore_at_exit = false;
      }
    }
    IrCmd::StorePointer => {
      if let Some(reg) = store_arm_reg(state, function, ops[0]) {
        // cpp 上游已将 `LuauCodegenDsePtrStoreTagCheck` 行为合入为唯一路径（OptimizeDeadStore.cpp:1024-1042）：
        // 已知非 GCO tag 时不能把指针 store 当作 GCO 处理，否则会破坏
        // `maybe_gco → known_tag == K_UNKNOWN_TAG || is_gco(known_tag)` 不变量并触发 flush 断言
        // 若已知 tag 且不是指针，不能生成非法形式的完整 store
        let known_tag = state.info[reg].known_tag;
        let maybe_gco = known_tag == K_UNKNOWN_TAG || is_gco(known_tag);

        if maybe_gco
          && try_replace_value_with_full_store(
            state,
            constant_map,
            function,
            block_idx,
            index,
            ops[0],
            ops[1],
            reg,
          )
        {
          state.info[reg].maybe_gco = true;
          state.has_gco_to_clear = true;
          return mark_dead_stores_in_inst_tail(state, function, index);
        }

        // 虽存入了指针，TValue 仍可能带非 GCO 的 tag
        note_value_store(state, function, reg, index, maybe_gco);
      }
    }
    IrCmd::StoreDouble | IrCmd::StoreInt64 | IrCmd::StoreInt => {
      if let Some(reg) = store_arm_reg(state, function, ops[0])
        && !try_replace_value_with_full_store(
          state,
          constant_map,
          function,
          block_idx,
          index,
          ops[0],
          ops[1],
          reg,
        )
      {
        note_value_store(state, function, reg, index, false);
      }
    }
    IrCmd::StoreVector => {
      if let Some(reg) = store_arm_reg(state, function, ops[0])
        && !try_replace_vector_value_with_full_store(state, function, index, reg)
      {
        note_value_store(state, function, reg, index, false);
      }
    }
    IrCmd::StoreTvalue => {
      if let Some(reg) = store_arm_reg(state, function, ops[0]) {
        note_tvalue_store(state, function, reg, index);

        let reg_info = &mut state.info[reg];
        reg_info.known_tag = try_get_operand_tag(function, ops[1]).unwrap_or(K_UNKNOWN_TAG);
        reg_info.maybe_gco = reg_info.known_tag == K_UNKNOWN_TAG || is_gco(reg_info.known_tag);

        state.has_gco_to_clear |= reg_info.maybe_gco;
      }
    }
    IrCmd::StoreSplitTvalue => {
      if let Some(reg) = store_arm_reg(state, function, ops[0]) {
        note_tvalue_store(state, function, reg, index);

        let reg_info = &mut state.info[reg];
        let tag = function.tag_op(ops[1]);
        reg_info.maybe_gco = is_gco(tag);
        reg_info.known_tag = tag;
        state.has_gco_to_clear |= reg_info.maybe_gco;
      }
    }

    // guard 检查可能跳到使用了我们部分或全部已存值的 block
    IrCmd::CheckTag => {
      state.check_live_ins(
        function,
        ops.get(2).copied().unwrap_or_default(),
        index,
        true,
      );

      // tag guard 在当前 block 确立寄存器的 tag 值
      if let Some(load) = function.as_inst_op_ref(ops[0])
        && load.cmd == IrCmd::LoadTag
        && load.ops[0].kind() == IrOpKind::VmReg
      {
        let reg = vm_reg_op(load.ops[0]);
        let tag = function.tag_op(ops[1]);
        state.info[reg as usize].known_tag = tag;
      }
    }
    IrCmd::TryNumToIndex => {
      state.check_live_ins(function, ops[1], index, true);
    }
    IrCmd::TryCallFastgettm => {
      state.check_live_ins(function, ops[2], index, true);
    }
    IrCmd::CheckFastcallRes => {
      state.check_live_ins(function, ops[1], index, true);
    }
    IrCmd::CheckTruthy => {
      // 本指令 lowering 有两条到 exit 的跳转，导致无法生成 exit sync 记录
      state.check_live_ins(function, ops[2], index, false);
    }
    IrCmd::CheckReadonly => {
      state.check_live_ins(function, ops[1], index, true);
    }
    IrCmd::CheckNoMetatable => {
      state.check_live_ins(function, ops[1], index, true);
    }
    IrCmd::CheckSafeEnv => {
      state.check_live_ins(function, ops[0], index, true);
    }
    IrCmd::CheckArraySize => {
      state.check_live_ins(function, ops[2], index, true);
    }
    IrCmd::CheckDivInt64 => {
      // 本指令 lowering 有两条到 exit 的跳转，导致无法生成 exit sync 记录
      state.check_live_ins(function, ops[2], index, false);
    }
    IrCmd::CheckSlotMatch => {
      state.check_live_ins(function, ops[2], index, true);
    }
    IrCmd::CheckNodeNoNext => {
      state.check_live_ins(function, ops[1], index, true);
    }
    IrCmd::CheckNodeValue => {
      state.check_live_ins(function, ops[1], index, true);
    }
    IrCmd::CheckBufferLen => {
      state.check_live_ins(function, ops[5], index, true);
    }
    IrCmd::CheckUserdataTag => {
      state.check_live_ins(function, ops[2], index, true);
    }
    IrCmd::CheckCmpNum | IrCmd::CheckCmpInt | IrCmd::CheckCmpInt64 => {
      state.check_live_ins(function, ops[3], index, true);
    }

    IrCmd::JumpIfTruthy
    | IrCmd::JumpIfFalsy
    | IrCmd::JumpEqTag
    | IrCmd::JumpCmpInt
    | IrCmd::JumpCmpInt64
    | IrCmd::JumpEqPointer
    | IrCmd::JumpCmpNum
    | IrCmd::JumpCmpFloat
    | IrCmd::JumpFornLoopCond
    | IrCmd::JumpSlotMatch
    | IrCmd::JumpCmpProtoid => {
      visit_vm_reg_defs_uses(state, function, cmd, &ops);
      state.check_live_outs(function, block_idx);
    }

    IrCmd::JUMP => {
      // 理想情况下可移除写入非 live out 寄存器的 store
      // 但链式优化依赖前驱中存储的数据，即使它不是显式 live out
    }
    IrCmd::RETURN => {
      visit_vm_reg_defs_uses(state, function, cmd, &ops);

      // 函数末尾可以杀掉写入非 live out 寄存器的 store
      state.check_live_outs(function, block_idx);
    }
    IrCmd::AdjustStackToReg => {
      // visitVmRegDefsUses 把 adjustment 视作 fastcall 寄存器定义点，但死 store 移除数的是实际写入
    }

    // 这一组指令内部可能触发 GC assist
    IrCmd::CmpAny
    | IrCmd::DoArith
    | IrCmd::DoLen
    | IrCmd::GetTable
    | IrCmd::SetTable
    | IrCmd::GetCachedImport
    | IrCmd::CONCAT
    | IrCmd::INTERRUPT
    | IrCmd::CheckGc
    | IrCmd::CALL
    | IrCmd::ForgloopFallback
    | IrCmd::FallbackGetglobal
    | IrCmd::FallbackSetglobal
    | IrCmd::FallbackGettableks
    | IrCmd::FallbackSettableks
    | IrCmd::FallbackNamecall
    | IrCmd::FallbackDupclosure
    | IrCmd::FallbackForgprep => {
      if state.has_gco_to_clear {
        state.flush_gco_regs(function);
      }

      visit_vm_reg_defs_uses(state, function, cmd, &ops);
    }

    IrCmd::NewUserdata => {
      state.has_allocations = true;
    }

    IrCmd::MarkDead => {
      state.mark_unused_at_exit(function, vm_reg_op(ops[0]), function.int_op(ops[1]));
    }

    _ => {
      // guard 必须被显式覆盖
      CODEGEN_ASSERT!(!is_non_terminating_jump(cmd));
      visit_vm_reg_defs_uses(state, function, cmd, &ops);
    }
  }

  mark_dead_stores_in_inst_tail(state, function, index);
}

// cpp markDeadStoresInInst 尾部的无条件 switch（上游已删除 LuauCodegenVmExitSync 门控）。
// 按索引即时重读槽位 cmd：本指令若在 try_replace 中被整条替换，
// 与 cpp 的 `IrInst&` 内存别名语义一致（读替换后的新 cmd）。
fn mark_dead_stores_in_inst_tail(
  state: &mut RemoveDeadStoreState,
  function: &IrFunction,
  index: u32,
) {
  let cmd = function.instructions[index as usize].cmd;

  // 带 SSA 操作数的 pending store 不得推迟到 ExitSync block，越过可能使操作数物理位置失效的指令
  // 注：cpp 的 INVOKE_FASTPCALL 臂在本仓 IrCmd 中尚未移植，随该指令移植一并补
  match cmd {
    IrCmd::CmpAny
    | IrCmd::DoArith
    | IrCmd::DoLen
    | IrCmd::GetTable
    | IrCmd::SetTable
    | IrCmd::CONCAT
    | IrCmd::GetCachedImport
    | IrCmd::ForgloopFallback
    | IrCmd::FallbackGetglobal
    | IrCmd::FallbackSetglobal
    | IrCmd::FallbackGettableks
    | IrCmd::FallbackSettableks
    | IrCmd::FallbackNamecall
    | IrCmd::FallbackDupclosure
    | IrCmd::FallbackForgprep
    | IrCmd::CALL
    | IrCmd::SETLIST
    | IrCmd::FORGLOOP => {
      state.invalidate_value_propagation(function);
    }
    _ => {}
  }
}
