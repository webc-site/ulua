use alloc::vec::Vec;
use core::{ffi::c_int, mem::take};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag,
  fflag::LuauBackedgeHeapCheck,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fast_call::is_fast_call,
  },
  macros::{
    luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
    luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op,
  },
  records::dense_hash_map::DenseHashMap,
};
use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  records::proto::Proto,
};

use crate::{
  enums::{
    ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_condition::IrCondition,
    ir_const_kind::IrConstKind, ir_op_kind::IrOpKind,
  },
  functions::{
    add_use::add_use,
    after_inst_for_n_loop::after_inst_for_n_loop,
    analyze_bytecode_types::analyze_bytecode_types,
    before_inst_for_n_prep::before_inst_for_n_prep,
    build_argument_type_checks::build_argument_type_checks,
    build_bytecode_blocks::build_bytecode_blocks,
    can_invalidate_safe_env::can_invalidate_safe_env,
    has_typed_parameters::has_typed_parameters,
    is_block_terminator::is_block_terminator,
    is_direct_compare::is_direct_compare,
    is_pseudo::is_pseudo,
    kill_ir_utils::kill_ir_function_ir_inst_at,
    load_bytecode_type_info::load_bytecode_type_info,
    proto_views::code,
    translate_fast_call_n::translate_fast_call_n,
    translate_inst_binary::{
      translate_inst_binary, translate_inst_binary_k, translate_inst_binary_rk,
    },
    translate_inst_capture::translate_inst_capture,
    translate_inst_close_upvals::translate_inst_close_upvals,
    translate_inst_cmp_proto::translate_inst_cmp_proto,
    translate_inst_concat::translate_inst_concat,
    translate_inst_dup_table::translate_inst_dup_table,
    translate_inst_for_g_loop_ipairs::translate_inst_for_g_loop_ipairs,
    translate_inst_for_g_prep_next::{
      translate_inst_for_g_prep_inext, translate_inst_for_g_prep_next,
    },
    translate_inst_for_n_loop::translate_inst_for_n_loop,
    translate_inst_for_n_prep::translate_inst_for_n_prep,
    translate_inst_get_global::translate_inst_get_global,
    translate_inst_get_import::translate_inst_get_import,
    translate_inst_get_table::{translate_inst_get_table, translate_table_access},
    translate_inst_get_table_ks::{translate_inst_get_table_ks, translate_inst_set_table_ks},
    translate_inst_get_table_n::{translate_inst_get_table_n, translate_table_access_n},
    translate_inst_get_upval::translate_inst_get_upval,
    translate_inst_jump::{translate_inst_jump, translate_inst_jump_back, translate_inst_jump_x},
    translate_inst_jump_if::translate_inst_jump_if,
    translate_inst_jump_if_cond::translate_inst_jump_if_cond,
    translate_inst_jump_if_eq::{translate_inst_jump_if_eq, translate_inst_jump_if_eq_shortcut},
    translate_inst_jumpx_eq::{translate_inst_jumpx_eq, translate_inst_jumpx_eq_shortcut},
    translate_inst_length::translate_inst_length,
    translate_inst_load::{
      translate_inst_load_b, translate_inst_load_k, translate_inst_load_kx, translate_inst_load_n,
      translate_inst_load_nil,
    },
    translate_inst_logical::translate_inst_and_or_x,
    translate_inst_minus::translate_inst_minus,
    translate_inst_move::translate_inst_move,
    translate_inst_namecall::translate_inst_namecall,
    translate_inst_new_closure::translate_inst_new_closure,
    translate_inst_new_table::translate_inst_new_table,
    translate_inst_not::translate_inst_not,
    translate_inst_set_global::translate_inst_set_global,
    translate_inst_set_upval::translate_inst_set_upval,
    update_use_counts::update_use_counts,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    bytecode_mapping::BytecodeMapping,
    host_ir_hooks::HostIrHooks,
    ir_block::{
      IrBlock, K_BLOCK_FLAG_SAFE_ENV_CHECK, K_BLOCK_FLAG_SAFE_ENV_CLEAR, K_BLOCK_NO_START_PC,
    },
    ir_const::IrConst,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    label::Label,
    register_a_64::RegisterA64,
    register_x_64::RegisterX64,
  },
  type_aliases::{instruction_ir_builder::Instruction, ir_ops::IrOps},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopInfo {
  pub step: IrOp,
  pub startpc: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstantKey {
  pub kind: IrConstKind,
  pub value: u64,
}

/// 常量去重表；`IrBuilder` 与 `IrFunction` 的 const_* 构造共享此类型
pub type ConstantMap = DenseHashMap<ConstantKey, u32>;

#[derive(Debug)]
pub struct IrBuilder {
  pub host_hooks: *const HostIrHooks, // const HostIrHooks&
  pub in_terminated_block: bool,
  pub interrupt_requested: bool,

  pub active_fastcall_fallback: bool,
  pub fastcall_fallback_return: IrOp,
  pub cmd_skip_target: i32,

  pub function: IrFunction,

  pub active_block_idx: u32,

  /// 该字节码指令处的 block 索引
  pub inst_index_to_block: Vec<u32>,

  pub numeric_loop_stack: Vec<LoopInfo>,

  pub constant_map: ConstantMap,
}

impl IrBuilder {
  pub fn begin_block(&mut self, block: IrOp) {
    let target = &mut self.function.blocks[block.index() as usize];
    self.active_block_idx = block.index();

    CODEGEN_ASSERT!(
      target.start == !0u32 || target.start == self.function.instructions.len() as u32
    );

    target.start = self.function.instructions.len() as u32;
    target.sortkey = target.start;

    self.in_terminated_block = false;
  }

  /// 压入新块的公共骨架：`block`/`fallback_block` 仅 `kind`、`startpc` 两字段取值不同，单源防漂移。
  pub(crate) fn push_block(&mut self, kind: IrBlockKind, startpc: u32) -> IrOp {
    let index = self.function.blocks.len() as u32;
    self.function.blocks.push(IrBlock {
      kind,
      flags: 0,
      use_count: 0,
      start: !0u32,
      finish: !0u32,
      sortkey: 0,
      chainkey: 0,
      expected_next_block: !0u32,
      startpc,
      label: Label::default(),
    });
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, index)
  }

  pub fn block(&mut self, mut kind: IrBlockKind) -> IrOp {
    CODEGEN_ASSERT!(kind != IrBlockKind::Fallback);

    if kind == IrBlockKind::Internal && self.active_fastcall_fallback {
      kind = IrBlockKind::Fallback;
    }

    self.push_block(kind, !0u32)
  }

  pub fn block_at_inst(&mut self, index: u32) -> IrOp {
    let block_index = self.inst_index_to_block[index as usize];
    if block_index != u32::MAX {
      return IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_index);
    }

    let result = self.block(IrBlockKind::Internal);
    self.function.block_op(result).startpc = index;
    result
  }

  /// # Safety
  /// `proto` 必须指向存活的 `Proto`，其 `code/sizecode` 字节码区在 IR 构建全程只读存活
  /// （契约与 C++ 参考实现一致）。
  pub unsafe fn build_function_ir(&mut self, proto: *mut Proto) {
    // Safety: 契约保证 proto 为存活 Proto；IR 构建阶段字节码流不可变（无重编译改写 proto），
    // 该共享引用只读，且本函数调用栈上不写 Proto 字段（写点在 bind_native_protos 等阶段）。
    let proto_ref = unsafe { &*proto };

    // IrFunction::proto 仍是跨 crate 的 C-ABI 裸指针 spine（VM 侧布局不变），此处按值存入；
    // 其余只读消费一律走 `proto_views` 的安全视图。
    self.function.proto = proto;
    self.function.variadic = proto_ref.is_vararg != 0;

    load_bytecode_type_info(&mut self.function);

    let generate_type_checks = has_typed_parameters(&self.function.bc_type_info);
    let mut entry = if generate_type_checks {
      self.block(IrBlockKind::Internal)
    } else {
      IrOp::default()
    };

    self.rebuild_bytecode_basic_blocks(proto_ref);

    // 指令流视图：`code.len()` 与原 `max(sizecode, 0)` 等价（空基址/负长度折成空切片）
    let code = code(proto_ref);
    let sizecode = code.len() as c_int;

    // Safety: host_hooks 为 IrBuilder 构造点接线、非空且比 self 长寿的钩子表，
    // 重建的共享借用仅存活于本语句的类型分析期间。
    analyze_bytecode_types(&mut self.function, unsafe { &*self.host_hooks });

    self.function.bc_mapping.resize(
      code.len(),
      BytecodeMapping {
        ir_location: !0u32,
        asm_location: !0u32,
      },
    );

    if generate_type_checks {
      self.begin_block(entry);
      build_argument_type_checks(self, entry);

      let block0 = self.block_at_inst(0);
      self.inst_ir_cmd_ir_op(IrCmd::JUMP, block0);
    } else {
      entry = self.block_at_inst(0);
    }

    self.function.entry_block = entry.index();

    let mut i = 0i32;
    while i < sizecode {
      let op = LuauOpcode::from(luau_insn_op(code[i as usize]) as u8);
      let mut nexti = i + get_op_length(op);
      CODEGEN_ASSERT!(nexti <= sizecode);

      self.function.bc_mapping[i as usize] = BytecodeMapping {
        ir_location: self.function.instructions.len() as u32,
        asm_location: !0u32,
      };

      if self.inst_index_to_block[i as usize] != !0u32 {
        let block = self.block_at_inst(i as u32);
        self.begin_block(block);
        self.function.block_op(block).startpc = i as u32;
      }

      if op == LuauOpcode::LOP_FORNPREP {
        before_inst_for_n_prep(self, code, i);
      }

      if !self.in_terminated_block {
        if self.interrupt_requested {
          self.interrupt_requested = false;
          let pcpos = self.const_uint(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);
        }

        self.translate_inst(op, code, i);

        if self.cmd_skip_target != -1 {
          nexti = self.cmd_skip_target;
          self.cmd_skip_target = -1;
        }
      }

      if op == LuauOpcode::LOP_FORNLOOP {
        after_inst_for_n_loop(self);
      }

      i = nexti;
      CODEGEN_ASSERT!(i <= sizecode);

      if (i as usize) < self.inst_index_to_block.len()
        && self.inst_index_to_block[i as usize] != !0u32
        && let Some(last) = self.function.instructions.last()
        && !is_block_terminator(last.cmd)
      {
        let block = self.block_at_inst(i as u32);
        self.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
      }
    }

    update_use_counts(&mut self.function);
  }

  pub fn check_safe_env(&mut self, pcpos: i32) {
    let active_block_idx = self.active_block_idx as usize;
    let active: &mut IrBlock = &mut self.function.blocks[active_block_idx];

    if active.startpc != K_BLOCK_NO_START_PC && (active.flags & K_BLOCK_FLAG_SAFE_ENV_CLEAR) == 0 {
      active.flags |= K_BLOCK_FLAG_SAFE_ENV_CHECK;
    }

    let exit_op = self.vm_exit(pcpos as u32);
    self.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, exit_op);
  }

  pub fn clone(&mut self, source_idxs: Vec<u32>, remove_current_terminator: bool) {
    let mut inst_redir: DenseHashMap<u32, u32> = DenseHashMap::new(!0u32);

    for source_idx in source_idxs {
      let source = self.function.blocks[source_idx as usize];

      if remove_current_terminator && self.in_terminated_block {
        let finish = self.function.blocks[self.active_block_idx as usize].finish;
        kill_ir_function_ir_inst_at(&mut self.function, finish);
        self.in_terminated_block = false;
      }

      if (source.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
        CODEGEN_ASSERT!(source.startpc != K_BLOCK_NO_START_PC);
        let exit = self.vm_exit(source.startpc);
        self.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, exit);
      }

      for index in source.start..=source.finish {
        CODEGEN_ASSERT!((index as usize) < self.function.instructions.len());
        let mut clone = self.function.instructions[index as usize].clone();

        if is_pseudo(clone.cmd) {
          CODEGEN_ASSERT!(clone.use_count == 0);
          continue;
        }

        for op in clone.ops.as_mut_slice() {
          if op.kind() == IrOpKind::Inst {
            if let Some(&new_index) = inst_redir.find(&op.index()) {
              *op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, new_index);
            } else {
              CODEGEN_ASSERT!(false);
            }
          }
        }

        for &op in clone.ops.as_slice() {
          add_use(&mut self.function, op);
        }

        *inst_redir.get_or_insert(index) = self.function.instructions.len() as u32;
        self.inst_ir_cmd_ir_ops(clone.cmd, &clone.ops);
      }
    }
  }

  pub fn cond(&mut self, cond: IrCondition) -> IrOp {
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Condition, cond as u8 as u32)
  }

  pub fn const_any(&mut self, constant: IrConst, as_common_key: u64) -> IrOp {
    // intern 收口在 `IrFunction::const_any`；此处仅聚拢两个互不相交字段
    self
      .function
      .const_any(&mut self.constant_map, constant, as_common_key)
  }

  pub fn const_double(&mut self, value: f64) -> IrOp {
    self.function.const_double(&mut self.constant_map, value)
  }

  pub fn const_import(&mut self, value: u32) -> IrOp {
    self.const_any(IrConst::Import(value), value as u64)
  }

  pub fn const_int(&mut self, value: i32) -> IrOp {
    self.function.const_int(&mut self.constant_map, value)
  }

  pub fn const_int_64(&mut self, value: i64) -> IrOp {
    self.function.const_int_64(&mut self.constant_map, value)
  }

  pub fn const_tag(&mut self, value: u8) -> IrOp {
    self.function.const_tag(&mut self.constant_map, value)
  }

  pub fn const_uint(&mut self, value: u32) -> IrOp {
    self.function.const_uint(&mut self.constant_map, value)
  }

  pub fn fallback_block(&mut self, pcpos: u32) -> IrOp {
    // 原断言的 `index` 即 push 前的 blocks.len()，前移到 push 之前等价
    CODEGEN_ASSERT!(!self.function.blocks.is_empty());
    self.push_block(IrBlockKind::Fallback, pcpos)
  }

  /// FASTCALL 序列收尾：登记 fallback 块或直接跳过。
  /// `code` 为字节码只读切片，`i` 为当前 FASTCALL 指令下标；越界由切片索引 panic 兜底。
  pub fn handle_fastcall_fallback(
    &mut self,
    fallback_or_undef: IrOp,
    code: &[Instruction],
    i: i32,
  ) {
    let skip = luau_insn_c(code[i as usize]) as i32;

    if fallback_or_undef.kind() != IrOpKind::Undef {
      let next = self.block_at_inst((i + skip + 2) as u32);
      self.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
      self.begin_block(fallback_or_undef);

      self.active_fastcall_fallback = true;
      self.fastcall_fallback_return = next;
    } else {
      self.cmd_skip_target = i + skip + 2;
    }
  }

  pub fn inst_ir_cmd(&mut self, cmd: IrCmd) -> IrOp {
    // C++ `inst(cmd, {})`——空操作数列表（不是单个 `undef`）。
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[])
  }

  pub fn inst_ir_cmd_ir_op(&mut self, cmd: IrCmd, a: IrOp) -> IrOp {
    // C++ `inst(cmd, {a})`——长度为 1 的操作数列表。原移植用
    // `undef` 补齐到两个操作数，留下多余的 `undef`
    // （如 `RETURN 0u, undef`）。
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a])
  }

  pub fn inst_ir_cmd_ir_op_ir_op(&mut self, cmd: IrCmd, a: IrOp, b: IrOp) -> IrOp {
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a, b])
  }

  pub fn inst_ir_cmd_ir_op_ir_op_ir_op(&mut self, cmd: IrCmd, a: IrOp, b: IrOp, c: IrOp) -> IrOp {
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a, b, c])
  }

  pub fn inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    &mut self,
    cmd: IrCmd,
    a: IrOp,
    b: IrOp,
    c: IrOp,
    d: IrOp,
  ) -> IrOp {
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a, b, c, d])
  }

  pub fn inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    &mut self,
    cmd: IrCmd,
    a: IrOp,
    b: IrOp,
    c: IrOp,
    d: IrOp,
    e: IrOp,
  ) -> IrOp {
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a, b, c, d, e])
  }

  pub fn inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
    &mut self,
    cmd: IrCmd,
    a: IrOp,
    b: IrOp,
    c: IrOp,
    d: IrOp,
    e: IrOp,
    f: IrOp,
  ) -> IrOp {
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a, b, c, d, e, f])
  }

  pub fn inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
    &mut self,
    cmd: IrCmd,
    a: IrOp,
    b: IrOp,
    c: IrOp,
    d: IrOp,
    e: IrOp,
    f: IrOp,
    g: IrOp,
  ) -> IrOp {
    let ops = [a, b, c, d, e, f, g];
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &ops)
  }

  pub fn inst_ir_cmd_initializer_list_ir_op(&mut self, cmd: IrCmd, ops: &[IrOp]) -> IrOp {
    let index = self.function.instructions.len() as u32;
    self.function.instructions.push(IrInst {
      cmd,
      ops: ops.iter().cloned().collect(),
      last_use: 0,
      use_count: 0,
      reg_x64: RegisterX64::default(),
      reg_a64: RegisterA64::default(),
      reused_reg: false,
      spilled: false,
      needs_reload: false,
    });

    CODEGEN_ASSERT!(!self.in_terminated_block);

    if is_block_terminator(cmd) {
      self.function.blocks[self.active_block_idx as usize].finish = index;
      self.in_terminated_block = true;
    }

    if can_invalidate_safe_env(cmd) {
      self.function.blocks[self.active_block_idx as usize].flags |= K_BLOCK_FLAG_SAFE_ENV_CLEAR;
    }

    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, index)
  }

  pub fn inst_ir_cmd_ir_ops(&mut self, cmd: IrCmd, ops: &IrOps) -> IrOp {
    let index = self.function.instructions.len() as u32;
    self.function.instructions.push(IrInst {
      cmd,
      ops: ops.clone(),
      last_use: 0,
      use_count: 0,
      reg_x64: RegisterX64::default(),
      reg_a64: RegisterA64::default(),
      reused_reg: false,
      spilled: false,
      needs_reload: false,
    });

    CODEGEN_ASSERT!(!self.in_terminated_block);

    if is_block_terminator(cmd) {
      self.function.blocks[self.active_block_idx as usize].finish = index;
      self.in_terminated_block = true;
    }

    if can_invalidate_safe_env(cmd) {
      self.function.blocks[self.active_block_idx as usize].flags |= K_BLOCK_FLAG_SAFE_ENV_CLEAR;
    }

    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, index)
  }

  pub fn ir_builder_ir_builder(host_hooks: &HostIrHooks) -> Self {
    Self {
      host_hooks: host_hooks as *const HostIrHooks,
      in_terminated_block: false,
      interrupt_requested: false,
      active_fastcall_fallback: false,
      fastcall_fallback_return: Default::default(),
      cmd_skip_target: -1,
      function: Default::default(),
      active_block_idx: !0u32,
      inst_index_to_block: Vec::new(),
      numeric_loop_stack: Vec::new(),
      constant_map: DenseHashMap::new(ConstantKey {
        kind: IrConstKind::Tag,
        value: !0u64,
      }),
    }
  }

  pub fn is_internal_block(&self, block: IrOp) -> bool {
    let target: &IrBlock = &self.function.blocks[block.index() as usize];
    target.kind == IrBlockKind::Internal
  }

  pub fn load_and_check_tag(&mut self, loc: IrOp, tag: u8, fallback: IrOp) {
    let tag_op = self.inst_ir_cmd_ir_op(IrCmd::LoadTag, loc);
    let const_tag_op = self.const_tag(tag);
    self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_op, const_tag_op, fallback);
  }

  /// 依字节码流重建基本块边界（cpp `rebuildBytecodeBasicBlocks`）。
  ///
  /// 契约：`proto` 指向存活 `Proto`（与 C++ 参考实现一致），其 `code`/`sizecode` 描述的
  /// 指令流在本次调用期间只读存活；指令流经 [`proto_views::code`] 的安全切片视图读取。
  pub fn rebuild_bytecode_basic_blocks(&mut self, proto: &Proto) {
    let code = code(proto);

    self.inst_index_to_block.resize(code.len(), u32::MAX);

    let mut jump_targets = vec![0; code.len()];

    let mut i: usize = 0;
    while i < code.len() {
      let insn = code[i];
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);

      let target = get_jump_target(insn, i as u32);

      if target >= 0 && !is_fast_call(op) {
        jump_targets[target as usize] = 1;
      }

      i += get_op_length(op) as usize;
      debug_assert!(i <= code.len());
    }

    jump_targets[0] = 1;

    // 块边界即跳转目标位：按标志迭代（review.md §3），索引回写 `inst_index_to_block`
    for (index, &is_jump_target) in jump_targets.iter().enumerate() {
      if is_jump_target != 0 {
        let b = self.block(IrBlockKind::Bytecode);
        self.inst_index_to_block[index] = b.index();
      }
    }

    build_bytecode_blocks(&mut self.function, &jump_targets);
  }

  /// 「`const_tag(tag)` → `StoreTag(reg, tag_op)`」两步式的单点坍缩：向寄存器槽
  /// `reg` 写入类型标签 `tag`。`tag` 为 VM 运行时标签字节（本仓 LuaType 为自定义
  /// 重编号，保留调用点的原始表达式，不做枚举改写）。
  pub fn store_tag(&mut self, reg: IrOp, tag: u8) {
    let tag_op = self.const_tag(tag);
    self.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg, tag_op);
  }

  /// 单条字节码指令 → IR  lowering 分发。
  /// `code` 为 `proto.code` 只读切片、`i` 为当前指令下标；越界由切片索引 panic 兜底，
  /// 各 translate_inst_* 子程序同此契约，本身均为 safe 函数。
  pub fn translate_inst(&mut self, op: LuauOpcode, code: &[Instruction], i: i32) {
    {
      match op {
        LuauOpcode::LOP_NOP => {}
        LuauOpcode::LOP_LOADNIL => translate_inst_load_nil(self, code, i),
        LuauOpcode::LOP_LOADB => translate_inst_load_b(self, code, i),
        LuauOpcode::LOP_LOADN => translate_inst_load_n(self, code, i),
        LuauOpcode::LOP_LOADK => translate_inst_load_k(self, code, i),
        LuauOpcode::LOP_LOADKX => translate_inst_load_kx(self, code, i),
        LuauOpcode::LOP_MOVE => translate_inst_move(self, code, i),
        LuauOpcode::LOP_GETGLOBAL => translate_inst_get_global(self, code, i),
        LuauOpcode::LOP_SETGLOBAL => translate_inst_set_global(self, code, i),
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let interrupt_pc = self.const_uint(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_pc);

          let savedpc = if fflag::LuauCallFeedback.get() {
            i + get_op_length(op)
          } else {
            i + 1
          };
          let savedpc = self.const_uint(savedpc as u32);
          self.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

          let ra = self.vm_reg(luau_insn_a(code[i as usize]) as u8);
          let b = self.const_int(luau_insn_b(code[i as usize]) as i32 - 1);
          let c = self.const_int(luau_insn_c(code[i as usize]) as i32 - 1);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, ra, b, c);

          if self.active_fastcall_fallback {
            let ret = self.fastcall_fallback_return;
            self.inst_ir_cmd_ir_op(IrCmd::JUMP, ret);
            self.begin_block(ret);
            self.active_fastcall_fallback = false;
          }
        }
        LuauOpcode::LOP_RETURN => {
          let interrupt_pc = self.const_uint(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_pc);
          let ra = self.vm_reg(luau_insn_a(code[i as usize]) as u8);
          let b = self.const_int(luau_insn_b(code[i as usize]) as i32 - 1);
          self.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, ra, b);
        }
        LuauOpcode::LOP_GETTABLE => translate_inst_get_table(self, code, i),
        LuauOpcode::LOP_SETTABLE => translate_table_access(self, code, i, IrCmd::SetTable),
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_GETUDATAKS => {
          translate_inst_get_table_ks(self, code, i)
        }
        LuauOpcode::LOP_SETTABLEKS | LuauOpcode::LOP_SETUDATAKS => {
          translate_inst_set_table_ks(self, code, i)
        }
        LuauOpcode::LOP_GETTABLEN => translate_inst_get_table_n(self, code, i),
        LuauOpcode::LOP_SETTABLEN => translate_table_access_n(self, code, i, IrCmd::SetTable),
        LuauOpcode::LOP_JUMP => translate_inst_jump(self, code, i),
        LuauOpcode::LOP_JUMPBACK => translate_inst_jump_back(self, code, i),
        LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
          translate_inst_jump_if(self, code, i, op == LuauOpcode::LOP_JUMPIFNOT)
        }
        LuauOpcode::LOP_JUMPIFEQ | LuauOpcode::LOP_JUMPIFNOTEQ => {
          let not_eq = op == LuauOpcode::LOP_JUMPIFNOTEQ;
          if is_direct_compare(code, i) {
            translate_inst_jump_if_eq_shortcut(self, code, i, not_eq);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jump_if_eq(self, code, i, not_eq);
          }
        }
        LuauOpcode::LOP_JUMPIFLE => {
          translate_inst_jump_if_cond(self, code, i, IrCondition::LessEqual)
        }
        LuauOpcode::LOP_JUMPIFLT => translate_inst_jump_if_cond(self, code, i, IrCondition::Less),
        LuauOpcode::LOP_JUMPIFNOTLE => {
          translate_inst_jump_if_cond(self, code, i, IrCondition::NotLessEqual)
        }
        LuauOpcode::LOP_JUMPIFNOTLT => {
          translate_inst_jump_if_cond(self, code, i, IrCondition::NotLess)
        }
        LuauOpcode::LOP_JUMPX => translate_inst_jump_x(self, code, i),
        LuauOpcode::LOP_JUMPXEQKNIL
        | LuauOpcode::LOP_JUMPXEQKB
        | LuauOpcode::LOP_JUMPXEQKN
        | LuauOpcode::LOP_JUMPXEQKS => {
          if is_direct_compare(code, i) {
            translate_inst_jumpx_eq_shortcut(self, code, i, op);
            self.cmd_skip_target = i + 3;
          } else {
            translate_inst_jumpx_eq(self, code, i, op);
          }
        }
        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_IDIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW => translate_inst_binary(self, code, i, op_to_tm(op)),
        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_IDIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK => translate_inst_binary_k(self, code, i, op_to_tm(op)),
        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          translate_inst_binary_rk(self, code, i, op_to_tm(op))
        }
        LuauOpcode::LOP_NOT => translate_inst_not(self, code, i),
        LuauOpcode::LOP_MINUS => translate_inst_minus(self, code, i),
        LuauOpcode::LOP_LENGTH => translate_inst_length(self, code, i),
        LuauOpcode::LOP_NEWTABLE => translate_inst_new_table(self, code, i),
        LuauOpcode::LOP_DUPTABLE => translate_inst_dup_table(self, code, i),
        LuauOpcode::LOP_SETLIST => {
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(luau_insn_a(code[i as usize]) as u8);
          let rb = self.vm_reg(luau_insn_b(code[i as usize]) as u8);
          let c = self.const_int(luau_insn_c(code[i as usize]) as i32 - 1);
          let aux = self.const_uint(code[i as usize + 1]);
          let undef = self.undef();
          self.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::SETLIST,
            pcpos,
            ra,
            rb,
            c,
            aux,
            undef,
          );
        }
        LuauOpcode::LOP_GETUPVAL => translate_inst_get_upval(self, code, i),
        LuauOpcode::LOP_SETUPVAL => translate_inst_set_upval(self, code, i),
        LuauOpcode::LOP_CLOSEUPVALS => translate_inst_close_upvals(self, code, i),
        LuauOpcode::LOP_FASTCALL
        | LuauOpcode::LOP_FASTCALL1
        | LuauOpcode::LOP_FASTCALL2
        | LuauOpcode::LOP_FASTCALL2K
        | LuauOpcode::LOP_FASTCALL3 => {
          let undef = self.undef();
          let (valid, nparams, arg2, arg3) = match op {
            LuauOpcode::LOP_FASTCALL => (false, 0, undef, undef),
            LuauOpcode::LOP_FASTCALL1 => (true, 1, undef, undef),
            LuauOpcode::LOP_FASTCALL2 => (true, 2, self.vm_reg(code[i as usize + 1] as u8), undef),
            LuauOpcode::LOP_FASTCALL2K => (true, 2, self.vm_const(code[i as usize + 1]), undef),
            _ => {
              let aux = code[i as usize + 1];
              (
                true,
                3,
                self.vm_reg((aux & 0xff) as u8),
                self.vm_reg(((aux >> 8) & 0xff) as u8),
              )
            }
          };
          let fallback = translate_fast_call_n(self, code, i, valid, nparams, arg2, arg3);
          self.handle_fastcall_fallback(fallback, code, i);
        }
        LuauOpcode::LOP_FORNPREP => translate_inst_for_n_prep(self, code, i),
        LuauOpcode::LOP_FORNLOOP => translate_inst_for_n_loop(self, code, i),
        LuauOpcode::LOP_FORGLOOP => {
          let aux = code[i as usize + 1] as i32;
          if aux < 0 {
            translate_inst_for_g_loop_ipairs(self, code, i);
          } else {
            let ra = luau_insn_a(code[i as usize]) as u8;
            let loop_repeat = self.block_at_inst((i + 1 + luau_insn_d(code[i as usize])) as u32);
            let loop_exit =
              self.block_at_inst((i + get_op_length(LuauOpcode::LOP_FORGLOOP)) as u32);
            let fallback = self.fallback_block(i as u32);

            let pcpos = self.const_uint(i as u32);
            self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);

            if LuauBackedgeHeapCheck.get() {
              self.inst_ir_cmd(IrCmd::CheckGc);
            }

            let reg_ra = self.vm_reg(ra);
            self.load_and_check_tag(reg_ra, LuaType::Nil as u8, fallback);

            let reg_ra = self.vm_reg(ra);
            let aux_op = self.const_int(aux);
            self.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
              IrCmd::FORGLOOP,
              reg_ra,
              aux_op,
              loop_repeat,
              loop_exit,
            );

            self.begin_block(fallback);
            let savedpc = self.const_uint((i + 1) as u32);
            self.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);
            let reg_ra = self.vm_reg(ra);
            let aux_op = self.const_int(aux);
            self.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
              IrCmd::ForgloopFallback,
              reg_ra,
              aux_op,
              loop_repeat,
              loop_exit,
            );

            self.begin_block(loop_exit);
          }
        }
        LuauOpcode::LOP_FORGPREP_NEXT => translate_inst_for_g_prep_next(self, code, i),
        LuauOpcode::LOP_FORGPREP_INEXT => translate_inst_for_g_prep_inext(self, code, i),
        LuauOpcode::LOP_AND | LuauOpcode::LOP_ANDK | LuauOpcode::LOP_OR | LuauOpcode::LOP_ORK => {
          let is_and = matches!(op, LuauOpcode::LOP_AND | LuauOpcode::LOP_ANDK);
          let raw_c = luau_insn_c(code[i as usize]);
          let c = if matches!(op, LuauOpcode::LOP_AND | LuauOpcode::LOP_OR) {
            self.vm_reg(raw_c as u8)
          } else {
            self.vm_const(raw_c as u32)
          };
          translate_inst_and_or_x(self, code, i, c, is_and);
        }
        LuauOpcode::LOP_COVERAGE => {
          let pcpos = self.const_uint(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::COVERAGE, pcpos);
        }
        LuauOpcode::LOP_GETIMPORT => translate_inst_get_import(self, code, i),
        LuauOpcode::LOP_CONCAT => translate_inst_concat(self, code, i),
        LuauOpcode::LOP_CAPTURE => translate_inst_capture(self, code, i),
        LuauOpcode::LOP_NAMECALL | LuauOpcode::LOP_NAMECALLUDATA => {
          if translate_inst_namecall(self, code, i) {
            if fflag::LuauCallFeedback.get() {
              let namecall = get_op_length(LuauOpcode::LOP_NAMECALL);
              let call_op =
                LuauOpcode::from(luau_insn_op(code[i as usize + namecall as usize]) as u8);
              CODEGEN_ASSERT!(matches!(
                call_op,
                LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB
              ));
              let call = get_op_length(call_op);
              self.cmd_skip_target = i + namecall + call;
            } else {
              self.cmd_skip_target = i + 3;
            }
          }
        }
        LuauOpcode::LOP_PREPVARARGS => {
          let pcpos = self.const_uint(i as u32);
          let a = self.const_int(luau_insn_a(code[i as usize]) as i32);
          self.inst_ir_cmd_ir_op_ir_op(IrCmd::FallbackPrepvarargs, pcpos, a);
        }
        LuauOpcode::LOP_GETVARARGS => {
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(luau_insn_a(code[i as usize]) as u8);
          let b = self.const_int(luau_insn_b(code[i as usize]) as i32 - 1);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackGetvarargs, pcpos, ra, b);
        }
        LuauOpcode::LOP_NEWCLOSURE => translate_inst_new_closure(self, code, i),
        LuauOpcode::LOP_DUPCLOSURE => {
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(luau_insn_a(code[i as usize]) as u8);
          let kd = self.vm_const(luau_insn_d(code[i as usize]) as u32);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackDupclosure, pcpos, ra, kd);
        }
        LuauOpcode::LOP_FORGPREP => {
          let loop_start = self.block_at_inst((i + 1 + luau_insn_d(code[i as usize])) as u32);
          let pcpos = self.const_uint(i as u32);
          let ra = self.vm_reg(luau_insn_a(code[i as usize]) as u8);
          self.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FallbackForgprep, pcpos, ra, loop_start);
        }
        // cpp IrBuilder.cpp:677-682：NCG 不支持 class，NEWCLASS/NEWCLASSMEMBER
        // 一律发射 vmExit 退回 VM 执行，否则 release 下静默跳指令=误编译。
        LuauOpcode::LOP_NEWCLASSMEMBER | LuauOpcode::LOP_NEWCLASS => {
          let exit = self.vm_exit(i as u32);
          self.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);
        }
        // cpp IrBuilder.cpp:688-693：本仓无 IrCmd::InvokeFastpcall，等价于上游
        // LuauCodeGenFastpcall 关闭时的优雅回退（跳到下一块并开新块）。
        LuauOpcode::LOP_FASTPCALL => {
          let next = self.block_at_inst((i + get_op_length(op)) as u32);
          self.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
          self.begin_block(next);
        }
        LuauOpcode::LOP_CMPPROTO => translate_inst_cmp_proto(self, code, i),
        _ => CODEGEN_ASSERT!(false),
      }
    }
  }

  pub fn undef(&mut self) -> IrOp {
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0)
  }

  pub fn vm_const(&mut self, index: u32) -> IrOp {
    IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmConst, index)
  }

  pub fn vm_exit(&mut self, pcpos: u32) -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::VmExit, pcpos)
  }

  pub fn vm_reg(&mut self, index: u8) -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::VmReg, index as u32)
  }

  pub fn vm_upvalue(&mut self, index: u8) -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::VmUpvalue, index as u32)
  }

  /// function 分离借用：把 `self.function` 临时移出 builder 交闭包独占修改，
  /// 返回后放回原位。调用侧因此不再需要 `*mut IrFunction` 裸指针绕开
  /// `&mut build` 与 `&mut build.function` 的借用冲突。
  ///
  /// 约束：闭包内不得调用任何需要 `&mut self` 的 builder 方法（function 此时
  /// 已被移出 builder）；builder 的发射方法（block/begin_block/inst_*）只能
  /// 在闭包外调用。移出/放回仅移动结构体本身，无堆分配。
  pub(crate) fn with_function<R>(&mut self, f: impl FnOnce(&mut IrFunction) -> R) -> R {
    let mut function = take(&mut self.function);
    let result = f(&mut function);
    self.function = function;
    result
  }
}

const fn op_to_tm(op: LuauOpcode) -> TMS {
  match op {
    LuauOpcode::LOP_ADD | LuauOpcode::LOP_ADDK => TMS::TmAdd,
    LuauOpcode::LOP_SUB | LuauOpcode::LOP_SUBK | LuauOpcode::LOP_SUBRK => TMS::TmSub,
    LuauOpcode::LOP_MUL | LuauOpcode::LOP_MULK => TMS::TmMul,
    LuauOpcode::LOP_DIV | LuauOpcode::LOP_DIVK | LuauOpcode::LOP_DIVRK => TMS::TmDiv,
    LuauOpcode::LOP_IDIV | LuauOpcode::LOP_IDIVK => TMS::TmIDiv,
    LuauOpcode::LOP_MOD | LuauOpcode::LOP_MODK => TMS::TmMod,
    LuauOpcode::LOP_POW | LuauOpcode::LOP_POWK => TMS::TmPow,
    _ => TMS::TmAdd,
  }
}
