use alloc::vec::Vec;
use core::array::from_fn;

use ulua_common::{fflag::LuauCodegenDseRestoreHints, records::dense_hash_set::DenseHashSet};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::{
    add_use::add_use, any_argument_match::any_argument_match,
    get_cmd_value_kind::get_cmd_value_kind, is_gco::is_gco, is_pseudo::is_pseudo,
    kill_ir_utils::kill_ir_function_ir_inst_at, reg_bitset::reg_bit_test, remove_use::remove_use,
    visit_arguments::visit_arguments, vm_exit_op::vm_exit_op, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_function::IrFunction, ir_op::IrOp, store_location_hint::StoreLocationHint,
    store_reg_info::StoreRegInfo, vm_exit_store_info::VmExitStoreInfo,
    vm_exit_store_record::VmExitStoreRecord,
  },
  traits::tag_access::TagAccess,
};

/// DSE（消除死存储）过程的寄存器状态。
///
/// 纯数据记录：只持有寄存器槽位快照、剩余使用计数与传播标记，不再自指
/// `IrFunction`（原 `*mut IrFunction` 字段与 `function_ref`/`function_mut` 一组
/// 裸指针 chokepoint 已整体拆除）。函数视图改由各方法以独立 `&mut IrFunction`
/// 参数即时传入，块/指令一律 `(block_idx, index)` 索引化定位，借用切分由参数
/// 天然保证（对齐 const-prop 波 `cur_ref/cur_mut` 收口后的模式）。
///
/// `remaining_uses` 借用区间与 state 完全重合（链内独占），故用
/// `&'a mut Vec<u32>` 生命周期引用表达，天然安全、无 chokepoint。
#[derive(Debug)]
pub struct RemoveDeadStoreState<'a> {
  remaining_uses: &'a mut Vec<u32>,
  pub(crate) info: [StoreRegInfo; 256],
  pub(crate) max_reg: i32,
  pub(crate) has_gco_to_clear: bool,
  pub(crate) has_allocations: bool,
  pub(crate) non_propagating_store: DenseHashSet<u32>,
  pub(crate) recorded_vm_exit_syncs: Vec<u32>,
}

impl<'a> RemoveDeadStoreState<'a> {
  pub fn remove_dead_store_state_remove_dead_store_state(remaining_uses: &'a mut Vec<u32>) -> Self {
    Self {
      remaining_uses,
      info: from_fn(|_| StoreRegInfo::default()),
      max_reg: 255,
      has_gco_to_clear: false,
      has_allocations: false,
      non_propagating_store: DenseHashSet::new(!0u32),
      recorded_vm_exit_syncs: Vec::new(),
    }
  }

  /// 剩余使用计数只读视图：生命周期引用字段，安全派生。
  #[inline]
  pub(crate) fn remaining_uses_ref(&self) -> &[u32] {
    &self.remaining_uses[..]
  }

  /// 剩余使用计数独占可变借用：生命周期引用字段，安全派生。
  #[inline]
  pub(crate) fn remaining_uses_mut(&mut self) -> &mut [u32] {
    &mut self.remaining_uses[..]
  }

  pub fn capture(&mut self, _reg: i32) {
    // 空实现与 C++ 源一致：void capture(int reg) {}
  }

  // 检查控制流时使用，例如退出到 fallback block。
  // 函数视图为独立参数：读写函数与读写 state 字段的借用天然分对象，
  // 原「自指裸指针 + 即取即释派生借用」的访问器全部拆除。
  pub fn check_live_ins(
    &mut self,
    function: &mut IrFunction,
    op: IrOp,
    inst_idx: u32,
    record_vm_exit_sync: bool,
  ) {
    if op.kind() == IrOpKind::VmExit {
      if record_vm_exit_sync && vm_exit_op(op) != K_VM_EXIT_ENTRY_GUARD_PC {
        // 建 sync 条目并登记 vm_exit；借用即取即释
        {
          let sync_info = function.vm_exit_info.get_or_insert(inst_idx);
          CODEGEN_ASSERT!(sync_info.reg_stores.is_empty());
          sync_info.vm_exit = op;
        }

        self.recorded_vm_exit_syncs.push(inst_idx);

        // 逆序以便先捕获词法上更近的 VM 寄存器
        let max_reg = self.max_reg;
        let mut i = max_reg;
        while i >= 0 {
          let (tag_idx, value_idx, tvalue_idx) = (
            self.info[i as usize].tag_inst_idx,
            self.info[i as usize].value_inst_idx,
            self.info[i as usize].tvalue_inst_idx,
          );
          let (ignore_at_exit, maybe_gco) = (
            self.info[i as usize].ignore_at_exit,
            self.info[i as usize].maybe_gco,
          );

          // 若值无法传播进 exit，store 必须保留为被 exit 使用
          if (tag_idx != !0u32 && self.non_propagating_store.contains(&tag_idx))
            || (value_idx != !0u32 && self.non_propagating_store.contains(&value_idx))
            || (tvalue_idx != !0u32 && self.non_propagating_store.contains(&tvalue_idx))
          {
            self.use_reg(i as u8);
            i -= 1;
            continue;
          }

          if ignore_at_exit && !maybe_gco {
            i -= 1;
            continue;
          }

          // sync 条目已由上方 get_or_insert 建立，只读判长
          let too_many_syncs = function
            .vm_exit_info
            .get(&inst_idx)
            .is_some_and(|sync_info| sync_info.reg_stores.len() >= 16);
          if too_many_syncs {
            self.use_reg(i as u8);
            i -= 1;
            continue;
          }

          let has_partial_overlap = (tag_idx != !0u32 || value_idx != !0u32) && tvalue_idx != !0u32;

          if has_partial_overlap {
            self.use_reg(i as u8);
            i -= 1;
            continue;
          }

          let mut store_info = VmExitStoreInfo {
            reg: i as u8,
            ..Default::default()
          };

          if tag_idx != !0u32 {
            CODEGEN_ASSERT!(tvalue_idx == !0u32);
            record_store(function, &mut store_info, tag_idx);
          }

          if value_idx != !0u32 {
            CODEGEN_ASSERT!(tvalue_idx == !0u32);
            record_store(function, &mut store_info, value_idx);
          }

          if tvalue_idx != !0u32 {
            CODEGEN_ASSERT!(tag_idx == !0u32 && value_idx == !0u32);

            // 只读快照 cmd 与 nil-tag 判定，借用即取即释
            let (store_cmd, nil_tag_store) = {
              let inst = &function.instructions[tvalue_idx as usize];
              let cmd = inst.cmd;
              (
                cmd,
                cmd == IrCmd::StoreTag && function.tag_op(inst.ops[1]) == LuaType::Nil as u8,
              )
            };
            CODEGEN_ASSERT!(
              matches!(
                store_cmd,
                IrCmd::StoreSplitTvalue | IrCmd::StoreTvalue | IrCmd::StoreVector
              ) || nil_tag_store
            );

            record_store(function, &mut store_info, tvalue_idx);
          }

          if !store_info.stores.empty() {
            function
              .vm_exit_info
              .get_or_insert(inst_idx)
              .reg_stores
              .push(store_info);
          }

          i -= 1;
        }
      } else {
        let max_reg = self.max_reg;
        for i in 0..=max_reg {
          let (ignore_at_exit, maybe_gco) = (
            self.info[i as usize].ignore_at_exit,
            self.info[i as usize].maybe_gco,
          );

          if ignore_at_exit && !maybe_gco {
            continue;
          }

          self.use_reg(i as u8);
        }

        self.has_gco_to_clear = false;
      }
    } else if op.kind() == IrOpKind::Block {
      let target = op.index() as usize;
      if target < function.cfg.r#in.len() {
        let max_reg = self.max_reg;
        for i in 0..=max_reg {
          // 借用即取即释：live-in 判定只读，use_reg 需 &mut self，两借用不可并持
          let is_in = {
            let in_ref = &function.cfg.r#in[target];
            reg_bit_test(&in_ref.regs, i as usize)
              || (in_ref.vararg_seq && i >= in_ref.vararg_start as i32)
          };

          if is_in {
            self.use_reg(i as u8);
          }
        }
      } else {
        self.read_all_regs();
      }
    } else if op.kind() == IrOpKind::Undef {
      // debug abort 无需处理
    } else {
      CODEGEN_ASSERT!(false); // unexpected jump target type
    }
  }

  // 检查 block 终止符时，凡非 live out 的寄存器都可按「有新值被定义」移除。
  // 目标块以索引传入：函数视图为独立参数，与 info 槽位借用天然互斥，
  // 原「函数指针 + info 裸指针」拆分借用 chokepoint 已拆除。
  pub fn check_live_outs(&mut self, function: &mut IrFunction, block_idx: u32) {
    if (block_idx as usize) >= function.cfg.out.len() {
      return;
    }

    // 循环不变谓词：live-out/captured 位图（Copy 字段）一次性快照，
    // 随后 kill 走函数可变借用，与只读快照不再交叠
    let out = &function.cfg.out[block_idx as usize];
    let (out_regs, out_vararg_seq, out_vararg_start) =
      (out.regs, out.vararg_seq, out.vararg_start as i32);
    let captured_regs = function.cfg.captured.regs;

    let max_reg = self.max_reg;
    let info = &mut self.info;

    for i in 0..=max_reg {
      let is_out = reg_bit_test(&out_regs, i as usize) || (out_vararg_seq && i >= out_vararg_start);

      // captured 寄存器的 store 不移除，因为不追踪其函数外 use
      if !is_out && !reg_bit_test(&captured_regs, i as usize) {
        let reg_info = &mut info[i as usize];
        kill_tag_and_value_store_pair_core(function, reg_info);
        kill_t_value_store_core(function, reg_info);
      }
    }
  }

  pub fn def(&mut self, function: &mut IrFunction, op: IrOp, offset: i32) {
    let reg = vm_reg_op(op) + offset;
    self.def_reg(function, reg as u8);
  }

  pub fn def_range(&mut self, function: &mut IrFunction, start: i32, count: i32) {
    if count == -1 {
      self.def_varargs(function, start as u8);
    } else {
      for i in start..(start + count) {
        self.def_reg(function, i as u8);
      }
    }
  }

  // 寄存器值被定义时，杀掉之前的 store。
  // 函数视图为独立参数：kill core 借函数可变视图、info 槽位借 state 字段，天然互斥。
  pub fn def_reg(&mut self, function: &mut IrFunction, reg: u8) {
    // captured 寄存器的 store 不移除，因为不追踪其函数外 use
    if reg_bit_test(&function.cfg.captured.regs, reg as usize) {
      return;
    }

    let reg_info = &mut self.info[reg as usize];
    kill_tag_and_value_store_pair_core(function, reg_info);
    kill_t_value_store_core(function, reg_info);

    reg_info.tag_inst_idx = !0u32;
    reg_info.value_inst_idx = !0u32;
    reg_info.tvalue_inst_idx = !0u32;

    // 不透明的寄存器定义消除了对实际 tag 值的已知性
    reg_info.known_tag = 0xff;

    // 定义了新值；在再次 MARK_DEAD 前，它可能被 VM exit 使用
    reg_info.ignore_at_exit = false;
  }

  pub fn def_varargs(&mut self, function: &mut IrFunction, vararg_start: u8) {
    let max_reg = 255;
    for i in vararg_start..=max_reg {
      self.def_reg(function, i);
    }
  }

  // 部分清除可能持有 GC 对象的寄存器信息。函数视图为独立参数，
  // 借用交叠由「先快照 Copy 字段」规避，无裸指针回转。
  pub fn flush_gco_regs(&mut self, function: &IrFunction) {
    let max_reg = self.max_reg;

    for i in 0..=max_reg {
      let idx = i as usize;
      if !self.info[idx].maybe_gco {
        continue;
      }

      // Copy 字段快照：后续 has_remaining_uses / invalidate 需 &mut self，不得持有 info 借用
      let (tag_inst_idx, value_inst_idx, tvalue_inst_idx, known_tag) = (
        self.info[idx].tag_inst_idx,
        self.info[idx].value_inst_idx,
        self.info[idx].tvalue_inst_idx,
        self.info[idx].known_tag,
      );

      // 若恰知确切 tag，它必为 GCO，否则 'maybeGCO' 应为 false
      // （不变量由 StorePointer 等写点维持：已知非 GCO tag 时 maybe_gco 必须保持 false）
      CODEGEN_ASSERT!(known_tag == 0xff || is_gco(known_tag));

      // 若存的值仍被使用且可能是 GCO 对象，必须将其钉在栈上
      let tag_used_after = tag_inst_idx != !0u32 && self.has_remaining_uses(function, tag_inst_idx);
      let value_used_after =
        value_inst_idx != !0u32 && self.has_remaining_uses(function, value_inst_idx);
      let tvalue_used_after =
        tvalue_inst_idx != !0u32 && self.has_remaining_uses(function, tvalue_inst_idx);

      if tag_used_after || value_used_after || tvalue_used_after {
        self.info[idx].tag_inst_idx = !0u32;
        self.info[idx].value_inst_idx = !0u32;
        self.info[idx].tvalue_inst_idx = !0u32;
      }

      // 若 GCO 值仍在，不能再向更后传播，那会产生新 use
      self.invalidate_value_propagation_store_reg_info(function, idx);

      // GC 的间接寄存器读不清除已知 tag
      self.info[idx].maybe_gco = false;
    }

    self.has_gco_to_clear = false;
  }

  /// 纯读判定：函数经独立只读参数、计数为 `&self` 视图访问器。
  pub fn has_remaining_uses(&self, function: &IrFunction, inst_idx: u32) -> bool {
    let uses = self.remaining_uses_ref();
    function
      .instructions
      .get(inst_idx as usize)
      .is_some_and(|inst| {
        any_argument_match(inst, |op| {
          op.kind() == IrOpKind::Inst && uses[op.index() as usize] != 0
        })
      })
  }

  // 将 pending store 标为不可传播，防止其 use 被移进 VM exit block。
  /// 索引化变体：自行按槽位号读 info；函数视图为独立只读参数，
  /// 消除「函数自指裸指针 + info 再借用」的散点 unsafe。
  pub fn invalidate_value_propagation_store_reg_info(
    &mut self,
    function: &IrFunction,
    reg_idx: usize,
  ) {
    let (tag_inst_idx, value_inst_idx, tvalue_inst_idx) = (
      self.info[reg_idx].tag_inst_idx,
      self.info[reg_idx].value_inst_idx,
      self.info[reg_idx].tvalue_inst_idx,
    );

    if tag_inst_idx != !0u32 && inst_has_inst_arg(function, tag_inst_idx) {
      self.non_propagating_store.insert(tag_inst_idx);
    }

    if value_inst_idx != !0u32 && inst_has_inst_arg(function, value_inst_idx) {
      self.non_propagating_store.insert(value_inst_idx);
    }

    if tvalue_inst_idx != !0u32 && inst_has_inst_arg(function, tvalue_inst_idx) {
      self.non_propagating_store.insert(tvalue_inst_idx);
    }
  }

  pub fn invalidate_value_propagation(&mut self, function: &IrFunction) {
    let max_reg = self.max_reg;
    for i in 0..=max_reg {
      self.invalidate_value_propagation_store_reg_info(function, i as usize);
    }
  }

  pub fn mark_unused_at_exit(&mut self, function: &IrFunction, start: i32, count: i32) {
    CODEGEN_ASSERT!(count != 0);

    let e = if count == -1 {
      self.max_reg
    } else {
      start + count - 1
    };

    // 循环不变谓词：captured 位图（[u64; 4]，Copy）一次性快照（函数为独立只读参数）
    let captured_regs = function.cfg.captured.regs;

    for i in start..=e {
      // captured 寄存器的 store 不移除，因为不追踪其函数外 use
      if !reg_bit_test(&captured_regs, i as usize) {
        self.info[i as usize].ignore_at_exit = true;
      }
    }
  }

  pub fn maybe_def(&mut self, function: &mut IrFunction, op: IrOp) {
    if op.kind() == IrOpKind::VmReg {
      let reg = vm_reg_op(op);
      self.def_reg(function, reg as u8);
    }
  }

  pub fn maybe_use(&mut self, op: IrOp) {
    if op.kind() == IrOpKind::VmReg {
      let reg = vm_reg_op(op);
      self.use_reg(reg as u8);
    }
  }

  // VmExit 信息含有一些数据，store 被作为无用移除时需要同步。
  // 函数视图为独立参数；同步点索引列表只读遍历，与函数借用互不交叠。
  pub fn prune_vm_exit_info(&mut self, function: &mut IrFunction) {
    for &inst_idx in &self.recorded_vm_exit_syncs {
      prune_vm_exit_sync(function, inst_idx);
    }
  }

  pub fn read_all_regs(&mut self) {
    for i in 0..=self.max_reg {
      self.use_reg(i as u8);
    }

    self.has_gco_to_clear = false;
  }

  pub fn use_(&mut self, op: IrOp, offset: i32) {
    let reg = vm_reg_op(op) + offset;
    self.use_reg(reg as u8);
  }

  pub fn use_range(&mut self, start: i32, count: i32) {
    if count == -1 {
      self.use_varargs(start as u8);
    } else {
      for i in start..(start + count) {
        self.use_reg(i as u8);
      }
    }
  }

  pub fn use_reg(&mut self, reg: u8) {
    let reg_info: &mut StoreRegInfo = &mut self.info[reg as usize];

    // 读寄存器不清除已知 tag
    reg_info.tag_inst_idx = !0u32;
    reg_info.value_inst_idx = !0u32;
    reg_info.tvalue_inst_idx = !0u32;
    reg_info.maybe_gco = false;
  }

  pub fn use_varargs(&mut self, vararg_start: u8) {
    let max_reg: u8 = 255;
    for i in vararg_start..=max_reg {
      self.use_reg(i);
    }
  }
}

impl<'a> TagAccess for RemoveDeadStoreState<'a> {
  /// cpp OptimizeDeadStore.cpp：get/set 直接读写 `state.info[i].knownTag`
  fn get_tag(&self, i: usize) -> u8 {
    self.info[i].known_tag
  }

  fn set_tag(&mut self, i: usize, tag: u8) {
    self.info[i].known_tag = tag;
  }
}

// handler.pcpos 的哨兵值，表示 entry guard（IrData.h）
const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

// IrVisitUseDef 风格 helper：把 store 捕获进 exit sync 记录并为其操作数添加 use
/// 备份用 clone 兼作遍历载体：visit_arguments 需要指令可变借用，add_use 同时可变借用函数，
/// 两者对同一函数重叠不可行——改在克隆上遍历。add_use 只按值读取操作数、不回写，
/// 且 store 指令的操作数不会引用自身，遍历副本与原版逐位一致。
fn record_store(function: &mut IrFunction, store_info: &mut VmExitStoreInfo, inst_idx: u32) {
  let mut backup = function.instructions[inst_idx as usize].clone();
  visit_arguments(&mut backup, |op| add_use(function, op));
  store_info
    .stores
    .push_back(VmExitStoreRecord { inst_idx, backup });
}

/// 指令是否存在 Inst 类操作数：只读谓词，独立于 state 借用。
fn inst_has_inst_arg(function: &IrFunction, inst_idx: u32) -> bool {
  any_argument_match(&function.instructions[inst_idx as usize], |op| {
    op.kind() == IrOpKind::Inst
  })
}

/// TValue 存储 kill 核心：只借函数视图，供拆分借用上下文（def_reg / check_live_outs）复用。
/// `kill_ir_function_ir_inst_at` 为索引化变体，替代原“函数借用 + 指令裸指针”双借用写法。
pub(crate) fn kill_t_value_store_core(function: &mut IrFunction, reg_info: &mut StoreRegInfo) {
  // 仅当 TValue 未被部分 tag/value 写覆盖时才能 kill
  if reg_info.tvalue_inst_idx == !0u32
    || reg_info.tag_inst_idx != !0u32
    || reg_info.value_inst_idx != !0u32
  {
    return;
  }

  if LuauCodegenDseRestoreHints.get() {
    record_hint_before_kill_core(function, reg_info.tvalue_inst_idx);
  }

  kill_ir_function_ir_inst_at(function, reg_info.tvalue_inst_idx);

  reg_info.tvalue_inst_idx = !0u32;
  reg_info.maybe_gco = false;
}

/// tag/value 配对存储 kill 核心：只借函数视图，供拆分借用上下文复用。
/// 顺序与原版一致：先 kill tag（无提示），再 kill value（带恢复提示）。
pub(crate) fn kill_tag_and_value_store_pair_core(
  function: &mut IrFunction,
  reg_info: &mut StoreRegInfo,
) {
  // 部分 store 只有在整对建立后才可移除
  if !tag_value_pair_established(reg_info) {
    return;
  }

  if reg_info.tag_inst_idx != !0u32 {
    kill_ir_function_ir_inst_at(function, reg_info.tag_inst_idx);
    reg_info.tag_inst_idx = !0u32;
  }

  if reg_info.value_inst_idx != !0u32 {
    if LuauCodegenDseRestoreHints.get() {
      record_hint_before_kill_core(function, reg_info.value_inst_idx);
    }

    kill_ir_function_ir_inst_at(function, reg_info.value_inst_idx);
    reg_info.value_inst_idx = !0u32;
  }

  reg_info.maybe_gco = false;
}

/// Value 存储 kill 核心：只借函数视图，供拆分借用上下文复用。
/// `kill_ir_function_ir_inst_at` 为索引化变体，替代原“函数借用 + 指令裸指针”双借用写法。
pub(crate) fn kill_value_store_core(function: &mut IrFunction, reg_info: &mut StoreRegInfo) {
  if reg_info.value_inst_idx == !0u32 {
    return;
  }

  if LuauCodegenDseRestoreHints.get() {
    record_hint_before_kill_core(function, reg_info.value_inst_idx);
  }

  kill_ir_function_ir_inst_at(function, reg_info.value_inst_idx);

  reg_info.value_inst_idx = !0u32;
  reg_info.maybe_gco = false;
}

/// 单个 vm exit 同步点的修剪：全部经由 &mut IrFunction 顺序短借用消除散点 unsafe。
/// 原版 `visit_arguments(指令可变借用, |op| remove_use(函数可变借用, op))` 为同对象重叠
/// 双借用，改为操作数快照遍历：remove_use 只改被引用指令的使用计数（归零即 kill 清空
/// 其 ops），store 指令的操作数不可能引用自身，被访问指令的 ops 在遍历期间不被触动，
/// 快照与原版逐位一致。
fn prune_vm_exit_sync(function: &mut IrFunction, inst_idx: u32) {
  let mut i = 0usize;
  while i
    < function
      .vm_exit_info
      .get_or_insert(inst_idx)
      .reg_stores
      .len()
  {
    let mut j = 0usize;
    while j
      < function.vm_exit_info.get_or_insert(inst_idx).reg_stores[i]
        .stores
        .size() as usize
    {
      let store_inst_idx = function.vm_exit_info.get_or_insert(inst_idx).reg_stores[i]
        .stores
        .as_slice()[j]
        .inst_idx;

      let alive = function.instructions[store_inst_idx as usize].cmd != IrCmd::NOP;
      if alive {
        // 操作数快照（Copy 向量）：遍历期函数已可自由再借用
        let store_cmd = function.instructions[store_inst_idx as usize].cmd;
        if !is_pseudo(store_cmd) {
          let ops = function.instructions[store_inst_idx as usize].ops.clone();
          for op in ops.as_slice() {
            remove_use(function, *op);
          }
        }

        let sync_info = function.vm_exit_info.get_or_insert(inst_idx);
        let stores = &mut sync_info.reg_stores[i].stores;
        let back = stores.back().clone();
        stores.as_mut_slice()[j] = back;
        stores.pop_back();
      } else {
        j += 1;
      }
    }

    let sync_info = function.vm_exit_info.get_or_insert(inst_idx);
    if sync_info.reg_stores[i].stores.empty() {
      let back = sync_info
        .reg_stores
        .last()
        .expect("reg_stores 非空：外层循环条件 i < reg_stores.len() 保证 last 必存在")
        .clone();
      sync_info.reg_stores[i] = back;
      sync_info.reg_stores.pop();
    } else {
      i += 1;
    }
  }
}

/// kill 前恢复位置提示的核心实现：只借函数视图，供 kill core 在拆分借用上下文复用。
pub(crate) fn record_hint_before_kill_core(function: &mut IrFunction, store_inst_idx: u32) {
  let store_cmd = function.instructions[store_inst_idx as usize].cmd;

  let dest: IrOp = function.instructions[store_inst_idx as usize].ops[0];

  if dest.kind() != IrOpKind::VmReg {
    return;
  }

  let value: IrOp;
  let mut kind = IrValueKind::Unknown;

  match store_cmd {
    IrCmd::StoreDouble => {
      value = function.instructions[store_inst_idx as usize].ops[1];
      kind = IrValueKind::Double;
    }
    IrCmd::StoreInt => {
      value = function.instructions[store_inst_idx as usize].ops[1];
      kind = IrValueKind::Int;
    }
    IrCmd::StoreInt64 => {
      value = function.instructions[store_inst_idx as usize].ops[1];
      kind = IrValueKind::Int64;
    }
    IrCmd::StorePointer => {
      value = function.instructions[store_inst_idx as usize].ops[1];
      kind = IrValueKind::Pointer;
    }
    IrCmd::StoreTvalue => {
      value = function.instructions[store_inst_idx as usize].ops[1];
      kind = IrValueKind::Tvalue;
    }
    IrCmd::StoreSplitTvalue => {
      value = function.instructions[store_inst_idx as usize].ops[2];

      if value.kind() == IrOpKind::Inst {
        let inst_cmd = function.inst_op(value).cmd;
        kind = get_cmd_value_kind(inst_cmd);
      }

      if kind == IrValueKind::Unknown {
        return;
      }
    }
    IrCmd::StoreVector => {
      // 多组件值，不适合做单值恢复 hint
      return;
    }
    _ => return,
  }

  if value.kind() != IrOpKind::Inst {
    return;
  }

  function.record_store_location_hint(
    store_inst_idx,
    StoreLocationHint {
      op: dest,
      inst_idx: value.index(),
      kind,
    },
  );
}

/// tag/value 配对已建立判定：纯字段谓词，独立于 state 借用，供 kill core 复用。
pub(crate) fn tag_value_pair_established(reg_info: &StoreRegInfo) -> bool {
  let tag_established = reg_info.tag_inst_idx != !0u32 || reg_info.known_tag != 0xff;
  let value_established =
    reg_info.value_inst_idx != !0u32 || reg_info.known_tag == LuaType::Nil as u8;
  tag_established && value_established
}
