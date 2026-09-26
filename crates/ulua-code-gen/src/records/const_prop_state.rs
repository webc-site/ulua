use alloc::vec::Vec;
use core::mem::size_of;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};
use ulua_vm::{enums::lua_type::LuaType, records::lua_t_value::TValue};

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::{
    get_cmd_value_kind::get_cmd_value_kind,
    get_const_value_kind::get_const_value_kind,
    has_side_effects::has_side_effects,
    is_gco::is_gco,
    kill_ir_utils::kill_ir_function_ir_inst_at,
    produces_dirty_high_register_bits::produces_dirty_high_register_bits,
    reg_bitset::reg_bit_test,
    replace_ir_utils::{replace_ir_function_ir_block_u32_ir_inst, replace_ir_function_ir_op_ir_op},
    substitute::substitute,
    vm_reg_op::vm_reg_op,
    vm_upvalue_op::vm_upvalue_op,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    ir_operand::{op_a_ref, op_c_ref, op_d_ref, op_e_ref},
    op_a::op_a, op_b_ref::op_b_ref,
  },
  records::{
    array_value_entry::ArrayValueEntry,
    buffer_access_base::BufferAccessBase,
    buffer_load_store_info::BufferLoadStoreInfo,
    ir_builder::IrBuilder,
    ir_data::{K_INVALID_INST_IDX, K_INVALID_UPVALUE, K_UNKNOWN_TAG},
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_inst_eq::IrInstEq,
    ir_inst_hash::IrInstHash,
    ir_op::IrOp,
    node_slot_state::NodeSlotState,
    numbered_instruction::NumberedInstruction,
    register_info::RegisterInfo,
    register_link::RegisterLink,
  },
  traits::tag_access::TagAccess,
  type_aliases::ir_ops::IrOps,
};

#[derive(Debug)]
pub struct ConstPropState {
  /// cpp `ConstPropState` 持有 `IrBuilder& build` 引用；Rust 侧保留最小裸指针门面，
  /// `function` 视图一律经 `build.function` 派生（与 cpp 同源同对象），不再有第二指针。
  pub build: *mut IrBuilder,
  pub regs: [RegisterInfo; 256],
  pub max_reg: i32,
  pub inst_pos: u32,
  pub in_safe_env: bool,
  pub checked_gc: bool,
  pub inst_link: DenseHashMap<u32, RegisterLink>,
  pub inst_tag: DenseHashMap<u32, u8>,
  pub inst_value: DenseHashMap<u32, IrOp>,
  pub value_map: DenseHashMap<IrInst, u32, IrInstHash, IrInstEq>,
  pub upvalue_map: DenseHashMap<u8, u32>,
  pub hash_value_cache: DenseHashMap<u32, u32>,
  pub array_value_cache: Vec<ArrayValueEntry>,
  pub try_num_to_index_cache: Vec<u32>,
  pub get_slot_node_cache: Vec<NumberedInstruction>,
  pub check_slot_match_cache: Vec<NodeSlotState>,
  pub get_arr_addr_cache: Vec<u32>,
  pub check_array_size_cache: Vec<u32>,
  pub check_buffer_len_cache: Vec<u32>,
  pub useradata_tag_cache: Vec<u32>,
  pub buffer_load_store_info: Vec<BufferLoadStoreInfo>,
  pub load_env_idx: u32,
  pub inst_not_readonly: DenseHashSet<u32>,
  pub inst_no_metatable: DenseHashSet<u32>,
  pub inst_array_size: DenseHashMap<u32, i32>,
  pub range_end_temp: Vec<u32>,
}

impl ConstPropState {
  /// Safety: `build` 在 `const_prop_in_block_chains` 构造时以 `&mut` 传入后存为裸指针，
  /// 所指 `IrBuilder`（含内联 `function` 字段）生命周期覆盖整个常量传播过程。访问器即时派生
  /// 借用、语句内消费，调用侧不再各写 unsafe 解引用。
  #[inline]
  fn build_ptr(&self) -> *mut IrBuilder {
    self.build
  }

  #[inline]
  pub(crate) fn function_ref(&self) -> &IrFunction {
    // Safety: 见函数注释；`(*build).function` 即构造时传入的 `build.function`，仅派生共享借用。
    unsafe { &(*self.build_ptr()).function }
  }

  /// Safety: 见 `function_ref`；派生唯一可变借用，调用方须保证当前无其他借用。
  #[inline]
  pub(crate) fn function_mut(&mut self) -> &mut IrFunction {
    // Safety: 见函数注释。
    unsafe { &mut (*self.build_ptr()).function }
  }

  /// Safety: 同 `function_ref`；派生唯一可变借用。
  #[inline]
  pub(crate) fn build_mut(&mut self) -> &mut IrBuilder {
    // Safety: 见函数注释。
    unsafe { &mut *self.build }
  }

  pub fn clear(&mut self) {
    self.regs[..=self.max_reg as usize].fill(RegisterInfo::default());
    self.max_reg = 0;
    self.inst_pos = 0;
    self.in_safe_env = false;
    self.checked_gc = false;
    self.inst_link.clear();
    self.inst_tag.clear();
    self.inst_value.clear();
    self.value_map.clear();
    self.upvalue_map.clear();
    self.hash_value_cache.clear();
    self.array_value_cache.clear();
    self.try_num_to_index_cache.clear();
    self.get_slot_node_cache.clear();
    self.check_slot_match_cache.clear();
    self.get_arr_addr_cache.clear();
    self.check_array_size_cache.clear();
    self.check_buffer_len_cache.clear();
    self.useradata_tag_cache.clear();
    self.buffer_load_store_info.clear();
    self.load_env_idx = K_INVALID_INST_IDX;
    self.inst_not_readonly.clear();
    self.inst_no_metatable.clear();
    self.inst_array_size.clear();
    self.range_end_temp.clear();
  }

  pub fn const_prop_state_const_prop_state(build: &mut IrBuilder) -> Self {
    Self {
      build: build as *mut IrBuilder,
      regs: [RegisterInfo::default(); 256],
      max_reg: 0,
      inst_pos: 0,
      in_safe_env: false,
      checked_gc: false,
      inst_link: DenseHashMap::new(K_INVALID_INST_IDX),
      inst_tag: DenseHashMap::new(K_INVALID_INST_IDX),
      inst_value: DenseHashMap::new(K_INVALID_INST_IDX),
      value_map: DenseHashMap::new(IrInst::default()),
      upvalue_map: DenseHashMap::new(K_INVALID_UPVALUE),
      hash_value_cache: DenseHashMap::new(K_INVALID_INST_IDX),
      array_value_cache: Vec::new(),
      try_num_to_index_cache: Vec::new(),
      get_slot_node_cache: Vec::new(),
      check_slot_match_cache: Vec::new(),
      get_arr_addr_cache: Vec::new(),
      check_array_size_cache: Vec::new(),
      check_buffer_len_cache: Vec::new(),
      useradata_tag_cache: Vec::new(),
      buffer_load_store_info: Vec::new(),
      load_env_idx: K_INVALID_INST_IDX,
      inst_not_readonly: DenseHashSet::new(K_INVALID_INST_IDX),
      inst_no_metatable: DenseHashSet::new(K_INVALID_INST_IDX),
      inst_array_size: DenseHashMap::new(K_INVALID_INST_IDX),
      range_end_temp: Vec::new(),
    }
  }

  pub fn create_reg_link(&mut self, inst_idx: u32, reg_op: IrOp) {
    CODEGEN_ASSERT!(!self.inst_link.contains(&inst_idx));
    let reg = vm_reg_op(reg_op) as u8;
    let version = self.regs[reg as usize].version;
    self
      .inst_link
      .try_insert(inst_idx, RegisterLink { reg, version });
  }

  pub fn find_substitute_component_load_from_store_vector(
    &mut self,
    vm_reg: IrOp,
    offset: i32,
  ) -> Option<IrOp> {
    let versioned_load = self.versioned_vm_reg_load_ir_cmd_ir_op(IrCmd::LoadFloat, vm_reg);

    if let Some(prev_idx) = self.get_previous_inst_index(&versioned_load) {
      let store = &self.function_ref().instructions[prev_idx as usize];

      CODEGEN_ASSERT!(store.cmd == IrCmd::StoreVector);

      let arg_op = if offset == 0 {
        op_b_ref(store)
      } else if offset == 4 {
        op_c_ref(store)
      } else if offset == 8 {
        op_d_ref(store)
      } else {
        return None;
      };

      if let Some(arg_cmd) = self
        .function_ref()
        .as_inst_op_ref(arg_op)
        .map(|arg| arg.cmd)
      {
        if matches!(
          arg_cmd,
          IrCmd::LoadFloat | IrCmd::BufferReadf32 | IrCmd::NumToFloat | IrCmd::UintToFloat
        ) {
          return Some(arg_op);
        }
      } else if arg_op.kind() == IrOpKind::Constant {
        let double_val = self.function_ref().double_op(arg_op);
        return Some(self.build_mut().const_double(double_val));
      }
    }

    None
  }

  pub fn forward_buffer_store_to_load(
    &mut self,
    store_inst: &mut IrInst,
    load_cmd: IrCmd,
    access_size: u8,
  ) {
    let tag = self.function_ref().tag_op(op_d_ref(store_inst));

    // 以未知偏移写入会清除同类内存（buffer/userdata）中的所有值
    // userdata 本可追溯指针来源，但尚无这样的用例
    if op_b_ref(store_inst).kind() != IrOpKind::Constant {
      let mut i = 0;
      while i < self.buffer_load_store_info.len() {
        let info = self.buffer_load_store_info[i];
        if info.tag == tag {
          // 用尾项覆盖第 i 项并弹出（cpp 原语），即 swap_remove，序不必保持
          self.buffer_load_store_info.swap_remove(i);
        } else {
          i += 1;
        }
      }

      return;
    }

    let offset = self.function_ref().int_op(op_b_ref(store_inst));

    // 常量偏移的写会失效所有对象中的该 range，除非已知指针互不相干
    let mut i = 0;
    while i < self.buffer_load_store_info.len() {
      let intersecting_range = offset + i32::from(access_size)
        > self.buffer_load_store_info[i].offset
        && offset
          < self.buffer_load_store_info[i].offset
            + i32::from(self.buffer_load_store_info[i].access_size);

      if intersecting_range && self.buffer_load_store_info[i].tag == tag {
        // 先快照两条指针指令的判定字段（cmd 与下标），再统一判定，
        // 避免跨 buffer_load_store_info 借用持有 function 的可变借用
        let addr_op = self.buffer_load_store_info[i].address;
        let (curr_is_userdata, info_is_userdata) = {
          let function = self.function_mut();
          (
            function.inst_op(op_a(store_inst)).cmd == IrCmd::NewUserdata,
            function.inst_op(addr_op).cmd == IrCmd::NewUserdata,
          )
        };

        // 来自不同分配的指针不可能相同
        if curr_is_userdata && info_is_userdata && op_a(store_inst) != addr_op {
          i += 1;
          continue;
        }

        // 用尾项覆盖第 i 项并弹出（cpp 原语），即 swap_remove，序不必保持
        self.buffer_load_store_info.swap_remove(i);
      } else {
        i += 1;
      }
    }

    let mut value = op_c_ref(store_inst);

    // 小类型 store 会截断数据
    // 动态值在 'substituteOrRecordBufferLoad' 中处理
    if op_c_ref(store_inst).kind() == IrOpKind::Constant {
      if load_cmd == IrCmd::BufferReadi8 {
        let v_i32 = self.function_ref().int_op(op_c_ref(store_inst));
        value = self.build_mut().const_int(v_i32 as i8 as i32);
      } else if load_cmd == IrCmd::BufferReadi16 {
        let v_i32 = self.function_ref().int_op(op_c_ref(store_inst));
        value = self.build_mut().const_int(v_i32 as i16 as i32);
      } else if load_cmd == IrCmd::BufferReadf32 {
        let v_f64 = self.function_ref().double_op(op_c_ref(store_inst));
        value = self.build_mut().const_double(v_f64 as f32 as f64);
      }
    }

    // 记录本次 store 值，供日后复用
    let info = BufferLoadStoreInfo {
      load_cmd,
      access_size,
      tag,
      from_store: true,
      address: op_a(store_inst),
      value,
      offset,
    };

    self.buffer_load_store_info.push(info);
  }

  /// 前向记录表节点存储：按目标地址指令的 cmd 把存储位置登记进对应缓存。
  /// `target_addr` 只读（`&IrInst`），与经 `function_ref` 的只读视图同源共存
  /// （shared + shared）。
  pub fn forward_table_store_to_load(
    &mut self,
    target_addr: &IrInst,
    write_offset_op: IrOp,
    inst_idx: u32,
  ) {
    if target_addr.cmd == IrCmd::GetSlotNodeAddr {
      CODEGEN_ASSERT!(self.function_ref().int_op(write_offset_op) == 0);

      let key = self.function_ref().get_inst_index(target_addr);
      *self.hash_value_cache.get_or_insert(key) = inst_idx;
    } else if target_addr.cmd == IrCmd::GetArrAddr {
      // 与 C++ 一致：复用 getCombinedArrayLoadOffsetOp，避免内联副本漏掉回绕语义
      let offset_op = self.get_combined_array_load_offset_op(target_addr, write_offset_op);

      let key = self.function_ref().get_inst_index(target_addr);
      self.array_value_cache.push(ArrayValueEntry {
        pointer: key,
        offset: offset_op,
        value: inst_idx,
      });
    } else {
      CODEGEN_ASSERT!(matches!(
        target_addr.cmd,
        IrCmd::TableSetnum | IrCmd::GetClosureUpvalAddr
      ));
    }
  }

  pub fn forward_vm_reg_store_to_load(&mut self, store_inst: &mut IrInst, load_cmd: IrCmd) {
    let store_reg = op_a(store_inst);
    let stored_value = op_b_ref(store_inst);

    CODEGEN_ASSERT!(store_reg.kind() == IrOpKind::VmReg);
    CODEGEN_ASSERT!(stored_value.kind() == IrOpKind::Inst);

    let reg = vm_reg_op(store_reg) as usize;
    // [u64; 4] 为 Copy，复制后与 self.regs 的访问无重叠
    let captured_regs = self.function_ref().cfg.captured.regs;
    if reg_bit_test(&captured_regs, reg) {
      return;
    }

    let mut versioned_reg = store_reg;
    versioned_reg = IrOp::ir_op_ir_op_kind_u32(
      IrOpKind::VmReg,
      (vm_reg_op(versioned_reg) as u32) | (self.regs[reg].version << 8),
    );
    let mut ops = IrOps::new();
    ops.push(versioned_reg);
    let key = IrInst {
      cmd: load_cmd,
      ops,
      ..IrInst::default()
    };
    *self.value_map.get_or_insert(key) = stored_value.index();
  }

  pub fn forward_vm_upvalue_store_to_load(&mut self, store_inst: &mut IrInst) {
    let upvalue_index = vm_upvalue_op(op_a(store_inst)) as u8;
    let value_index = op_b_ref(store_inst).index();
    *self.upvalue_map.get_or_insert(upvalue_index) = value_index;
  }

  pub fn get_combined_array_load_offset_op(
    &mut self,
    array_addr_inst: &IrInst,
    load_offset_op: IrOp,
  ) -> IrOp {
    CODEGEN_ASSERT!(array_addr_inst.cmd == IrCmd::GetArrAddr);

    let load_offset = self.function_ref().as_int_op(load_offset_op).unwrap_or(0);

    let op_b_inst = op_b_ref(array_addr_inst);
    if op_b_inst.kind() == IrOpKind::Constant {
      // cpp 的 int 乘法按回绕语义；fuzz 路径可构造超大常量偏移，debug 下溢出会 panic
      let array_addr_offset = self
        .function_ref()
        .int_op(op_b_inst)
        .wrapping_mul(size_of::<TValue>() as i32);

      if array_addr_offset != 0 || load_offset == 0 {
        CODEGEN_ASSERT!(load_offset == 0);
        return self.build_mut().const_int(array_addr_offset);
      }
    } else {
      CODEGEN_ASSERT!(op_b_inst.kind() == IrOpKind::Inst);
      CODEGEN_ASSERT!(load_offset == 0);
      return op_b_inst;
    }

    CODEGEN_ASSERT!(load_offset_op.kind() == IrOpKind::Constant);
    load_offset_op
  }

  // set 只读（cpp 传非 const 引用是历史遗留），可变状态仅限 self.range_end_temp
  pub fn get_max_internal_overlap(&mut self, set: &[NumberedInstruction], slot: usize) -> i32 {
    // 活跃期起点早于 slot 且至今未结束的槽位计入初始重叠数（cpp 同款预处理）
    let mut curr = 1
      + set[..slot]
        .iter()
        .filter(|entry| entry.finish_pos >= set[slot].start_pos)
        .count() as i32;

    let mut max = curr;

    // 收集生命周期结束点并排序
    self.range_end_temp.clear();

    for item in set.iter().skip(slot + 1) {
      self.range_end_temp.push(item.finish_pos);
    }

    self.range_end_temp.sort_unstable();

    // 遍历分开存放的 lifetime begin/end 数组，按最小值推进
    let mut i1 = slot + 1;
    let mut i2 = 0usize;

    while i1 < set.len() && i2 < self.range_end_temp.len() {
      if self.range_end_temp[i2] == set[i1].start_pos {
        i1 += 1;
        i2 += 1;
      } else if self.range_end_temp[i2] < set[i1].start_pos {
        CODEGEN_ASSERT!(curr > 0);
        curr -= 1;
        i2 += 1;
      } else {
        curr += 1;
        i1 += 1;

        if curr > max {
          max = curr;
        }
      }
    }

    // 可能残留未处理的 end 条目，但 start 条目必已全部处理
    // 注意 end 条目只会减小当前值，不影响最终结果（最大值）
    max
  }

  pub fn get_offset_base(&mut self, value: IrOp) -> BufferAccessBase {
    let mut base = BufferAccessBase {
      op: value,
      scale: 1,
      offset: 0,
    };

    loop {
      if base.op.kind() != IrOpKind::Inst {
        break;
      }

      // 只快照 cmd 与两个操作数（均 Copy），避免整条 IrInst clone；
      // 借用即取即释：函数只读访问器窗口内完成读取，后续常量读取另起窗口。
      let (cmd, lhs_op, rhs_op) = {
        let function = self.function_ref();
        let inst = &function.instructions[base.op.index() as usize];
        let size = inst.ops.size();
        let lhs = if size > 0 {
          inst.ops[0]
        } else {
          IrOp::default()
        };
        let rhs = if size > 1 {
          inst.ops[1]
        } else {
          IrOp::default()
        };
        (inst.cmd, lhs, rhs)
      };

      let (lhs_num, rhs_num, lhs_int, rhs_int) = {
        let function = self.function_ref();
        (
          function.as_double_op(lhs_op),
          function.as_double_op(rhs_op),
          function.as_int_op(lhs_op),
          function.as_int_op(rhs_op),
        )
      };

      // 逐 cmd 折叠寻址链：臂顺序即原 else-if 求值顺序（同 cmd 先 lhs 后 rhs），行为等价；
      // guard 内 `if let` 链直接绑定常量值，免 is_some_and+unwrap 双读。
      match cmd {
        IrCmd::AddNum
          if let Some(n) = lhs_num
            && self.is_valid_double_for_immediate(n) =>
        {
          base.offset += (n as i32) * base.scale;
          base.op = rhs_op;
        }
        IrCmd::AddNum
          if let Some(n) = rhs_num
            && self.is_valid_double_for_immediate(n) =>
        {
          base.offset += (n as i32) * base.scale;
          base.op = lhs_op;
        }
        IrCmd::SubNum
          if let Some(n) = rhs_num
            && self.is_valid_double_for_immediate(n) =>
        {
          base.offset -= (n as i32) * base.scale;
          base.op = lhs_op;
        }
        IrCmd::MulNum
          if let Some(n) = lhs_num
            && self.is_valid_double_for_immediate(n) =>
        {
          base.scale *= n as i32;
          base.op = rhs_op;
        }
        IrCmd::MulNum
          if let Some(n) = rhs_num
            && self.is_valid_double_for_immediate(n) =>
        {
          base.scale *= n as i32;
          base.op = lhs_op;
        }
        IrCmd::AddInt
          if let Some(n) = lhs_int
            && self.is_valid_integer_for_immediate(n) =>
        {
          base.offset += n * base.scale;
          base.op = rhs_op;
        }
        IrCmd::AddInt
          if let Some(n) = rhs_int
            && self.is_valid_integer_for_immediate(n) =>
        {
          base.offset += n * base.scale;
          base.op = lhs_op;
        }
        IrCmd::SubInt
          if let Some(n) = rhs_int
            && self.is_valid_integer_for_immediate(n) =>
        {
          base.offset -= n * base.scale;
          base.op = lhs_op;
        }
        IrCmd::TruncateUint => {
          base.op = lhs_op;
        }
        _ => break,
      }

      if !self.is_valid_integer_for_immediate(base.offset)
        || !self.is_valid_integer_for_immediate(base.scale)
      {
        break;
      }
    }

    base
  }

  /// 返回记录的既有指令下标；该指令已死且无副作用时返回 `None`
  pub fn get_previous_inst_index(&mut self, inst: &IrInst) -> Option<u32> {
    let prev_idx = *self.value_map.find(inst)?;

    let prev_inst = &self.function_ref().instructions[prev_idx as usize];
    (prev_inst.use_count != 0 || has_side_effects(prev_inst.cmd)).then_some(prev_idx)
  }

  pub fn get_previous_versioned_load_for_tag(&mut self, tag: u8, vm_reg: IrOp) -> (IrCmd, u32) {
    if !self.build.is_null() {
      let reg_index = vm_reg_op(vm_reg) as usize;
      // 未捕获寄存器才可复用既有加载；借用随语句结束，不跨越后续 &mut 调用
      let captured = !reg_bit_test(&self.function_ref().cfg.captured.regs, reg_index);
      if captured {
        let load_cmd = if tag == LuaType::Boolean as u8 {
          IrCmd::LoadInt
        } else if tag == LuaType::Number as u8 {
          IrCmd::LoadDouble
        } else if tag == LuaType::Integer as u8 {
          IrCmd::LoadInt64
        } else if tag == LuaType::Vector as u8 {
          IrCmd::LoadFloat
        } else if is_gco(tag) {
          IrCmd::LoadPointer
        } else {
          IrCmd::NOP
        };

        if load_cmd != IrCmd::NOP
          && let Some(prev_idx) = self.get_previous_versioned_load_index(load_cmd, vm_reg)
        {
          return (load_cmd, prev_idx);
        }
      }
    }

    (IrCmd::NOP, !0u32)
  }

  /// 查找 vm_reg 当前版本下 `cmd` 类加载的既有指令下标；
  /// 无记录、或该指令已死且无副作用时返回 `None`。
  pub fn get_previous_versioned_load_index(&mut self, cmd: IrCmd, vm_reg: IrOp) -> Option<u32> {
    CODEGEN_ASSERT!(vm_reg.kind() == IrOpKind::VmReg);

    let reg = vm_reg_op(vm_reg) as usize;
    let mut ops = IrOps::new();
    ops.push(IrOp::ir_op_ir_op_kind_u32(
      IrOpKind::VmReg,
      (vm_reg_op(vm_reg) as u32) | (self.regs[reg].version << 8),
    ));
    let versioned_load = IrInst {
      cmd,
      ops,
      ..IrInst::default()
    };

    let prev_idx = *self.value_map.find(&versioned_load)?;

    let inst = &self.function_ref().instructions[prev_idx as usize];
    (inst.use_count != 0 || has_side_effects(inst.cmd)).then_some(prev_idx)
  }

  pub fn invalidate_captured_registers(&mut self) {
    let max_reg = self.max_reg;
    // [u64; 4] 为 Copy，复制后与 self.regs 的可变借用无重叠
    let captured_regs = self.function_ref().cfg.captured.regs;
    for i in 0..=max_reg {
      let reg = i as usize;
      if reg_bit_test(&captured_regs, reg) {
        Self::invalidate_register_info_bool_bool(&mut self.regs[reg], true, true);
      }
    }
  }

  pub fn invalidate_heap_buffer_data(&mut self) {
    self.check_buffer_len_cache.clear();
    self.buffer_load_store_info.clear();
  }

  pub fn invalidate_heap(&mut self) {
    // cpp 无条件清理基于指令的堆状态缓存（OptimizeConstProp.cpp:316-323）
    self.inst_not_readonly.clear();
    self.inst_no_metatable.clear();
    self.inst_array_size.clear();

    self.invalidate_heap_table_data();

    // buffer 尺寸不可变，故不失效 buffer 长度检查
    self.buffer_load_store_info.clear();
  }

  pub fn invalidate_heap_table_data(&mut self) {
    self.get_slot_node_cache.clear();
    self.check_slot_match_cache.clear();
    self.get_arr_addr_cache.clear();
    self.check_array_size_cache.clear();
    self.hash_value_cache.clear();
    self.array_value_cache.clear();
  }

  /// 仅操作寄存器槽位本身，不读 state，无需 `&mut self`；
  /// 调用侧直接以 `&mut self.regs[idx]` 传入，免去裸指针绕借用。
  pub fn invalidate_register_info_bool_bool(
    reg: &mut RegisterInfo,
    invalidate_tag: bool,
    invalidate_value: bool,
  ) {
    if invalidate_tag {
      reg.tag = K_UNKNOWN_TAG;
    }

    if invalidate_value {
      reg.value = IrOp::default();
    }

    reg.version += 1;
  }

  pub fn invalidate_ir_op(&mut self, reg_op: IrOp) {
    // TODO: 使用 Proto 的 maxstacksize
    let vm_reg = vm_reg_op(reg_op);
    let max_reg = if vm_reg > self.max_reg {
      vm_reg
    } else {
      self.max_reg
    };
    self.max_reg = max_reg;

    Self::invalidate_register_info_bool_bool(&mut self.regs[vm_reg as usize], true, true);
  }

  pub fn invalidate_register_range(&mut self, first_reg: i32, count: i32) {
    if count == -1 {
      self.invalidate_registers_from(first_reg);
    } else {
      let max_reg = self.max_reg;
      for i in first_reg..(first_reg + count).min(max_reg + 1) {
        Self::invalidate_register_info_bool_bool(&mut self.regs[i as usize], true, true);
      }
    }
  }

  pub fn invalidate_registers_from(&mut self, first_reg: i32) {
    for i in first_reg..=self.max_reg {
      Self::invalidate_register_info_bool_bool(&mut self.regs[i as usize], true, true);
    }
  }

  /// cpp OptimizeConstProp.cpp:345-349 `invalidateTableArraySize`：清空按指令
  /// 索引记录的数组大小缓存（移植期开关 LuauCodegenExtraTableOpts 在 cpp 中
  /// 已删除，旧的逐寄存器 `knownTableArraySize` 旁路不复存在）。
  pub fn invalidate_table_array_size(&mut self) {
    self.inst_array_size.clear();
    self.invalidate_heap_table_data();
  }

  pub fn invalidate_table_store_location(
    &mut self,
    target_addr: IrInst,
    write_offset_op: IrOp,
    tag: u8,
  ) {
    match target_addr.cmd {
      IrCmd::GetSlotNodeAddr => {
        let target_key = op_c_ref(&target_addr);

        // hash_value_cache 即时读取 Copy 操作数（只读窗口），再定点改写失效项
        let keys: Vec<u32> = self
          .hash_value_cache
          .iter()
          .map(|(&pointer_idx, _)| pointer_idx)
          .collect();
        for pointer_idx in keys {
          let key_op = {
            let function = self.function_ref();
            op_c_ref(&function.instructions[pointer_idx as usize])
          };
          if key_op == target_key {
            *self.hash_value_cache.get_or_insert(pointer_idx) = K_INVALID_INST_IDX;
          }
        }

        if tag == K_UNKNOWN_TAG || tag == LuaType::Nil as u8 {
          // 下标迭代：函数只读窗口与 cache 元素写借用分离，消除散点 unsafe 解引用
          for el_idx in 0..self.check_slot_match_cache.len() {
            let pointer = self.check_slot_match_cache[el_idx].pointer;
            let hit = {
              let function = self.function_ref();
              let slot_addr_op = op_a_ref(&function.instructions[pointer as usize]);
              slot_addr_op.kind() == IrOpKind::Inst
                && op_c_ref(&function.instructions[slot_addr_op.index() as usize]) == target_key
            };
            if hit {
              self.check_slot_match_cache[el_idx].known_to_not_be_nil = false;
            }
          }
        }
      }
      IrCmd::GetArrAddr => {
        let offset_op = self.get_combined_array_load_offset_op(&target_addr, write_offset_op);
        let opt_offset = self.function_ref().as_int_op(offset_op);

        if let Some(offset) = opt_offset {
          let mut i = 0;
          while i < self.array_value_cache.len() {
            // 判定在只读窗口内完成，swap_remove 借用另起
            let remove = {
              let entry = &self.array_value_cache[i];
              entry.offset.kind() != IrOpKind::Constant
                || self.function_ref().int_op(entry.offset) == offset
            };

            if remove {
              self.array_value_cache.swap_remove(i);
            } else {
              i += 1;
            }
          }
        } else {
          self.array_value_cache.clear();
        }
      }
      IrCmd::TableSetnum => {
        debug_assert!(self.array_value_cache.is_empty());
      }
      _ => {
        debug_assert!(target_addr.cmd == IrCmd::GetClosureUpvalAddr);
      }
    }
  }

  pub fn invalidate_tag(&mut self, reg_op: IrOp) {
    let reg = vm_reg_op(reg_op);
    if reg > self.max_reg {
      self.max_reg = reg;
    }

    Self::invalidate_register_info_bool_bool(&mut self.regs[reg as usize], true, false);
  }

  pub fn invalidate_user_call(&mut self) {
    self.invalidate_heap();
    self.invalidate_captured_registers();
    self.invalidate_value_propagation();

    self.in_safe_env = false;
  }

  pub fn invalidate_value(&mut self, reg_op: IrOp) {
    let reg_index = vm_reg_op(reg_op);
    if reg_index > self.max_reg {
      self.max_reg = reg_index;
    }

    Self::invalidate_register_info_bool_bool(&mut self.regs[reg_index as usize], false, true);
  }

  pub fn invalidate_value_propagation(&mut self) {
    self.value_map.clear();
    self.upvalue_map.clear();

    self.try_num_to_index_cache.clear();

    self.buffer_load_store_info.clear();

    self.hash_value_cache.clear();
    self.array_value_cache.clear();

    // 其他 map 的清除虽已让 instValue 键不再命中，这里还省内存与 map 体积
    self.inst_value.clear();

    self.load_env_idx = K_INVALID_INST_IDX;
  }

  pub fn is_valid_double_for_immediate(&mut self, d: f64) -> bool {
    (-4095.0..=4095.0).contains(&d) && (d as i32) as f64 == d
  }

  pub fn is_valid_integer_for_immediate(&mut self, i: i32) -> bool {
    (-4095..=4095).contains(&i)
  }

  pub fn save_tag(&mut self, op: IrOp, tag: u8) {
    if let Some(info) = self.try_get_register_info(op)
      && info.tag != tag
    {
      info.tag = tag;
      info.version += 1;
    }
  }

  pub fn save_value(&mut self, op: IrOp, value: IrOp) {
    CODEGEN_ASSERT!(value.kind() == IrOpKind::Constant);

    if let Some(info) = self.try_get_register_info(op)
      && info.value != value
    {
      info.value = value;
      info.version += 1;
    }
  }

  pub fn substitute_or_record(&mut self, inst: &mut IrInst, inst_idx: u32) {
    if let Some(prev_idx) = self.value_map.find(inst).copied() {
      let prev_is_valid = {
        let prev = &self.function_ref().instructions[prev_idx as usize];
        prev.use_count != 0 || has_side_effects(prev.cmd)
      };

      if prev_is_valid {
        substitute(
          self.function_mut(),
          inst,
          IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
        );
        return;
      }
    }

    *self.value_map.get_or_insert(inst.clone()) = inst_idx;
  }

  pub fn substitute_or_record_buffer_load(
    &mut self,
    block_idx: u32,
    inst_idx: u32,
    load_inst: &mut IrInst,
    access_size: u8,
  ) {
    if op_b_ref(load_inst).kind() != IrOpKind::Constant {
      return;
    }

    let offset = self.function_ref().int_op(op_b_ref(load_inst));
    let tag = self.function_ref().tag_op(op_c_ref(load_inst));
    let address = op_a(load_inst);

    // 循环体内有 `&mut self` 调用（substitute_or_record / const_int），无法持有
    // buffer_load_store_info 的迭代借用；元素是 Copy，按下标取值即可，
    // 不再整表 clone（cpp 是零分配遍历）
    for i in 0..self.buffer_load_store_info.len() {
      let info = self.buffer_load_store_info[i];

      if info.address == address && info.offset == offset && info.tag == tag {
        if info.from_store {
          match load_inst.cmd {
            IrCmd::BufferReadi8 => {
              if info.load_cmd == IrCmd::BufferReadi8 {
                if info.value.kind() == IrOpKind::Inst {
                  replace_ir_function_ir_block_u32_ir_inst(
                    self.function_mut(),
                    block_idx,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::Sexti8Int, &[info.value]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  substitute(self.function_mut(), load_inst, info.value);
                }
                return;
              }
            }
            IrCmd::BufferReadu8 => {
              if info.load_cmd == IrCmd::BufferReadi8 {
                if info.value.kind() == IrOpKind::Inst {
                  let mask = self.build_mut().const_int(0xff);
                  replace_ir_function_ir_block_u32_ir_inst(
                    self.function_mut(),
                    block_idx,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::BitandUint, &[info.value, mask]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  let value = self.function_ref().int_op(info.value) as u8 as i32;
                  let value = self.build_mut().const_int(value);
                  substitute(self.function_mut(), load_inst, value);
                }
                return;
              }
            }
            IrCmd::BufferReadi16 => {
              if info.load_cmd == IrCmd::BufferReadi16 {
                if info.value.kind() == IrOpKind::Inst {
                  replace_ir_function_ir_block_u32_ir_inst(
                    self.function_mut(),
                    block_idx,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::Sexti16Int, &[info.value]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  substitute(self.function_mut(), load_inst, info.value);
                }
                return;
              }
            }
            IrCmd::BufferReadu16 => {
              if info.load_cmd == IrCmd::BufferReadi16 {
                if info.value.kind() == IrOpKind::Inst {
                  let mask = self.build_mut().const_int(0xffff);
                  replace_ir_function_ir_block_u32_ir_inst(
                    self.function_mut(),
                    block_idx,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::BitandUint, &[info.value, mask]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  let value = self.function_ref().int_op(info.value) as u16 as i32;
                  let value = self.build_mut().const_int(value);
                  substitute(self.function_mut(), load_inst, value);
                }
                return;
              }
            }
            IrCmd::BufferReadi32 => {
              if info.load_cmd == IrCmd::BufferReadi32 {
                let dirty = self
                  .function_ref()
                  .as_inst_op_ref(info.value)
                  .is_some_and(|src| produces_dirty_high_register_bits(src.cmd));
                if dirty {
                  replace_ir_function_ir_block_u32_ir_inst(
                    self.function_mut(),
                    block_idx,
                    inst_idx,
                    IrInst::ir_inst_new(IrCmd::TruncateUint, &[info.value]),
                  );
                  self.substitute_or_record(load_inst, inst_idx);
                } else {
                  substitute(self.function_mut(), load_inst, info.value);
                }
                return;
              }
            }
            IrCmd::BufferReadf32 => {
              if info.load_cmd == IrCmd::BufferReadf32 {
                substitute(self.function_mut(), load_inst, info.value);
                return;
              }
            }
            IrCmd::BufferReadf64 => {
              if info.load_cmd == IrCmd::BufferReadf64 {
                substitute(self.function_mut(), load_inst, info.value);
                return;
              }
            }
            IrCmd::BufferReadi64 => {
              if info.load_cmd == IrCmd::BufferReadi64 {
                substitute(self.function_mut(), load_inst, info.value);
                return;
              }
            }
            _ => {
              CODEGEN_ASSERT!(false);
            }
          }
        } else if info.load_cmd == load_inst.cmd {
          substitute(self.function_mut(), load_inst, info.value);
          return;
        }
      }
    }

    let value_idx = self.function_ref().get_inst_index(load_inst);
    self.buffer_load_store_info.push(BufferLoadStoreInfo {
      load_cmd: load_inst.cmd,
      access_size,
      tag,
      from_store: false,
      address,
      value: IrOp::ir_op_kind_u32(IrOpKind::Inst, value_idx),
      offset,
    });
  }

  pub fn substitute_or_record_value_load_with_t_value_data(
    &mut self,
    load_inst: &mut IrInst,
  ) -> bool {
    CODEGEN_ASSERT!(op_a(load_inst).kind() == IrOpKind::VmReg);

    if let Some(prev_idx) =
      self.get_previous_versioned_load_index(IrCmd::LoadTvalue, op_a(load_inst))
    {
      if let Some(&value_op) = self.inst_value.find(&prev_idx) {
        if let Some(value) = self.function_ref().as_inst_op_ref(value_op) {
          if value.use_count != 0 && value.cmd == load_inst.cmd {
            substitute(self.function_mut(), load_inst, value_op);
            return true;
          }

          if value.use_count != 0
            && get_cmd_value_kind(value.cmd) == get_cmd_value_kind(load_inst.cmd)
          {
            substitute(self.function_mut(), load_inst, value_op);
            return true;
          }
        } else if value_op.kind() == IrOpKind::Constant {
          let constant = self.function_ref().const_op(value_op);
          if get_const_value_kind(&constant) == get_cmd_value_kind(load_inst.cmd) {
            substitute(self.function_mut(), load_inst, value_op);
            return true;
          }
        }
      } else {
        let idx = self.function_ref().get_inst_index(load_inst);
        self
          .inst_value
          .try_insert(prev_idx, IrOp::ir_op_kind_u32(IrOpKind::Inst, idx));
      }
    }

    false
  }

  pub fn substitute_or_record_vm_reg_load(&mut self, load_inst: &mut IrInst) -> bool {
    let reg_op = op_a(load_inst);
    CODEGEN_ASSERT!(reg_op.kind() == IrOpKind::VmReg);

    let reg = vm_reg_op(reg_op) as usize;
    // 经 `function_ref` 只读门面（records 契约单点）读 `cfg.captured.regs` 位集，
    // 借用随 `reg_bit_test` 只读消费即结束。
    let captured_regs = &self.function_ref().cfg.captured.regs;
    if reg_bit_test(captured_regs, reg) {
      return false;
    }

    // cpp OptimizeConstProp.cpp:493-494：LOAD_FLOAT 带额外 offset 操作数时
    // 走双操作数的 versionedVmRegLoad 重载
    let op_a = reg_op;
    let versioned_load = if load_inst.cmd == IrCmd::LoadFloat && load_inst.ops.size() > 1 {
      let op_b = load_inst.ops.as_slice()[1];
      self.versioned_vm_reg_load_ir_cmd_ir_op_ir_op(load_inst.cmd, op_a, op_b)
    } else {
      self.versioned_vm_reg_load_ir_cmd_ir_op(load_inst.cmd, op_a)
    };

    if let Some(prev_idx) = self.value_map.find(&versioned_load).copied() {
      // `prev_idx` 来自 `value_map` 中先前登记的合法指令下标；经 `function_ref`
      // 只读门面读 `instructions[prev_idx]` 的 use_count/cmd，借用即求即释。
      let prev = &self.function_ref().instructions[prev_idx as usize];
      let prev_is_valid = prev.use_count != 0 || has_side_effects(prev.cmd);

      if prev_is_valid {
        if !self.inst_link.contains(&prev_idx) {
          self.create_reg_link(prev_idx, reg_op);
        }

        // 与 cpp 一致，const-prop 将 `function` 与其中的 `load_inst` 作为同一 IR arena 的交错视图
        // (结构体文档所述的刻意共享可变上下文)。本 pass 单线程串行执行，`substitute` 在把 `load_inst`
        // 替换为对 `prev_idx` 的引用时，对该指令的读写按语句顺序进行、无第二写者并发；可变借用
        // 经 `function_mut` 门面单点派生、本调用即时消费。
        substitute(
          self.function_mut(),
          load_inst,
          IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
        );
        return true;
      }
    }

    // `get_inst_index` 只共享只读遍历 `instructions` 定位 `load_inst` 的下标，
    // 走 `function_ref` 门面、派生借用即时结束。
    let inst_idx = self.function_ref().get_inst_index(load_inst);
    *self.value_map.get_or_insert(versioned_load) = inst_idx;
    self.create_reg_link(inst_idx, reg_op);
    false
  }

  pub fn substitute_or_record_vm_upvalue_load(&mut self, load_inst: &mut IrInst) -> bool {
    CODEGEN_ASSERT!(op_a(load_inst).kind() == IrOpKind::VmUpvalue);

    let key = vm_upvalue_op(op_a(load_inst));
    let key_u8 = key as u8;
    // 命中判定先按值取出 `prev_idx`（u32 拷贝），释放 `upvalue_map` 借用，
    // 使可变借用可经 `function_mut` 门面单点派生。
    if let Some(&prev_idx) = self.upvalue_map.find(&key_u8)
      && prev_idx != u32::MAX
    {
      // 用前值替换该 load 指令。与 cpp 一致，const-prop 将 `function` 与其中的 `load_inst`
      // 作为同一 IR arena 的交错视图(结构体文档所述的刻意共享可变上下文)。本 pass 单线程串行
      // 执行，`substitute` 在把 `load_inst` 替换为对 `prev_idx` 的引用时按语句顺序读写该指令、
      // 无第二写者并发；可变借用经 `function_mut` 门面派生、本调用即时消费。
      substitute(
        self.function_mut(),
        load_inst,
        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
      );
      return true;
    }

    // `get_inst_index` 只共享只读遍历 `instructions` 定位 `load_inst` 的下标，
    // 走 `function_ref` 门面、派生借用即时结束。
    let inst_idx = self.function_ref().get_inst_index(load_inst);

    // 记录本次 upvalue load，供后续替换
    *self.upvalue_map.get_or_insert(key_u8) = inst_idx;
    false
  }

  pub fn substitute_tag_load_with_t_value_data(&mut self, load_inst: &mut IrInst) -> bool {
    CODEGEN_ASSERT!(op_a(load_inst).kind() == IrOpKind::VmReg);

    if let Some(prev_idx) =
      self.get_previous_versioned_load_index(IrCmd::LoadTvalue, op_a(load_inst))
      && let Some(tag) = self.inst_tag.find(&prev_idx)
      && *tag != K_UNKNOWN_TAG
    {
      let tag = *tag;
      let replacement = self.build_mut().const_tag(tag);
      substitute(self.function_mut(), load_inst, replacement);
      return true;
    }

    false
  }

  /// 与 cpp 闭包捕获的 `build.vmReg(i)` 等价：纯 `IrOp` 构造，无需 IrBuilder
  fn vm_reg_op(index: usize) -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::VmReg, index as u32)
  }

  /// 链接仅被读取（版本判定与寄存器号），返回共享引用即可
  pub fn try_get_reg_link(&mut self, inst_op: IrOp) -> Option<&RegisterLink> {
    if inst_op.kind() != IrOpKind::Inst {
      return None;
    }
    let link = self.inst_link.find(&inst_op.index())?;
    (link.version >= self.regs[link.reg as usize].version).then_some(link)
  }

  pub fn try_get_register_info(&mut self, op: IrOp) -> Option<&mut RegisterInfo> {
    if op.kind() == IrOpKind::VmReg {
      let vm_reg = vm_reg_op(op);
      if vm_reg > self.max_reg {
        self.max_reg = vm_reg;
      }
      return self.regs.get_mut(vm_reg as usize);
    }

    let reg = self.try_get_reg_link(op)?.reg;
    let reg_i32 = i32::from(reg);
    if reg_i32 > self.max_reg {
      self.max_reg = reg_i32;
    }
    self.regs.get_mut(reg as usize)
  }

  pub fn try_get_tag(&mut self, op: IrOp) -> u8 {
    if let Some(info) = self.try_get_register_info(op)
      && info.tag != K_UNKNOWN_TAG
    {
      return info.tag;
    }
    if op.kind() == IrOpKind::Inst
      && let Some(info) = self.inst_tag.find(&op.index())
    {
      return *info;
    }
    0xff
  }

  pub fn try_get_value(&mut self, op: IrOp) -> IrOp {
    if let Some(info) = self.try_get_register_info(op) {
      return info.value;
    }

    IrOp::ir_op_kind_u32(IrOpKind::None, 0)
  }

  pub fn try_merge_and_kill_buffer_length_check(
    &mut self,
    curr_idx: u32,
    prev_check: &mut IrInst,
    extra_offset: i32,
  ) -> bool {
    // 四边界快照，借用即取即释；curr 侧按索引即时读，不再随借用传指针
    let (prev_min_offset, prev_max_offset, curr_min_offset, curr_max_offset) = {
      let function = self.function_ref();
      let curr = &function.instructions[curr_idx as usize];
      (
        function.int_op(op_c_ref(prev_check)),
        function.int_op(op_d_ref(prev_check)),
        function.int_op(op_c_ref(curr)) + extra_offset,
        function.int_op(op_d_ref(curr)) + extra_offset,
      )
    };
    let new_min_offset = prev_min_offset.min(curr_min_offset);
    let new_max_offset = prev_max_offset.max(curr_max_offset);

    if new_max_offset - new_min_offset > 4095 {
      return false;
    }
    if !(-4095..=4095).contains(&new_min_offset) {
      return false;
    }

    if new_min_offset != prev_min_offset {
      let replacement = self.build_mut().const_int(new_min_offset);
      replace_ir_function_ir_op_ir_op(self.function_mut(), &mut prev_check.ops[2], replacement);
    }

    if new_max_offset != prev_max_offset {
      let replacement = self.build_mut().const_int(new_max_offset);
      replace_ir_function_ir_op_ir_op(self.function_mut(), &mut prev_check.ops[3], replacement);
    }

    // curr_check 已索引化：kill 只借函数视图，无指令借用重叠
    kill_ir_function_ir_inst_at(self.function_mut(), curr_idx);
    true
  }

  pub fn try_merge_buffer_range_check(
    &mut self,
    block_idx: u32,
    curr_idx: u32,
    prev: &mut IrInst,
  ) -> bool {
    // curr 侧判定字段即时快照（IrOp 为 Copy），不再持有指令借用跨 callee
    let (curr_a, curr_index_op) = {
      let function = self.function_ref();
      let curr = &function.instructions[curr_idx as usize];
      (op_a_ref(curr), op_b_ref(curr))
    };

    if curr_a != op_a_ref(prev) {
      return false;
    }

    let prev_index_op = op_b_ref(prev);

    // 快照两条 index 指令的判定字段（cmd 与 op_a）：
    // 后续 get_offset_base / replace 等 &mut self 调用不得持有指令借用
    let index_fields = {
      let function = self.function_ref();
      (
        function
          .as_inst_op_ref(curr_index_op)
          .map(|i| (i.cmd, op_a_ref(i))),
        function
          .as_inst_op_ref(prev_index_op)
          .map(|i| (i.cmd, op_a_ref(i))),
      )
    };
    let (Some((curr_index_cmd, curr_index_source)), Some((prev_index_cmd, prev_index_source))) =
      index_fields
    else {
      return false;
    };

    if curr_index_cmd == IrCmd::NumToInt && prev_index_cmd == IrCmd::NumToInt {
      let offset_base_curr = self.get_offset_base(curr_index_source);
      let offset_base_prev = self.get_offset_base(prev_index_source);
      if offset_base_curr.op == offset_base_prev.op
        && offset_base_curr.scale == offset_base_prev.scale
        && offset_base_curr.op.kind() != IrOpKind::Constant
      {
        let extra_offset = offset_base_curr.offset - offset_base_prev.offset;
        if extra_offset != 0 {
          if prev_index_op.index() >= curr_index_op.index() {
            return false;
          }
          let offset = self.build_mut().const_int(extra_offset);
          let mut ops = IrOps::new();
          ops.push(prev_index_op);
          ops.push(offset);
          replace_ir_function_ir_block_u32_ir_inst(
            self.function_mut(),
            block_idx,
            curr_index_op.index(),
            IrInst {
              cmd: IrCmd::AddInt,
              ops,
              ..IrInst::default()
            },
          );
        }

        if op_e_ref(prev).kind() == IrOpKind::Undef {
          // prev_index 指令未被上面的替换波及（已校验其下标更小），
          // 快照的 op_a 即当前值
          let replacement = prev_index_source;
          replace_ir_function_ir_op_ir_op(self.function_mut(), &mut prev.ops[4], replacement);
        }
        return self.try_merge_and_kill_buffer_length_check(curr_idx, prev, extra_offset);
      }
    } else if get_cmd_value_kind(curr_index_cmd) == IrValueKind::Int
      && get_cmd_value_kind(prev_index_cmd) == IrValueKind::Int
    {
      let offset_base_curr = self.get_offset_base(curr_index_op);
      let offset_base_prev = self.get_offset_base(prev_index_op);
      if offset_base_curr.op == offset_base_prev.op
        && offset_base_curr.scale == offset_base_prev.scale
      {
        let extra_offset = offset_base_curr.offset - offset_base_prev.offset;
        return self.try_merge_and_kill_buffer_length_check(curr_idx, prev, extra_offset);
      }
    }
    false
  }

  pub fn try_redirect_vm_reg_load_to_t_value_origin(&mut self, load_inst: &mut IrInst) -> bool {
    let source = op_a(load_inst);

    if let Some(prev_idx) = self.get_previous_versioned_load_index(IrCmd::LoadTvalue, source) {
      let tvalue_load = &self.function_ref().instructions[prev_idx as usize];
      let tvalue_source = op_a_ref(tvalue_load);

      if tvalue_load.cmd != IrCmd::LoadTvalue || tvalue_source.kind() != IrOpKind::VmReg {
        return false;
      }

      let prev_load_reg = vm_reg_op(tvalue_source);

      if prev_load_reg == vm_reg_op(source) {
        return false;
      }

      // 前驱加载仍链接在同一寄存器上才可重定向（cpp OptimizeConstProp.cpp:618）
      let prev_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx);
      match self.try_get_reg_link(prev_op) {
        Some(link) if i32::from(link.reg) == prev_load_reg => {}
        _ => return false,
      }

      replace_ir_function_ir_op_ir_op(self.function_mut(), &mut load_inst.ops[0], tvalue_source);
      true
    } else {
      false
    }
  }

  pub fn update_tag(&mut self, op: IrOp, tag: u8) {
    if let Some(info) = self.try_get_register_info(op) {
      info.tag = tag;
    }
  }

  pub fn versioned_vm_reg_load_ir_cmd_ir_op(&mut self, load_cmd: IrCmd, mut op: IrOp) -> IrInst {
    let version = self.regs[vm_reg_op(op) as usize].version;
    op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, (vm_reg_op(op) as u32) | (version << 8));

    let mut ops = IrOps::new();
    ops.push(op);

    IrInst {
      cmd: load_cmd,
      ops,
      ..IrInst::default()
    }
  }

  pub fn versioned_vm_reg_load_ir_cmd_ir_op_ir_op(
    &mut self,
    load_cmd: IrCmd,
    op_a: IrOp,
    op_b: IrOp,
  ) -> IrInst {
    let mut inst = self.versioned_vm_reg_load_ir_cmd_ir_op(load_cmd, op_a);
    inst.ops.push(op_b);
    inst
  }
}

impl TagAccess for ConstPropState {
  /// cpp OptimizeConstProp.cpp：get 读 `state.regs[i].tag`，set 走 `state.updateTag(vmReg(i), tag)`
  fn get_tag(&self, i: usize) -> u8 {
    self.regs[i].tag
  }

  fn set_tag(&mut self, i: usize, tag: u8) {
    self.update_tag(Self::vm_reg_op(i), tag);
  }
}
