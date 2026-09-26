use alloc::vec::Vec;
use core::mem::{offset_of, size_of};

use ulua_common::{
  fflag,
  macros::luau_assert::LUAU_UNREACHABLE,
  records::{dense_hash_map::DenseHashMap, small_vector::SmallVector},
};
use ulua_vm::{
  records::{global_state::global_State, lua_state::LuaState},
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind, kind_a_64::KindA64},
  functions::{
    alloc_spill::alloc_spill, cast_reg::cast_reg, countlz_bit_utils::countlz_u32,
    countrz_bit_utils::countrz_u32, free_spill::free_spill, get_cmd_value_kind::get_cmd_value_kind,
    get_next_inst_use::get_next_inst_use, get_reload_address::get_reload_address,
    get_reload_offset::get_reload_offset, is_used_in_vm_exit_sync::is_used_in_vm_exit_sync,
    update_last_use_locations_in_block::update_last_use_locations_in_block, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64,
    assembly_builder_a_64::AssemblyBuilderA64,
    emit_common_a_64,
    emit_common_a_64::{K_EXTRA_SPILL_SLOTS, K_SPILL_SLOTS},
    exit_sync_arg_a_64::ExitSyncArgA64,
    ir_data::K_INVALID_INST_IDX,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    lowering_stats::LoweringStats,
    register_a_64::{RegisterA64, reg},
    set::Set,
    spill::Spill,
    value_restore_location::ValueRestoreLocation,
  },
  type_aliases::mem::mem,
};

/// cpp `IrRegAllocA64`（`CodeGen/src/IrRegAllocA64.h`）。
///
/// `build/function/stats` 是与宿主 `IrLoweringA64` 共享的同一组上游裸指针（见该
/// 结构文档），改为 `&mut` 会与 lowering 侧的读写形成双 `&mut` 而编译失败；保持
/// 裸指针，生命周期由构造点栈帧保证。
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrRegAllocA64 {
  pub build: *mut AssemblyBuilderA64,
  pub function: *mut IrFunction,
  pub stats: *mut LoweringStats,

  pub curr_inst_idx: u32,

  pub gpr: Set,
  pub simd: Set,

  pub spills: Vec<Spill>,

  pub free_spill_slots: u64,

  pub exit_sync_args: DenseHashMap<u32, SmallVector<ExitSyncArgA64, 2>>,

  pub alloc_action_count: u32,

  pub error: bool,
}

impl IrRegAllocA64 {
  // 共享裸指针访问器家族（function_mut/function_ref/build_mut/stats_mut）由宏收口，
  // 统一契约见 `crate::shared_ptr_accessors!`（与宿主 IrLoweringA64 同一守则）。
  crate::shared_ptr_accessors!(AssemblyBuilderA64);

  pub fn alloc_reg(&mut self, kind: KindA64, index: u32) -> RegisterA64 {
    self.alloc_action_count += 1;

    if self.get_set(kind).free == 0 {
      // 尝试找一个未被当前指令使用、next use 最远的寄存器来 spill
      if self.refill_free(kind, index).is_none() {
        self.error = true;
        return RegisterA64 {
          bits: (kind as u8) & RegisterA64::KIND_MASK,
        };
      }
    }

    let reg = self.take_free(kind);
    let set = self.get_set(kind);
    set.defs[reg as usize] = index;

    RegisterA64 {
      bits: ((kind as u8) & RegisterA64::KIND_MASK) | ((reg as u8) << RegisterA64::INDEX_SHIFT),
    }
  }

  pub fn alloc_reuse(&mut self, kind: KindA64, index: u32, oprefs: &[IrOp]) -> RegisterA64 {
    for &op in oprefs {
      if op.kind() != IrOpKind::Inst {
        continue;
      }

      // 判定字段经共享借用读出，写入按索引重新定位，全程安全
      let (last_use, reused_reg, reg_a64) = {
        let source = &self.function_ref().instructions[op.index() as usize];
        (source.last_use, source.reused_reg, source.reg_a64)
      };

      if last_use == index && !reused_reg && reg_a64.register_a_64_operator_ne(RegisterA64::NOREG) {
        let set = self.get_set(kind);
        set.defs[reg_a64.index() as usize] = index;

        let source = &mut self.function_mut().instructions[op.index() as usize];
        source.reused_reg = true;
        return source.reg_a64;
      }
    }

    self.alloc_reg(kind, index)
  }

  pub fn alloc_temp(&mut self, kind: KindA64) -> RegisterA64 {
    self.alloc_action_count += 1;

    if self.get_set(kind).free == 0 {
      // 尝试找一个未被当前指令使用、next use 最远的寄存器来 spill
      if self.refill_free(kind, self.curr_inst_idx).is_none() {
        self.error = true;
        return RegisterA64 {
          bits: (kind as u8) & RegisterA64::KIND_MASK,
        };
      }
    }

    let reg = self.take_free(kind);
    let set = self.get_set(kind);
    set.temp |= 1u32 << reg;
    CODEGEN_ASSERT!(set.defs[reg as usize] == K_INVALID_INST_IDX);

    RegisterA64 {
      bits: ((kind as u8) & RegisterA64::KIND_MASK) | ((reg as u8) << RegisterA64::INDEX_SHIFT),
    }
  }

  /// `alloc_reg`/`alloc_temp` 共用的 free 耗尽补充路径：把 next use 最远的占用者溢出，
  /// 成功（含溢出后 free 非空的断言）返回 `Some`，无可溢出返回 `None`（error 由调用方置位）。
  fn refill_free(&mut self, kind: KindA64, spill_index: u32) -> Option<()> {
    let furthest_use_target = self.find_instruction_with_furthest_next_use(kind);

    if furthest_use_target == K_INVALID_INST_IDX {
      return None;
    }

    self.spill_set_u32_u32(kind, spill_index, furthest_use_target);
    CODEGEN_ASSERT!(self.get_set(kind).free != 0);

    Some(())
  }

  /// `alloc_reg`/`alloc_temp` 共用的取空闲寄存器路径：high 端优先（chaos fflag 反向
  /// 从 low 端取以放大调用点冲突），取走并清位，返回寄存器编号。
  fn take_free(&mut self, kind: KindA64) -> i32 {
    let set = self.get_set(kind);
    let mut reg = 31 - countlz_u32(set.free);

    if fflag::DebugCodegenChaosA64.get() {
      reg = countrz_u32(set.free); // allocate from low end; this causes extra conflicts for calls
    }

    set.free &= !(1u32 << reg);
    reg
  }

  pub fn find_instruction_with_furthest_next_use(&self, kind: KindA64) -> u32 {
    if self.curr_inst_idx == K_INVALID_INST_IDX {
      return K_INVALID_INST_IDX;
    }

    let set = self.get_set_ref(kind);
    let mut furthest_use_target = K_INVALID_INST_IDX;
    let mut furthest_use_location: u32 = 0;

    for &reg_inst_user in set.defs.iter() {
      // 不能 spill 临时寄存器，或当前指令所定义值的寄存器
      if reg_inst_user == K_INVALID_INST_IDX || reg_inst_user == self.curr_inst_idx {
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

      if furthest_use_target == K_INVALID_INST_IDX || next_use > furthest_use_location {
        furthest_use_location = next_use;
        furthest_use_target = reg_inst_user;
      }
    }

    furthest_use_target
  }

  pub fn free_last_use_reg(&mut self, target: &mut IrInst, index: u32) {
    if target.last_use == index && !target.reused_reg {
      CODEGEN_ASSERT!(!target.spilled && !target.needs_reload);

      // 同一条指令内有多次 use 时，寄存器可能已被释放
      if target.reg_a64 == RegisterA64::NOREG {
        return;
      }

      self.free_reg(target.reg_a64);
      target.reg_a64 = RegisterA64::NOREG;
    }
  }

  pub fn free_last_use_regs(&mut self, inst: &IrInst, index: u32) {
    for &op in inst.ops.iter() {
      if op.kind() != IrOpKind::Inst {
        continue;
      }
      let op_index = op.index();

      // 谓词与断言先经共享借用判定；命中后仅回写 reg_a64，
      // 避免跨 free_reg（可能触发溢出链）持有指令借用
      let reg_to_free = {
        let target = &self.function_ref().instructions[op_index as usize];
        if target.last_use == index && !target.reused_reg {
          CODEGEN_ASSERT!(!target.spilled && !target.needs_reload);
          (target.reg_a64 != RegisterA64::NOREG).then_some(target.reg_a64)
        } else {
          None
        }
      };

      if let Some(reg) = reg_to_free {
        self.free_reg(reg);
        self.function_mut().instructions[op_index as usize].reg_a64 = RegisterA64::NOREG;
      }
    }
  }

  pub fn free_reg(&mut self, reg: RegisterA64) {
    let set = self.get_set(reg.kind());

    let bit = 1u32 << reg.index();

    CODEGEN_ASSERT!((set.base & bit) != 0);
    CODEGEN_ASSERT!((set.free & bit) == 0);
    CODEGEN_ASSERT!((set.temp & bit) == 0);

    set.free |= bit;
    set.defs[reg.index() as usize] = K_INVALID_INST_IDX;
  }

  pub fn get_alloc_token(&self) -> u32 {
    self.alloc_action_count
  }

  pub fn get_extra_spill_address_offset(&self, slot: u32) -> i32 {
    CODEGEN_ASSERT!(
      self.is_extra_spill_slot(slot),
      b"slot is not an extra spill slot\0".as_ptr() as *const i8
    );
    ((slot - K_SPILL_SLOTS) * 8) as i32
  }

  pub(crate) fn get_set(&mut self, kind: KindA64) -> &mut Set {
    match kind {
      KindA64::X | KindA64::W => &mut self.gpr,

      KindA64::S | KindA64::D | KindA64::Q => &mut self.simd,

      _ => {
        CODEGEN_ASSERT!(false, "Unexpected register kind");
        LUAU_UNREACHABLE!();
      }
    }
  }

  /// 只读版：供 `&self` 查询路径（furthest-next-use 等）使用
  pub(crate) fn get_set_ref(&self, kind: KindA64) -> &Set {
    match kind {
      KindA64::X | KindA64::W => &self.gpr,

      KindA64::S | KindA64::D | KindA64::Q => &self.simd,

      _ => {
        debug_assert!(false, "Unexpected register kind");
        LUAU_UNREACHABLE!();
      }
    }
  }

  pub fn ir_reg_alloc_a_64_ir_reg_alloc_a_64(
    build: &mut AssemblyBuilderA64,
    function: &mut IrFunction,
    stats: *mut LoweringStats,
    regs: &[(RegisterA64, RegisterA64)],
  ) -> Self {
    let mut alloc = IrRegAllocA64 {
      build: build as *mut AssemblyBuilderA64,
      function: function as *mut IrFunction,
      stats,
      curr_inst_idx: K_INVALID_INST_IDX,
      gpr: Set {
        base: 0,
        free: 0,
        temp: 0,
        defs: [u32::MAX; 32],
      },
      simd: Set {
        base: 0,
        free: 0,
        temp: 0,
        defs: [u32::MAX; 32],
      },
      spills: Vec::new(),
      free_spill_slots: 0,
      exit_sync_args: DenseHashMap::new(!0u32),
      alloc_action_count: 0,
      error: false,
    };

    for (first, second) in regs {
      CODEGEN_ASSERT!(first.kind() == second.kind() && first.index() <= second.index());

      let set = alloc.get_set(first.kind());
      for i in first.index()..=second.index() {
        set.base |= 1u32 << i;
      }
    }

    alloc.gpr.free = alloc.gpr.base;
    alloc.simd.free = alloc.simd.base;

    // cpp IrRegAllocA64.cpp:147-148：位图宽度 = 栈 spill 槽 + ulua 扩展 extra 槽，
    // 单一常量源见 records::emit_common_a_64（曾有局部 K_SPILL_SLOTS=16/K_EXTRA=8 遮蔽真值 22/2，已收敛）。
    CODEGEN_ASSERT!(K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS < 64);
    alloc.free_spill_slots = (1u64 << (K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS)) - 1u64;

    alloc
  }

  pub fn is_extra_spill_slot(&self, slot: u32) -> bool {
    slot >= K_SPILL_SLOTS
  }

  pub fn record_and_free_last_use(
    &mut self,
    block_idx: u32,
    target: &mut IrInst,
    origin_inst_idx: u32,
  ) {
    let mut arg = ExitSyncArgA64 {
      inst_idx: self.function_ref().get_inst_index(target),
      reg: RegisterA64::NOREG,
      // cpp: ExitSyncArgA64 默认 slot = kNoSpillSlot (-1) 哨兵
      slot: K_NO_SPILL_SLOT,
      original_reg: RegisterA64::NOREG,
      restore_location: ValueRestoreLocation::default(),
    };

    if target.spilled || target.needs_reload {
      // 纯查找：取首条 inst 匹配的 spill 记录；body 内仅按该下标做 swap-remove
      if let Some(i) = self.spills.iter().position(|s| s.inst == arg.inst_idx) {
        let spill = self.spills[i];

        arg.original_reg = spill.origin;
        arg.slot = spill.slot;

        // 捕获当前指令处的恢复位置状态
        if arg.slot == K_NO_SPILL_SLOT {
          arg.restore_location = self
            .function_ref()
            .find_restore_location_ir_inst_bool(target, false);
        }

        // 若这是最后一次 use，则不完整恢复以释放寄存器，并删除 spill 记录
        if target.last_use == origin_inst_idx && !target.reused_reg {
          if arg.slot >= 0 {
            free_spill(
              &mut self.free_spill_slots,
              spill.origin.kind(),
              spill.slot as u8,
            );
          }

          CODEGEN_ASSERT!(target.reg_a64 == RegisterA64::NOREG);
          target.spilled = false;
          target.needs_reload = false;

          self.spills[i] = self.spills[self.spills.len() - 1];
          self.spills.pop();
        }
      }
    } else {
      CODEGEN_ASSERT!(target.reg_a64 != RegisterA64::NOREG);
      arg.reg = target.reg_a64;
      arg.original_reg = target.reg_a64;

      if target.last_use == origin_inst_idx && !target.reused_reg {
        self.free_reg(target.reg_a64);
        target.reg_a64 = RegisterA64::NOREG;
      }
    }

    let entry = self.exit_sync_args.get_or_insert(block_idx);
    entry.push(arg);
  }

  /// cpp `restore(uint start)`：从 `start` 起恢复全部 spill 记录
  pub fn restore_usize(&mut self, start: usize) {
    CODEGEN_ASSERT!(start <= self.spills.len());

    if start < self.spills.len() {
      let mut i = start;
      while i < self.spills.len() {
        let s = self.spills[i]; // copy in case takeReg reallocates spills
        let reg = self.take_reg(s.origin, s.inst);

        self.restore_ir_reg_alloc_a_64_spill_register_a_64(&s, reg);

        i += 1;
      }

      self.spills.truncate(start);
    }
  }

  /// cpp `restore(Spill&, RegisterA64)`：恢复单条 spill 记录的值到 `reg`。
  /// 目标指令字段先快照、尾部统一写回，restore 位置查询改走索引化接口，
  /// 指令访问全程安全。
  pub fn restore_ir_reg_alloc_a_64_spill_register_a_64(&mut self, s: &Spill, reg: RegisterA64) {
    let inst_cmd = self.function_ref().instructions[s.inst as usize].cmd;
    CODEGEN_ASSERT!(
      self.function_ref().instructions[s.inst as usize].reg_a64 == RegisterA64::NOREG
    );

    if s.slot >= 0 {
      if self.is_extra_spill_slot(s.slot as u32) {
        let extra_offset = self.get_extra_spill_address_offset(s.slot as u32);

        // 需要算一个地址，但寄存器可能全被占用。
        // 若正在恢复整数寄存器，可直接拿它当临时寄存器。
        let emergency_temp = if reg.kind() == KindA64::W {
          cast_reg(KindA64::X, reg)
        } else if reg.kind() == KindA64::X {
          reg
        } else {
          X17
        };

        if reg.kind() != KindA64::W && reg.kind() != KindA64::X {
          self.build_mut().str(emergency_temp, s_temporary());
        }

        let build = self.build_mut();
        build.ldr(
          emergency_temp,
          mem(R_STATE, offset_of!(LuaState, global) as i32),
        );
        build.ldr(
          emergency_temp,
          mem(emergency_temp, offset_of!(global_State, ecbdata) as i32),
        );

        build.ldr(reg, mem(emergency_temp, extra_offset));

        if reg.kind() != KindA64::W && reg.kind() != KindA64::X {
          build.ldr(emergency_temp, s_temporary());
        }
      } else {
        self
          .build_mut()
          .ldr(reg, mem(SP, S_SPILL_AREA_DATA + s.slot as i32 * 8));
      }

      if s.slot != K_INVALID_SPILL {
        free_spill(&mut self.free_spill_slots, reg.kind(), s.slot as u8);
      }
    } else {
      let (spilled, needs_reload) = {
        let inst = &self.function_ref().instructions[s.inst as usize];
        (inst.spilled, inst.needs_reload)
      };
      CODEGEN_ASSERT!(!spilled && needs_reload);

      // 恢复值时允许跨 block 恢复，因为 spill 时已承诺目标位置。
      let restore_location = self
        .function_ref()
        .find_restore_location_u32_bool(s.inst, false);

      let addr = get_reload_address(restore_location);
      CODEGEN_ASSERT!(addr.base != XZR);

      let spill_value_kind = get_cmd_value_kind(inst_cmd);

      if spill_value_kind == IrValueKind::Int && restore_location.kind == IrValueKind::Double {
        // 处理从存放 double 的位置恢复 int/uint 值。
        let temp = self.alloc_temp(KindA64::D);
        self.build_mut().ldr(temp, addr);

        if restore_location.conversion_cmd == IrCmd::IntToNum {
          self.build_mut().fcvtzs(reg, temp);
        } else if restore_location.conversion_cmd == IrCmd::UintToNum {
          self.build_mut().fcvtzs(cast_reg(KindA64::X, reg), temp);
        } else {
          CODEGEN_ASSERT!(false);
        }

        // 临时寄存器可能占用了 spill 恢复流程中其他寄存器所需的位置。
        self.free_temp(temp);
      } else {
        self.build_mut().ldr(reg, addr);
      }
    }

    let inst = &mut self.function_mut().instructions[s.inst as usize];
    inst.spilled = false;
    inst.needs_reload = false;
    inst.reg_a64 = reg;
  }

  pub fn restore_reg(&mut self, inst: &mut IrInst) {
    let index = self.function_ref().get_inst_index(inst);

    // 纯查找：首条 inst 匹配记录即恢复目标，处理后就地 swap-remove 并返回
    if let Some(i) = self.spills.iter().position(|s| s.inst == index) {
      let s = self.spills[i]; // copy in case allocReg reallocates spills
      let reg = self.alloc_reg(s.origin.kind(), index);

      self.restore_ir_reg_alloc_a_64_spill_register_a_64(&s, reg);

      self.spills[i] = self.spills[self.spills.len() - 1];
      self.spills.pop();
      return;
    }

    CODEGEN_ASSERT!(false, "Expected to find a spill record");
  }

  pub fn setup_exit_sync_entry(&mut self, block_idx: u32) {
    update_last_use_locations_in_block(self.function_mut(), block_idx);

    let Some(args) = self.exit_sync_args.find(&block_idx).cloned() else {
      return;
    };

    for arg in args.iter() {
      // 阶段一：指令标志复位与寄存器状态回填（借用随语句结束）
      let inst = &mut self.function_mut().instructions[arg.inst_idx as usize];
      inst.reused_reg = false;
      inst.needs_reload = false;
      inst.spilled = false;

      if arg.reg != RegisterA64::NOREG {
        inst.reg_a64 = arg.reg;
      } else if arg.slot >= 0 {
        inst.reg_a64 = RegisterA64::NOREG;
        inst.spilled = true;
      } else {
        inst.reg_a64 = RegisterA64::NOREG;
        inst.needs_reload = true;
      }

      // 阶段二：寄存器/溢出簿记（可能触发溢出链，不得持有指令借用）
      if arg.reg != RegisterA64::NOREG {
        self.take_reg(arg.reg, arg.inst_idx);
      } else if arg.slot >= 0 {
        self.spills.push(Spill {
          inst: arg.inst_idx,
          origin: arg.original_reg,
          slot: arg.slot,
        });

        // 标记 spill 槽已占用，restore() 时可释放它
        let mask = if arg.original_reg.kind() == KindA64::Q {
          3u64
        } else {
          1u64
        } << (arg.slot as u64);

        CODEGEN_ASSERT!((self.free_spill_slots & mask) == mask);
        self.free_spill_slots &= !mask;
      } else {
        // 重新记录快照时捕获的恢复位置
        // 源 block 中更晚的指令可能已在 IrValueLocationTracking 中使其失效
        self
          .function_mut()
          .record_restore_location(arg.inst_idx, arg.restore_location);

        self.spills.push(Spill {
          inst: arg.inst_idx,
          origin: arg.original_reg,
          slot: arg.slot,
        });
      }
    }
  }

  /// cpp `spill(Idx, std::initializer_list<RegisterA64>)`：
  /// 释放全部临时寄存器后，把两个集合中所有已分配寄存器逐一溢出。
  pub fn spill_u32_initializer_list_register_a_64(
    &mut self,
    index: u32,
    live: &[RegisterA64],
  ) -> usize {
    let sets = [KindA64::X, KindA64::Q];

    let start = self.spills.len();

    let mut poisongpr = 0u32;
    let mut poisonsimd = 0u32;

    if fflag::DebugCodegenChaosA64.get() {
      poisongpr = self.gpr.base & !self.gpr.free;
      poisonsimd = self.simd.base & !self.simd.free;

      for reg in live.iter() {
        if matches!(reg.kind(), KindA64::S | KindA64::D | KindA64::Q) {
          poisonsimd &= !(1u32 << reg.index());
        } else {
          poisongpr &= !(1u32 << reg.index());
        }
      }
    }

    for kind in sets {
      let set = self.get_set(kind);

      // 提前退出
      if set.free == set.base {
        continue;
      }

      // 释放所有临时寄存器
      CODEGEN_ASSERT!((set.free & set.temp) == 0);
      set.free |= set.temp;
      set.temp = 0;

      // spill 所有已分配寄存器，除非它们已不再使用
      let mut regs = set.base & !set.free;

      while regs != 0 {
        let reg = 31 - countlz_u32(regs);

        let target_inst_idx = self.get_set(kind).defs[reg as usize];

        CODEGEN_ASSERT!(target_inst_idx != K_INVALID_INST_IDX);
        CODEGEN_ASSERT!(
          self.function_ref().instructions[target_inst_idx as usize]
            .reg_a64
            .index()
            == reg as u8
        );

        // 集位回收由 spill 内部完成，与 cpp spill(Set&, Idx, Idx) 一致
        self.spill_set_u32_u32(kind, index, target_inst_idx);

        regs &= !(1u32 << reg as u32);
      }

      CODEGEN_ASSERT!(self.get_set(kind).free == self.get_set(kind).base);
    }

    if fflag::DebugCodegenChaosA64.get() {
      for i in 0..32 {
        if poisongpr & (1u32 << i) != 0 {
          self
            .build_mut()
            .mov_register_a_64_i32(reg(KindA64::X, i as u8), 0xdead);
        }
        if poisonsimd & (1u32 << i) != 0 {
          self
            .build_mut()
            .fmov_register_a_64_f64(reg(KindA64::D, i as u8), -0.125);
        }
      }
    }

    start
  }

  /// cpp `spill(Set&, Idx, Idx)`：把 `target_inst_idx` 指令的寄存器值溢出；
  /// `kind` 选定所属集合，尾部回收集位（free/defs 更新）。
  /// 原 `Set&` 参数改为 kind 选择，调用侧不再需要裸指针绕借用。
  pub fn spill_set_u32_u32(&mut self, kind: KindA64, index: u32, target_inst_idx: u32) {
    // 快照目标指令：决策期间指令表不被修改；restore 位置查询改走索引化接口
    // （原 find_restore_location_ir_inst_bool 经 get_inst_index 指针换算，
    // 只对指令表内元素有效，快照克隆不可用）
    let def = self.function_ref().instructions[target_inst_idx as usize].clone();
    let reg = def.reg_a64.index();

    CODEGEN_ASSERT!(!def.reused_reg);
    CODEGEN_ASSERT!(!def.spilled);
    CODEGEN_ASSERT!(!def.needs_reload);

    // cpp: def.lastUse == index && !isUsedInVmExitSync(...) —
    // 若值还要被 VM exit sync 读取, 即使是 last use 也不能跳过 spill
    if def.last_use == index
      && !is_used_in_vm_exit_sync(self.function_ref(), index, target_inst_idx)
    {
      // 与其 spill 一个永不再重载的寄存器，不如视其为不再需要。
    } else {
      let restore_location = self
        .function_ref()
        .find_restore_location_u32_bool(target_inst_idx, true);
      // has_restore_location_ir_inst_bool 的展开：仅按 op 判别
      let has_restore = restore_location.op.kind() != IrOpKind::None;

      if has_restore {
        // 若值的恢复位置是 lazy 的，需要将其物化。
        if restore_location.lazy {
          CODEGEN_ASSERT!(restore_location.op.kind() == IrOpKind::VmReg);
          CODEGEN_ASSERT!(restore_location.conversion_cmd == IrCmd::NOP);

          let store_reg = vm_reg_op(restore_location.op);
          let addr = mem(
            R_BASE,
            store_reg * size_of::<TValue>() as i32 + get_reload_offset(restore_location.kind),
          );

          self.build_mut().str(def.reg_a64, addr);

          // 部分值 store 不应有 VM/GC 解释，由 'nil' tag 保护。
          if restore_location.kind != IrValueKind::Tvalue {
            self.build_mut().str(
              WZR,
              mem(
                R_BASE,
                store_reg * size_of::<TValue>() as i32 + offset_of!(TValue, tt) as i32,
              ),
            );
          }

          self
            .function_mut()
            .materialize_restore_location(target_inst_idx);
        }

        // 检查值是否有可 spill 的恢复操作时，只允许同 block 内的。
        // 与其 spill 到栈，不如从 VM 栈/constants 重载。
        // 仍需记录这次 spill，restore(start) 才能工作。
        self.spills.push(Spill {
          inst: target_inst_idx,
          origin: def.reg_a64,
          slot: K_NO_SPILL_SLOT,
        });

        self.function_mut().instructions[target_inst_idx as usize].needs_reload = true;

        if let Some(stats) = self.stats_mut() {
          stats.spills_to_restore += 1;
        }
      } else {
        let mut slot = alloc_spill(&mut self.free_spill_slots, def.reg_a64.kind());
        if slot < 0 {
          slot = K_INVALID_SPILL as i32;
          self.error = true;
        }

        if self.is_extra_spill_slot(slot as u32) {
          let extra_offset = self.get_extra_spill_address_offset(slot as u32);

          // 棘手情形：寄存器全空，但仍需一个寄存器来算地址。
          // 尝试占用 x17，除非它正是被 spill 的寄存器。
          let emergency_temp = if def.reg_a64 == X17 || def.reg_a64 == W17 {
            X16
          } else {
            X17
          };

          let build = self.build_mut();
          build.str(emergency_temp, s_temporary());

          build.ldr(
            emergency_temp,
            mem(R_STATE, offset_of!(LuaState, global) as i32),
          );
          build.ldr(
            emergency_temp,
            mem(emergency_temp, offset_of!(global_State, ecbdata) as i32),
          );

          build.str(def.reg_a64, mem(emergency_temp, extra_offset));

          build.ldr(emergency_temp, s_temporary());
        } else {
          self
            .build_mut()
            .str(def.reg_a64, mem(SP, S_SPILL_AREA_DATA + slot * 8));
        }

        self.spills.push(Spill {
          inst: target_inst_idx,
          origin: def.reg_a64,
          slot: slot as i8,
        });

        self.function_mut().instructions[target_inst_idx as usize].spilled = true;

        if let Some(stats) = self.stats_mut() {
          stats.spills_to_slot += 1;

          if slot != K_INVALID_SPILL as i32 && (slot + 1) as u32 > stats.max_spill_slots_used {
            stats.max_spill_slots_used = (slot + 1) as u32;
          }
        }
      }
    }

    self.function_mut().instructions[target_inst_idx as usize].reg_a64 = RegisterA64::NOREG;

    let set = self.get_set(kind);
    set.free |= 1u32 << reg;
    set.defs[reg as usize] = K_INVALID_INST_IDX;
  }

  pub fn take_reg(&mut self, reg: RegisterA64, index: u32) -> RegisterA64 {
    let set = self.get_set(reg.kind());

    CODEGEN_ASSERT!((set.free & (1u32 << reg.index())) != 0);
    CODEGEN_ASSERT!(set.defs[reg.index() as usize] == K_INVALID_INST_IDX);

    set.free &= !(1u32 << reg.index());
    set.defs[reg.index() as usize] = index;

    reg
  }
}

const K_NO_SPILL_SLOT: i8 = -1;

const XZR: RegisterA64 = reg(KindA64::X, 31);

/// 溢出栈帧布局与哨兵值（cpp `IrRegAllocA64::spill` 系列共用）
pub(crate) const K_INVALID_SPILL: i8 = 64;

// 帧内偏移基址：单一常量源 records::emit_common_a_64（cpp EmitCommonA64.h:48-49 sTemporary/sSpillArea）
pub(crate) const S_TEMPORARY_DATA: i32 = emit_common_a_64::S_TEMPORARY as i32;

pub(crate) const S_SPILL_AREA_DATA: i32 = emit_common_a_64::S_SPILL_AREA as i32;

const WZR: RegisterA64 = reg(KindA64::W, 31);

const W17: RegisterA64 = reg(KindA64::W, 17);

const X16: RegisterA64 = reg(KindA64::X, 16);

pub(crate) const X17: RegisterA64 = reg(KindA64::X, 17);

pub(crate) const SP: RegisterA64 = reg(KindA64::None, 31);

pub(crate) const R_STATE: RegisterA64 = reg(KindA64::X, 19);

const R_BASE: RegisterA64 = reg(KindA64::X, 25);

pub(crate) fn s_temporary() -> AddressA64 {
  mem(SP, S_TEMPORARY_DATA)
}
