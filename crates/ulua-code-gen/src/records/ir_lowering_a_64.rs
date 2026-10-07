use alloc::vec::Vec;
use core::{
  ffi::c_void,
  mem::{offset_of, size_of, take},
  ptr::{self, NonNull, addr_of_mut},
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
    condition_a_64::ConditionA64, ir::IrValueKind, ir_block_kind::IrBlockKind,
    ir_op_kind::IrOpKind, kind_a_64::KindA64,
  },
  functions::{
    assembly::cast_reg,
    bit_utils::get_double_bits,
    cfg::predecessors,
    emit::a_64::emit_abort,
    emit_add_offset::emit_add_offset,
    get_cmd_value_kind::get_cmd_value_kind,
    ir::{is_gco, produces_dirty_high_register_bits, vm_const_op, vm_exit_op, vm_reg_op},
  },
  macros::codegen_assert::{CODEGEN_ASSERT, unsupported_instruction_form},
  records::{
    address_a_64::AddressA64,
    assembly_builder_a_64::{AssemblyBuilderA64, K_MAX_IMMEDIATE},
    ir_block::{IrBlock, K_BLOCK_NO_START_PC},
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    ir_reg_alloc_a_64::IrRegAllocA64,
    ir_value_location_tracking::IrValueLocationTracking,
    label::Label,
    lowering_stats::LoweringStats,
    module::ModuleHelpers,
    register_a_64::{RegisterA64, reg},
    vm_exit::{ExitHandler, InterruptHandler},
  },
  type_aliases::{ir::Instruction, mem::mem},
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
  pub stats: Option<NonNull<LoweringStats>>,
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
// 本波（unsafe 面收敛）：本文件内全部「逐语句 `(*self.build)` 发射」改经 `build_mut`
// 发射门面（check_object_barrier_conditions/check_safe_env/finish_function 两 log 门与
// handlers 循环/increment_counter_at/temp_addr/temp_addr_buffer/temp_double/temp_int_64/
// temp_uint），业务方法体 unsafe 清零；finish_function 的 interrupt_handlers 循环以
// `mem::take` 整取整还替代 iter_mut + 裸指针旁路（发射序列逐条等价）；reg_op 走
// `function_regs_mut` 混窗元组门面；proto 读数走 `IrFunction::proto_view` 安全边界
// （空指针由 Option 拦下，原为 UB 解引用）；`block_op` 由 `pub unsafe fn`
// 收敛为安全包装（op 合法性由 `IrFunction::block_op` 内部断言兜底，unsafe 收进 fn 内
// 一处局部解引用）。
// 尾榜保留（非本业务文件可清）：
//   1) 巨型混窗内 label 视图与发射借用同表达式共存（`cbz/cbnz/b/b_cond(..,
//      self.label_op(..))` 形——`&mut Label` 长借用会把 self 整体锁死无法表达）的站点，
//      已由业务文件逐处裸指针重建改为经下方 `with_op_label`/`with_target_label`/
//      `with_helper_label`/`with_block_mut` 闭包窗门面，unsafe 全部收口于本文件门面。
//   2) finalize_target_label 内 sync_info/inst_op 读跨越 `record_and_free_last_use(&mut self.regs)`
//      存活——function↔regs 共存别名窗口，视图访问器会把整个 self 锁死而无法表达，维持裸指针
//      （单元 D 逐处登记）。
//   3) helpers 字段与 regs 共享上游别名，构造点一次性注入（见结构体注释）。
//   4) `restore_callback_shim`：value_tracker 的 C 回调 shim（opaque 上下文重建），本质边界。
impl IrLoweringA64 {
  // 共享裸指针访问器家族（function_mut/function_ref/build_mut/stats_mut）由宏收口，
  // 统一契约见 `crate::shared_ptr_accessors!`；本结构特有的混窗/闭包窗门面在下方手写。
  crate::shared_ptr_accessors!(AssemblyBuilderA64);

  /// [`Self::build_function_regs_mut`] 的 function↔regs 二元形态：`reg_op` 形态的
  /// 指令视图需跨越 `restore_reg(&mut regs)` 存活——function↔regs 共存别名窗口，
  /// 单视图访问器会把整个 self 锁死而无法表达（与 x64 侧 `function_regs_mut` 同守则）。
  /// Safety: 两条借用皆即时消费于同一条调用语句、不长期持有；function↔regs 的上游
  /// 别名为 cpp 刻意设计（见结构体注释），单线程串行降低下无第二活跃别名。
  #[inline]
  pub(crate) fn function_regs_mut(&mut self) -> (&mut IrFunction, &mut IrRegAllocA64) {
    // Safety:见函数注释。
    unsafe { (&mut *self.function, &mut self.regs) }
  }

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
    // Safety: `label_op` 依其契约（op 为 Block 操作数且下标合法）返回指向
    // `function.blocks` 活元素 `.label` 字段的可变借用，非空、对齐；此处经裸指针把该寿命
    // 与 `&mut self` 解绑——块数组活过整个 lowering 栈帧，借用窗即时于 `emit` 语句内消费，
    // 单线程串行降低下该块与 builder 无其他活跃别名。
    let label = self.label_op(op) as *mut Label;
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
    // Safety: `block_op` 依其契约（op 为 Block 操作数且下标合法）返回指向
    // `function.blocks` 活元素的非空对齐指针；重建借用仅于 `emit` 语句内存活，单线程串行
    // 降低下该块无其他活跃可变别名。
    let block = unsafe { &mut *self.block_op(op) };
    emit(self, block)
  }

  pub(crate) fn alloc_and_increment_counter_at(&mut self, kind: CodeGenCounter, pcpos: u32) {
    // 视图访问器把 function 的每次访问收敛为语句内即时借用(契约见 records impl 注释);
    // increment_counter_at 只发射指令、不改写 extra_native_data, 故先取出 len 再跨调用无别名风险。
    // build 保留裸指针读取(AssemblyBuilder 发射门面波处理)。
    if !self.function_ref().record_counters {
      return;
    }

    // 发射日志门经 `build_mut` 发射门面派生借用（契约见宏与结构体注释），借用语句内即释。
    let build = self.build_mut();
    if build.log_text {
      build.log_append(format_args!(
        "; counter kind {} at pcpos {}\n",
        kind as u32, pcpos
      ));
    }

    self.function_mut().extra_native_data.push(kind as u32);
    self.function_mut().extra_native_data.push(pcpos);
    let len = self.function_ref().extra_native_data.len();
    self.increment_counter_at(len);
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
  /// 安全包装：op 合法性（`IrOpKind::Block` + 下标界内）由 `IrFunction::block_op` 内部
  /// 断言与越界检查兜底，调用方无需 unsafe；本 fn 内部保留一处局部 unsafe（`function`
  /// 字段解引用）。返回的裸指针契约：指向 `function.blocks` 活元素，不得越过 lowering
  /// 生命周期使用，重建借用须遵守各消费窗门面的 Safety 论证。
  pub(crate) fn block_op(&self, op: IrOp) -> *mut IrBlock {
    // 只复制裸指针字段（共享读），再从该指针构造访问
    let function = self.function;
    // Safety: self.function 为构造接线的非空 IrFunction*（arena 不变量：地址恒定、比持有者
    // 长寿），(*function).block_op(op) 解引用取存活对象；op 合法性由其内部断言兜底。
    unsafe { (*function).block_op(op) }
  }

  /// [`Self::block_op`] 的只读共享视图门面：判别/读块字段（kind、startpc 等）
  /// 的站点不再在调用点手写 `&*` 裸解引用，共享视图重建收口到本处。
  pub fn block_op_ref(&self, op: IrOp) -> &IrBlock {
    // Safety: 沿用 `block_op` 契约（op 为 Block 操作数且下标合法），其返回
    // 非空/对齐/指向 function.blocks 活元素的指针；此处仅降级为共享借用读取字段，单线程
    // 串行降低中调用点表达式内瞬时重建、不与其它可变别名重叠。
    unsafe { &*self.block_op(op) }
  }

  pub fn check_object_barrier_conditions(
    &mut self,
    object: RegisterA64,
    temp: RegisterA64,
    ra: RegisterA64,
    ra_op: IrOp,
    ratag: i32,
    skip: &mut Label,
  ) {
    let tempw = cast_reg(KindA64::W, temp);

    // 各发射点一律经 `build_mut` 发射门面派生借用、语句内即释；offset_of! 皆编译期
    // 常量，skip 指向调用方持有且借用期内有效的 Label；temp_addr 为安全方法（内部
    // 自管 build 借用），先于本块各发射点调用，顺序与拆分前一致。
    if ratag == -1 || !is_gco(ratag as u8) {
      if ra_op.kind() == IrOpKind::Inst {
        self.build_mut().umov_4s(tempw, ra, 3);
      } else {
        let addr = self.temp_addr(ra_op, offset_of!(TValue, tt) as i32, temp);
        self.build_mut().ldr(tempw, addr);
      }

      let build = self.build_mut();
      build.cmp_u16(tempw, LuaType::String as u16);
      build.b_cond(ConditionA64::Less, skip);
    }

    {
      let build = self.build_mut();
      build.ldrb(tempw, mem(object, offset_of!(GCheader, marked) as i32));
      build.tbz(tempw, BLACKBIT as u8, skip);
    }

    if ra_op.kind() == IrOpKind::Inst {
      self.build_mut().fmov_rr(temp, cast_reg(KindA64::D, ra));
    } else {
      let addr = self.temp_addr(ra_op, offset_of!(TValue, value) as i32, temp);
      self.build_mut().ldr(temp, addr);
    }

    let build = self.build_mut();
    build.ldrb(tempw, mem(temp, offset_of!(GCheader, marked) as i32));
    build.tst_imm(tempw, bit2mask(WHITE0BIT, WHITE1BIT) as u32);
    build.b_cond(ConditionA64::Equal, skip);
  }

  pub fn check_safe_env(&mut self, target: IrOp, index: u32, _next: &IrBlock) {
    let mut fresh = Label::default();
    let temp: RegisterA64 = self.regs.alloc_temp(KindA64::X);
    let tempw: RegisterA64 = cast_reg(KindA64::W, temp);
    // env/safeenv 访存偏移为编译期 offset_of! 常量，先于发射在安全域算好。
    let offset_env = offset_of!(ClosureAlias, env) as i32;
    let offset_safeenv = offset_of!(LuaTableAlias, safeenv) as i32;
    {
      let build = self.build_mut();
      build.ldr(temp, mem(R_CLOSURE, offset_env));
      build.ldrb(tempw, mem(temp, offset_safeenv));
    }
    let label = self.get_target_label(target, index, &mut fresh);
    self.with_target_label(label, |this, label| this.build_mut().cbz(tempw, label));
    self.finalize_target_label(target, index, &mut fresh);
  }

  pub(crate) fn double_op(&self, op: IrOp) -> f64 {
    // 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
    self.function_ref().double_op(op)
  }

  pub fn finalize_target_label(&mut self, op: IrOp, index: u32, fresh: &mut Label) {
    if op.kind() == IrOpKind::Undef {
      // `build_mut` 发射门面派生唯一可变借用交给 `emit_abort` 追加中止指令；借用语句内即释。
      emit_abort(self.build_mut(), fresh);
    } else if op.kind() == IrOpKind::Block && self.block_op_ref(op).kind == IrBlockKind::ExitSync {
      if self.exit_sync_inst_idx == index {
        CODEGEN_ASSERT!(self.exit_sync_alloc_token == self.regs.alloc_action_count);
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

  pub fn finish_block(&mut self, curr: &IrBlock, next: &IrBlock) {
    if !self.regs.spills.is_empty() {
      // 共享视图访问器即时派生借用，只读块下标/前驱/块 kind，全程安全（契约见宏注释）。
      let function = self.function_ref();
      let next_idx = function.get_block_index(next);
      let curr_idx = function.get_block_index(curr);

      for pred_idx in predecessors(&function.cfg, next_idx) {
        let pred_block = &function.blocks[pred_idx as usize];
        CODEGEN_ASSERT!(pred_idx == curr_idx || pred_block.kind == IrBlockKind::Dead);
      }

      CODEGEN_ASSERT!(next.use_count == 1);
    }
  }

  pub fn finish_function(&mut self) {
    // 发射日志门与各发射点一律经 `build_mut` 发射门面派生借用（契约见宏与结构体注释），
    // 语句内即释；helpers 常驻 label 经 `with_helper_label` 门面与发射借用同窗共存。
    // function/stats 的读写已收口到 function_mut/stats_mut 视图访问器。
    let build = self.build_mut();
    if build.log_text {
      build.log_append(format_args!("; interrupt handlers\n"));
    }

    // cpp 按引用遍历并就地绑定 label：写回真实元素，避免拷贝丢失 location。此处整体
    // 取出、就地发射后写回——发射借用经 `build_mut`/`with_helper_label` 门面派生，
    // 与原 iter_mut + 裸指针旁路的发射序列逐条等价。
    let mut handlers = take(&mut self.interrupt_handlers);
    for handler in handlers.iter_mut() {
      let build = self.build_mut();
      build.set_label_label(&mut handler.self_);
      build.mov_imm(
        X0,
        ((handler.pcpos + 1) * size_of::<Instruction>() as u32) as i32,
      );
      build.adr_label(X1, &mut handler.next);
      self.with_helper_label(
        |helpers| &mut helpers.interrupt,
        |this, label| this.build_mut().b_label(label),
      );
    }
    self.interrupt_handlers = handlers;

    let build = self.build_mut();
    if build.log_text {
      build.log_append(format_args!("; exit handlers\n"));
    }

    // 取出整表就地发射后写回（同上方 interrupt_handlers），消除下标遍历与拷贝
    let mut handlers = take(&mut self.exit_handlers);
    for handler in handlers.iter_mut() {
      // 两分支同首句 set_label_label, 提到分支外（不依赖 pcpos, 行为等价）。
      self.build_mut().set_label_label(&mut handler.self_);

      if handler.pcpos == K_VM_EXIT_ENTRY_GUARD_PC {
        self.alloc_and_increment_counter_at(CodeGenCounter::VmExitTaken, !0u32);

        self.with_helper_label(
          |helpers| &mut helpers.exit_continue_vm_clear_native_flag,
          |this, label| this.build_mut().b_label(label),
        );
      } else {
        self.alloc_and_increment_counter_at(CodeGenCounter::VmExitTaken, handler.pcpos);

        self
          .build_mut()
          .mov_imm(X0, (handler.pcpos * size_of::<Instruction>() as u32) as i32);
        self.with_helper_label(
          |helpers| &mut helpers.update_pc_and_continue_in_vm,
          |this, label| this.build_mut().b_label(label),
        );
      }
    }
    self.exit_handlers = handlers;

    // 设 label 与读回偏移在同一借用窗口内顺序完成。
    let end_offset = {
      let build = self.build_mut();
      let end = build.set_label();
      build.get_label_offset(&end)
    };
    self.function_mut().end_location = end_offset;
    // 发射 undefined 指令作中止跳转位。
    self.build_mut().udf();

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
  pub fn get_target_label(&mut self, op: IrOp, _index: u32, fresh: &mut Label) -> *mut Label {
    if op.kind() == IrOpKind::Undef {
      return fresh;
    }

    if op.kind() == IrOpKind::VmExit {
      if let Some(index) = self.exit_handler_map.find(&vm_exit_op(op)) {
        return &raw mut self.exit_handlers[*index as usize].self_;
      }

      return fresh;
    }

    self.label_op(op)
  }

  pub fn has_error(&self) -> bool {
    self.error || self.regs.error
  }

  pub(crate) fn increment_counter_at(&mut self, offset: usize) {
    let temp1 = self.regs.alloc_temp(KindA64::X);
    let temp2 = self.regs.alloc_temp(KindA64::X);

    // 访存偏移均为编译期 offset_of! 常量, 先在安全域算好。
    let p_off = (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32;
    let execdata_off = offset_of!(Proto, execdata) as i32;

    // 计数器发射仅针对携带有效 proto 的 L 闭包触发（与 x64 兄弟方法入口同一前提），
    // 空指针由 proto_view 的 Option 边界显式拦下（原实现此处为 UB 解引用）。
    let sizecode = self
      .function_ref()
      .proto_view()
      .expect("计数器发射要求被编译函数为携带有效 proto 的 L 闭包")
      .sizecode as usize;
    let counter_offset = (sizecode + offset) * 4;

    // 各发射点经 `build_mut` 发射门面派生借用、块内顺序取得并释放（契约见宏与结构体
    // 注释）；temp1/temp2 为刚分配、块末即 free_temp 归还的活寄存器。
    {
      let build = self.build_mut();
      build.ldr(temp1, mem(R_CLOSURE, p_off));
      build.ldr(temp1, mem(temp1, execdata_off));
      emit_add_offset(build, temp2, temp1, counter_offset);
      build.ldr(temp1, mem(temp2, 0));
      build.add_rr_u16(temp1, temp1, 1);
      build.str(temp1, mem(temp2, 0));
    }

    self.regs.free_temp(temp1);
    self.regs.free_temp(temp2);
  }

  pub(crate) fn int_64_op(&self, op: IrOp) -> i64 {
    // 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
    self.function_ref().int64_op(op)
  }

  pub(crate) fn int_op(&self, op: IrOp) -> i32 {
    // 转发到 IrFunction 的常量取值(原为自递归存根,运行即栈溢出)；裸指针收口在 `function_ref`。
    self.function_ref().int_op(op)
  }

  pub fn setup_restore_callback(&mut self) {
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
    // Safety: context 由 setup_restore_callback 以 `ptr::from_mut(self).cast::<c_void>()` 注册,
    // 指向回调登记期间持续存活的 IrLoweringA64; 回调在单线程串行 lowering 中同步触发, 此刻无其它借用
    // 持有该 self, 故 `&mut *(context.cast::<IrLoweringA64>())` 的重借用无别名冲突。
    // inst 为调用方传入的活指令, `&mut *inst` 仅在 restore_reg 调用期内临时借用。
    unsafe {
      let self_ = &mut *(context.cast::<IrLoweringA64>());
      self_.regs.restore_reg(&mut *inst);
    }
  }

  pub fn is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
    target.start == next.start
  }

  pub fn jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
    if !self.is_fallthrough_block(target, next) {
      // 经 `build_mut` 发射门面单点派生可变借用；target 为活块可变借用（外部参数，
      // 不经 self），与 build 不相互别名。
      self.build_mut().bl(&mut target.label);
    }
  }

  pub fn label_op(&mut self, op: IrOp) -> &mut Label {
    // Safety: 前置条件(op 为 Block 且下标合法)保证 block_op 返回指向
    // function.blocks 活元素的非空指针; 取 .label 字段地址重建 &mut 落在块存活期内, 接收者
    // 为 &mut self 独占、单线程降低, 不与其他对同一 block 的可变借用共存。
    unsafe { &mut *addr_of_mut!((*self.block_op(op)).label) }
  }

  pub fn reg_op(&mut self, op: IrOp) -> RegisterA64 {
    // 指令视图需跨越 `restore_reg(&mut regs)` 存活——function↔regs 共存别名窗口，
    // 经混窗元组门面一次派生两条借用（守则见 `function_regs_mut`）。
    let (function, regs) = self.function_regs_mut();
    let inst = function.inst_op(op);

    if inst.spilled || inst.needs_reload {
      regs.restore_reg(inst);
    }

    CODEGEN_ASSERT!((inst.reg_a64 != RegisterA64::NOREG));
    inst.reg_a64
  }

  pub fn start_block(&mut self, curr: &IrBlock) {
    if curr.startpc != K_BLOCK_NO_START_PC {
      let counter = if curr.kind == IrBlockKind::Fallback {
        CodeGenCounter::FallbackBlockExecuted
      } else {
        CodeGenCounter::RegularBlockExecuted
      };
      self.alloc_and_increment_counter_at(counter, curr.startpc);
    }

    if curr.kind == IrBlockKind::ExitSync {
      // 视图访问器即时派生共享借用读取 blocks 下标, 语句内消费(契约见 records impl 注释)。
      let block_index = self.function_ref().get_block_index(curr);
      self.regs.setup_exit_sync_entry(block_index);
    }
  }

  pub fn temp_addr(&mut self, op: IrOp, offset: i32, temp_storage: RegisterA64) -> AddressA64 {
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

      // `build_mut` 发射门面派生唯一可变借用交给 `emit_add_offset`; 单线程降级中该借用
      // 仅在本表达式求值期间存活, 不与 `self` 其他字段写冲突（契约见宏与结构体注释）。
      emit_add_offset(self.build_mut(), temp, R_CONSTANTS, constant_offset);
      return mem(temp, 0);
    } else if op.kind() == IrOpKind::Inst {
      // 视图访问器即时重建 &mut 借用读 cmd, 语句内消费(契约见 records impl 注释)。
      let cmd = self.function_mut().inst_op(op).cmd;
      CODEGEN_ASSERT!(get_cmd_value_kind(cmd) == IrValueKind::Pointer);
      return mem(self.reg_op(op), offset);
    }

    unsupported_instruction_form();
    mem(RegisterA64::NOREG, 0)
  }

  pub fn temp_addr_buffer(&mut self, buffer_op: IrOp, index_op: IrOp, tag: u8) -> AddressA64 {
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
      let buffer = self.reg_op(buffer_op);
      let index = self.reg_op(index_op);
      // `build_mut` 发射门面派生唯一可变借用追加一条 add 指令; temp/buffer/index 均为已分配寄存器。
      self.build_mut().add_rrr_i32(temp, buffer, index, 0);
      return mem(temp, data_offset);
    } else if index_op.kind() == IrOpKind::Constant {
      let buffer = self.reg_op(buffer_op);
      // 视图访问器即时派生共享借用读取常量(契约见 records impl 注释)。
      let index = self.function_ref().int_op(index_op);

      if (index as u32).wrapping_add(data_offset as u32) <= 255 {
        return mem(buffer, index + data_offset);
      }

      if index < 0 {
        return mem(buffer, data_offset);
      }

      let temp = self.regs.alloc_temp(KindA64::X);
      // `build_mut` 发射门面派生唯一可变借用交给 `emit_add_offset`; 单线程降级中
      // 该借用仅在本表达式求值期间存活, 不与 `self` 其他字段写冲突（契约见宏与结构体注释）。
      emit_add_offset(self.build_mut(), temp, buffer, index as usize);
      return mem(temp, data_offset);
    }

    unsupported_instruction_form();
    mem(RegisterA64::NOREG, 0)
  }

  pub fn temp_double(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let val = self.double_op(op);

      // `build_mut` 发射门面派生的借用均为只读查询或单条发射、语句内即释（契约见宏与
      // 结构体注释）；temp1/temp2 为刚分配、仍在作用域内的临时寄存器。
      if self.build_mut().is_fmov_supported_fp_64(val) {
        let temp = self.regs.alloc_temp(KindA64::D);
        self.build_mut().fmov_f64(temp, val);
        temp
      } else {
        let temp1 = self.regs.alloc_temp(KindA64::X);
        let temp2 = self.regs.alloc_temp(KindA64::D);

        let vali = get_double_bits(val);

        if (vali << 16) == 0 {
          let build = self.build_mut();
          build.movz(temp1, (vali >> 48) as u16, 48);
          build.fmov_rr(temp2, temp1);
        } else if (vali << 32) == 0 {
          let build = self.build_mut();
          build.movz(temp1, (vali >> 48) as u16, 48);
          build.movk(temp1, (vali >> 32) as u16, 32);
          build.fmov_rr(temp2, temp1);
        } else {
          // AddressA64 为纯结构字面量(base 已就位), 先于发射构好。
          let imm_addr = AddressA64 {
            kind: AddressKindA64::Imm,
            base: temp1,
            offset: RegisterA64::NOREG,
            data: 0,
          };
          let build = self.build_mut();
          build.adr_f64(temp1, val);
          build.ldr(temp2, imm_addr);
        }

        temp2
      }
    } else {
      unsupported_instruction_form();
      RegisterA64::NOREG
    }
  }

  pub fn temp_int(&mut self, op: IrOp) -> RegisterA64 {
    match op.kind() {
      IrOpKind::Inst => self.reg_op(op),
      IrOpKind::Constant => {
        let temp = self.regs.alloc_temp(KindA64::W);
        let int_val = self.int_op(op);
        // 经 `build_mut` 发射门面单点派生可变借用发射 mov；temp 为刚分配的活寄存器，
        // 借用语句内消费即释，单线程降低无并发 build 别名。
        self.build_mut().mov_imm(temp, int_val);
        temp
      }
      _ => {
        unsupported_instruction_form();
        RegisterA64::NOREG
      }
    }
  }

  pub fn temp_int_64(&mut self, op: IrOp) -> RegisterA64 {
    if op.kind() == IrOpKind::Inst {
      self.reg_op(op)
    } else if op.kind() == IrOpKind::Constant {
      let temp = self.regs.alloc_temp(KindA64::X);
      let u: u64 = self.int_64_op(op) as u64;

      // 统计非零半字（movz 路径）与非 0xFFFF 半字（movn 路径）：单遍 fold 半字序列，
      // 免手写累加循环（review.md §3 手动整数累加 → fold）。
      let (movz_count, movn_count) = (0..64)
        .step_by(16)
        .map(|shift| (u >> shift) as u16)
        .fold((0usize, 0usize), |(movz, movn), hw| {
          (movz + (hw != 0) as usize, movn + (hw != 0xFFFF) as usize)
        });

      if movz_count <= movn_count {
        // movz 路径：首个非零半字发 movz，其余发 movk；各发射经 `build_mut` 门面语句内即释。
        let mut first = true;
        for shift in (0..64).step_by(16) {
          let hw = (u >> shift) as u16;
          if hw != 0 {
            if first {
              self.build_mut().movz(temp, hw, shift);
              first = false;
            } else {
              self.build_mut().movk(temp, hw, shift);
            }
          }
        }

        if first {
          // 常量全零时补一条 movz 占位指令。
          self.build_mut().movz(temp, 0, 0);
        }
      } else {
        // movn 路径：首个非 0xFFFF 半字用 movn，其余用 movk
        let mut first = true;
        for shift in (0..64).step_by(16) {
          let hw = (u >> shift) as u16;
          if hw != 0xFFFF {
            if first {
              // movn 实参 !hw 仍为 u16, 不溢出。
              self.build_mut().movn(temp, !hw, shift);
              first = false;
            } else {
              self.build_mut().movk(temp, hw, shift);
            }
          }
        }

        if first {
          // 常量全 0xFFFF 时补一条 movn 占位指令。
          self.build_mut().movn(temp, 0, 0);
        }
      }

      temp
    } else {
      unsupported_instruction_form();
      RegisterA64::NOREG
    }
  }

  pub fn temp_uint(&mut self, op: IrOp) -> RegisterA64 {
    match op.kind() {
      IrOpKind::Inst => self.reg_op(op),
      IrOpKind::Constant => {
        let temp = self.regs.alloc_temp(KindA64::W);
        let value = self.int_op(op) as u32;
        // `build_mut` 发射门面派生唯一可变借用发射 mov, temp 为刚分配的活寄存器,
        // 单线程降低无并发 build 别名。
        self.build_mut().mov_imm(temp, value as i32);
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
