use ulua_common::FFlag::{LuauCodegenDsePtrStoreTagCheck, LuauCodegenVmExitSync};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    is_gco::is_gco, is_non_terminating_jump::is_non_terminating_jump,
    try_get_operand_tag::try_get_operand_tag,
    try_replace_tag_with_full_store::try_replace_tag_with_full_store,
    try_replace_value_with_full_store::try_replace_value_with_full_store,
    try_replace_vector_value_with_full_store::try_replace_vector_value_with_full_store,
    update_remaining_uses::update_remaining_uses, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_function::IrFunction, ir_inst::IrInst,
    remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo,
  },
};

#[inline]
fn reg_captured(function: &IrFunction, reg: i32) -> bool {
  (function.cfg.captured.regs[reg as usize / 64] & (1u64 << (reg as usize % 64))) != 0
}

// Port of IrVisitUseDef.h `visitVmRegDefsUses<T>` specialized for the RemoveDeadStoreState visitor
fn visit_vm_reg_defs_uses(
  state: &mut RemoveDeadStoreState,
  function: &mut IrFunction,
  inst: &mut IrInst,
) {
  // For correct analysis, all instruction uses must be handled before handling the definitions
  match inst.cmd {
    IrCmd::LoadTag
    | IrCmd::LoadPointer
    | IrCmd::LoadDouble
    | IrCmd::LoadInt
    | IrCmd::LoadInt64
    | IrCmd::LoadFloat
    | IrCmd::LoadTvalue => {
      state.maybe_use(inst.ops[0]);
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
      state.maybe_def(inst.ops[0]);
    }
    IrCmd::CmpAny => {
      state.use_(inst.ops[0], 0);
      state.use_(inst.ops[1], 0);
    }
    IrCmd::CmpTag => {
      state.maybe_use(inst.ops[0]);
    }
    IrCmd::JumpIfTruthy | IrCmd::JumpIfFalsy => {
      state.use_(inst.ops[0], 0);
    }
    IrCmd::JumpEqTag => {
      state.maybe_use(inst.ops[0]);
    }
    IrCmd::DoArith => {
      state.maybe_use(inst.ops[1]);
      state.maybe_use(inst.ops[2]);
      state.def(inst.ops[0], 0);
    }
    IrCmd::GetTable => {
      state.use_(inst.ops[1], 0);
      state.maybe_use(inst.ops[2]);
      state.def(inst.ops[0], 0);
    }
    IrCmd::SetTable => {
      state.use_(inst.ops[0], 0);
      state.use_(inst.ops[1], 0);
      state.maybe_use(inst.ops[2]);
    }
    IrCmd::DoLen => {
      state.use_(inst.ops[1], 0);
      state.def(inst.ops[0], 0);
    }
    IrCmd::GetCachedImport => {
      state.def(inst.ops[0], 0);
    }
    IrCmd::CONCAT => {
      state.use_range(vm_reg_op(inst.ops[0]), function.uint_op(inst.ops[1]) as i32);
      state.def_range(vm_reg_op(inst.ops[0]), function.uint_op(inst.ops[1]) as i32);
    }
    IrCmd::GetUpvalue => {}
    IrCmd::SetUpvalue => {}
    IrCmd::INTERRUPT => {}
    IrCmd::BarrierObj | IrCmd::BarrierTableForward => {
      state.maybe_use(inst.ops[1]);
    }
    IrCmd::CloseUpvals => {}
    IrCmd::CAPTURE => {
      state.maybe_use(inst.ops[0]);

      if function.uint_op(inst.ops[1]) == 1 {
        state.capture(vm_reg_op(inst.ops[0]));
      }
    }
    IrCmd::SETLIST => {
      state.use_(inst.ops[1], 0);
      state.use_range(vm_reg_op(inst.ops[2]), function.int_op(inst.ops[3]));
    }
    IrCmd::CALL => {
      state.use_(inst.ops[0], 0);
      state.use_range(vm_reg_op(inst.ops[0]) + 1, function.int_op(inst.ops[1]));
      state.def_range(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[2]));
    }
    IrCmd::RETURN => {
      state.use_range(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[1]));
    }
    IrCmd::FASTCALL => {
      state.use_(inst.ops[2], 0);
      state.def_range(vm_reg_op(inst.ops[1]), function.int_op(inst.ops[3]));
    }
    IrCmd::InvokeFastcall => {
      let count = function.int_op(inst.ops[5]);
      if count != -1 {
        // Only LOP_FASTCALL3 lowering is allowed to have third optional argument
        if count >= 3 && inst.ops[4].kind() == IrOpKind::Undef {
          CODEGEN_ASSERT!(
            inst.ops[3].kind() == IrOpKind::VmReg
              && vm_reg_op(inst.ops[3]) == vm_reg_op(inst.ops[2]) + 1
          );

          state.use_range(vm_reg_op(inst.ops[2]), count);
        } else {
          if count >= 1 {
            state.use_(inst.ops[2], 0);
          }

          if count >= 2 {
            state.maybe_use(inst.ops[3]);
          }

          if count >= 3 {
            state.maybe_use(inst.ops[4]);
          }
        }
      } else {
        state.use_varargs(vm_reg_op(inst.ops[2]) as u8);
      }

      state.def_range(vm_reg_op(inst.ops[1]), function.int_op(inst.ops[6]));
    }
    IrCmd::FORGLOOP => {
      // First register is not used by instruction, we check that it's still 'nil' with CHECK_TAG
      state.use_(inst.ops[0], 1);
      state.use_(inst.ops[0], 2);

      state.def(inst.ops[0], 2);
      state.def_range(vm_reg_op(inst.ops[0]) + 3, function.int_op(inst.ops[1]));
    }
    IrCmd::ForgloopFallback => {
      state.use_range(vm_reg_op(inst.ops[0]), 3);

      state.def(inst.ops[0], 2);
      state.def_range(
        vm_reg_op(inst.ops[0]) + 3,
        (function.int_op(inst.ops[1]) as u8) as i32,
      );
    }
    IrCmd::ForgprepXnextFallback => {
      state.use_(inst.ops[1], 0);
    }
    IrCmd::FallbackGetglobal => {
      state.def(inst.ops[1], 0);
    }
    IrCmd::FallbackSetglobal => {
      state.use_(inst.ops[1], 0);
    }
    IrCmd::FallbackGettableks => {
      state.use_(inst.ops[2], 0);
      state.def(inst.ops[1], 0);
    }
    IrCmd::FallbackSettableks => {
      state.use_(inst.ops[1], 0);
      state.use_(inst.ops[2], 0);
    }
    IrCmd::FallbackNamecall => {
      state.use_(inst.ops[2], 0);
      state.def_range(vm_reg_op(inst.ops[1]), 2);
    }
    IrCmd::FallbackPrepvarargs => {}
    IrCmd::FallbackGetvarargs => {
      state.def_range(vm_reg_op(inst.ops[1]), function.int_op(inst.ops[2]));
    }
    IrCmd::FallbackDupclosure => {
      state.def(inst.ops[1], 0);
    }
    IrCmd::FallbackForgprep => {
      // This instruction doesn't always redefine Rn, Rn+1, Rn+2, so we have to mark it as implicit use
      state.use_range(vm_reg_op(inst.ops[1]), 3);
      state.def_range(vm_reg_op(inst.ops[1]), 3);
    }
    IrCmd::AdjustStackToReg => {
      state.def_range(vm_reg_op(inst.ops[0]), -1);
    }
    IrCmd::AdjustStackToTop => {}
    IrCmd::GetTypeof => {
      state.use_(inst.ops[0], 0);
    }
    IrCmd::FINDUPVAL => {
      state.use_(inst.ops[0], 0);
    }
    IrCmd::MarkUsed => {
      state.use_range(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[1]));
    }
    IrCmd::MarkDead => {}
    _ => {
      // All instructions which reference registers have to be handled explicitly
      // 单次切片遍历，替代 C 风格索引循环
      for op in inst.ops.as_slice() {
        CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
      }
    }
  }
}

pub fn mark_dead_stores_in_inst(
  state: &mut RemoveDeadStoreState,
  build: &mut IrBuilder,
  function: &mut IrFunction,
  block: &mut IrBlock,
  inst: &mut IrInst,
  index: u32,
) {
  update_remaining_uses(state, inst, index);

  let nil = LuaType::Nil as u8;

  match inst.cmd {
    IrCmd::StoreTag => {
      if inst.ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(inst.ops[0]);

        if reg_captured(function, reg) {
          return;
        }

        let reg_info: &mut StoreRegInfo =
          unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

        reg_info.ignore_at_exit = false;

        if !try_replace_tag_with_full_store(
          state,
          build,
          function,
          block,
          index,
          inst.ops[0],
          inst.ops[1],
          reg_info,
        ) {
          let tag = function.tag_op(inst.ops[1]);

          reg_info.tag_inst_idx = index;

          if state.tag_value_pair_established(reg_info) {
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
    }
    IrCmd::StoreExtra => {
      // To simplify, extra field store is preserved along with all other stores made so far
      if inst.ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(inst.ops[0]);
        state.use_reg(reg as u8);
        state.info[reg as usize].ignore_at_exit = false;
      }
    }
    IrCmd::StorePointer => {
      if inst.ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(inst.ops[0]);

        if reg_captured(function, reg) {
          return;
        }

        let reg_info: &mut StoreRegInfo =
          unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

        reg_info.ignore_at_exit = false;

        let maybe_gco;

        if LuauCodegenDsePtrStoreTagCheck.get() {
          // If we have a known tag and it is not a pointer, we cannot generate a full store in invalid form
          maybe_gco = reg_info.known_tag == 0xff || is_gco(reg_info.known_tag);

          if maybe_gco
            && try_replace_value_with_full_store(
              state,
              build,
              function,
              block,
              index,
              inst.ops[0],
              inst.ops[1],
              reg_info,
            )
          {
            reg_info.maybe_gco = true;
            state.has_gco_to_clear = true;
            return mark_dead_stores_in_inst_tail(state, function, inst);
          }
        } else {
          if try_replace_value_with_full_store(
            state,
            build,
            function,
            block,
            index,
            inst.ops[0],
            inst.ops[1],
            reg_info,
          ) {
            reg_info.maybe_gco = true;
            state.has_gco_to_clear |= true;
            return mark_dead_stores_in_inst_tail(state, function, inst);
          }
          maybe_gco = true;
        }

        // Partial value store can be removed by a new one if the tag is known
        if reg_info.known_tag != 0xff {
          state.kill_value_store(reg_info);
        }

        reg_info.value_inst_idx = index;

        if state.tag_value_pair_established(reg_info) {
          reg_info.tvalue_inst_idx = !0u32;
        }

        if LuauCodegenDsePtrStoreTagCheck.get() {
          // While pointer was stored, TValue can still be under a non-GCO tag
          reg_info.maybe_gco = maybe_gco;
          state.has_gco_to_clear |= maybe_gco;
        } else {
          reg_info.maybe_gco = true;
          state.has_gco_to_clear = true;
        }
      }
    }
    IrCmd::StoreDouble | IrCmd::StoreInt64 | IrCmd::StoreInt => {
      if inst.ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(inst.ops[0]);

        if reg_captured(function, reg) {
          return;
        }

        let reg_info: &mut StoreRegInfo =
          unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

        reg_info.ignore_at_exit = false;

        if !try_replace_value_with_full_store(
          state,
          build,
          function,
          block,
          index,
          inst.ops[0],
          inst.ops[1],
          reg_info,
        ) {
          // Partial value store can be removed by a new one if the tag is known
          if reg_info.known_tag != 0xff {
            state.kill_value_store(reg_info);
          }

          reg_info.value_inst_idx = index;

          if state.tag_value_pair_established(reg_info) {
            reg_info.tvalue_inst_idx = !0u32;
          }

          reg_info.maybe_gco = false;
        }
      }
    }
    IrCmd::StoreVector => {
      if inst.ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(inst.ops[0]);

        if reg_captured(function, reg) {
          return;
        }

        let reg_info: &mut StoreRegInfo =
          unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

        reg_info.ignore_at_exit = false;

        if !try_replace_vector_value_with_full_store(state, build, function, block, index, reg_info)
        {
          // Partial value store can be removed by a new one if the tag is known
          if reg_info.known_tag != 0xff {
            state.kill_value_store(reg_info);
          }

          reg_info.value_inst_idx = index;

          if state.tag_value_pair_established(reg_info) {
            reg_info.tvalue_inst_idx = !0u32;
          }

          reg_info.maybe_gco = false;
        }
      }
    }
    IrCmd::StoreTvalue => {
      if inst.ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(inst.ops[0]);

        if reg_captured(function, reg) {
          return;
        }

        let reg_info: &mut StoreRegInfo =
          unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

        reg_info.ignore_at_exit = false;

        state.kill_tag_and_value_store_pair(reg_info);
        state.kill_t_value_store(reg_info);

        reg_info.tag_inst_idx = !0u32;
        reg_info.value_inst_idx = !0u32;

        reg_info.tvalue_inst_idx = index;

        reg_info.known_tag = try_get_operand_tag(function, inst.ops[1]).unwrap_or(0xff);
        reg_info.maybe_gco = reg_info.known_tag == 0xff || is_gco(reg_info.known_tag);

        state.has_gco_to_clear |= reg_info.maybe_gco;
      }
    }
    IrCmd::StoreSplitTvalue => {
      if inst.ops[0].kind() == IrOpKind::VmReg {
        let reg = vm_reg_op(inst.ops[0]);

        if reg_captured(function, reg) {
          return;
        }

        let reg_info: &mut StoreRegInfo =
          unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

        reg_info.ignore_at_exit = false;

        state.kill_tag_and_value_store_pair(reg_info);
        state.kill_t_value_store(reg_info);

        reg_info.tag_inst_idx = !0u32;
        reg_info.value_inst_idx = !0u32;

        reg_info.tvalue_inst_idx = index;
        let tag = function.tag_op(inst.ops[1]);
        reg_info.maybe_gco = is_gco(tag);
        reg_info.known_tag = tag;
        state.has_gco_to_clear |= reg_info.maybe_gco;
      }
    }

    // Guard checks can jump to a block which might be using some or all the values we stored
    IrCmd::CheckTag => {
      state.check_live_ins(inst.ops.get(2).copied().unwrap_or_default(), index, true);

      // Tag guard establishes the tag value of the register in the current block
      let load_ptr = function.as_inst_op(inst.ops[0]);
      if !load_ptr.is_null() {
        let load = unsafe { &*load_ptr };
        if load.cmd == IrCmd::LoadTag && load.ops[0].kind() == IrOpKind::VmReg {
          let reg = vm_reg_op(load.ops[0]);
          let tag = function.tag_op(inst.ops[1]);
          state.info[reg as usize].known_tag = tag;
        }
      }
    }
    IrCmd::TryNumToIndex => {
      state.check_live_ins(inst.ops[1], index, true);
    }
    IrCmd::TryCallFastgettm => {
      state.check_live_ins(inst.ops[2], index, true);
    }
    IrCmd::CheckFastcallRes => {
      state.check_live_ins(inst.ops[1], index, true);
    }
    IrCmd::CheckTruthy => {
      // This instruction has two jumps to the exit in the lowering and that prevents exit sync record from being generated
      state.check_live_ins(inst.ops[2], index, false);
    }
    IrCmd::CheckReadonly => {
      state.check_live_ins(inst.ops[1], index, true);
    }
    IrCmd::CheckNoMetatable => {
      state.check_live_ins(inst.ops[1], index, true);
    }
    IrCmd::CheckSafeEnv => {
      state.check_live_ins(inst.ops[0], index, true);
    }
    IrCmd::CheckArraySize => {
      state.check_live_ins(inst.ops[2], index, true);
    }
    IrCmd::CheckDivInt64 => {
      // This instruction has two jumps to the exit in the lowering and that prevents exit sync record from being generated
      state.check_live_ins(inst.ops[2], index, false);
    }
    IrCmd::CheckSlotMatch => {
      state.check_live_ins(inst.ops[2], index, true);
    }
    IrCmd::CheckNodeNoNext => {
      state.check_live_ins(inst.ops[1], index, true);
    }
    IrCmd::CheckNodeValue => {
      state.check_live_ins(inst.ops[1], index, true);
    }
    IrCmd::CheckBufferLen => {
      state.check_live_ins(inst.ops[5], index, true);
    }
    IrCmd::CheckUserdataTag => {
      state.check_live_ins(inst.ops[2], index, true);
    }
    IrCmd::CheckCmpNum | IrCmd::CheckCmpInt | IrCmd::CheckCmpInt64 => {
      state.check_live_ins(inst.ops[3], index, true);
    }

    IrCmd::JumpIfTruthy
    | IrCmd::JumpIfFalsy
    | IrCmd::JumpEqTag
    | IrCmd::JumpCmpInt
    | IrCmd::JumpEqPointer
    | IrCmd::JumpCmpNum
    | IrCmd::JumpCmpFloat
    | IrCmd::JumpFornLoopCond
    | IrCmd::JumpSlotMatch
    | IrCmd::JumpCmpProtoid => {
      visit_vm_reg_defs_uses(state, function, inst);
      state.check_live_outs(block);
    }

    IrCmd::JUMP => {
      // Ideally, we would be able to remove stores to registers that are not live out from a block
      // But during chain optimizations, we rely on data stored in the predecessor even when it's not an explicit live out
    }
    IrCmd::RETURN => {
      visit_vm_reg_defs_uses(state, function, inst);

      // At the end of a function, we can kill stores to registers that are not live out
      state.check_live_outs(block);
    }
    IrCmd::AdjustStackToReg => {
      // visitVmRegDefsUses considers adjustment as the fast call register definition point, but for dead store removal, we count the actual writes
    }

    // This group of instructions can trigger GC assist internally
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
        state.flush_gco_regs();
      }

      visit_vm_reg_defs_uses(state, function, inst);
    }

    IrCmd::NewUserdata => {
      state.has_allocations = true;
    }

    IrCmd::MarkDead => {
      state.mark_unused_at_exit(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[1]));
    }

    _ => {
      // Guards have to be covered explicitly
      CODEGEN_ASSERT!(!is_non_terminating_jump(inst.cmd));
      visit_vm_reg_defs_uses(state, function, inst);
    }
  }

  mark_dead_stores_in_inst_tail(state, function, inst);
}

// The trailing `if (FFlag::LuauCodegenVmExitSync)` switch from markDeadStoresInInst
fn mark_dead_stores_in_inst_tail(
  state: &mut RemoveDeadStoreState,
  _function: &mut IrFunction,
  inst: &mut IrInst,
) {
  if LuauCodegenVmExitSync.get() {
    // Pending stores with SSA operands must not be deferred to ExitSync blocks past instructions that can invalidate operand physical location
    match inst.cmd {
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
        state.invalidate_value_propagation();
      }
      _ => {}
    }
  }
}
