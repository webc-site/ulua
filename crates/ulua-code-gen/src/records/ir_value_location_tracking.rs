use core::{ffi::c_void, ptr::null_mut};

use crate::{
  enums::{ir::IrValueKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    get_cmd_value_kind::get_cmd_value_kind, ir::vm_reg_op, regalloc::x_64::reg_bit_test,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    ir_operand::{op_a, op_a_ref, op_b_ref, op_c_ref, op_d_ref, op_g_ref},
  },
  records::{
    ir_data::K_INVALID_INST_IDX,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    vm_exit::{StoreLocationHint, ValueRestoreLocation},
  },
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrValueLocationTracking {
  pub function: *mut IrFunction,
  pub vm_reg_value: [u32; 256],
  pub vm_reg_dependent: [u32; 256],
  pub max_reg: i32,
  /// DELIBERATE DEVIATION / 保留理由：`restore_callback` 的配套 opaque 上下文，
  /// 与回调指针构成一对「类型擦除的函数 + user-data」，不收 `Option<NonNull<T>>`：
  /// 消费点在本文件 `invalidate_inst_restore_location` 内
  /// `callback(self.restore_callback_ctx, inst)`——`callback` 即
  /// `restore_callback: Option<unsafe fn(*mut c_void, *mut IrInst)>` 字段；
  /// 注册点经 `set_restore_callback`（本文件，`self.restore_callback_ctx = context`）
  /// 来自两个不同具体类型：`IrLoweringX64::reset_restore_callback` 传
  /// `ptr::from_mut(&mut self.regs).cast::<c_void>()`（shim 内
  /// `context.cast::<IrRegAllocX64>()` 还原），`IrLoweringA64::setup_restore_callback`
  /// 传 `ptr::from_mut(self).cast::<c_void>()`（shim 内
  /// `context.cast::<IrLoweringA64>()` 还原）。字段一旦被单一 `T` 钉住，跟踪器就得
  /// 对 A64/X64 两套 lowering 泛型化（其宿主 `IrLowering*` 结构本身尚未泛型），
  /// 那是独立一轮结构改写，不在本项范围。
  /// `null_mut()` 初值（`new`）对应 cpp `IrValueLocationTracking` 的
  /// `restoreCallbackContext` 零初值：回调槽 `None` 时该值无任何消费方
  /// （`if let Some(callback) = self.restore_callback` 短路）。
  pub restore_callback_ctx: *mut c_void,
  // 恢复回调：设置方/调用方均在本 crate 内（Rust↔Rust），用 Rust ABI fn 指针
  /// # Safety
  ///
  /// 以 `Some` 调用时：第一个实参须为配套的 `restore_callback_ctx`（且在其存活期内），
  /// 第二个实参须指向存活的 `IrInst`；被调函数不得违反二者别名/生命周期假设。
  pub restore_callback: Option<unsafe fn(*mut c_void, *mut IrInst)>,
}

impl IrValueLocationTracking {
  // —— `function` 裸指针 → 视图的单源收口（仿 ulua-code-gen `VmFrame::view` 形态）——
  //
  // 本文件所有「经 self.function 重建借用」只经下方两壳；机器语义与 ABI 论证
  // 单源一处，各方法体不再逐处重复 Safety 长注。指针按值传入（字段 Copy），
  // 派生借用不与后续 `&mut self` 方法调用冲突（与原逐语句形态一致）。

  /// 可变视图重建点（读改写 `find/record_restore_location` 等）。
  ///
  /// # Safety
  /// `ptr` 为构造点（`new`）接线、比本跟踪器长寿的非空 `*mut IrFunction`；
  /// 重建借用仅于本跟踪器所属的单线程串行 lowering 阶段按语句顺序即时消费，
  /// 目标对象与跟踪器字段不重叠，无第二活跃别名。
  #[inline]
  fn function_view<'s>(ptr: *mut IrFunction) -> &'s mut IrFunction {
    // Safety: 契约即上方 `# Safety`，由构造点 `new(&mut IrFunction)` 保证。
    unsafe { &mut *ptr }
  }

  /// [`Self::function_view`] 的只读形态。
  ///
  /// # Safety
  /// 同 [`Self::function_view`]，且仅只读访问。
  #[inline]
  fn function_view_ro<'s>(ptr: *mut IrFunction) -> &'s IrFunction {
    // Safety: 契约即上方 `# Safety`，同可变形态。
    unsafe { &*ptr }
  }

  pub fn after_inst_lowering(&mut self, inst: &mut IrInst, inst_idx: u32) {
    match inst.cmd {
      IrCmd::LoadTag
      | IrCmd::LoadPointer
      | IrCmd::LoadDouble
      | IrCmd::LoadInt
      | IrCmd::LoadInt64
      | IrCmd::LoadTvalue => {
        if op_a(inst).kind() == IrOpKind::VmReg {
          self.invalidate_restore_op(op_a(inst), false);
        }

        self.record_restore_op(inst_idx, op_a(inst));
      }

      IrCmd::StorePointer
      | IrCmd::StoreDouble
      | IrCmd::StoreInt
      | IrCmd::StoreInt64
      | IrCmd::StoreTvalue => {
        let source_op = op_b_ref(inst);

        if source_op.kind() == IrOpKind::Inst {
          // 视图重建与安全论证单源于 `function_view`。
          let function = Self::function_view(self.function);
          let can_remat_args = can_rematerialize_arguments_at(function, source_op.index());
          let source = &function.instructions[source_op.index() as usize];

          if source.last_use != inst_idx || can_remat_args {
            self.record_restore_op(source_op.index(), op_a(inst));
          }
        }
      }

      IrCmd::StoreSplitTvalue => {
        let source_op = op_c_ref(inst);

        if source_op.kind() == IrOpKind::Inst {
          // 视图重建与安全论证单源于 `function_view`。
          let function = Self::function_view(self.function);
          let can_remat_args = can_rematerialize_arguments_at(function, source_op.index());
          let source = &function.instructions[source_op.index() as usize];

          if source.last_use != inst_idx || can_remat_args {
            self.record_restore_op(source_op.index(), op_a(inst));
          }
        }
      }

      IrCmd::NumToUint | IrCmd::NumToInt => {
        // cpp IrValueLocationTracking.cpp:277-303：正向重materialize——把整型
        // 转换结果登记为可从 Double 寄存器槽恢复（移植期开关
        // LuauCodegenForwardRematerialize 在 cpp 中已删除，此路径无条件执行）。
        let arg = op_a(inst);

        if arg.kind() == IrOpKind::Inst {
          // 视图重建与安全论证单源于 `function_view`。
          let function = Self::function_view(self.function);
          let owner_loc = function.find_restore_location_u32_bool(arg.index(), true);

          if owner_loc.op.kind() == IrOpKind::VmReg
            && owner_loc.kind == IrValueKind::Double
            && owner_loc.conversion_cmd == IrCmd::NOP
            && !owner_loc.lazy
          {
            let reg = vm_reg_op(owner_loc.op) as usize;
            let captured = reg_bit_test(&function.cfg.captured.regs, reg);

            if !captured && self.vm_reg_dependent[reg] == K_INVALID_INST_IDX {
              let forward_cmd = if inst.cmd == IrCmd::NumToUint {
                IrCmd::UintToNum
              } else {
                IrCmd::IntToNum
              };

              function.record_restore_location(
                inst_idx,
                ValueRestoreLocation {
                  op: owner_loc.op,
                  kind: IrValueKind::Double,
                  conversion_cmd: forward_cmd,
                  lazy: false,
                },
              );

              self.vm_reg_dependent[reg] = inst_idx;
            }
          }
        }
      }

      _ => {}
    }
  }

  pub fn before_inst_lowering(&mut self, inst: &mut IrInst) {
    match inst.cmd {
      IrCmd::StoreTag => self.invalidate_restore_op(op_a(inst), true),
      IrCmd::StoreExtra
      | IrCmd::StorePointer
      | IrCmd::StoreDouble
      | IrCmd::StoreInt
      | IrCmd::StoreInt64
      | IrCmd::StoreVector
      | IrCmd::StoreTvalue
      | IrCmd::StoreSplitTvalue => self.invalidate_restore_op(op_a(inst), false),

      IrCmd::AdjustStackToReg => self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)), -1),
      IrCmd::FASTCALL => {
        // 视图重建与安全论证单源于 `function_view_ro`。
        let function = Self::function_view_ro(self.function);
        self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), function.int_op(op_d_ref(inst)));
      }
      IrCmd::InvokeFastcall => {
        // 视图重建与安全论证单源于 `function_view_ro`。
        let function = Self::function_view_ro(self.function);
        self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), function.int_op(op_g_ref(inst)));
      }

      IrCmd::DoArith | IrCmd::DoLen | IrCmd::GetTable | IrCmd::GetCachedImport => {
        self.invalidate_restore_op(op_a(inst), false)
      }
      IrCmd::CONCAT => {
        // 视图重建与安全论证单源于 `function_view_ro`。
        let function = Self::function_view_ro(self.function);
        self.invalidate_restore_vm_regs(
          vm_reg_op(op_a(inst)),
          function.uint_op(op_b_ref(inst)) as i32,
        );
      }
      IrCmd::GetUpvalue => {}
      IrCmd::CALL => self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)), -1),
      // JIT setmetatable 快速通道：成功路径仅写 ra 一个寄存器
      IrCmd::SetMetatableChecked => {
        self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)), 1)
      }
      IrCmd::FORGLOOP | IrCmd::ForgloopFallback => {
        self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)) + 2, -1)
      }
      IrCmd::FallbackGetglobal | IrCmd::FallbackGettableks => {
        self.invalidate_restore_op(op_b_ref(inst), false)
      }
      IrCmd::FallbackNamecall => self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), 2),
      IrCmd::FallbackGetvarargs => {
        // 视图重建与安全论证单源于 `function_view_ro`。
        let function = Self::function_view_ro(self.function);
        self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), function.int_op(op_c_ref(inst)));
      }
      IrCmd::FallbackDupclosure => self.invalidate_restore_op(op_b_ref(inst), false),
      IrCmd::FallbackForgprep => self.invalidate_restore_vm_regs(vm_reg_op(op_b_ref(inst)), 3),

      IrCmd::LoadTag
      | IrCmd::LoadPointer
      | IrCmd::LoadDouble
      | IrCmd::LoadInt64
      | IrCmd::LoadInt
      | IrCmd::LoadFloat
      | IrCmd::LoadTvalue
      | IrCmd::CmpAny
      | IrCmd::CmpTag
      | IrCmd::JumpIfTruthy
      | IrCmd::JumpIfFalsy
      | IrCmd::JumpEqTag
      | IrCmd::SelectInt64
      | IrCmd::SetTable
      | IrCmd::SetUpvalue
      | IrCmd::INTERRUPT
      | IrCmd::BarrierObj
      | IrCmd::BarrierTableForward
      | IrCmd::CloseUpvals
      | IrCmd::CAPTURE
      | IrCmd::SETLIST
      | IrCmd::RETURN
      | IrCmd::ForgprepXnextFallback
      | IrCmd::FallbackSetglobal
      | IrCmd::FallbackSettableks
      | IrCmd::FallbackPrepvarargs
      | IrCmd::AdjustStackToTop
      | IrCmd::GetTypeof
      | IrCmd::NEWCLOSURE
      | IrCmd::FINDUPVAL
      | IrCmd::CheckTag
      | IrCmd::CheckTruthy
      | IrCmd::CheckStackRoom
      | IrCmd::AddNum
      | IrCmd::SubNum
      | IrCmd::MulNum
      | IrCmd::DivNum
      | IrCmd::IdivNum
      | IrCmd::ModNum
      | IrCmd::MinNum
      | IrCmd::MaxNum
      | IrCmd::JumpCmpNum
      | IrCmd::FloorNum
      | IrCmd::CeilNum
      | IrCmd::RoundNum
      | IrCmd::SqrtNum
      | IrCmd::AbsNum
      // SETTABLE 数字键哈希直插快路（本 fork 扩展）：只读寄存器键/表指针，
      // 不写任何 VM 寄存器，无恢复位置可失效。
      | IrCmd::GetHashNodeAddrNum
      | IrCmd::JumpIfNodeKeyNotNum
      | IrCmd::StoreNodeKeyNum => {}

      _ => {
        for op in inst.ops.as_slice() {
          CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
        }
      }
    }
  }

  pub(crate) fn invalidate_restore_op(&mut self, location: IrOp, skip_value_invalidation: bool) {
    if location.kind() == IrOpKind::VmReg {
      let reg = vm_reg_op(location) as usize;
      let inst_idx = self.vm_reg_value[reg];

      if inst_idx != K_INVALID_INST_IDX {
        // cpp IrValueLocationTracking.cpp:371-384：仅改 tag 且被跟踪指令的值类型为
        // Double/Pointer/Int/Int64 时整函数提前返回——vmRegValue、vmRegDependent 与
        // 恢复位置关联全部保留。若在此处照常清空 vmRegValue（却保留恢复位置），
        // 后续前向重materialize 会在 vmRegValue 无效的寄存器上写入 vmRegDependent，
        // 破坏 record_restore_op 处 vmRegDependent 必为空的断言。
        if skip_value_invalidation {
          // 视图重建与安全论证单源于 `function_view_ro`。
          let function = Self::function_view_ro(self.function);
          let cmd = function.instructions[inst_idx as usize].cmd;

          match get_cmd_value_kind(cmd) {
            IrValueKind::Double | IrValueKind::Pointer | IrValueKind::Int | IrValueKind::Int64 => {
              return;
            }
            _ => {}
          }
        }

        self.invalidate_inst_restore_location(inst_idx, location);
        self.vm_reg_value[reg] = K_INVALID_INST_IDX;

        // cpp IrValueLocationTracking.cpp:410-440：链式依赖指令的恢复位置
        // 同批失效（移植期开关 LuauCodegenForwardRematerialize 在 cpp 中已删除）。
        let dep_inst_idx = self.vm_reg_dependent[reg];

        if dep_inst_idx != K_INVALID_INST_IDX {
          self.invalidate_inst_restore_location(dep_inst_idx, location);
          self.vm_reg_dependent[reg] = K_INVALID_INST_IDX;
        }
      }
    } else if location.kind() == IrOpKind::VmConst {
      CODEGEN_ASSERT!(false);
    }
  }

  /// cpp IrValueLocationTracking.cpp:386-405/413-439 的公共体：对单条被跟踪
  /// 指令执行失效（needsReload 时先经回调物化恢复位置，再清除同位置恢复记录）。
  /// `skipValueInvalidation` 的提前返回已按 cpp 形态上移到调用方。
  fn invalidate_inst_restore_location(&mut self, inst_idx: u32, location: IrOp) {
    // 视图重建与安全论证单源于 `function_view`。
    let function = Self::function_view(self.function);
    let needs_reload = function.instructions[inst_idx as usize].needs_reload;

    if needs_reload {
      CODEGEN_ASSERT!(
        !function
          .find_restore_location_u32_bool(inst_idx, false)
          .lazy
      );

      if let Some(callback) = self.restore_callback {
        // Safety: inst 为经边界检查索引得到的、指向存活 IrInst 元素的合法 *mut; self.restore_callback_ctx
        // 为构造期接线、比 self 长寿的配套上下文, 满足 restore_callback 的 fn 契约(第一参为其 ctx、第二参
        // 指向存活 IrInst)。回调与 function 借用同处单线程降级阶段, 无并发别名。
        unsafe {
          let inst = &mut function.instructions[inst_idx as usize];
          callback(self.restore_callback_ctx, inst);
        }
      }
    }

    let curr_restore_location = function.find_restore_location_u32_bool(inst_idx, false);

    if location == curr_restore_location.op {
      function.record_restore_location(inst_idx, ValueRestoreLocation::default());
    }
  }

  pub(crate) fn invalidate_restore_vm_regs(&mut self, start: i32, count: i32) {
    let mut end = if count == -1 { 255 } else { start + count };

    if end > self.max_reg {
      end = self.max_reg;
    }

    let mut reg = start;
    while reg <= end {
      self.invalidate_restore_op(
        IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, reg as u32),
        false,
      );
      reg += 1;
    }
  }

  pub fn new(function: &mut IrFunction) -> Self {
    let mut tracking = Self {
      function: function as *mut IrFunction,
      vm_reg_value: [K_INVALID_INST_IDX; 256],
      vm_reg_dependent: [K_INVALID_INST_IDX; 256],
      max_reg: 0,
      restore_callback_ctx: null_mut(),
      restore_callback: None,
    };

    // 对应 C++ 构造函数对数组的显式初始化。
    tracking.vm_reg_value.fill(K_INVALID_INST_IDX);
    tracking.vm_reg_dependent.fill(K_INVALID_INST_IDX);

    tracking
  }

  pub fn process_store_location_hint(&mut self, hint: &StoreLocationHint) {
    CODEGEN_ASSERT!(hint.op.kind() == IrOpKind::VmReg);

    if hint.inst_idx != K_INVALID_INST_IDX {
      // 视图重建与安全论证单源于 `function_view`；inst_idx 已由外层排除且经带界索引。
      let function = Self::function_view(self.function);

      if function.instructions[hint.inst_idx as usize].use_count == 0 {
        return;
      }

      let reg = vm_reg_op(hint.op);
      let existing_loc = function.find_restore_location_u32_bool(hint.inst_idx, false);

      if existing_loc.op.kind() != IrOpKind::None {
        return;
      }

      if reg > self.max_reg {
        self.max_reg = reg;
      }

      let captured = reg_bit_test(&function.cfg.captured.regs, reg as usize);

      self.invalidate_restore_op(hint.op, false);

      if !captured {
        function.record_restore_location(
          hint.inst_idx,
          ValueRestoreLocation {
            op: hint.op,
            kind: hint.kind,
            conversion_cmd: IrCmd::NOP,
            lazy: true,
          },
        );
      }

      self.vm_reg_value[reg as usize] = hint.inst_idx;
    }
  }

  pub(crate) fn record_restore_op(&mut self, inst_idx: u32, location: IrOp) {
    // 视图重建与安全论证单源于 `function_view`；inst_idx 经带界安全索引访问。
    let function = Self::function_view(self.function);
    // 只取后续所需两个纯读数（cmd / opA），免整条 IrInst 克隆：record_restore_location
    // 只改 value_restore_ops，不回写 instructions，先读后改语义与克隆版逐字一致。
    let inst = &function.instructions[inst_idx as usize];
    let inst_cmd = inst.cmd;
    let inst_source = op_a_ref(inst);

    if location.kind() == IrOpKind::VmReg {
      let reg = vm_reg_op(location);

      if reg > self.max_reg {
        self.max_reg = reg;
      }

      let captured = reg_bit_test(&function.cfg.captured.regs, reg as usize);

      if !captured {
        function.record_restore_location(
          inst_idx,
          ValueRestoreLocation {
            op: location,
            kind: get_cmd_value_kind(inst_cmd),
            conversion_cmd: IrCmd::NOP,
            lazy: false,
          },
        );
      }

      self.vm_reg_value[reg as usize] = inst_idx;

      // cpp IrValueLocationTracking.cpp:334：任何依赖值在 beforeInstLowering
      // 里被清理后，记录新恢复位置时必须为空（移植期开关
      // LuauCodegenForwardRematerialize 在 cpp 中已删除，该路径无条件执行）。
      CODEGEN_ASSERT!(self.vm_reg_dependent[reg as usize] == K_INVALID_INST_IDX);

      if can_be_rematerialized(inst_cmd) && inst_source.kind() == IrOpKind::Inst {
        let dep_inst_idx = inst_source.index();

        if !captured {
          function.record_restore_location(
            dep_inst_idx,
            ValueRestoreLocation {
              op: location,
              kind: get_cmd_value_kind(inst_cmd),
              conversion_cmd: inst_cmd,
              lazy: false,
            },
          );
        }

        // cpp IrValueLocationTracking.cpp:348：记录依赖链当前位置，无条件执行
        self.vm_reg_dependent[reg as usize] = dep_inst_idx;
      }
    } else if location.kind() == IrOpKind::VmConst {
      function.record_restore_location(
        inst_idx,
        ValueRestoreLocation {
          op: location,
          kind: get_cmd_value_kind(inst_cmd),
          conversion_cmd: IrCmd::NOP,
          lazy: false,
        },
      );
    }
  }

  /// # Safety
  ///
  /// `context` 被原样存入，稍后作为第一个实参回传给 `callback`：调用方须保证 `context`
  /// 指向的对象在本 `IrValueLocationTracking` 生命周期内保持存活且地址稳定；且若
  /// `callback` 为 `Some`，它必须能以该 `context` 与一个有效 `*mut IrInst` 被安全调用。
  pub fn set_restore_callback(
    &mut self,
    context: *mut c_void,
    callback: Option<unsafe fn(*mut c_void, *mut IrInst)>,
  ) {
    self.restore_callback_ctx = context;
    self.restore_callback = callback;
  }
}

/// cpp IrValueLocationTracking.cpp:29-32 `canBeRematerialized`。
/// 以索引而非指针访问 instructions：借用约束下无法把 `&mut self.function`
/// 内的引用同时传入需要再次可变借用的方法（`IrInst` 指针身份被
/// `get_inst_index` 的断言依赖，不能传 clone）。
fn can_be_rematerialized(cmd: IrCmd) -> bool {
  matches!(cmd, IrCmd::UintToNum | IrCmd::IntToNum)
}

/// cpp IrValueLocationTracking.cpp:34-47 `canRematerializeArguments`。
fn can_rematerialize_arguments_at(function: &mut IrFunction, inst_idx: u32) -> bool {
  let inst = &mut function.instructions[inst_idx as usize];

  if can_be_rematerialized(inst.cmd) && op_a(inst).kind() == IrOpKind::Inst {
    let dep_inst_idx = op_a(inst).index();

    if function.instructions[dep_inst_idx as usize].last_use != inst_idx {
      return true;
    }
  }

  false
}
