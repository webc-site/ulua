use alloc::vec::Vec;
use core::{
  ffi::c_void,
  mem::{offset_of, size_of},
  ptr,
  ptr::addr_of_mut,
};

use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{bitmask::bit2mask, blackbit::BLACKBIT, white_0_bit::WHITE0BIT, white_1_bit::WHITE1BIT},
  records::{
    closure::{Closure, Closure as ClosureAlias, LClosure},
    g_cheader::GCheader,
    lua_table::LuaTable as LuaTableAlias,
    luau_buffer::LuauBuffer,
    proto::Proto,
    udata::Udata,
  },
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{
    address_kind_a_64::AddressKindA64, code_gen_counter::CodeGenCounter,
    condition_a_64::ConditionA64, ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind,
    ir_value_kind::IrValueKind, kind_a_64::KindA64,
  },
  functions::{
    cast_reg::cast_reg, emit_abort::emit_abort, emit_add_offset::emit_add_offset,
    float_bits::get_double_bits, get_cmd_value_kind::get_cmd_value_kind, is_gco::is_gco,
    predecessors::predecessors,
    produces_dirty_high_register_bits::produces_dirty_high_register_bits, vm_const_op::vm_const_op,
    vm_exit_op::vm_exit_op, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::{CODEGEN_ASSERT, unsupported_instruction_form},
  records::{
    address_a_64::AddressA64,
    assembly_builder_a_64::{AssemblyBuilderA64, K_MAX_IMMEDIATE},
    exit_handler::ExitHandler,
    interrupt_handler_ir_lowering_a_64::InterruptHandler,
    ir_block::{IrBlock, K_BLOCK_NO_START_PC},
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    ir_reg_alloc_a_64::IrRegAllocA64,
    ir_value_location_tracking::IrValueLocationTracking,
    label::Label,
    lowering_stats::LoweringStats,
    module_helpers::ModuleHelpers,
    register_a_64::{RegisterA64, reg},
  },
  type_aliases::{instruction_ir_builder::Instruction, mem::mem},
};

/// cpp `IrLoweringA64`（`CodeGen/src/IrLoweringA64.h`）。
///
/// `build/helpers/function/stats` 与 `regs`（内嵌的 `IrRegAllocA64`）共享同一组
/// 上游裸指针成员：lowering 与寄存器分配在同一调用栈上并发读写 builder/function，
/// 是 cpp 刻意设计的别名关系。Rust 借用系统无法表达该共存别名，若改为
/// `&mut AssemblyBuilderA64`/`&mut IrFunction` 会直接编译失败（双 `&mut`），
/// 因此这些字段保持裸指针形态并在构造点（`lower_function`）一次性注入，
/// 生命周期由构造点栈帧保证——属内部共享可变上下文，非可去除的 FFI 杂质。
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrLoweringA64 {
  pub build: *mut AssemblyBuilderA64,
  pub helpers: *mut ModuleHelpers,
  pub function: *mut IrFunction,
  pub stats: *mut LoweringStats,
  pub regs: IrRegAllocA64,
  pub value_tracker: IrValueLocationTracking,
  pub interrupt_handlers: Vec<InterruptHandler>,
  pub exit_handlers: Vec<ExitHandler>,
  pub exit_handler_map: DenseHashMap<u32, u32>,
  pub exit_sync_alloc_token: u32,
  pub exit_sync_inst_idx: u32,
  pub error: bool,
}

// == IrLoweringA64 function/stats 指针洗白面（首域单元 A–E 收口尾榜）==
// 已收口（本组访问器为唯一裸解引用点）：`&self` 接收者的全部操作数取值
// （int/uint/int64/double/tag/const/import）、start_block 的 get_block_index 走 `function_ref`、
// temp_addr 的 inst_op 读走 `function_mut`；finish_function 的 end_location 写走 `function_mut`；
// 统计的判空 + 自增走 `stats_mut`。调用点不再手写 `(*self.function)`/`(*self.stats)`。
// C1 波追加：发射视图 `build_mut`（方法链式 `(*self.build).emitX(..)` 全部走此单点）与
// 混窗元组门面 `build_function_regs_mut`（emit_builtin 旁路需 build↔function↔regs 三视图
// 同窗共存，沿 `label_op_mut_pair` 同守则）。lower_inst 巨文件内 `(*self.function)` 裸读
// 已随之清零。
// C1 接力二棒：单视图 `&mut *self.build` 旁路 15 站洗白至 `build_mut`
// （emit_add_offset 5/emit_fallback 9/emit_abort 1，fallback 系先拷 uint_op 实参）；
// 随收 jump_or_fallthrough/temp_int 两站。其后 label 混窗经下方闭包窗门面全部收口,
// lower_inst 巨文件内 unsafe 与裸指针解引用清零。
// 尾榜保留（非本业务文件可清）：
//   1) 巨型混窗内 label 视图与发射借用同表达式共存（`cbz/cbnz/b/b_condition_a_64_label(..,
//      self.label_op(..))` 形——`&mut Label` 长借用会把 self 整体锁死无法表达）的站点，
//      已由业务文件逐处裸指针重建改为经下方 `with_op_label`/`with_target_label`/
//      `with_helper_label`/`with_block_mut` 闭包窗门面，unsafe 全部收口于本文件门面。
//   2) finalize_target_label 内 sync_info/inst_op 读跨越 `record_and_free_last_use(&mut self.regs)`
//      存活——function↔regs 共存别名窗口，视图访问器会把整个 self 锁死而无法表达，维持裸指针
//      （单元 D 逐处登记）。
//   3) helpers 字段与 regs 共享上游别名，构造点一次性注入（见结构体注释）。
impl IrLoweringA64 {
  // 共享裸指针访问器家族（function_mut/function_ref/build_mut/stats_mut）由宏收口，
  // 统一契约见 `crate::shared_ptr_accessors!`；本结构特有的混窗/闭包窗门面在下方手写。
  crate::shared_ptr_accessors!(AssemblyBuilderA64);

  /// 混窗元组门面（旁路透传专用）：`emit_builtin` 自由函数在同一调用里同时需要
  /// build、function、regs 三条可变视图，逐字段访问器会双 `&mut self` 冲突；此处一次
  /// 派生三个同源借用，与门面化前的 `(&mut *self.build, &mut *self.function, &mut self.regs)`
  /// 实参列逐位等价。
  /// Safety: 三条借用皆即时消费于同一条调用语句、不长期持有；build/function 与 regs 的
  /// 上游别意为 cpp 刻意设计（见结构体注释），单线程串行降低下无第二活跃别名。
  #[inline]
  pub(crate) fn build_function_regs_mut(
    &mut self,
  ) -> (&mut AssemblyBuilderA64, &mut IrFunction, &mut IrRegAllocA64) {
    // Safety:见函数注释。
    unsafe { (&mut *self.build, &mut *self.function, &mut self.regs) }
  }

  /// 闭包窗门面（原『混窗·尾榜1』收口）：取 `op` 块的 `.label` 可变视图并与 `&mut self`
  /// 同窗交给 `emit`。发射调用点形如 `cbz/cbnz/b/bcond(.., label_op(..))` 时，实参位的
  /// `&mut Label` 长借用会把 `self` 整体锁死、无法经普通访问器表达，故收敛于此。
  #[inline]
  pub(crate) fn with_op_label<R>(
    &mut self,
    op: IrOp,
    emit: impl FnOnce(&mut Self, &mut Label) -> R,
  ) -> R {
    // Safety: `ir_lowering_a_64_label_op` 依其契约（op 为 Block 操作数且下标合法）返回指向
    // `function.blocks` 活元素 `.label` 字段的可变借用，非空、对齐；此处经裸指针把该寿命
    // 与 `&mut self` 解绑——块数组活过整个 lowering 栈帧，借用窗即时于 `emit` 语句内消费，
    // 单线程串行降低下该块与 builder 无其他活跃别名。
    let label = self.ir_lowering_a_64_label_op(op) as *mut Label;
    emit(self, unsafe { &mut *label })
  }

  /// `get_target_label` 返回指针的消费窗门面：重建 `&mut Label` 并与 `&mut self` 同窗
  /// 交给 `emit`。上游契约：指针在取回与消费之间不发生 `exit_handlers` 重分配。
  #[inline]
  pub(crate) fn with_target_label<R>(
    &mut self,
    target: *mut Label,
    emit: impl FnOnce(&mut Self, &mut Label) -> R,
  ) -> R {
    // Safety: `target` 出自 `get_target_label`——指向调用方 `fresh`（活局部）、`exit_handlers`
    // 元素的 `self_` 字段（窗口内无重分配）或 `label_op`（块存活期），皆非空、对齐；
    // 重建的可变借用仅在 `emit` 语句内存活，单线程串行降低下无其他活跃别名。
    emit(self, unsafe { &mut *target })
  }

  /// helpers 常驻 label 的消费窗门面：从 `ModuleHelpers` 取 `pick` 选中的 `Label` 字段，
  /// 与 `&mut self` 同窗交给 `emit`（发射方法自身仍走 `emit_*` 门面）。
  #[inline]
  pub(crate) fn with_helper_label<R>(
    &mut self,
    pick: impl FnOnce(&mut ModuleHelpers) -> &mut Label,
    emit: impl FnOnce(&mut Self, &mut Label) -> R,
  ) -> R {
    // Safety: `helpers` 为构造点一次性注入、比本 lowering 长寿的 `*mut ModuleHelpers`；
    // `pick` 返回其内活 `Label` 字段的借用，仅于 `emit` 语句内即时消费，无其他活跃别名。
    let helpers = unsafe { &mut *self.helpers };
    let label = pick(helpers);
    emit(self, label)
  }

  /// 按 `op` 取块的可变视图并与 `&mut self` 同窗交给 `emit`（`jump_or_fallthrough_op`
  /// 形的『块视图 + self 方法』混窗收口）。
  #[inline]
  pub(crate) fn with_block_mut<R>(
    &mut self,
    op: IrOp,
    emit: impl FnOnce(&mut Self, &mut IrBlock) -> R,
  ) -> R {
    // Safety: `ir_lowering_a_64_block_op` 依其契约（op 为 Block 操作数且下标合法）返回指向
    // `function.blocks` 活元素的非空对齐指针；重建借用仅于 `emit` 语句内存活，单线程串行
    // 降低下该块无其他活跃可变别名。
    let block = unsafe { &mut *self.ir_lowering_a_64_block_op(op) };
    emit(self, block)
  }

  pub fn ir_lowering_a_64_alloc_and_increment_counter_at(
    &mut self,
    kind: CodeGenCounter,
    pcpos: u32,
  ) {
    // 视图访问器把 function 的每次访问收敛为语句内即时借用(契约见 records impl 注释);
    // increment_counter_at 只发射指令、不改写 extra_native_data, 故先取出 len 再跨调用无别名风险。
    // build 保留裸指针读取(AssemblyBuilder 发射门面波处理)。
    if !self.function_ref().record_counters {
      return;
    }

    // Safety: self.build 为构造接线的非空、比持有者长寿的裸指针（arena 不变量）,
    // 读 log_text/发射日志在存活对象上; 单线程串行 lowering, 无并存可变借用别名。
    unsafe {
      if (*self.build).log_text {
        (*self.build).log_append(format_args!(
          "; counter kind {} at pcpos {}\n",
          kind as u32, pcpos
        ));
      }
    }

    self.function_mut().extra_native_data.push(kind as u32);
    self.function_mut().extra_native_data.push(pcpos);
    let len = self.function_ref().extra_native_data.len();
    self.ir_lowering_a_64_increment_counter_at(len);
    self.function_mut().extra_native_data.push(0);
    self.function_mut().extra_native_data.push(0);
  }

  /// cpp `IrLoweringA64::blockOp`（`src/IrLoweringA64.h:73` `IrBlock& blockOp(IrOp op) const`）
  /// → `function.blockOp(op)`。
  ///
  /// 上游是 const 成员函数经引用成员改写：Rust 侧用"裸指针字段的内部可变性"表达，
  /// 接收者保持 `&self`，但只从裸指针字段派生访问，不再用
  /// `self as *const Self as *mut Self` 把 `&self` 伪造成 `&mut Self`（mut_from_ref UB）。
  ///
  /// # Safety
  /// `op` 必须是 `IrOpKind::Block` 且下标在 `function.blocks` 内；返回指针不得越过 lowering 生命周期使用。
  pub unsafe fn ir_lowering_a_64_block_op(&self, op: IrOp) -> *mut IrBlock {
    // 只复制裸指针字段（共享读），再从该指针构造访问
    let function = self.function;
    // Safety: self.function 为构造接线的非空 IrFunction*（arena 不变量：地址恒定、比持有者
    // 长寿），(*function).block_op(op) 解引用取存活对象；返回的 *mut IrBlock 指向 function.blocks
    // 内元素，其存活受本方法 `# Safety` 段约束（不越过 lowering 生命周期使用）。
    unsafe { (*function).block_op(op) }
  }

  /// [`Self::ir_lowering_a_64_block_op`] 的只读共享视图门面：判别/读块字段（kind、startpc 等）
  /// 的站点不再在调用点手写 `&*` 裸解引用，共享视图重建收口到本处。
  pub fn ir_lowering_a_64_block_op_ref(&self, op: IrOp) -> &IrBlock {
    // Safety: 沿用 `ir_lowering_a_64_block_op` 契约（op 为 Block 操作数且下标合法），其返回
    // 非空/对齐/指向 function.blocks 活元素的指针；此处仅降级为共享借用读取字段，单线程
    // 串行降低中调用点表达式内瞬时重建、不与其它可变别名重叠。
    unsafe { &*self.ir_lowering_a_64_block_op(op) }
  }

  pub fn ir_lowering_a_64_check_object_barrier_conditions(
    &mut self,
    object: RegisterA64,
    temp: RegisterA64,
    ra: RegisterA64,
    ra_op: IrOp,
    ratag: i32,
    skip: &mut Label,
  ) {
    let tempw = cast_reg(KindA64::W, temp);

    // 契约: self.build 为构造接线的非空 AssemblyBuilderA64*（比持有者长寿），各块仅经
    // (*self.build) 即时发射指令；offset_of! 皆编译期常量，skip/addr 指向借用期内有效的
    // Label/Operand，单线程串行 lowering 下重建可变借用无别名冲突。
    // temp_addr 为安全方法（内部自管 build 借用），先于本块各发射点调用，顺序与拆分前一致。
    if ratag == -1 || !is_gco(ratag as u8) {
      if ra_op.kind() == IrOpKind::Inst {
        // Safety: 见上; umov_4s 经 build 即时可变借用发射。
        unsafe {
          (*self.build).umov_4s(tempw, ra, 3);
        }
      } else {
        let addr = self.ir_lowering_a_64_temp_addr(ra_op, offset_of!(TValue, tt) as i32, temp);
        // Safety: 见上; addr 为借用期内有效 Operand, ldr 经 build 即时借用发射。
        unsafe {
          (*self.build).ldr(tempw, addr);
        }
      }

      // Safety: 见上; cmp + 条件分支同块借用 build 与 skip。
      unsafe {
        (*self.build).cmp_register_a_64_u16(tempw, LuaType::String as u16);
        (*self.build).b_condition_a_64_label(ConditionA64::Less, skip);
      }
    }

    // Safety: 见上; ldrb/tbz 同块借用 build 与 skip。
    unsafe {
      (*self.build).ldrb(tempw, mem(object, offset_of!(GCheader, marked) as i32));
      (*self.build).tbz(tempw, BLACKBIT as u8, skip);
    }

    if ra_op.kind() == IrOpKind::Inst {
      // Safety: 见上; fmov 经 build 即时可变借用发射。
      unsafe {
        (*self.build).fmov_register_a_64_register_a_64(temp, cast_reg(KindA64::D, ra));
      }
    } else {
      let addr = self.ir_lowering_a_64_temp_addr(ra_op, offset_of!(TValue, value) as i32, temp);
      // Safety: 见上; addr 为借用期内有效 Operand, ldr 经 build 即时借用发射。
      unsafe {
        (*self.build).ldr(temp, addr);
      }
    }

    // Safety: 见上; ldrb/tst/条件分支同块借用 build 与 skip。
    unsafe {
      (*self.build).ldrb(tempw, mem(temp, offset_of!(GCheader, marked) as i32));
      (*self.build).tst_register_a_64_u32(tempw, bit2mask(WHITE0BIT, WHITE1BIT) as u32);
      (*self.build).b_condition_a_64_label(ConditionA64::Equal, skip);
    }
  }

  pub fn ir_lowering_a_64_check_safe_env(&mut self, target: IrOp, index: u32, _next: &IrBlock) {
    let mut fresh = Label::default();
    let temp: RegisterA64 = self.regs.alloc_temp(KindA64::X);
    let tempw: RegisterA64 = cast_reg(KindA64::W, temp);
    // Safety: offset_of! 为编译期常量、无运行时解引用；self.build 是构造接线的非空
    // AssemblyBuilderA64*（比持有者长寿），(*self.build) 发射 ldr/ldrb/cbz 皆作用于该存活对象；
    // label 由 &mut *label 在借用期内重建，单线程串行 lowering 无并存别名。
    unsafe {
      let offset_env = offset_of!(ClosureAlias, env);
      let offset_safeenv = offset_of!(LuaTableAlias, safeenv);
      (*self.build).ldr(temp, mem(R_CLOSURE, offset_env as i32));
      (*self.build).ldrb(tempw, mem(temp, offset_safeenv as i32));
      let label = self.ir_lowering_a_64_get_target_label(target, index, &mut fresh);
      (*self.build).cbz(tempw, &mut *label);
      self.ir_lowering_a_64_finalize_target_label(target, index, &mut fresh);
    }
  }

  pub fn ir_lowering_a_64_double_op(&self, op: IrOp) -> f64 {
    // 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
    self.function_ref().double_op(op)
  }

  pub fn ir_lowering_a_64_finalize_target_label(
    &mut self,
    op: IrOp,
    index: u32,
    fresh: &mut Label,
  ) {
    if op.kind() == IrOpKind::Undef {
      // Safety: `self.build` 为构造点注入的非空/对齐/长寿裸指针,`&mut *self.build` 重建唯一可变借用交给
      // `emit_abort` 追加中止指令;单线程降级中该借用仅在本语句存活,不与其他字段写冲突。
      unsafe {
        emit_abort(&mut *self.build, fresh);
      }
    } else if op.kind() == IrOpKind::Block
      && self.ir_lowering_a_64_block_op_ref(op).kind == IrBlockKind::ExitSync
    {
      if self.exit_sync_inst_idx == index {
        CODEGEN_ASSERT!(self.exit_sync_alloc_token == self.regs.get_alloc_token());
      }

      // 保留: sync_info 与循环体内的 inst_op 视图需跨越 record_and_free_last_use(&mut self.regs)
      // 存活——function↔regs 共存别名窗口, 视图访问器会把整个 self 锁住而无法表达, 维持裸指针
      // 解引用(单元 D 登记)。
      // Safety: `self.function` 非空/对齐/长寿;`index` 为当前指令下标,在 `vm_exit_info` map 中按键查找只读借用。
      let sync_info = unsafe { (*self.function).vm_exit_info.find(&index) };
      CODEGEN_ASSERT!(sync_info.is_some());
      // 不变式（cpp 同源 LUAU_ASSERT+解引用）：ExitSync 块必由先前 VmExit 建过同步点，
      // find 失配即 IR 同步信息被破坏，属编译器内部不变量。
      let sync_info =
        sync_info.expect("ExitSync 块对应的 vm_exit_info 同步记录必存在（cpp 同源断言）");

      for arg_op in &sync_info.arg_ops {
        // Safety: `self.function` 有效;`arg_op` 来自已存在 vm_exit_info 记录的操作数,`inst_op` 据其下标只读取指令。
        let inst_op = unsafe { (*self.function).inst_op(*arg_op) };
        self
          .regs
          .record_and_free_last_use(op.index(), inst_op, index);
      }
    } else if op.kind() == IrOpKind::VmExit && fresh.id != 0 {
      let exit_handler_idx = self.exit_handlers.len() as u32;
      self
        .exit_handler_map
        .try_insert(vm_exit_op(op), exit_handler_idx);
      self.exit_handlers.push(ExitHandler {
        self_: *fresh,
        pcpos: vm_exit_op(op),
      });
    }
  }

  pub fn ir_lowering_a_64_finish_block(&mut self, curr: &IrBlock, next: &IrBlock) {
    if !self.regs.spills.is_empty() {
      // Safety: self.function 为构造点接线、比 lowering 长寿的非空 *mut IrFunction; 此处重建
      // 共享引用只读取块下标/前驱/块 kind, 全程只读, 单线程降低无并发 &mut 别名。
      let function = unsafe { &*self.function };
      let next_idx = function.get_block_index(next);
      let curr_idx = function.get_block_index(curr);

      let preds = predecessors(&function.cfg, next_idx);
      for pred_idx in preds {
        let pred_block = &function.blocks[pred_idx as usize];
        CODEGEN_ASSERT!(pred_idx == curr_idx || pred_block.kind == IrBlockKind::Dead);
      }

      CODEGEN_ASSERT!(next.use_count == 1);
    }
  }

  pub fn ir_lowering_a_64_finish_function(&mut self) {
    // Safety: build/helpers 为构造点接线、非空且比本次 lowering 长寿的裸指针;
    // 各段内 `(*self.build)` 对发射器的可变借用与 `(*self.helpers)` label 字段的读取, 均在单线程
    // 串行 lower 阶段按行为阶段依次取得并释放, 同一时刻无重叠别名（裸解引用不借入 self, 故可与
    // interrupt_handlers 的可变迭代、counter 方法的 &mut self 借用共存）。function/stats 的读写
    // 已收口到 function_mut/stats_mut 视图访问器（契约见 records impl 注释）。
    // Safety: build 发射器为构造点接线的非空裸指针, log 门读与 log_append 借用同块取得并释放。
    unsafe {
      if (*self.build).log_text {
        (*self.build).log_append(format_args!("; interrupt handlers\n"));
      }
    }

    // cpp 按引用遍历并就地绑定 label：写回真实元素，避免拷贝丢失 location
    for handler in self.interrupt_handlers.iter_mut() {
      // Safety: build 为构造点接线的非空裸指针; handler 借用 interrupt_handlers,
      // 与 build/helpers 所指对象互不重叠, 单线程串行发射无别名冲突。
      unsafe {
        (*self.build).set_label_label(&mut handler.self_);
        (*self.build).mov_register_a_64_i32(
          X0,
          ((handler.pcpos + 1) * size_of::<Instruction>() as u32) as i32,
        );
        (*self.build).adr_register_a_64_label(X1, &mut handler.next);
        (*self.build).b_label(&mut (*self.helpers).interrupt);
      }
    }

    // Safety: build 发射器为构造点接线的非空裸指针, log 门读与 log_append 借用同块取得并释放。
    unsafe {
      if (*self.build).log_text {
        (*self.build).log_append(format_args!("; exit handlers\n"));
      }
    }

    // 循环体内有 &mut self 的 counter 方法调用，无法持有 exit_handlers 的可变迭代借用
    for i in 0..self.exit_handlers.len() {
      let mut handler = self.exit_handlers[i];
      // 两分支同首句 set_label_label, 提到分支外（不依赖 pcpos, 行为等价）。
      // Safety: build 为构造点接线的非空裸指针, 与 handler 局部拷贝无别名。
      unsafe {
        (*self.build).set_label_label(&mut handler.self_);
      }

      if handler.pcpos == K_VM_EXIT_ENTRY_GUARD_PC {
        self.ir_lowering_a_64_alloc_and_increment_counter_at(CodeGenCounter::VmExitTaken, !0u32);

        // Safety: helpers 为构造点接线的非空裸指针, label 字段与 build 发射器互不重叠。
        unsafe {
          (*self.build).b_label(&mut (*self.helpers).exit_continue_vm_clear_native_flag);
        }
      } else {
        self.ir_lowering_a_64_alloc_and_increment_counter_at(
          CodeGenCounter::VmExitTaken,
          handler.pcpos,
        );

        // Safety: build 发射器与 helpers label 的裸借用即时消费, 单线程串行无重叠别名。
        unsafe {
          (*self.build)
            .mov_register_a_64_i32(X0, (handler.pcpos * size_of::<Instruction>() as u32) as i32);
          (*self.build).b_label(&mut (*self.helpers).update_pc_and_continue_in_vm);
        }
      }
    }

    // Safety: build 发射器为构造点接线的非空裸指针, 设 label 与读回偏移在同一借用窗口内顺序完成。
    let end_offset = unsafe {
      let end = (*self.build).set_label();
      (*self.build).get_label_offset(&end)
    };
    self.function_mut().end_location = end_offset;
    // Safety: build 发射器为构造点接线的非空裸指针, 发射 undefined 指令作中止跳转位。
    unsafe {
      (*self.build).udf();
    }

    // stats 判空+解引用样板收口到 stats_mut 访问器; self/self.regs 的读数为安全字段直取。
    let error = self.error;
    let regs_error = self.regs.error;
    if let Some(stats) = self.stats_mut() {
      if error {
        stats.lowering_errors += 1;
      }

      if regs_error {
        stats.reg_alloc_errors += 1;
      }
    }
  }

  /// cpp `Label* getTargetLabel(IrOp op, size_t index, Label& fresh)`（IrLoweringA64.cpp）。
  ///
  /// 返回裸指针与上游签名一致：目标 label 可来自 `fresh`、`exit_handlers`（持续 push 可能
  /// 重分配的 Vec）或 `function` 内 block，借用检查无法表达其与后续 `&mut self` 调用的
  /// 别名窗口。上游契约：返回值仅存活到同块的 `finalize_target_label`，期间不发生
  /// `exit_handlers` 重分配。旧版返回 `&mut Label` 再在每个调用点补 `as *mut Label`
  /// 的写法与 cpp 脱节，已收敛到此形态。
  pub fn ir_lowering_a_64_get_target_label(
    &mut self,
    op: IrOp,
    _index: u32,
    fresh: &mut Label,
  ) -> *mut Label {
    if op.kind() == IrOpKind::Undef {
      return fresh;
    }

    if op.kind() == IrOpKind::VmExit {
      if let Some(index) = self.exit_handler_map.find(&vm_exit_op(op)) {
        return &raw mut self.exit_handlers[*index as usize].self_;
      }

      return fresh;
    }

    self.ir_lowering_a_64_label_op(op)
  }

  pub fn ir_lowering_a_64_has_error(&self) -> bool {
    self.error || self.regs.error
  }

  pub fn ir_lowering_a_64_increment_counter_at(&mut self, offset: usize) {
    let temp1 = self.regs.alloc_temp(KindA64::X);
    let temp2 = self.regs.alloc_temp(KindA64::X);

    // 访存偏移均为编译期 offset_of! 常量, 先在安全域算好。
    let p_off = (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32;
    let execdata_off = offset_of!(Proto, execdata) as i32;

    // Safety: `(*self.function_ref().proto)` 对 Proto 布局镜像的二次解引用成立——计数器
    // 发射仅针对携带有效 proto 的 L 闭包触发(与 x64 兄弟方法入口 CODEGEN_ASSERT 同一前提)。
    let counter_offset = unsafe { (*self.function_ref().proto).sizecode as usize + offset } * 4;

    // Safety: build 为构造点接线、非空且比本次 lowering 长寿的裸指针; 对 (*self.build).ldr/str/add
    // 及 emit_add_offset(&mut *self.build) 的可变借用处于单线程串行 lower 阶段, 依次取得并释放, 无重叠
    // 别名。temp1/temp2 为刚分配、块末即 free_temp 归还的活寄存器。
    unsafe {
      (*self.build).ldr(temp1, mem(R_CLOSURE, p_off));
      (*self.build).ldr(temp1, mem(temp1, execdata_off));
      emit_add_offset(&mut *self.build, temp2, temp1, counter_offset);
      (*self.build).ldr(temp1, mem(temp2, 0));
      (*self.build).add_register_a_64_register_a_64_u16(temp1, temp1, 1);
      (*self.build).str(temp1, mem(temp2, 0));
    }

    self.regs.free_temp(temp1);
    self.regs.free_temp(temp2);
  }

  pub fn ir_lowering_a_64_int_64_op(&self, op: IrOp) -> i64 {
    // 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
    self.function_ref().int64_op(op)
  }

  pub fn ir_lowering_a_64_int_op(&self, op: IrOp) -> i32 {
    // 转发到 IrFunction 的常量取值(原为自递归存根,运行即栈溢出)；裸指针收口在 `function_ref`。
    self.function_ref().int_op(op)
  }

  pub fn ir_lowering_a_64_ir_lowering_a_64(&mut self) {
    self.exit_handler_map = DenseHashMap::new(!0u32);

    let self_ptr = ptr::from_mut(self).cast::<c_void>();
    self
      .value_tracker
      .set_restore_callback(self_ptr, Some(Self::restore_callback_shim));
  }

  /// C 回调 shim：还原寄存器后执行 restore 回调。
  /// # Safety
  /// `context` 指向注册时写入的闭包上下文，`inst` 指向存活指令。
  unsafe fn restore_callback_shim(context: *mut c_void, inst: *mut IrInst) {
    // Safety: context 由 ir_lowering_a_64_ir_lowering_a_64 以 `ptr::from_mut(self).cast::<c_void>()` 注册,
    // 指向回调登记期间持续存活的 IrLoweringA64; 回调在单线程串行 lowering 中同步触发, 此刻无其它借用
    // 持有该 self, 故 `&mut *(context.cast::<IrLoweringA64>())` 的重借用无别名冲突。
    // inst 为调用方传入的活指令, `&mut *inst` 仅在 restore_reg 调用期内临时借用。
    unsafe {
      let self_ = &mut *(context.cast::<IrLoweringA64>());
      self_.regs.restore_reg(&mut *inst);
    }
  }

  pub fn ir_lowering_a_64_is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
    target.start == next.start
  }

  pub fn ir_lowering_a_64_jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
    if !self.ir_lowering_a_64_is_fallthrough_block(target, next) {
      // 经 `build_mut` 发射门面单点派生可变借用；target 为活块可变借用（外部参数，
      // 不经 self），与 build 不相互别名。
      self.build_mut().bl(&mut target.label);
    }
  }

  pub fn ir_lowering_a_64_label_op(&mut self, op: IrOp) -> &mut Label {
    // Safety: 前置条件(op 为 Block 且下标合法)保证 ir_lowering_a_64_block_op 返回指向
    // function.blocks 活元素的非空指针; 取 .label 字段地址重建 &mut 落在块存活期内, 接收者
    // 为 &mut self 独占、单线程降低, 不与其他对同一 block 的可变借用共存。
    unsafe { &mut *addr_of_mut!((*self.ir_lowering_a_64_block_op(op)).label) }
  }

  pub fn ir_lowering_a_64_reg_op(&mut self, op: IrOp) -> RegisterA64 {
    let function = self.function;
    // Safety: self.function 为构造点接线、比 lowering 长寿的非空 *mut IrFunction; inst_op 取其
    // 可变访问本条指令, 单线程串行降低此刻不持有对同一 function/指令的其它借用, 无别名冲突。
    let inst = unsafe { (*function).inst_op(op) };

    if inst.spilled || inst.needs_reload {
      self.regs.restore_reg(inst);
    }

    CODEGEN_ASSERT!((inst.reg_a64 != RegisterA64::NOREG));
    inst.reg_a64
  }

  pub fn ir_lowering_a_64_start_block(&mut self, curr: &IrBlock) {
    if curr.startpc != K_BLOCK_NO_START_PC {
      let counter = if curr.kind == IrBlockKind::Fallback {
        CodeGenCounter::FallbackBlockExecuted
      } else {
        CodeGenCounter::RegularBlockExecuted
      };
      self.ir_lowering_a_64_alloc_and_increment_counter_at(counter, curr.startpc);
    }

    if curr.kind == IrBlockKind::ExitSync {
      // 视图访问器即时派生共享借用读取 blocks 下标, 语句内消费(契约见 records impl 注释)。
      let block_index = self.function_ref().get_block_index(curr);
      self.regs.setup_exit_sync_entry(block_index);
    }
  }

  pub fn ir_lowering_a_64_temp_addr(
    &mut self,
    op: IrOp,
    offset: i32,
    temp_storage: RegisterA64,
  ) -> AddressA64 {
    CODEGEN_ASSERT!(offset % 4 == 0);
    CODEGEN_ASSERT!(offset >= 0 && (offset as usize / 4) <= K_MAX_IMMEDIATE);

    if op.kind() == IrOpKind::VmReg {
      return mem(R_BASE, vm_reg_op(op) * size_of::<TValue>() as i32 + offset);
    } else if op.kind() == IrOpKind::VmConst {
      let constant_offset = vm_const_op(op) as usize * size_of::<TValue>() + offset as usize;

      if constant_offset / 4 <= AddressA64::K_MAX_OFFSET {
        return mem(R_CONSTANTS, constant_offset as i32);
      }

      let temp = if temp_storage == RegisterA64::NOREG {
        self.regs.alloc_temp(KindA64::X)
      } else {
        temp_storage
      };
      CODEGEN_ASSERT!(
        temp.kind() == KindA64::X,
        "temp storage, when provided, must be an 'x' register"
      );

      // Safety: `self.build` 为构造点接线的非空/对齐/比 lowering 长寿裸指针, `&mut *self.build` 重建唯一
      // 可变借用交给 `emit_add_offset`; 单线程降级中该借用仅在本表达式求值期间存活, 不与 `self` 其他字段写冲突。
      unsafe {
        emit_add_offset(&mut *self.build, temp, R_CONSTANTS, constant_offset);
      }
      return mem(temp, 0);
    } else if op.kind() == IrOpKind::Inst {
      // 视图访问器即时重建 &mut 借用读 cmd, 语句内消费(契约见 records impl 注释)。
      let cmd = self.function_mut().inst_op(op).cmd;
      CODEGEN_ASSERT!(get_cmd_value_kind(cmd) == IrValueKind::Pointer);
      return mem(self.ir_lowering_a_64_reg_op(op), offset);
    }

    unsupported_instruction_form();
    mem(RegisterA64::NOREG, 0)
  }

  pub fn ir_lowering_a_64_temp_addr_buffer(
    &mut self,
    buffer_op: IrOp,
    index_op: IrOp,
    tag: u8,
  ) -> AddressA64 {
    CODEGEN_ASSERT!(tag == LuaType::UserData as u8 || tag == LuaType::Buffer as u8);

    // 对齐 cpp 的 offsetof(Buffer, data)/offsetof(Udata, data), 由编译器计算
    let data_offset = if tag == LuaType::Buffer as u8 {
      offset_of!(LuauBuffer, data) as i32
    } else {
      offset_of!(Udata, data) as i32
    };

    if index_op.kind() == IrOpKind::Inst {
      // 视图访问器即时重建 &mut 借用读 cmd, 语句内消费(契约见 records impl 注释)。
      let cmd = self.function_mut().inst_op(index_op).cmd;
      CODEGEN_ASSERT!(!produces_dirty_high_register_bits(cmd));

      let temp = self.regs.alloc_temp(KindA64::X);
      let buffer = self.ir_lowering_a_64_reg_op(buffer_op);
      let index = self.ir_lowering_a_64_reg_op(index_op);
      // Safety: `self.build` 为非空/对齐/长寿裸指针,向 builder 追加一条 add 指令;temp/buffer/index 均为已分配寄存器。
      unsafe {
        (*self.build).add_register_a_64_register_a_64_register_a_64_i32(temp, buffer, index, 0);
      }
      return mem(temp, data_offset);
    } else if index_op.kind() == IrOpKind::Constant {
      let buffer = self.ir_lowering_a_64_reg_op(buffer_op);
      // 视图访问器即时派生共享借用读取常量(契约见 records impl 注释)。
      let index = self.function_ref().int_op(index_op);

      if (index as u32).wrapping_add(data_offset as u32) <= 255 {
        return mem(buffer, index + data_offset);
      }

      if index < 0 {
        return mem(buffer, data_offset);
      }

      let temp = self.regs.alloc_temp(KindA64::X);
      // Safety: `self.build` 有效,`&mut *self.build` 重建唯一可变借用交给 `emit_add_offset`;单线程降级中
      // 该借用仅在本语句存活,不与 `self` 其他字段写冲突。
      unsafe {
        emit_add_offset(&mut *self.build, temp, buffer, index as usize);
      }
      return mem(temp, data_offset);
    }

    unsupported_instruction_form();
    mem(RegisterA64::NOREG, 0)
  }

  pub fn ir_lowering_a_64_temp_double(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.ir_lowering_a_64_reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let val = self.ir_lowering_a_64_double_op(op);

      // Safety: `self.build` 为 `lower_function` 由 `&mut` 注入的非空/对齐/长寿裸指针,仅调用其只读查询判定 f64 是否可 fmov。
      if unsafe { (*self.build).is_fmov_supported_fp_64(val) } {
        let temp = self.regs.alloc_temp(KindA64::D);
        // Safety: 同上,`self.build` 有效,向 builder 追加一条 fmov(f64) 指令。
        unsafe { (*self.build).fmov_register_a_64_f64(temp, val) };
        temp
      } else {
        let temp1 = self.regs.alloc_temp(KindA64::X);
        let temp2 = self.regs.alloc_temp(KindA64::D);

        let vali = get_double_bits(val);

        if (vali << 16) == 0 {
          unsafe {
            // Safety: `self.build` 为构造点注入的非空/对齐/长寿裸指针,追加 movz + fmov 指令;
            // temp1/temp2 为刚分配、仍在作用域内的临时寄存器。
            (*self.build).movz(temp1, (vali >> 48) as u16, 48);
            (*self.build).fmov_register_a_64_register_a_64(temp2, temp1);
          }
        } else if (vali << 32) == 0 {
          // Safety: 同上,`self.build` 有效,追加 movz + movk + fmov 指令,均为 builder 自身缓冲操作。
          unsafe {
            (*self.build).movz(temp1, (vali >> 48) as u16, 48);
            (*self.build).movk(temp1, (vali >> 32) as u16, 32);
            (*self.build).fmov_register_a_64_register_a_64(temp2, temp1);
          }
        } else {
          // AddressA64 为纯结构字面量(base 已就位), 先于 unsafe 构好。
          let imm_addr = AddressA64 {
            kind: AddressKindA64::Imm,
            base: temp1,
            offset: RegisterA64::NOREG,
            data: 0,
          };
          // Safety: 同上,`self.build` 有效,追加 adr(常量池写入由 builder 管理) + ldr;temp1/temp2 未释放。
          unsafe {
            (*self.build).adr_register_a_64_f64(temp1, val);
            (*self.build).ldr(temp2, imm_addr);
          }
        }

        temp2
      }
    } else {
      unsupported_instruction_form();
      RegisterA64::NOREG
    }
  }

  pub fn ir_lowering_a_64_temp_int(&mut self, op: IrOp) -> RegisterA64 {
    match op.kind() {
      IrOpKind::Inst => self.ir_lowering_a_64_reg_op(op),
      IrOpKind::Constant => {
        let temp = self.regs.alloc_temp(KindA64::W);
        let int_val = self.ir_lowering_a_64_int_op(op);
        // 经 `build_mut` 发射门面单点派生可变借用发射 mov；temp 为刚分配的活寄存器，
        // 借用语句内消费即释，单线程降低无并发 build 别名。
        self.build_mut().mov_register_a_64_i32(temp, int_val);
        temp
      }
      _ => {
        unsupported_instruction_form();
        RegisterA64::NOREG
      }
    }
  }

  pub fn ir_lowering_a_64_temp_int_64(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.ir_lowering_a_64_reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let temp = self.regs.alloc_temp(KindA64::X);
      let u: u64 = self.ir_lowering_a_64_int_64_op(op) as u64;

      // 统计非零半字（movz 路径）与非 0xFFFF 半字（movn 路径）
      let mut movz_count = 0;
      let mut movn_count = 0;
      for shift in (0..64).step_by(16) {
        let hw = (u >> shift) as u16;
        if hw != 0 {
          movz_count += 1;
        }
        if hw != 0xFFFF {
          movn_count += 1;
        }
      }

      if movz_count <= movn_count {
        // movz 路径：首个非零半字发 movz，其余发 movk
        let mut first = true;
        for shift in (0..64).step_by(16) {
          let hw = (u >> shift) as u16;
          if hw != 0 {
            if first {
              // Safety: `self.build` 为 `lower_function` 由 `&mut` 注入的非空/对齐/长寿裸指针,仅追加 movz 指令。
              unsafe { (*self.build).movz(temp, hw, shift) };
              first = false;
            } else {
              // Safety: 同上,`self.build` 有效,追加 movk 指令。
              unsafe { (*self.build).movk(temp, hw, shift) };
            }
          }
        }

        if first {
          // Safety: 同上,`self.build` 有效,常量全零时追加一条 movz 占位指令。
          unsafe { (*self.build).movz(temp, 0, 0) };
        }
      } else {
        // movn 路径：首个非 0xFFFF 半字用 movn，其余用 movk
        let mut first = true;
        for shift in (0..64).step_by(16) {
          let hw = (u >> shift) as u16;
          if hw != 0xFFFF {
            if first {
              // Safety: 同上,`self.build` 有效,追加 movn 指令(!hw 仍为 u16,不溢出)。
              unsafe { (*self.build).movn(temp, !hw, shift) };
              first = false;
            } else {
              // Safety: 同上,`self.build` 有效,追加 movk 指令。
              unsafe { (*self.build).movk(temp, hw, shift) };
            }
          }
        }

        if first {
          // Safety: 同上,`self.build` 有效,常量全 0xFFFF 时追加一条 movn 占位指令。
          unsafe { (*self.build).movn(temp, 0, 0) };
        }
      }

      temp
    } else {
      unsupported_instruction_form();
      RegisterA64::NOREG
    }
  }

  pub fn ir_lowering_a_64_temp_uint(&mut self, op: IrOp) -> RegisterA64 {
    match op.kind() {
      IrOpKind::Inst => self.ir_lowering_a_64_reg_op(op),
      IrOpKind::Constant => {
        let temp = self.regs.alloc_temp(KindA64::W);
        let value = self.ir_lowering_a_64_int_op(op) as u32;
        // Safety: self.build 为构造点接线、比 lowering 长寿的非空 *mut AssemblyBuilderA64;
        // 重建其唯一可变借用发射 mov, temp 为刚分配的活寄存器, 单线程降低无并发 build 别名。
        unsafe { (*self.build).mov_register_a_64_i32(temp, value as i32) };
        temp
      }
      _ => {
        unsupported_instruction_form();
        RegisterA64::NOREG
      }
    }
  }
}

const R_CLOSURE: RegisterA64 = reg(KindA64::X, 23);

const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

const X0: RegisterA64 = reg(KindA64::X, 0);

const X1: RegisterA64 = reg(KindA64::X, 1);

const R_BASE: RegisterA64 = reg(KindA64::X, 25);

const R_CONSTANTS: RegisterA64 = reg(KindA64::X, 22);
