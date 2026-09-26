use alloc::vec::Vec;
use core::mem::offset_of;

use ulua_common::records::{dense_hash_map::DenseHashMap, small_vector::SmallVector};
use ulua_vm::records::{global_state::global_State, lua_state::LuaState};

use crate::{
  enums::{
    ir_cmd::IrCmd,
    ir_op_kind::IrOpKind,
    ir_value_kind::{IrValueKind, K_VALUE_DWORD_SIZE},
    size_x_64::SizeX64,
  },
  functions::{
    get_cmd_value_kind::get_cmd_value_kind, get_next_inst_use::get_next_inst_use,
    get_xmm_register_count::get_xmm_register_count, luau_constant::luau_constant,
    luau_constant_tag::luau_constant_tag, luau_constant_value::luau_constant_value,
    luau_reg::luau_reg, luau_reg_tag::luau_reg_tag, luau_reg_value::luau_reg_value,
    luau_reg_value_int::luau_reg_value_int, luau_reg_value_int_64::luau_reg_value_int_64,
    qword_reg::qword_reg, update_last_use_locations_in_block::update_last_use_locations_in_block,
    vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64,
    emit_common_x_64::{K_SPILL_SLOTS, R_STATE},
    exit_sync_arg_x_64::ExitSyncArgX64,
    ir_data::K_INVALID_INST_IDX,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    ir_spill_x_64::IrSpillX64,
    lowering_stats::LoweringStats,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
    value_restore_location::ValueRestoreLocation,
  },
};

/// cpp `IrRegAllocX64`（`CodeGen/src/IrRegAllocX64.h`）。
///
/// `build/function/stats` 是与宿主 `IrLoweringX64` 共享的同一组上游裸指针（见该
/// 结构文档），改为 `&mut` 会与 lowering 侧的读写形成双 `&mut` 而编译失败；保持
/// 裸指针，生命周期由构造点栈帧保证。
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrRegAllocX64 {
  pub build: *mut AssemblyBuilderX64,
  pub function: *mut IrFunction,
  pub stats: *mut LoweringStats,

  pub curr_inst_idx: u32,

  pub free_gpr_map: [bool; 16],
  pub gpr_inst_users: [u32; 16],
  pub free_xmm_map: [bool; 16],
  pub xmm_inst_users: [u32; 16],
  pub usable_xmm_reg_count: u8,

  pub used_spill_slot_halfs: [u64; 8], // std::bitset<512> mapped to fixed-size array
  pub max_used_slot: u32,

  pub next_spill_id: u32,
  pub spills: Vec<IrSpillX64>,

  pub exit_sync_args: DenseHashMap<u32, SmallVector<ExitSyncArgX64, 2>>,

  pub alloc_action_count: u32,
}

impl IrRegAllocX64 {
  /// `static const RegisterX64 kGprAllocOrder[] = {rax, rdx, rcx, rbx, rsi, rdi, r8, r9, r10, r11};`
  /// (IrRegAllocX64.cpp:23)
  pub const K_GPR_ALLOC_ORDER: [RegisterX64; 10] = [
    RegisterX64::RAX,
    RegisterX64::RDX,
    RegisterX64::RCX,
    RegisterX64::RBX,
    RegisterX64::RSI,
    RegisterX64::RDI,
    RegisterX64::R8,
    RegisterX64::R9,
    RegisterX64::R10,
    RegisterX64::R11,
  ];

  // 共享裸指针访问器家族（function_mut/function_ref/build_mut/stats_mut）由宏收口，
  // 统一契约见 `crate::shared_ptr_accessors!`（与宿主 IrLoweringX64 同一守则）。
  crate::shared_ptr_accessors!(AssemblyBuilderX64);

  /// spill 槽半字位图：每个 u64 字覆盖 64 个半字（512 半字 = 8 字）
  const HALVES_PER_WORD: u32 = 64;

  /// 占用 `[start, end)` 的 spill 槽半字（新 spill 落槽）
  fn occupy_spill_slots(&mut self, start: u32, end: u32) {
    for pos in start..end {
      self.used_spill_slot_halfs[(pos / Self::HALVES_PER_WORD) as usize] |=
        1u64 << (pos % Self::HALVES_PER_WORD);
    }
  }

  /// 跨块快照重建：逐位断言原本空闲后占用（沿用原单循环的逐位判定序）
  fn occupy_free_spill_slots(&mut self, start: u32, end: u32) {
    for pos in start..end {
      let word = (pos / Self::HALVES_PER_WORD) as usize;
      let mask = 1u64 << (pos % Self::HALVES_PER_WORD);
      CODEGEN_ASSERT!(self.used_spill_slot_halfs[word] & mask == 0);
      self.used_spill_slot_halfs[word] |= mask;
    }
  }

  /// 释放 `[start, end)` 的 spill 槽半字（restore / last-use 释放）
  fn free_spill_slots(&mut self, start: u32, end: u32) {
    for pos in start..end {
      self.used_spill_slot_halfs[(pos / Self::HALVES_PER_WORD) as usize] &=
        !(1u64 << (pos % Self::HALVES_PER_WORD));
    }
  }

  pub fn alloc_reg(&mut self, size: SizeX64, inst_idx: u32) -> RegisterX64 {
    self.alloc_action_count += 1;

    if size == SizeX64::Xmmword {
      // 找首个空闲 xmm：position 提前终止，命中即占用并返回
      if let Some(i) = self.free_xmm_map[..self.usable_xmm_reg_count as usize]
        .iter()
        .position(|&free| free)
      {
        self.free_xmm_map[i] = false;
        self.xmm_inst_users[i] = inst_idx;
        return RegisterX64 {
          bits: ((i as u8) << RegisterX64::INDEX_SHIFT) | (size as u8),
        };
      }
    } else {
      for reg in IrRegAllocX64::K_GPR_ALLOC_ORDER.iter() {
        if self.free_gpr_map[reg.index() as usize] {
          self.free_gpr_map[reg.index() as usize] = false;
          self.gpr_inst_users[reg.index() as usize] = inst_idx;
          return RegisterX64 {
            bits: (reg.index() << RegisterX64::INDEX_SHIFT) | (size as u8),
          };
        }
      }
    }

    // 寄存器耗尽，spill next use 最远的那个值
    let reg_inst_users = if size == SizeX64::Xmmword {
      &self.xmm_inst_users
    } else {
      &self.gpr_inst_users
    };

    let furthest_use_target = self.find_instruction_with_furthest_next_use(reg_inst_users);
    if furthest_use_target != K_INVALID_INST_IDX {
      // 快照目标指令寄存器（Copy），借用即止，take_reg 不再受指令借用牵连
      let mut reg = self.function_ref().instructions[furthest_use_target as usize].reg_x64;
      reg.bits = (reg.index() << RegisterX64::INDEX_SHIFT) | (size as u8);

      return self.take_reg(reg, inst_idx);
    }

    CODEGEN_ASSERT!(false, "Out of registers to allocate");
    RegisterX64::NOREG
  }

  pub fn alloc_reg_or_reuse(
    &mut self,
    size: SizeX64,
    inst_idx: u32,
    oprefs: &[IrOp],
  ) -> RegisterX64 {
    for &op in oprefs {
      if op.kind() != IrOpKind::Inst {
        continue;
      }

      // 判定字段经共享借用快照读出，回写按索引重新定位（对齐 A64 allocReuse 手法）
      let idx = op.index() as usize;
      let (last_use, reused_reg, spilled, needs_reload, source_reg) = {
        let source = &self.function_ref().instructions[idx];
        (
          source.last_use,
          source.reused_reg,
          source.spilled,
          source.needs_reload,
          source.reg_x64,
        )
      };

      if last_use == inst_idx && !reused_reg && !spilled && !needs_reload {
        if (size == SizeX64::Xmmword) != (source_reg.size() == SizeX64::Xmmword) {
          continue;
        }

        CODEGEN_ASSERT!(source_reg.register_x_64_operator_ne(RegisterX64::NOREG));

        self.function_mut().instructions[idx].reused_reg = true;

        if size == SizeX64::Xmmword {
          self.xmm_inst_users[source_reg.index() as usize] = inst_idx;
        } else {
          self.gpr_inst_users[source_reg.index() as usize] = inst_idx;
        }

        return RegisterX64 {
          bits: (size as u8) | (source_reg.index() << RegisterX64::INDEX_SHIFT as u8),
        };
      }
    }

    self.alloc_reg(size, inst_idx)
  }

  pub fn assert_all_free(&self) {
    for reg in Self::K_GPR_ALLOC_ORDER {
      CODEGEN_ASSERT!(self.free_gpr_map[reg.index() as usize]);
    }

    for free in &self.free_xmm_map {
      CODEGEN_ASSERT!(*free);
    }
  }

  pub fn assert_no_spills(&self) {
    CODEGEN_ASSERT!(self.spills.is_empty());
  }

  pub fn can_take_reg(&self, reg: RegisterX64) -> bool {
    let free_map = if reg.size() == SizeX64::Xmmword {
      &self.free_xmm_map
    } else {
      &self.free_gpr_map
    };

    let inst_users = if reg.size() == SizeX64::Xmmword {
      &self.xmm_inst_users
    } else {
      &self.gpr_inst_users
    };

    free_map[reg.index() as usize] || inst_users[reg.index() as usize] != K_INVALID_INST_IDX
  }

  pub fn find_instruction_with_furthest_next_use(&self, reg_inst_users: &[u32; 16]) -> u32 {
    if self.curr_inst_idx == u32::MAX {
      return u32::MAX;
    }

    let mut furthest_use_target = u32::MAX;
    let mut furthest_use_location: u32 = 0;

    for &reg_inst_user in reg_inst_users.iter() {
      // 不能 spill 临时寄存器，或当前指令所定义值的寄存器
      if reg_inst_user == u32::MAX || reg_inst_user == self.curr_inst_idx {
        continue;
      }

      let mut in_vm_exit_sync = false;

      let next_use = get_next_inst_use(
        self.function_ref(),
        reg_inst_user,
        self.curr_inst_idx,
        &mut in_vm_exit_sync,
      );

      // 不能 spill 即将被当前指令使用的值
      if next_use == self.curr_inst_idx && !in_vm_exit_sync {
        continue;
      }

      if furthest_use_target == u32::MAX || next_use > furthest_use_location {
        furthest_use_location = next_use;
        furthest_use_target = reg_inst_user;
      }
    }

    furthest_use_target
  }

  pub fn find_spill_stack_slot(&mut self, value_kind: IrValueKind) -> u32 {
    if matches!(value_kind, IrValueKind::Float | IrValueKind::Int) {
      for i in 0..(self.used_spill_slot_halfs.len() as u32 * 64) {
        let bit_index = i as usize;
        let word_index = bit_index / 64;
        let bit_offset = bit_index % 64;
        if self.used_spill_slot_halfs[word_index] & (1u64 << bit_offset) != 0 {
          continue;
        }

        return i;
      }
    } else {
      // 位图尾部 3 个半字不够最宽的 Tvalue（4 半字）起泡，排除出搜索上界
      let max_start = self.used_spill_slot_halfs.len() as u32 * 64 - 3;

      let mut i = 0u32;
      while i < max_start {
        let bit_index = i as usize;
        let word_index = bit_index / 64;
        let bit_offset = bit_index % 64;

        if self.used_spill_slot_halfs[word_index] & (1u64 << bit_offset) != 0 {
          i += 2;
          continue;
        }

        let next_bit_offset = (bit_index + 1) % 64;
        let next_word_index = bit_index / 64 + (if bit_offset == 63 { 1 } else { 0 });
        if next_word_index < self.used_spill_slot_halfs.len()
          && self.used_spill_slot_halfs[next_word_index] & (1u64 << next_bit_offset) != 0
        {
          i += 2;
          continue;
        }

        if value_kind == IrValueKind::Tvalue {
          let bit_offset2 = (bit_index + 2) % 64;
          let word_index2 = bit_index / 64
            + (if bit_offset == 62 { 1 } else { 0 })
            + (if bit_offset == 63 { 1 } else { 0 });
          if word_index2 < self.used_spill_slot_halfs.len()
            && self.used_spill_slot_halfs[word_index2] & (1u64 << bit_offset2) != 0
          {
            // cpp: 手动 i += 2 后 continue 再经循环增量, 实际步进 4 —
            // i+2 位置已知占用, 无需重测该对齐位置
            i += 4;
            continue;
          }

          let bit_offset3 = (bit_index + 3) % 64;
          let word_index3 = bit_index / 64
            + (if bit_offset == 61 { 1 } else { 0 })
            + (if bit_offset == 62 { 1 } else { 0 })
            + (if bit_offset == 63 { 1 } else { 0 });
          if word_index3 < self.used_spill_slot_halfs.len()
            && self.used_spill_slot_halfs[word_index3] & (1u64 << bit_offset3) != 0
          {
            // 同上, 步进 4
            i += 4;
            continue;
          }
        }

        return i;
      }
    }

    CODEGEN_ASSERT!(false, "Nowhere to spill");
    !0u32
  }

  pub fn free_last_use_reg(&mut self, target: &mut IrInst, inst_idx: u32) {
    if self.is_last_use_reg(target, inst_idx) {
      debug_assert!(!target.spilled && !target.needs_reload);

      // 同一条指令内有多次 use 时，寄存器可能已被释放
      if target.reg_x64 == RegisterX64::NOREG {
        return;
      }

      self.free_reg(target.reg_x64);
      target.reg_x64 = RegisterX64::NOREG;
    }
  }

  pub fn free_last_use_regs(&mut self, inst: &IrInst, inst_idx: u32) {
    for &op in inst.ops.iter() {
      if op.kind() != IrOpKind::Inst {
        continue;
      }
      let idx = op.index() as usize;

      // 谓词与断言先经共享借用判定（与 free_last_use_reg 判定一致，
      // is_last_use_reg 为 &self 可与共享借用共存）；命中后仅回写 reg_x64，
      // 避免跨 free_reg 持有指令借用
      let reg_to_free = {
        let target = &self.function_ref().instructions[idx];
        if self.is_last_use_reg(target, inst_idx) {
          debug_assert!(!target.spilled && !target.needs_reload);
          (target.reg_x64 != RegisterX64::NOREG).then_some(target.reg_x64)
        } else {
          None
        }
      };

      if let Some(reg) = reg_to_free {
        self.free_reg(reg);
        self.function_mut().instructions[idx].reg_x64 = RegisterX64::NOREG;
      }
    }
  }

  pub fn free_reg(&mut self, reg: RegisterX64) {
    if reg.size() == SizeX64::Xmmword {
      CODEGEN_ASSERT!(!self.free_xmm_map[reg.index() as usize]);
      self.free_xmm_map[reg.index() as usize] = true;
      self.xmm_inst_users[reg.index() as usize] = K_INVALID_INST_IDX;
    } else {
      CODEGEN_ASSERT!(!self.free_gpr_map[reg.index() as usize]);
      self.free_gpr_map[reg.index() as usize] = true;
      self.gpr_inst_users[reg.index() as usize] = K_INVALID_INST_IDX;
    }
  }

  pub fn get_alloc_token(&self) -> u32 {
    self.alloc_action_count
  }

  pub fn get_extra_spill_address_offset(&self, slot: u32) -> i32 {
    debug_assert!(self.is_extra_spill_slot(slot));
    ((slot - K_SPILL_SLOTS * 2) * 4) as i32
  }

  pub fn get_restore_address(
    &self,
    inst: &IrInst,
    restore_location: ValueRestoreLocation,
  ) -> OperandX64 {
    let op = restore_location.op;
    CODEGEN_ASSERT!(op.kind() != IrOpKind::None);

    let _inst_kind = get_cmd_value_kind(inst.cmd);

    match restore_location.kind {
      IrValueKind::Unknown
      | IrValueKind::None
      | IrValueKind::Float
      | IrValueKind::Count
      | IrValueKind::Int64 => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg_value_int_64(vm_reg_op(op))
        } else {
          luau_constant_value(vm_const_op(op))
        }
      }
      IrValueKind::Tag => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg_tag(vm_reg_op(op))
        } else {
          luau_constant_tag(vm_const_op(op))
        }
      }
      IrValueKind::Int => {
        CODEGEN_ASSERT!(op.kind() == IrOpKind::VmReg);
        luau_reg_value_int(vm_reg_op(op))
      }
      IrValueKind::Pointer | IrValueKind::Double => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg_value(vm_reg_op(op))
        } else {
          luau_constant_value(vm_const_op(op))
        }
      }
      IrValueKind::Tvalue => {
        if op.kind() == IrOpKind::VmReg {
          luau_reg(vm_reg_op(op))
        } else {
          luau_constant(vm_const_op(op))
        }
      }
    }
  }

  pub fn ir_reg_alloc_x_64_ir_reg_alloc_x_64(
    build: &mut AssemblyBuilderX64,
    function: &mut IrFunction,
    stats: *mut LoweringStats,
  ) -> Self {
    let usable_xmm_reg_count = get_xmm_register_count(build.abi);

    Self {
      build: build as *mut AssemblyBuilderX64,
      function: function as *mut IrFunction,
      stats,
      curr_inst_idx: !0u32,
      free_gpr_map: [true; 16],
      gpr_inst_users: [u32::MAX; 16],
      free_xmm_map: [true; 16],
      xmm_inst_users: [u32::MAX; 16],
      usable_xmm_reg_count,
      used_spill_slot_halfs: [0u64; 8],
      max_used_slot: 0,
      next_spill_id: 1,
      spills: Vec::new(),
      exit_sync_args: DenseHashMap::new(u32::MAX),
      alloc_action_count: 0,
    }
  }

  pub fn is_extra_spill_slot(&self, slot: u32) -> bool {
    CODEGEN_ASSERT!(slot != emit_common_x_64::K_NO_STACK_SLOT);
    slot >= emit_common_x_64::K_SPILL_SLOTS * 2
  }

  pub fn is_last_use_reg(&self, target: &IrInst, inst_idx: u32) -> bool {
    target.last_use == inst_idx && !target.reused_reg
  }

  /// 按索引溢出目标指令的寄存器值：决策字段经共享借用快照读出，
  /// 回写一律按 `inst_idx` 重新定位（对齐 IrRegAllocA64::spill 手法），
  /// 不跨 `&mut self` 调用持有指令借用，`function` 裸指针零解引用。
  pub fn preserve(&mut self, inst_idx: u32) {
    let idx = inst_idx as usize;
    let (cmd, original_loc) = {
      let inst = &self.function_ref().instructions[idx];
      (inst.cmd, inst.reg_x64)
    };

    let mut spill = IrSpillX64 {
      inst_idx,
      value_kind: get_cmd_value_kind(cmd),
      spill_id: self.next_spill_id,
      stack_slot: IrSpillX64::K_NO_STACK_SLOT,
      original_loc,
    };
    self.next_spill_id += 1;

    // find_restore_location_ir_inst_bool 的索引化展开（get_inst_index 恒等还原）；
    // has_restore_location 仅按 op 判别，此处单次查询两用
    let restore_location = self
      .function_ref()
      .find_restore_location_u32_bool(inst_idx, true);

    if restore_location.op.kind() == IrOpKind::None {
      let i = self.find_spill_stack_slot(spill.value_kind);

      if self.is_extra_spill_slot(i) {
        let extra_offset = self.get_extra_spill_address_offset(i);
        let emergency_temp =
          if original_loc.size() == SizeX64::Xmmword || original_loc.index() != 11 {
            RegisterX64::R11
          } else {
            RegisterX64::R10
          };

        let build = self.build_mut();
        build.mov(temp_qword(), OperandX64::reg(emergency_temp));
        build.mov(
          OperandX64::reg(emergency_temp),
          mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, global) as i32),
        );
        build.lea_operand_x_64_operand_x_64(
          OperandX64::reg(emergency_temp),
          mem(
            SizeX64::None,
            emergency_temp,
            offset_of!(global_State, ecbdata) as i32 + extra_offset,
          ),
        );

        store_reg_to_mem(build, emergency_temp, 0, spill.value_kind, original_loc);

        build.mov(OperandX64::reg(emergency_temp), temp_qword());
      } else {
        let build = self.build_mut();
        store_reg_to_mem(
          build,
          RegisterX64::RSP,
          S_SPILL_AREA + i as i32 * 4,
          spill.value_kind,
          original_loc,
        );
      }

      let end = i + K_VALUE_DWORD_SIZE[spill.value_kind as usize];

      self.occupy_spill_slots(i, end);

      if end.div_ceil(2) > self.max_used_slot {
        self.max_used_slot = end.div_ceil(2);
      }

      spill.stack_slot = i as u8;
      self.function_mut().instructions[idx].spilled = true;

      if let Some(stats) = self.stats_mut() {
        stats.spills_to_slot += 1;
      }
    } else {
      let loc = restore_location;

      if loc.lazy {
        CODEGEN_ASSERT!(loc.op.kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(loc.conversion_cmd == IrCmd::NOP);

        let store_reg = vm_reg_op(loc.op);
        let build = self.build_mut();
        match spill.value_kind {
          IrValueKind::Tvalue => build.vmovups(luau_reg(store_reg), OperandX64::reg(original_loc)),
          IrValueKind::Double => build.vmovsd_operand_x_64_operand_x_64(
            luau_reg_value(store_reg),
            OperandX64::reg(original_loc),
          ),
          IrValueKind::Pointer | IrValueKind::Int64 => {
            build.mov(luau_reg_value(store_reg), OperandX64::reg(original_loc))
          }
          IrValueKind::Tag | IrValueKind::Int => {
            build.mov(luau_reg_value_int(store_reg), OperandX64::reg(original_loc))
          }
          _ => CODEGEN_ASSERT!(false, "Unsupported value kind for lazy store"),
        }

        if spill.value_kind != IrValueKind::Tvalue {
          build.mov(luau_reg_tag(store_reg), OperandX64::imm(0));
        }

        self
          .function_mut()
          .materialize_restore_location(spill.inst_idx);
      }

      self.function_mut().instructions[idx].needs_reload = true;

      if let Some(stats) = self.stats_mut() {
        stats.spills_to_restore += 1;
      }
    }

    self.spills.push(spill);

    self.free_reg(original_loc);
    self.function_mut().instructions[idx].reg_x64 = RegisterX64::NOREG;
  }

  pub fn preserve_and_free_inst_values(&mut self) {
    // 占用表里的 inst_idx 直接喂给索引化 preserve，全程无裸指针
    for inst_idx in self.gpr_inst_users {
      if inst_idx != K_INVALID_INST_IDX {
        self.preserve(inst_idx);
      }
    }

    for inst_idx in self.xmm_inst_users {
      if inst_idx != K_INVALID_INST_IDX {
        self.preserve(inst_idx);
      }
    }
  }

  pub fn record_and_free_last_use(
    &mut self,
    block_idx: u32,
    target: &mut IrInst,
    origin_inst_idx: u32,
  ) {
    let mut arg = ExitSyncArgX64 {
      inst_idx: self.function_ref().get_inst_index(target),
      reg: RegisterX64::NOREG,
      // cpp: ExitSyncArgX64 默认 stackSlot = kNoStackSlot (0xff) 哨兵
      stack_slot: IrSpillX64::K_NO_STACK_SLOT,
      original_reg: RegisterX64::NOREG,
      restore_location: ValueRestoreLocation::default(),
    };

    if target.spilled || target.needs_reload {
      // 纯查找：取首条 inst_idx 匹配的 spill 记录；body 内仅按该下标做 swap-remove
      if let Some(i) = self.spills.iter().position(|s| s.inst_idx == arg.inst_idx) {
        let spill = self.spills[i].clone();

        arg.original_reg = spill.original_loc;
        arg.stack_slot = spill.stack_slot;

        // 捕获当前指令处的恢复位置状态
        if arg.stack_slot == IrSpillX64::K_NO_STACK_SLOT {
          arg.restore_location = self
            .function_ref()
            .find_restore_location_ir_inst_bool(target, false);
        }

        // 若这是最后一次 use，则不完整恢复以释放寄存器，并删除 spill 记录
        if self.is_last_use_reg(target, origin_inst_idx) {
          if arg.stack_slot != IrSpillX64::K_NO_STACK_SLOT {
            let end = arg.stack_slot as u32 + K_VALUE_DWORD_SIZE[spill.value_kind as usize];

            self.free_spill_slots(arg.stack_slot as u32, end);
          }

          CODEGEN_ASSERT!(target.reg_x64 == RegisterX64::NOREG);
          target.spilled = false;
          target.needs_reload = false;

          self.spills[i] = self.spills[self.spills.len() - 1].clone();
          self.spills.pop();
        }
      }
    } else {
      CODEGEN_ASSERT!(target.reg_x64 != RegisterX64::NOREG);
      arg.reg = target.reg_x64;
      arg.original_reg = target.reg_x64;

      if self.is_last_use_reg(target, origin_inst_idx) {
        self.free_reg(target.reg_x64);
        target.reg_x64 = RegisterX64::NOREG;
      }
    }

    let entry = self.exit_sync_args.get_or_insert(block_idx);
    entry.push(arg);
  }

  pub fn restore(&mut self, inst: &mut IrInst, into_original_location: bool) {
    let inst_idx = self.function_ref().get_inst_index(inst);

    // 纯查找：首条 inst_idx 匹配记录即恢复目标，处理后就地 swap-remove；未命中直接返回
    let Some(i) = self.spills.iter().position(|s| s.inst_idx == inst_idx) else {
      return;
    };

    let original_loc = self.spills[i].original_loc;
    let reg = if into_original_location {
      self.take_reg(original_loc, inst_idx)
    } else {
      self.alloc_reg(original_loc.size(), inst_idx)
    };

    let restore_location = self
      .function_ref()
      .find_restore_location_ir_inst_bool(inst, false);
    let emergency_temp = if reg.size() == SizeX64::Xmmword {
      RegisterX64::R11
    } else {
      qword_reg(reg)
    };
    let spill = self.spills[i].clone();

    let mut restore_addr;

    if spill.stack_slot != IrSpillX64::K_NO_STACK_SLOT {
      if self.is_extra_spill_slot(spill.stack_slot as u32) {
        let extra_offset = self.get_extra_spill_address_offset(spill.stack_slot as u32);

        let build = self.build_mut();
        if reg.size() == SizeX64::Xmmword {
          build.mov(temp_qword(), OperandX64::reg(emergency_temp));
        }

        build.mov(
          OperandX64::reg(emergency_temp),
          mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, global) as i32),
        );
        build.lea_operand_x_64_operand_x_64(
          OperandX64::reg(emergency_temp),
          mem(
            SizeX64::None,
            emergency_temp,
            offset_of!(global_State, ecbdata) as i32 + extra_offset,
          ),
        );

        restore_addr = mem(reg.size(), emergency_temp, 0);
      } else {
        restore_addr = mem(
          SizeX64::None,
          RegisterX64::RSP,
          S_SPILL_AREA + spill.stack_slot as i32 * 4,
        );
        restore_addr.mem_size = reg.size();
      }

      if matches!(spill.value_kind, IrValueKind::Double | IrValueKind::Int64) {
        restore_addr.mem_size = SizeX64::Qword;
      } else if spill.value_kind == IrValueKind::Float {
        restore_addr.mem_size = SizeX64::Dword;
      }

      let end = spill.stack_slot as u32 + K_VALUE_DWORD_SIZE[spill.value_kind as usize];

      self.free_spill_slots(spill.stack_slot as u32, end);
    } else {
      restore_addr = self.get_restore_address(inst, restore_location);
    }

    let build = self.build_mut();
    match spill.value_kind {
      IrValueKind::Tvalue => build.vmovups(OperandX64::reg(reg), restore_addr),
      IrValueKind::Double => {
        build.vmovsd_operand_x_64_operand_x_64(OperandX64::reg(reg), restore_addr)
      }
      IrValueKind::Int if restore_location.kind == IrValueKind::Double => {
        if restore_location.conversion_cmd == IrCmd::IntToNum {
          build.vcvttsd2si(OperandX64::reg(reg), restore_addr);
        } else if restore_location.conversion_cmd == IrCmd::UintToNum {
          build.vcvttsd2si(OperandX64::reg(qword_reg(reg)), restore_addr);
        } else {
          CODEGEN_ASSERT!(
            false,
            "re-materialization not supported for this conversion command"
          );
        }
      }
      IrValueKind::Tag | IrValueKind::Int | IrValueKind::Int64 | IrValueKind::Pointer => {
        build.mov(OperandX64::reg(reg), restore_addr);
      }
      IrValueKind::Float => {
        build.vmovss_operand_x_64_operand_x_64(OperandX64::reg(reg), restore_addr)
      }
      _ => CODEGEN_ASSERT!(false, "value kind not supported for restore"),
    }

    if spill.stack_slot != IrSpillX64::K_NO_STACK_SLOT
      && self.is_extra_spill_slot(spill.stack_slot as u32)
      && reg.size() == SizeX64::Xmmword
    {
      let build = self.build_mut();
      build.mov(OperandX64::reg(emergency_temp), temp_qword());
    }

    inst.reg_x64 = reg;
    inst.spilled = false;
    inst.needs_reload = false;

    self.spills[i] = self.spills[self.spills.len() - 1].clone();
    self.spills.pop();
  }

  pub fn setup_exit_sync_entry(&mut self, block_idx: u32) {
    update_last_use_locations_in_block(self.function_mut(), block_idx);

    let Some(args) = self.exit_sync_args.find(&block_idx).cloned() else {
      return;
    };

    // 指令表读写一律按 inst_idx 即时定位，借用不跨 self 方法调用
    for arg in args.iter() {
      let idx = arg.inst_idx as usize;

      {
        let inst = &mut self.function_mut().instructions[idx];
        inst.reused_reg = false;
        inst.needs_reload = false;
        inst.spilled = false;
      }

      if arg.reg != RegisterX64::NOREG {
        self.function_mut().instructions[idx].reg_x64 = arg.reg;

        self.take_reg(arg.reg, arg.inst_idx);
      } else if arg.stack_slot != IrSpillX64::K_NO_STACK_SLOT {
        let value_kind = get_cmd_value_kind(self.function_ref().instructions[idx].cmd);

        {
          let inst = &mut self.function_mut().instructions[idx];
          inst.reg_x64 = RegisterX64::NOREG;
          inst.spilled = true;
        }

        let spill = IrSpillX64 {
          inst_idx: arg.inst_idx,
          value_kind,
          spill_id: 0,
          stack_slot: arg.stack_slot,
          original_loc: arg.original_reg,
        };

        // cpp: 此处重建的 spill 不分配 spillId (保持默认 0),
        // 使 ScopedSpills 不把跨块快照的 spill 视作本作用域创建

        // 标记 spill 槽已占用，以便 restore 能释放它
        let end = spill.stack_slot as u32 + K_VALUE_DWORD_SIZE[spill.value_kind as usize];
        self.occupy_free_spill_slots(spill.stack_slot as u32, end);

        self.spills.push(spill);
      } else {
        let value_kind = get_cmd_value_kind(self.function_ref().instructions[idx].cmd);

        // 值带有恢复地址（可 rematerialize）
        {
          let inst = &mut self.function_mut().instructions[idx];
          inst.reg_x64 = RegisterX64::NOREG;
          inst.needs_reload = true;
        }

        // 重新记录快照时捕获的恢复位置
        // 源 block 中更晚的指令可能已在 IrValueLocationTracking 中使其失效
        self
          .function_mut()
          .record_restore_location(arg.inst_idx, arg.restore_location);

        let spill = IrSpillX64 {
          inst_idx: arg.inst_idx,
          value_kind,
          spill_id: 0,
          stack_slot: IrSpillX64::K_NO_STACK_SLOT,
          original_loc: arg.original_reg,
        };

        self.spills.push(spill);
      }
    }
  }

  pub fn should_free_gpr(&self, reg: RegisterX64) -> bool {
    if reg.register_x_64_operator_eq(RegisterX64::NOREG) {
      return false;
    }

    CODEGEN_ASSERT!(reg.size() != SizeX64::Xmmword);

    for &gpr in &Self::K_GPR_ALLOC_ORDER {
      if reg.index() == gpr.index() {
        return true;
      }
    }

    false
  }

  pub fn take_reg(&mut self, reg: RegisterX64, inst_idx: u32) -> RegisterX64 {
    if reg.size() == SizeX64::Xmmword {
      if !self.free_xmm_map[reg.index() as usize] {
        // 占用者索引直接传给索引化 preserve，不再经指令表裸指针借用
        let user = self.xmm_inst_users[reg.index() as usize];
        CODEGEN_ASSERT!(user != K_INVALID_INST_IDX);
        self.preserve(user);
      }

      CODEGEN_ASSERT!(self.free_xmm_map[reg.index() as usize]);
      self.free_xmm_map[reg.index() as usize] = false;
      self.xmm_inst_users[reg.index() as usize] = inst_idx;
    } else {
      if !self.free_gpr_map[reg.index() as usize] {
        let user = self.gpr_inst_users[reg.index() as usize];
        CODEGEN_ASSERT!(user != K_INVALID_INST_IDX);
        self.preserve(user);
      }

      CODEGEN_ASSERT!(self.free_gpr_map[reg.index() as usize]);
      self.free_gpr_map[reg.index() as usize] = false;
      self.gpr_inst_users[reg.index() as usize] = inst_idx;
    }

    reg
  }
}

// 溢出栈帧布局常量与寻址助手：preserve/restore 共用，单一来源收口于本文件
// （a64 同形先例见 `ir_reg_alloc_a_64_spill_ir_reg_alloc_a_64` 的共享常量段）。
pub(crate) const S_TEMPORARY_SLOT: i32 = 64;

pub(crate) const S_SPILL_AREA: i32 = 72;

pub(crate) fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}

pub(crate) fn temp_qword() -> OperandX64 {
  mem(SizeX64::Qword, RegisterX64::RSP, S_TEMPORARY_SLOT)
}

/// 按 `kind` 的存储宽度把 `reg` 存入 `mem(宽度, base, disp)`：本函数的应急槽与
/// 常规栈槽两条溢出路径共用（原两段同形 match 收口为一处）。
fn store_reg_to_mem(
  build: &mut AssemblyBuilderX64,
  base: RegisterX64,
  disp: i32,
  kind: IrValueKind,
  reg: RegisterX64,
) {
  match kind {
    IrValueKind::Tvalue => build.vmovups(mem(SizeX64::Xmmword, base, disp), OperandX64::reg(reg)),
    IrValueKind::Double => {
      build.vmovsd_operand_x_64_operand_x_64(mem(SizeX64::Qword, base, disp), OperandX64::reg(reg))
    }
    IrValueKind::Pointer | IrValueKind::Int64 => {
      build.mov(mem(SizeX64::Qword, base, disp), OperandX64::reg(reg))
    }
    IrValueKind::Tag | IrValueKind::Int => {
      build.mov(mem(SizeX64::Dword, base, disp), OperandX64::reg(reg))
    }
    IrValueKind::Float => {
      build.vmovss_operand_x_64_operand_x_64(mem(SizeX64::Dword, base, disp), OperandX64::reg(reg))
    }
    _ => CODEGEN_ASSERT!(false, "Unsupported value kind"),
  }
}
