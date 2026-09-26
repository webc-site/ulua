use crate::{
  enums::{abix_64::ABIX64, category_x_64::CategoryX64, ir_op_kind::IrOpKind, size_x_64::SizeX64},
  functions::same_underlying_register::same_underlying_register,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, call_argument::CallArgument,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64,
    operand_x_64::OperandX64, register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
  },
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrCallWrapperX64 {
  pub regs: *mut IrRegAllocX64,
  pub build: *mut AssemblyBuilderX64,
  pub inst_idx: u32,
  pub(crate) args: [CallArgument; 6],
  pub(crate) arg_count: i32,
  pub(crate) gpr_pos: i32,
  pub(crate) xmm_pos: i32,
  pub(crate) func_op: OperandX64,
  pub(crate) result_reg: RegisterX64,
  pub(crate) result_inst_idx: u32,
  pub(crate) gpr_uses: [u8; 16],
  pub(crate) xmm_uses: [u8; 16],
}

impl IrCallWrapperX64 {
  pub(crate) const K_MAX_CALL_ARGUMENTS: i32 = 6;

  /// Safety:`regs` 由调用方以 `&mut IrRegAllocX64` 传入后存为裸指针，所指对象
  /// 生命周期覆盖本包装器的使用期，且与本对象（调用侧栈上临时）无内存重叠。
  /// 访问器即时派生唯一借用、语句内消费，不长期持有，避免与内部同样派生
  /// `&mut` 的路径（如 `remove_register_use`）产生别名假设冲突。
  #[inline]
  pub(crate) fn regs(&mut self) -> &mut IrRegAllocX64 {
    // Safety:见函数注释。
    unsafe { &mut *self.regs }
  }

  /// Safety:同 `regs`，`build` 指向调用方保证有效的 `AssemblyBuilderX64`。
  #[inline]
  pub(crate) fn build(&mut self) -> &mut AssemblyBuilderX64 {
    // Safety:见函数注释。
    unsafe { &mut *self.build }
  }

  pub fn add_argument_size_x_64_operand_x_64_ir_op(
    &mut self,
    target_size: SizeX64,
    source: OperandX64,
    source_op: IrOp,
  ) {
    // 指令操作数依赖当前指令索引做生命周期追踪
    CODEGEN_ASSERT!(self.inst_idx != K_INVALID_INST_IDX || source_op.kind() == IrOpKind::None);

    CODEGEN_ASSERT!(self.arg_count < Self::K_MAX_CALL_ARGUMENTS);

    let target = self.get_next_argument_target(target_size);

    let idx = self.arg_count as usize;
    self.arg_count += 1;

    self.args[idx] = CallArgument {
      target_size,
      source,
      source_op,
      target,
      candidate: true,
    };

    // Safety: self.build 为构造接线的非空、比持有者长寿的 AssemblyBuilderX64*（arena 不变量），
    // (*self.build).abi 是其存活对象上的字段只读，无别名，读结果仅用于选择 ABI 计数分支。
    if unsafe { (*self.build).abi } == ABIX64::WINDOWS {
      // Windows 上 gpr/xmm 寄存器位置同步移动
      self.gpr_pos += 1;
      self.xmm_pos += 1;
    } else if target_size == SizeX64::Xmmword {
      self.xmm_pos += 1;
    } else {
      self.gpr_pos += 1;
    }
  }

  pub fn add_argument_size_x_64_scoped_reg_x_64(
    &mut self,
    target_size: SizeX64,
    scoped_reg: &mut ScopedRegX64,
  ) {
    let source = scoped_reg.release();
    self.add_argument_size_x_64_operand_x_64_ir_op(
      target_size,
      OperandX64::from(source),
      IrOp::new(),
    );
  }

  pub fn add_register_use(&mut self, reg: RegisterX64) {
    if reg.size() == SizeX64::Xmmword {
      self.xmm_uses[reg.index() as usize] += 1;
    } else if reg.size() != SizeX64::None {
      self.gpr_uses[reg.index() as usize] += 1;
    }
  }

  pub fn call(&mut self, func: &OperandX64) {
    self.func_op = *func;

    // 先释放结果寄存器，保证参数处理期间没有任何存活值从中保留
    if self.result_reg != RegisterX64::NOREG {
      let result_reg = self.result_reg;
      self.regs().free_reg(result_reg);
    }

    self.count_register_uses();

    // 以下标遍历 args：循环体内调用 &mut self 方法，迭代器借用会与之冲突；
    // arg 字段全为 Copy，即取即释，无借用残留。
    for i in 0..self.arg_count as usize {
      let source_op = self.args[i].source_op;

      if source_op.kind() != IrOpKind::None {
        // 阶段一（共享借用）：判定最后一次使用并读出当前寄存器
        let inst_idx = self.inst_idx;
        let (is_last_use, reg) = {
          let regs = self.regs();
          match regs.function_ref().as_inst_op_ref(source_op) {
            Some(inst) => (regs.is_last_use_reg(inst, inst_idx), inst.reg_x64),
            None => (false, RegisterX64::NOREG),
          }
        };

        if is_last_use {
          // 阶段二（可变借用）：source 是该指令最后一次使用，清除操作数上的寄存器
          if let Some(inst) = self.regs().function_mut().as_inst_op_mut(source_op) {
            inst.reg_x64 = RegisterX64::NOREG;
          }
        } else if reg.size() == SizeX64::Xmmword || self.regs().should_free_gpr(reg) {
          // 非最后一次使用且寄存器为易失寄存器时，取走所有权（顺带溢出该操作数）
          self.regs().take_reg(reg, K_INVALID_INST_IDX);
        }
      }

      let source = self.args[i].source;
      let target = self.args[i].target;

      // 立即数不与其他参数干涉，统一挪到最后处理
      if source.cat == CategoryX64::Imm {
        self.args[i].candidate = false;
      }
      // 经栈传递的参数可立即处理
      else if target.cat == CategoryX64::Mem {
        if source.cat == CategoryX64::Mem {
          // cpp `ScopedRegX64 tmp(regs, size);`：构造即分配
          let mut tmp = ScopedRegX64::with_size(self.regs(), target.mem_size);

          self.free_source_registers(source);

          if source.mem_size == SizeX64::None {
            self
              .build()
              .lea_operand_x_64_operand_x_64(OperandX64::reg(tmp.reg), source);
          } else {
            self.build().mov(OperandX64::reg(tmp.reg), source);
          }

          self.build().mov(target, OperandX64::reg(tmp.reg));

          tmp.free();
        } else {
          self.free_source_registers(source);

          self.build().mov(target, source);
        }

        self.args[i].candidate = false;
      }
      // 已在目标位置的参数跳过
      else if source.cat == CategoryX64::Reg && same_underlying_register(target.base, source.base)
      {
        self.free_source_registers(source);

        // 若目标寄存器未被其他参数用作 source，禁止寄存器分配器将其分出
        if self.get_register_uses(target.base) == 0 {
          self.regs().take_reg(target.base, K_INVALID_INST_IDX);
        } else {
          // 否则确保最后一次 source 使用完成时不会释放它
          self.add_register_use(target.base);
        }

        self.args[i].candidate = false;
      }
    }

    // 循环直到所有参数就位
    loop {
      // 找一个目标寄存器不与现存 source 冲突的候选参数
      match ir_call_wrapper_x_64_find_non_interfering_argument(self) {
        Some(idx) => {
          // 本分支只处理寄存器目标
          CODEGEN_ASSERT!(self.args[idx].target.cat == CategoryX64::Reg);

          self.free_source_registers(self.args[idx].source);

          let target_base = self.args[idx].target.base;
          CODEGEN_ASSERT!(self.get_register_uses(target_base) == 0);
          self.regs().take_reg(target_base, K_INVALID_INST_IDX);

          self.move_to_target(self.args[idx]);

          self.args[idx].candidate = false;
        }
        None => {
          // 全部寄存器交叉干涉（如 rcx<-rdx, rdx<-rcx）时，须改名其中一个
          let conflict = self.find_conflicting_target();
          if conflict != RegisterX64::NOREG {
            self.rename_conflicting_register(conflict);
          } else {
            for item in &self.args[..self.arg_count as usize] {
              CODEGEN_ASSERT!(!item.candidate);
            }
            break;
          }
        }
      }
    }

    // 最后处理立即数参数
    for i in 0..self.arg_count as usize {
      if self.args[i].source.cat == CategoryX64::Imm {
        // 可能与函数地址的 source 寄存器冲突，先标记为候选以侦测
        self.args[i].candidate = true;

        let conflict = self.find_conflicting_target();
        if conflict != RegisterX64::NOREG {
          self.rename_conflicting_register(conflict);
        }

        let target = self.args[i].target;
        if target.cat == CategoryX64::Reg {
          self.regs().take_reg(target.base, K_INVALID_INST_IDX);
        }

        self.move_to_target(self.args[i]);

        self.args[i].candidate = false;
      }
    }

    // 释放函数调用使用的寄存器
    self.remove_register_use(self.func_op.base);
    self.remove_register_use(self.func_op.index);

    // 调用发生前，参数寄存器在分配器中全部标记为空闲
    for i in 0..self.arg_count as usize {
      let target = self.args[i].target;
      if target.cat == CategoryX64::Reg {
        self.regs().free_reg(target.base);
      }
    }

    self.regs().preserve_and_free_inst_values();
    self.regs().assert_all_free();

    let func_op = self.func_op;
    self.build().call_operand_x_64(func_op);

    if self.result_reg != RegisterX64::NOREG {
      // 结果寄存器在调用前分配、调用前临时释放，此处收回
      let (result_reg, result_inst_idx) = (self.result_reg, self.result_inst_idx);
      self.regs().take_reg(result_reg, result_inst_idx);

      // 跳过向 eax/rax/xmm0 结果寄存器的搬运
      if self.result_reg.index() != 0 {
        let return_reg = RegisterX64 {
          bits: self.result_reg.size() as u8,
        };
        let result_reg = self.result_reg;
        self
          .build()
          .mov(OperandX64::reg(result_reg), OperandX64::reg(return_reg));
      }
    }
  }

  pub fn count_register_uses(&mut self) {
    // 快照各参数的寄存器字段（RegisterX64 是 Copy），规避与 add_register_use 可变借用冲突，去除原 unsafe
    for i in 0..self.arg_count as usize {
      let (base, index) = (self.args[i].source.base, self.args[i].source.index);
      self.add_register_use(base);
      self.add_register_use(index);
    }

    self.add_register_use(self.func_op.base);
    self.add_register_use(self.func_op.index);
  }

  pub fn find_conflicting_target(&self) -> RegisterX64 {
    // 全部只读借用（&self 方法），迭代器替代索引遍历
    for arg in &self.args[..self.arg_count as usize] {
      if arg.candidate {
        if self.interferes_with_active_target(arg.source.base) {
          return arg.source.base;
        }

        if self.interferes_with_active_target(arg.source.index) {
          return arg.source.index;
        }
      }
    }

    if self.interferes_with_active_target(self.func_op.base) {
      return self.func_op.base;
    }

    if self.interferes_with_active_target(self.func_op.index) {
      return self.func_op.index;
    }

    RegisterX64::NOREG
  }

  /// 释放参数 source 操作数占用的寄存器（base 与 index）。
  /// 按值取 `OperandX64`（Copy），调用侧即可与 `&mut self` 方法自然共存。
  pub fn free_source_registers(&mut self, source: OperandX64) {
    self.remove_register_use(source.base);
    self.remove_register_use(source.index);
  }

  pub fn get_next_argument_target(&self, size: SizeX64) -> OperandX64 {
    if size == SizeX64::Xmmword {
      CODEGEN_ASSERT!((self.xmm_pos as usize) < K_XMM_ORDER.len());
      return K_XMM_ORDER[self.xmm_pos as usize];
    }

    // Safety: self.build 为构造接线的非空 AssemblyBuilderX64*（比持有者长寿），(*self.build).abi
    // 是该存活对象上 u32/enum 字段的只读访问，无别名风险。
    let gpr_order = if unsafe { (*self.build).abi } == ABIX64::WINDOWS {
      &K_WINDOWS_GPR_ORDER
    } else {
      &K_SYSTEMV_GPR_ORDER
    };

    CODEGEN_ASSERT!((self.gpr_pos as usize) < gpr_order.len());
    let mut target = gpr_order[self.gpr_pos as usize];

    // 保留请求的参数尺寸
    if target.cat == CategoryX64::Reg {
      target.base = RegisterX64 {
        bits: (target.base.bits & RegisterX64::INDEX_MASK) | size as u8,
      };
    } else if target.cat == CategoryX64::Mem {
      target.mem_size = size;
    }

    target
  }

  pub fn get_register_uses(&self, reg: RegisterX64) -> i32 {
    if reg.size() == SizeX64::Xmmword {
      self.xmm_uses[reg.index() as usize] as i32
    } else if reg.size() != SizeX64::None {
      self.gpr_uses[reg.index() as usize] as i32
    } else {
      0
    }
  }

  #[inline]
  pub fn interferes_with_active_sources(
    &self,
    target_arg: &CallArgument,
    target_arg_index: i32,
  ) -> bool {
    // 跳过自身、命中即短路
    self
      .args
      .iter()
      .take(self.arg_count as usize)
      .enumerate()
      .any(|(i, arg)| {
        arg.candidate
          && i as i32 != target_arg_index
          && self.interferes_with_operand(&arg.source, target_arg.target.base)
      })
  }

  pub fn interferes_with_active_target(&self, source_reg: RegisterX64) -> bool {
    // 命中即短路
    self
      .args
      .iter()
      .take(self.arg_count as usize)
      .any(|arg| arg.candidate && same_underlying_register(arg.target.base, source_reg))
  }

  #[inline]
  pub fn interferes_with_operand(&self, op: &OperandX64, reg: RegisterX64) -> bool {
    same_underlying_register(op.base, reg) || same_underlying_register(op.index, reg)
  }

  pub fn ir_call_wrapper_x_64_ir_call_wrapper_x_64(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    inst_idx: u32,
  ) -> Self {
    let mut wrapper = Self {
      regs: regs as *mut IrRegAllocX64,
      build: build as *mut AssemblyBuilderX64,
      inst_idx,
      args: [
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
        CallArgument::default(),
      ],
      arg_count: 0,
      gpr_pos: 0,
      xmm_pos: 0,
      func_op: OperandX64::operand_x_64_register_x_64(RegisterX64::NOREG),
      result_reg: RegisterX64::NOREG,
      result_inst_idx: 0,
      gpr_uses: [0u8; 16],
      xmm_uses: [0u8; 16],
    };

    wrapper.gpr_uses.fill(0);
    wrapper.xmm_uses.fill(0);

    wrapper
  }

  /// 将参数值搬运到目标位置（寄存器/栈槽）。
  /// 按值取 `CallArgument`（Copy，字段全为值类型），调用侧可直接传 `self.args[i]`，
  /// 消除原裸指针再借用样板。
  pub fn move_to_target(&mut self, arg: CallArgument) {
    let source_cat = arg.source.cat;
    if source_cat == CategoryX64::Reg {
      let source = arg.source.base;

      if source.size() == SizeX64::Xmmword {
        self.build().vmovsd_operand_x_64_operand_x_64_operand_x_64(
          arg.target,
          source.into(),
          source.into(),
        );
      } else {
        self.build().mov(arg.target, source.into());
      }
    } else if source_cat == CategoryX64::Imm {
      self.build().mov(arg.target, arg.source);
    } else if arg.source.mem_size == SizeX64::None {
      self
        .build()
        .lea_operand_x_64_operand_x_64(arg.target, arg.source);
    } else if arg.target.base.size() == SizeX64::Xmmword && arg.source.mem_size == SizeX64::Xmmword
    {
      self.build().vmovups(arg.target, arg.source);
    } else if arg.target.base.size() == SizeX64::Xmmword {
      self
        .build()
        .vmovsd_operand_x_64_operand_x_64(arg.target, arg.source);
    } else {
      self.build().mov(arg.target, arg.source);
    }
  }

  pub fn remove_register_use(&mut self, reg: RegisterX64) {
    if reg.size() == SizeX64::Xmmword {
      CODEGEN_ASSERT!(self.xmm_uses[reg.index() as usize] != 0);
      self.xmm_uses[reg.index() as usize] -= 1;

      if self.xmm_uses[reg.index() as usize] == 0 {
        self.regs().free_reg(reg);
      }
    } else if reg.size() != SizeX64::None {
      CODEGEN_ASSERT!(self.gpr_uses[reg.index() as usize] != 0);
      self.gpr_uses[reg.index() as usize] -= 1;

      if self.gpr_uses[reg.index() as usize] == 0 && self.regs().should_free_gpr(reg) {
        self.regs().free_reg(reg);
      }
    }
  }

  pub fn rename_conflicting_register(&mut self, conflict: RegisterX64) {
    // 取一个新寄存器
    let fresh_reg = self.regs().alloc_reg(conflict.size(), K_INVALID_INST_IDX);

    if conflict.size() == SizeX64::Xmmword {
      self.build().vmovsd_operand_x_64_operand_x_64_operand_x_64(
        OperandX64::reg(fresh_reg),
        OperandX64::reg(conflict),
        OperandX64::reg(conflict),
      );
    } else {
      self
        .build()
        .mov(OperandX64::reg(fresh_reg), OperandX64::reg(conflict));
    }

    self.rename_source_registers(conflict, fresh_reg);
  }

  pub fn rename_register(
    &mut self,
    target: &mut RegisterX64,
    reg: RegisterX64,
    replacement: RegisterX64,
  ) {
    if same_underlying_register(*target, reg) {
      self.add_register_use(replacement);
      self.remove_register_use(*target);

      // 只改 index，size 保持不变
      let replacement_index = replacement.index();
      *target = RegisterX64 {
        bits: (target.bits & RegisterX64::SIZE_MASK)
          | (replacement_index << RegisterX64::INDEX_SHIFT),
      };
    }
  }

  pub fn rename_source_registers(&mut self, reg: RegisterX64, replacement: RegisterX64) {
    // RegisterX64 是 Copy：先取出、重命名、再写回，等价 cpp 的原地重命名，
    // 且不再需要裸指针转手来规避与 &mut self 的借用冲突。
    for i in 0..(self.arg_count as usize) {
      if self.args[i].candidate {
        let mut base = self.args[i].source.base;
        let mut index = self.args[i].source.index;

        self.rename_register(&mut base, reg, replacement);
        self.rename_register(&mut index, reg, replacement);

        self.args[i].source.base = base;
        self.args[i].source.index = index;
      }
    }

    let mut func_base = self.func_op.base;
    let mut func_index = self.func_op.index;

    self.rename_register(&mut func_base, reg, replacement);
    self.rename_register(&mut func_index, reg, replacement);

    self.func_op.base = func_base;
    self.func_op.index = func_index;
  }

  pub fn set_result_register(&mut self, reg: RegisterX64, inst_idx: u32) {
    CODEGEN_ASSERT!(!reg.register_x_64_operator_eq(RegisterX64::NOREG));

    self.result_reg = reg;
    self.result_inst_idx = inst_idx;
  }

  // C++: `static RegisterX64 suggestArgumentRegister(SizeX64 size, AssemblyBuilderX64& build);`
  // （静态模板；实例化 N = 0..3）。不触碰实例状态，因此在 Rust 中
  // 也是关联函数——调用方写成
  // `IrCallWrapperX64::suggest_argument_register::<N>(size, build)`。
  pub fn suggest_argument_register<const N: usize>(
    size: SizeX64,
    build: &mut AssemblyBuilderX64,
  ) -> RegisterX64 {
    // static_assert(N <= 3, "Argument index must be 0-3 (Windows passes args 4+ on the stack)");
    const { assert!(N <= 3) };

    if size == SizeX64::Xmmword {
      return K_XMM_ORDER[N].base;
    }

    let gpr_order = if build.abi == ABIX64::WINDOWS {
      &K_WINDOWS_GPR_ORDER
    } else {
      &K_SYSTEMV_GPR_ORDER
    };

    let mut target = gpr_order[N];
    CODEGEN_ASSERT!(target.cat == CategoryX64::Reg);

    target.base = RegisterX64 {
      bits: (target.base.bits & RegisterX64::INDEX_MASK) | size as u8,
    };
    target.base
  }

  pub fn suggest_next_argument_register(&self, size: SizeX64) -> RegisterX64 {
    let target = self.get_next_argument_target(size);
    // Safety: self.regs 是构造接线的非空分配器裸指针（比持有者长寿、内部可变性）；&mut *self.regs
    // 重建可变借用——lowering 为单线程串行，同一时刻仅此一处使用该寄存器分配器，无并存 &/&mut
    // 别名，故独占成立（延续本 crate 既有的裸指针字段可变性不变量，非 mut_from_ref）。
    let regs = unsafe { &mut *self.regs };

    if target.cat != CategoryX64::Reg {
      return regs.alloc_reg(size, K_INVALID_INST_IDX);
    }

    if !regs.can_take_reg(target.base) {
      return regs.alloc_reg(size, K_INVALID_INST_IDX);
    }

    regs.take_reg(target.base, K_INVALID_INST_IDX)
  }
}

/// 找到一个目标寄存器不与现存 source 冲突的候选参数，返回其在 `args` 中的下标；
/// 无候选时返回 `None`。
pub fn ir_call_wrapper_x_64_find_non_interfering_argument(
  receiver: &IrCallWrapperX64,
) -> Option<usize> {
  (0..receiver.arg_count as usize).find(|&i| {
    let arg = &receiver.args[i];
    arg.candidate
      && !receiver.interferes_with_active_sources(arg, i as i32)
      && !receiver.interferes_with_operand(&receiver.func_op, arg.target.base)
  })
}

// Windows ABI 下 callee 可用的寄存器 'home' 位置字节数。
const K_STACK_REG_HOME_STORAGE: i32 = 4 * 8;

// static const std::array<OperandX64, 6> kWindowsGprOrder
pub(crate) const K_WINDOWS_GPR_ORDER: [OperandX64; 6] = [
  OperandX64::reg(RegisterX64::RCX),
  OperandX64::reg(RegisterX64::RDX),
  OperandX64::reg(RegisterX64::R8),
  OperandX64::reg(RegisterX64::R9),
  OperandX64::mem(
    SizeX64::None,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_REG_HOME_STORAGE,
  ),
  OperandX64::mem(
    SizeX64::None,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_REG_HOME_STORAGE + 8,
  ),
];

// static const std::array<OperandX64, 6> kSystemvGprOrder
pub(crate) const K_SYSTEMV_GPR_ORDER: [OperandX64; 6] = [
  OperandX64::reg(RegisterX64::RDI),
  OperandX64::reg(RegisterX64::RSI),
  OperandX64::reg(RegisterX64::RDX),
  OperandX64::reg(RegisterX64::RCX),
  OperandX64::reg(RegisterX64::R8),
  OperandX64::reg(RegisterX64::R9),
];

// static const std::array<OperandX64, 4> kXmmOrder
pub(crate) const K_XMM_ORDER: [OperandX64; 4] = [
  OperandX64::reg(RegisterX64::XMM0),
  OperandX64::reg(RegisterX64::XMM1),
  OperandX64::reg(RegisterX64::XMM2),
  OperandX64::reg(RegisterX64::XMM3),
];
