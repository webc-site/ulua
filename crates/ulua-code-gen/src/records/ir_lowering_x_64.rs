use alloc::vec::Vec;
use core::{
  ffi::c_void,
  mem::{offset_of, size_of, take},
  ptr::{self, NonNull, addr_of_mut},
};

use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_vm::{
  enums::lua_type::LuaType,
  records::{
    closure::{Closure, LClosure},
    lua_table::LuaTable,
    luau_buffer::LuauBuffer,
    proto::Proto,
    udata::Udata,
  },
};

use crate::{
  enums::{
    alignment_data_x_64::AlignmentDataX64, code_gen_counter::CodeGenCounter,
    condition_x_64::ConditionX64, ir::IrValueKind, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_op_kind::IrOpKind, size_x_64::SizeX64,
  },
  functions::{
    assembly::x_64::{dword_reg, mem, qword_reg},
    cfg::predecessors,
    emit::x_64::{
      jump_if_falsy, jump_if_truthy, luau_constant_tag, luau_reg_tag, luau_reg_value,
      luau_reg_value_int, s_closure,
    },
    get_cmd_value_kind::get_cmd_value_kind,
    get_negated_condition_condition_x_64::get_negated_condition,
    ir::{produces_dirty_high_register_bits, vm_const_op, vm_exit_op, vm_reg_op},
    luau_constant_value::luau_constant_value,
    luau_reg_value_int_64::luau_reg_value_int_64,
  },
  macros::codegen_assert::{CODEGEN_ASSERT, unsupported_instruction_form},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{K_FUNCTION_ALIGNMENT, K_SPILL_SLOTS, R_STATE},
    ir_block::{IrBlock, K_BLOCK_NO_START_PC},
    ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_const::IrConst,
    ir_data::K_INVALID_INST_IDX,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64,
    ir_value_location_tracking::IrValueLocationTracking,
    label::Label,
    lowering_stats::LoweringStats,
    module::ModuleHelpers,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
    scoped_reg_x_64::ScopedRegX64,
    vm_exit::{ExitHandler, InterruptHandler},
  },
  type_aliases::ir::Instruction,
};

/// cpp `IrLoweringX64`（`CodeGen/src/IrLoweringX64.h`）。
///
/// `build/helpers/function/stats` 与 `regs`（内嵌的 `IrRegAllocX64`）共享同一组
/// 上游裸指针，是 cpp 刻意设计的别名关系；改为 `&mut` 会双 `&mut` 编译失败。
/// 保持裸指针并在构造点一次性注入，生命周期由构造点栈帧保证。
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrLoweringX64 {
  pub build: *mut AssemblyBuilderX64,
  pub helpers: *mut ModuleHelpers,

  pub function: *mut IrFunction,
  pub stats: Option<NonNull<LoweringStats>>,

  pub regs: IrRegAllocX64,

  pub value_tracker: IrValueLocationTracking,

  pub interrupt_handlers: Vec<InterruptHandler>,
  pub exit_handlers: Vec<ExitHandler>,
  pub exit_handler_map: DenseHashMap<u32, u32>,

  pub vector_and_mask: OperandX64,
  pub vector_or_mask: OperandX64,

  pub exit_sync_alloc_token: u32,
  pub exit_sync_inst_idx: u32,
}

// == IrLoweringX64 function/stats 指针洗白面（首域单元 A–E 收口尾榜）==
// 已收口（本组访问器为唯一裸解引用点）：`&self` 接收者的全部操作数取值
// （int/uint/int64/double/tag/const/import）、start_block 的 get_block_index 走 `function_ref`、
// buffer_addr 的 inst_op 读走 `function_mut`；finish_function 的 end_location 写走 `function_mut`；
// 统计的判空 + 自增走 `stats_mut`。调用点不再手写 `(*self.function)`/`(*self.stats)`。
// C1 波追加：发射视图 `build_mut`（方法链式 `(*self.build).emitX(..)` 全部走此单点）与
// 混窗元组门面 `build_regs_mut`/`function_regs_mut`（同表达式内需 build↔regs /
// function↔regs 共存别名的旁路透传点，沿 `label_op_mut_pair` 同守则）。
// lower_inst 两巨文件内 `(*self.function)` 裸读已随之清零。
// C1 接力二棒：`(&mut self.regs, &mut *self.build)` 旁路对 41 站洗白至
// `build_regs_mut`（IrCallWrapperX64 构造 20/emit_fallback 8/call_* 系 13），
// 只读操作数先拷后走 `build_mut` 单视图 3 站；其余 label 混窗经下方闭包窗门面收口。
// 尾榜收口（本波）：
//   1) 巨型混窗内 label 视图与发射借用同表达式共存的语句（FORGLOOP 系、
//      `jcc/jmp_label/jump_on_number_cmp(.., label_op_mut(..))` 实参位、`bytes` 旁路、
//      helpers 常驻 label、build↔helpers / build↔regs↔helpers 多视图形），已全部改经
//      下方 `with_op_label`/`with_target_label`/`with_helper_label`/`with_build_helpers`/
//      `with_build_regs_helpers`/`with_block_mut` 闭包窗门面；lower_inst 巨文件内
//      `unsafe` 与裸指针解引用清零，unsafe 只剩本文件访问器/门面单点。
//   2) finalize_target_label 内 sync_info/inst_op 读跨越 `record_and_free_last_use(&mut self.regs)`
//      存活——function↔regs 共存别名窗口，视图访问器会把整个 self 锁死而无法表达，维持裸指针
//      （单元 D 逐处登记）；jump_or_abort_on_undef_no_finalize 的块判定已走 `block_op_ref` 门面。
//   3) helpers 字段与 regs 共享上游别名，构造点一次性注入（见结构体注释）。
// unsafe 面收敛波：本文件内全部「逐语句 `(*self.build)` 发射」改经 `build_mut` 发射门面
// （check_safe_env/finish_function 两 log 门与 handlers 循环/increment_counter_at/
// jump_or_abort_on_undef_no_finalize/jump_or_fallthrough/mem_reg_{double,float,int_64}_op/
// store_float/vec_op/vector_and_mask_op），业务方法体 unsafe 清零；finish_function 的
// interrupt_handlers 循环以 `mem::take` 整取整还替代 iter_mut + 裸指针旁路（发射序列逐条
// 等价）；reg_op 走 `function_regs_mut` 混窗元组门面；proto 读数走 `IrFunction::proto_view`
// 安全边界（空指针由 Option 拦下，原为 UB 解引用）；`block_op`/`label_op` 由 `pub unsafe fn`
// 收敛为安全包装（op 合法性由 `IrFunction::block_op` 内部断言兜底，unsafe 收进 fn 内一处
// 局部解引用）；`jump_if_branch_op`（build↔双 label 三视图同表达式）保留为登记的单一窗口。
impl IrLoweringX64 {
  // 共享裸指针访问器家族（function_mut/function_ref/build_mut/stats_mut）由宏收口，
  // 统一契约见 `crate::shared_ptr_accessors!`；本结构特有的混窗/闭包窗门面在下方手写。
  crate::shared_ptr_accessors!(AssemblyBuilderX64);

  /// 混窗元组门面（旁路透传专用）：`emit_builtin` 等自由函数在同一调用里同时需要
  /// build 与 regs 的可变视图，逐字段访问器会双 `&mut self` 冲突；此处一次派生两个
  /// 同源借用，与门面化前的 `(&mut *self.build, &mut self.regs)` 实参列逐位等价。
  /// Safety: 两条借用皆即时消费于同一条调用语句、不长期持有；build↔regs 上游别名
  /// 为 cpp 刻意设计（见结构体注释），单线程串行降低下无第二活跃别名。
  #[inline]
  pub(crate) fn build_regs_mut(&mut self) -> (&mut AssemblyBuilderX64, &mut IrRegAllocX64) {
    // Safety:见函数注释。
    unsafe { (&mut *self.build, &mut self.regs) }
  }

  /// 混窗元组门面：`free_last_use_reg(&mut function 的 inst_op 视图, ..)` 形态的
  /// function↔regs 共存借用（单元 D 登记的别名窗口的即时消费形）。
  /// Safety: 同 `build_regs_mut`；两条借用即时消费、语句内结束。
  #[inline]
  pub(crate) fn function_regs_mut(&mut self) -> (&mut IrFunction, &mut IrRegAllocX64) {
    // Safety:见函数注释。
    unsafe { (&mut *self.function, &mut self.regs) }
  }

  /// `label_op` 的安全取址门面：返回块 `.label` 裸指针（指针契约见
  /// [`Self::label_op`]），供 `with_target_label`/配对门面消费。
  #[inline]
  pub(crate) fn op_label_ptr(&mut self, op: IrOp) -> *mut Label {
    // Safety:无——`label_op` 已收敛为安全包装（op 合法性由 `IrFunction::block_op`
    // 内部断言兜底），此处仅透传其返回指针。
    self.label_op(op)
  }

  /// 双分支 label 视图门面（自 lower_inst 迁入）：跳转类指令在同一调用里同时拿
  /// taken/fallback 两个块 label，两次 `label_op_mut` 会双 `&mut self` 冲突；此处一次
  /// 调用产出两个块 label 的可变视图。
  #[inline]
  pub(crate) fn label_op_mut_pair(&mut self, a: IrOp, b: IrOp) -> (&mut Label, &mut Label) {
    // Safety: `a`/`b` 为同一指令的 taken/fallback 两个 Block 操作数，`label_op` 返回指向
    // 各自块 `.label` 字段的非空指针。二者可能指向同一块（退化 CFG 与模糊测试合法输入，
    // 上游 C++ 同款语义）：指针相等时两个可变视图别名同一字段，但下游 `jump_if_*` 族经
    // `jcc` 只登记 jump 指针、不写 label 本体，单线程串行降低下与门面化前的双裸指针重建
    // 逐位等价，故不设互异性断言（unit C 曾加 debug_assert_ne! 致 226 例回归，已除）。
    let pa = self.op_label_ptr(a);
    let pb = self.op_label_ptr(b);
    // Safety: 两指针皆非空、对齐、块存活期内；借用即时消费于同一条调用语句。
    unsafe { (&mut *pa, &mut *pb) }
  }

  /// 双分支 truthy/falsy 跳转门面（自 lower_inst 迁入，混窗·尾榜1）：同一调用同时
  /// 持有 taken/fallback 两个块 label 的可变视图，且与 `build` 别名并存。
  pub(crate) fn jump_if_branch_op(
    &mut self,
    truthy: bool,
    test: IrOp,
    taken: IrOp,
    fallback: IrOp,
  ) {
    let build = self.build;
    let (a, b) = self.label_op_mut_pair(taken, fallback);
    // Safety: `build` 为构造点接线、非空且比 lowering 长寿的裸指针（瞬时读字段复制），
    // 指向存活的 AssemblyBuilderX64，本调用期间无其他对 build 的活跃借用；`vm_reg_op`
    // 只读取操作数打包值。
    unsafe {
      if truthy {
        jump_if_truthy(&mut *build, vm_reg_op(test), a, b);
      } else {
        jump_if_falsy(&mut *build, vm_reg_op(test), a, b);
      }
    }
  }

  /// 闭包窗门面（原『混窗·尾榜1』收口）：取 `op` 块的 `.label` 可变视图并与
  /// `&mut self` 同窗交给 `emit`。发射调用点形如 `jcc/jmp_label(.., label_op_mut(..))`
  /// 时，实参位的 `&mut Label` 长借用会把 `self` 整体锁死、无法经普通访问器表达。
  #[inline]
  pub(crate) fn with_op_label<R>(
    &mut self,
    op: IrOp,
    emit: impl FnOnce(&mut Self, &mut Label) -> R,
  ) -> R {
    // Safety: `op_label_ptr` 依 `label_op` 契约返回指向 `function.blocks` 活元素
    // `.label` 字段的非空对齐指针；重建的可变借用仅于 `emit` 语句内存活，块数组活过
    // 整个 lowering 栈帧，单线程串行降低下该块与 builder 无其他活跃别名。
    let label = self.op_label_ptr(op);
    emit(self, unsafe { &mut *label })
  }

  /// `get_target_label`/`op_label_ptr` 返回指针的消费窗门面：重建 `&mut Label` 并与
  /// `&mut self` 同窗交给 `emit`。上游契约：指针在取回与消费之间不发生 `exit_handlers`
  /// 重分配。
  #[inline]
  pub(crate) fn with_target_label<R>(
    &mut self,
    target: *mut Label,
    emit: impl FnOnce(&mut Self, &mut Label) -> R,
  ) -> R {
    // Safety: `target` 出自 `get_target_label`/`op_label_ptr`——指向调用方 `fresh`（活
    // 局部）、`exit_handlers` 元素的 `self_` 字段（窗口内无重分配）或 `label_op`（块存活
    // 期），皆非空、对齐；重建借用仅在 `emit` 语句内存活，单线程串行无其他活跃别名。
    emit(self, unsafe { &mut *target })
  }

  /// helpers 常驻 label 的消费窗门面（原 `let helpers = self.helpers;` + 裸指针旁路）：
  /// 从 `ModuleHelpers` 取 `pick` 选中的 `Label` 字段，与 `&mut self` 同窗交给 `emit`。
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

  /// build↔helpers 双视图门面（原『混窗·尾榜3』`emit_inst_return` 形）。
  /// Safety: 两条借用即时消费于同一条调用语句；build/helpers 上游别意为 cpp 刻意设计
  /// （见结构体注释），单线程串行降低下无第二活跃别名。
  #[inline]
  pub(crate) fn with_build_helpers<R>(
    &mut self,
    emit: impl FnOnce(&mut AssemblyBuilderX64, &mut ModuleHelpers) -> R,
  ) -> R {
    // Safety:见函数注释。
    unsafe { emit(&mut *self.build, &mut *self.helpers) }
  }

  /// build↔regs↔helpers 三视图门面（原『混窗·尾榜3』`emit_inst_call` 形）。
  /// Safety: 三条借用即时消费于同一条调用语句；同 `with_build_helpers` 与
  /// `build_regs_mut` 的守则。
  #[inline]
  pub(crate) fn with_build_regs_helpers<R>(
    &mut self,
    emit: impl FnOnce(&mut AssemblyBuilderX64, &mut IrRegAllocX64, &mut ModuleHelpers) -> R,
  ) -> R {
    // Safety:见函数注释。
    unsafe { emit(&mut *self.build, &mut self.regs, &mut *self.helpers) }
  }

  /// 按 `op` 取块的可变视图并与 `&mut self` 同窗交给 `emit`
  /// （`jump_or_fallthrough_op` 形的『块视图 + self 方法』混窗收口）。
  #[inline]
  pub(crate) fn with_block_mut<R>(
    &mut self,
    op: IrOp,
    emit: impl FnOnce(&mut Self, &mut IrBlock) -> R,
  ) -> R {
    // Safety: `block_op` 依其契约（op 为 Block 操作数且下标合法）返回指向
    // `function.blocks` 活元素的非空对齐指针；重建借用仅于 `emit` 语句内存活，单线程
    // 串行降低下该块无其他活跃可变别名。
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

    // 发射日志门经 `build_mut` 发射门面派生借用（契约见宏与结构体注释），语句内即释。
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

  /// cpp `IrLoweringX64::blockOp`（`src/IrLoweringX64.h:75` `IrBlock& blockOp(IrOp op) const`）
  /// → `function.blockOp(op)`。
  ///
  /// 上游是 const 成员函数通过引用成员改写：Rust 侧对应"裸指针字段的内部可变性"，
  /// 因此接收者保持 `&self`（与上游 const 一致），但**只从裸指针字段派生** `&mut`，
  /// 不再用 `self as *const Self as *mut Self` 把 `&self` 伪造成 `&mut Self`（那是 mut_from_ref UB，
  /// 会让调用方持有的 `&self` 失效）。
  ///
  /// 返回裸指针而不是 `&mut IrBlock`：同一条指令的 lowering 需要在拿着本块的同时
  /// 借用 `self.build` 发射跳转，引用形态会把整个 `self` 锁住。
  ///
  /// 安全包装：op 合法性（`IrOpKind::Block` + 下标界内）由 `IrFunction::block_op` 内部
  /// 断言与越界检查兜底，调用方无需 unsafe；本 fn 内部保留一处局部 unsafe（`function`
  /// 字段解引用）。返回的裸指针契约：指向 `function.blocks` 活元素，不得越过 lowering
  /// 生命周期使用，重建借用须遵守各消费窗门面的 Safety 论证。
  pub fn block_op(&self, op: IrOp) -> *mut IrBlock {
    // 只复制裸指针字段（共享读），再从该指针构造访问；不经由 &self 伪造 &mut Self
    let function = self.function;
    // Safety: function 为自构造点接线、比 lowering 长寿的非空 *mut IrFunction(本行只是拷贝该
    // 裸指针字段, 不产生借用); 在此前提下 block_op 返回指向 function.blocks 元素的指针,
    // op 合法性由其内部断言兜底, 单线程降低无并发 &mut 别名。
    unsafe { (*function).block_op(op) }
  }

  /// [`Self::block_op`] 的只读共享视图门面：判别/读块字段（kind、startpc 等）的站点
  /// 不再在调用点手写 `&*` 裸解引用，共享视图重建收口到本处。
  pub fn block_op_ref(&self, op: IrOp) -> &IrBlock {
    // Safety: 沿用 `block_op` 契约（op 为 Block 操作数且下标合法），其返回非空/对齐/指向
    // function.blocks 活元素的指针；此处仅降级为共享借用读取字段，单线程串行降低中调用点
    // 表达式内瞬时重建、不与其它可变别名重叠。
    unsafe { &*self.block_op(op) }
  }

  pub fn buffer_addr_op(&mut self, buffer_op: IrOp, index_op: IrOp, tag: u8) -> OperandX64 {
    CODEGEN_ASSERT!(tag == LuaType::UserData as u8 || tag == LuaType::Buffer as u8);
    // 对齐 cpp 的 offsetof(Buffer, data)/offsetof(Udata, data), 由编译器计算
    let data_offset = if tag == LuaType::Buffer as u8 {
      offset_of!(LuauBuffer, data)
    } else {
      offset_of!(Udata, data)
    };

    if index_op.kind() == IrOpKind::Inst {
      // 视图访问器即时重建 &mut 借用读 cmd, 语句内消费(契约见 records impl 注释)。
      let cmd = self.function_mut().inst_op(index_op).cmd;
      CODEGEN_ASSERT!(!produces_dirty_high_register_bits(cmd));

      let buffer_reg = self.reg_op(buffer_op);
      let index_reg = self.reg_op(index_op);
      let scaled_index = qword_reg(index_reg);
      return OperandX64::mem(
        SizeX64::Qword,
        scaled_index,
        1,
        buffer_reg,
        data_offset as i32,
      );
    } else if index_op.kind() == IrOpKind::Constant {
      let buffer_reg = self.reg_op(buffer_op);
      let index_val = self.int_op(index_op);
      return OperandX64::mem(
        SizeX64::Qword,
        RegisterX64::NOREG,
        1,
        buffer_reg,
        index_val + data_offset as i32,
      );
    }

    unsupported_instruction_form();
    OperandX64::reg(RegisterX64::NOREG)
  }

  /// cpp `IrCallWrapperX64 callWrap(regs, build, index)` 构造门面：混窗元组内一次派生
  /// build↔regs 双视图并绑定为调用包装器（不推实参）。
  ///
  /// # Safety（沿 [`Self::build_regs_mut`] 契约）
  /// 两条借用皆即时消费于本调用、不长期持有；build↔regs 上游别名为 cpp 刻意设计
  /// （见结构体注释），单线程串行降低下无第二活跃别名。
  #[inline]
  pub(crate) fn call_wrap(&mut self, index: u32) -> IrCallWrapperX64 {
    let (build, regs) = self.build_regs_mut();
    IrCallWrapperX64::new(regs, build, index)
  }

  /// [`Self::call_wrap`] 的最常用形态：构造后立即推入固定首实参 `lua_State*`
  /// （cpp 侧每个 helper 调用的 `callWrap.addArgument(SizeX64::qword, luaState)`）。
  ///
  /// # Safety（沿 [`Self::build_regs_mut`] 契约）
  /// 同 [`Self::call_wrap`]。
  #[inline]
  pub(crate) fn call_wrap_state(&mut self, index: u32) -> IrCallWrapperX64 {
    let mut call_wrap = self.call_wrap(index);
    call_wrap.add_argument_op(SizeX64::Qword, OperandX64::reg(R_STATE), IrOp::default());
    call_wrap
  }

  pub fn check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
    let tmp = self.alloc_scoped_reg(SizeX64::Qword);
    let tmp_reg = OperandX64::reg(tmp.reg);
    // env/safeenv 访存槽为编译期 offset_of! 常量, 先于发射构好。
    let env_slot = mem(SizeX64::Qword, tmp.reg, offset_of!(Closure, env) as i32);
    let safeenv_slot = mem(SizeX64::Byte, tmp.reg, offset_of!(LuaTable, safeenv) as i32);

    // 各发射点经 `build_mut` 发射门面派生借用、块内顺序取得并释放（契约见宏与结构体
    // 注释）；tmp 持有 self.regs 内活槽（owner 为裸指针、不借入 self）、其借用已独立于本块。
    {
      let build = self.build_mut();
      build.mov(tmp_reg, s_closure());
      build.mov(tmp_reg, env_slot);
      build.cmp(safeenv_slot, OperandX64::imm(0));
    }

    self.jump_or_abort_on_undef_condition(ConditionX64::Equal, target, index, next);
  }

  /// cpp `IrLoweringX64::constOp` -> `function->constOp(op)`（读取 IrConst）
  ///
  /// 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
  pub fn const_op(&self, op: IrOp) -> IrConst {
    self.function_ref().const_op(op)
  }

  /// cpp `IrLoweringX64::doubleOp` -> `function->doubleOp(op)`（读取 Number 常量）
  ///
  /// 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
  pub fn double_op(&self, op: IrOp) -> f64 {
    self.function_ref().double_op(op)
  }

  pub fn finalize_target_label(&mut self, op: IrOp, index: u32, fresh: &mut Label) {
    if op.kind() == IrOpKind::Block && self.block_op_ref(op).kind == IrBlockKind::ExitSync {
      // 若分支经 jumpOrAbortOnUndefNoFinalize 发出，校验其后没有新分配
      if self.exit_sync_inst_idx == index {
        CODEGEN_ASSERT!(self.exit_sync_alloc_token == self.regs.alloc_action_count);
      }

      // 快照 exit sync 所需值的当前寄存器/spill 位置，并在最后 use 处释放寄存器
      // 保留: sync_info 与循环体内的 inst_op 视图需跨越 record_and_free_last_use(&mut self.regs)
      // 存活——function↔regs 共存别名窗口, 视图访问器会把整个 self 锁住而无法表达, 维持裸指针
      // 解引用(单元 D 登记)。
      // Safety: self.function 有效(同接线不变量); vm_exit_info.find(&index) 仅只读查找返回 Option,
      // 派生共享借用在本表达式求值后即结束。
      let sync_info = unsafe { (*self.function).vm_exit_info.find(&index) };
      CODEGEN_ASSERT!(sync_info.is_some());
      // 不变式（cpp 同源 LUAU_ASSERT+解引用）：ExitSync 块必由先前 VmExit 建过同步点，
      // find 失配即 IR 同步信息被破坏，属编译器内部不变量。
      let sync_info =
        sync_info.expect("ExitSync 块对应的 vm_exit_info 同步记录必存在（cpp 同源断言）");

      for arg_op in &sync_info.arg_ops {
        // Safety: self.function 有效; arg_op 为 sync_info.arg_ops 登记的合法 Inst 操作数, inst_op 据其下标
        // 取回指令, 只读不越过指令表, 单线程降级无别名冲突。
        let inst_op = unsafe { (*self.function).inst_op(*arg_op) };
        self
          .regs
          .record_and_free_last_use(op.index(), inst_op, index);
      }
    } else if op.kind() == IrOpKind::VmExit && fresh.id != 0 {
      let exit_handler_idx = self.exit_handlers.len() as u32;
      *self.exit_handler_map.get_or_insert(vm_exit_op(op)) = exit_handler_idx;
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

    // cpp 按引用遍历并就地绑定 label：整体取出、就地发射后写回——发射借用经
    // `build_mut`/`with_helper_label` 门面派生，与原 iter_mut + 裸指针旁路逐条等价。
    let mut handlers = take(&mut self.interrupt_handlers);
    for handler in handlers.iter_mut() {
      let build = self.build_mut();
      build.set_label_label(&mut handler.self_);
      build.mov(
        OperandX64::reg(dword_reg(RegisterX64::RAX)),
        OperandX64::imm((handler.pcpos + 1) as i32),
      );
      build.lea_label(RegisterX64::RBX, &mut handler.next);
      self.with_helper_label(
        |helpers| &mut helpers.interrupt,
        |this, label| this.build_mut().jmp_label(label),
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
      self.build_mut().set_label_label(&mut handler.self_);

      if handler.pcpos == K_VM_EXIT_ENTRY_GUARD_PC {
        self.alloc_and_increment_counter_at(CodeGenCounter::VmExitTaken, !0u32);

        self.with_helper_label(
          |helpers| &mut helpers.exit_continue_vm_clear_native_flag,
          |this, label| this.build_mut().jmp_label(label),
        );
      } else {
        self.alloc_and_increment_counter_at(CodeGenCounter::VmExitTaken, handler.pcpos);

        self.build_mut().mov(
          OperandX64::reg(dword_reg(RegisterX64::RDX)),
          OperandX64::imm((handler.pcpos * size_of::<Instruction>() as u32) as i32),
        );
        self.with_helper_label(
          |helpers| &mut helpers.update_pc_and_continue_in_vm,
          |this, label| this.build_mut().jmp_label(label),
        );
      }
    }
    self.exit_handlers = handlers;

    let mut end = Label::default();
    // 设 label 与读回偏移在同一借用窗口内顺序完成。
    let end_offset = {
      let build = self.build_mut();
      build.set_label(&mut end);
      build.get_label_offset(&end)
    };
    self.function_mut().end_location = end_offset;
    // 发射 undefined 指令作中止跳转位。
    self.build_mut().ud_2();

    // stats 判空+解引用样板收口到 stats_mut 访问器; regs 读数为安全字段直取。
    let max_used_slot = self.regs.max_used_slot;
    if let Some(stats) = self.stats_mut() {
      // 与 has_error 同一判据: 超出 spill 容量即记 regAllocErrors (cpp IrLoweringX64.cpp:3803)
      if max_used_slot > K_SPILL_SLOTS {
        stats.reg_alloc_errors += 1;
      }

      if max_used_slot > stats.max_spill_slots_used {
        stats.max_spill_slots_used = max_used_slot;
      }
    }
  }

  /// cpp `Label* getTargetLabel(IrOp op, size_t index, Label& fresh)`（IrLoweringX64.cpp）。
  /// 返回裸指针的理由与 `get_target_label` 相同：label 可来自
  /// `fresh`/`exit_handlers`/`function`，需在后续 `&mut self` 调用间存活（别名窗口）。
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

    // 到达此分支时 op 必为 Block 下标（Undef/VmExit 已提前返回），
    // 与 cpp getTargetLabel 的前置条件一致；`label_op` 内部断言兜底 op 合法性。
    self.label_op(op)
  }

  /// 对齐 cpp `IrLoweringX64::hasError`：寄存器分配器使用的 8 字节 spill 槽
  /// 超过栈帧预留的 `kSpillSlots` 时放弃本函数编译并回退解释器。
  pub fn has_error(&self) -> bool {
    self.regs.max_used_slot > K_SPILL_SLOTS
  }

  /// cpp `IrLoweringX64::importOp` -> `function->importOp(op)`（读取 import 下标）
  ///
  /// 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
  pub fn import_op(&self, op: IrOp) -> u32 {
    self.function_ref().import_op(op)
  }

  pub fn increment_counter_at(&mut self, offset: usize) {
    // 计数器写入依赖 entry 帧的 Closure 局部槽 (sClosure), 前提是被编译函数
    // 确为携带有效 proto 的 L 闭包; 空指针由 proto_view 的 Option 边界显式拦下
    // （原实现此处为 UB 解引用）。
    let sizecode = self
      .function_ref()
      .proto_view()
      .expect("计数器发射要求被编译函数为携带有效 proto 的 L 闭包")
      .sizecode;

    let tmp = self.alloc_scoped_reg(SizeX64::Qword);
    let tmp_reg = OperandX64::reg(tmp.reg);

    // 三个内存操作数为纯构造(偏移全部来自编译期 offset_of! 与栈槽读数)。
    let p_slot = OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      tmp.reg,
      (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32,
    );
    let execdata_slot = OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      tmp.reg,
      offset_of!(Proto, execdata) as i32,
    );
    let counter_slot = OperandX64::mem(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      tmp.reg,
      (((sizecode as u32) + (offset as u32)) * 4) as i32,
    );

    // 各发射经 `build_mut` 发射门面派生借用、块内顺序取得并释放（契约见宏与结构体
    // 注释）；tmp 为刚分配、在该帧内存活的寄存器, s_closure() 读取当前 entry 帧的活栈槽。
    {
      let build = self.build_mut();
      // Get counter slot — cpp: build.mov(tmp.reg, sClosure), 即 qword[rsp + kStackOffsetToLocals]
      build.mov(tmp_reg, s_closure());
      build.mov(tmp_reg, p_slot);
      build.mov(tmp_reg, execdata_slot);
      // 自增
      build.inc(counter_slot);
    }
  }

  /// cpp `IrLoweringX64::int64Op` -> `function->int64Op(op)`（读取 Int64 常量）
  ///
  /// 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
  pub fn int64_op(&self, op: IrOp) -> i64 {
    self.function_ref().int64_op(op)
  }

  /// cpp `IrLoweringX64::intOp` -> `function->intOp(op)`（读取 Int 常量）
  ///
  /// 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
  pub fn int_op(&self, op: IrOp) -> i32 {
    self.function_ref().int_op(op)
  }

  pub fn new(
    build: &mut AssemblyBuilderX64,
    helpers: &mut ModuleHelpers,
    function: &mut IrFunction,
    stats: Option<NonNull<LoweringStats>>,
  ) -> Self {
    let regs = IrRegAllocX64::new(build, function, stats);
    let value_tracker = IrValueLocationTracking::new(function);
    let exit_handler_map = DenseHashMap::new(!0u32);

    let self_ = Self {
      build: ptr::from_mut(build),
      helpers: ptr::from_mut(helpers),
      function: ptr::from_mut(function),
      stats,
      regs,
      value_tracker,
      interrupt_handlers: Vec::new(),
      exit_handlers: Vec::new(),
      exit_handler_map,
      vector_and_mask: OperandX64::reg(RegisterX64::NOREG),
      vector_or_mask: OperandX64::reg(RegisterX64::NOREG),
      exit_sync_alloc_token: 0,
      // IrLoweringX64.h:112 `uint32_t exitSyncInstIdx = kInvalidInstIdx;`
      // 必须是 kInvalidInstIdx 而非 0, 否则函数第 0 条指令上的 ExitSync 分支
      // 会被误判为"同一指令已记录过 token", 跳过 token 快照。
      exit_sync_inst_idx: K_INVALID_INST_IDX,
    };

    build.align(K_FUNCTION_ALIGNMENT, AlignmentDataX64::Ud2);

    self_
  }

  pub fn reset_restore_callback(&mut self) {
    let regs_ptr = ptr::from_mut(&mut self.regs).cast::<c_void>();
    self
      .value_tracker
      .set_restore_callback(regs_ptr, Some(Self::restore_callback_shim));
  }

  /// C 回调 shim：还原寄存器后执行 restore 回调。
  /// # Safety
  /// `context` 指向注册时写入的闭包上下文，`inst` 指向存活指令。
  unsafe fn restore_callback_shim(context: *mut c_void, inst: *mut IrInst) {
    // Safety: context 由 reset_restore_callback 以 `ptr::from_mut(&mut self.regs).cast::<c_void>()` 注册,
    // 因此指向在回调登记期间持续存活的 IrRegAllocX64; 回调在单线程串行 lowering 中同步触发,
    // 此刻无其它借用持有该 regs, `&mut *(context.cast::<IrRegAllocX64>())` 的重借用无别名冲突。
    // inst 为调用方传入的活指令, `&mut *inst` 仅在 restore 调用期内临时借用。
    unsafe {
      let regs = &mut *(context.cast::<IrRegAllocX64>());
      regs.restore(&mut *inst, false);
    }
  }

  pub fn is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
    target.start == next.start
  }

  pub fn jump_or_abort_on_undef_condition(
    &mut self,
    cond: ConditionX64,
    target: IrOp,
    index: u32,
    next: &IrBlock,
  ) {
    let mut fresh = Label::default();
    self.jump_or_abort_on_undef_no_finalize(cond, target, index, next, &mut fresh);
    self.finalize_target_label(target, index, &mut fresh);
  }

  pub fn jump_or_abort_on_undef_ir_op_u32_ir_block(
    &mut self,
    target: IrOp,
    index: u32,
    next: &IrBlock,
  ) {
    self.jump_or_abort_on_undef_condition(ConditionX64::Count, target, index, next);
  }

  pub fn jump_or_abort_on_undef_no_finalize(
    &mut self,
    cond: ConditionX64,
    target: IrOp,
    index: u32,
    next: &IrBlock,
    fresh: &mut Label,
  ) {
    if target.kind() == IrOpKind::Block && self.block_op_ref(target).kind == IrBlockKind::ExitSync {
      let token = self.regs.alloc_action_count;

      if self.exit_sync_inst_idx != index {
        self.exit_sync_inst_idx = index;
        self.exit_sync_alloc_token = token;
      } else {
        CODEGEN_ASSERT!(self.exit_sync_alloc_token == token);
      }
    }

    let label = self.get_target_label(target, index, fresh);

    // label 为 get_target_label 返回的指向存活 Label(目标块标签或本次 fresh)的非空指针;
    // 各发射经 `build_mut` 发射门面派生借用、配对经 `with_target_label` 消费窗门面，
    // 块判定/should_jump 等安全读数(block_op_ref/is_fallthrough_block)在发射之间顺序求值。
    if target.kind() == IrOpKind::Undef {
      if cond == ConditionX64::Count {
        self.build_mut().ud_2();
      } else {
        self.with_target_label(label, |this, label| {
          let build = this.build_mut();
          build.jcc(get_negated_condition(cond), label);
          build.ud_2();
          build.set_label(label);
        });
      }
    } else if cond == ConditionX64::Count {
      let should_jump = if target.kind() == IrOpKind::VmExit {
        true
      } else {
        !self.is_fallthrough_block(self.block_op_ref(target), next)
      };

      if should_jump {
        self.with_target_label(label, |this, label| this.build_mut().jmp_label(label));
      }
    } else {
      self.with_target_label(label, |this, label| this.build_mut().jcc(cond, label));
    }
  }

  pub fn jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
    if !self.is_fallthrough_block(target, next) {
      // `build_mut` 发射门面派生可变借用登记跳转; target 为调用方传入的活块可变借用,
      // 与 build 互不别名（A64 侧同款）。
      self.build_mut().jmp_label(&mut target.label);
    }
  }

  /// cpp `IrLoweringX64::labelOp`（`src/IrLoweringX64.h:76` `Label& labelOp(IrOp op) const`）
  /// → `blockOp(op).label`。
  ///
  /// 与 A64 侧不同，这里返回裸指针：X64 的分支 lowering 会把同一条指令的两个 label
  /// 作为一次 `cmov/jcc` 调用的多个实参传入，`&mut Label` 会让两个来自同一 `self` 的可变借用共存。
  ///
  /// 保留登记（b14 #29 单元 C）：调用点单 label 可变访问一律走 `label_op_mut`/成对走
  /// `label_op_mut_pair` 门面；本裸指针边界仅服务于门面内部与三类别名窗口站点
  /// （`get_target_label` 返回值、FORGLOOP 的 `emit_inst_for_g_loop` 实参、JumpSlot* 的
  /// mismatch——均在两次以上 `&mut self` 调用之间持有 label，借用检查无法表达）。
  ///
  /// 安全包装：op 合法性（`IrOpKind::Block` + 下标界内）由 `IrFunction::block_op` 内部
  /// 断言与越界检查兜底，调用方无需 unsafe；本 fn 内部保留一处局部 unsafe（`function`
  /// 字段解引用）。返回的裸指针契约：指向 `function.blocks` 活元素的 `.label` 字段，
  /// 不得越过 lowering 生命周期使用，重建借用须遵守各消费窗门面的 Safety 论证。
  pub fn label_op(&self, op: IrOp) -> *mut Label {
    // Safety: 契约(op 为 Block 且下标合法, 由 IrFunction::block_op 内部断言兜底)保证
    // block_op 返回 function.blocks 内的非空活块指针; 解引用取 .label 字段地址
    // (addr_of_mut! 仅取址不移动)落在该块存活期内, 单线程降低无别名冲突。
    unsafe { addr_of_mut!((*self.block_op(op)).label) }
  }

  pub fn mem_reg_double_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => OperandX64::reg(self.reg_op(op)),
      IrOpKind::Constant => {
        let imm = self.double_op(op);
        // double_op 对 function 的只读借用已结束, 此刻经 `build_mut` 门面重建 build 的
        // 唯一可变借用发射立即数, 单线程降低无并发 build &mut 别名。
        self.build_mut().f64(imm)
      }
      IrOpKind::VmReg => luau_reg_value(vm_reg_op(op)),
      IrOpKind::VmConst => luau_constant_value(vm_const_op(op)),
      _ => {
        CODEGEN_ASSERT!(false, "Unsupported operand kind");
        OperandX64::reg(RegisterX64::NOREG)
      }
    }
  }

  pub fn mem_reg_float_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => {
        // 视图访问器即时派生共享借用按 op.index() 有界访问 instructions 取回 .cmd(Copy),
        // 越界仍按原语义 panic(契约见 records impl 注释)。
        let cmd = self.function_ref().instructions[op.index() as usize].cmd;
        CODEGEN_ASSERT!(get_cmd_value_kind(cmd) == IrValueKind::Float);
        OperandX64::reg(self.reg_op(op))
      }
      IrOpKind::Constant => {
        let double_val = self.double_op(op);
        let float_val = double_val as f32;
        // float_val 为已算好的操作数值, 经 `build_mut` 门面向 builder 追加一条浮点常量。
        self.build_mut().f32(float_val)
      }
      _ => {
        CODEGEN_ASSERT!(false, "Unsupported operand kind");
        OperandX64::reg(RegisterX64::NOREG)
      }
    }
  }

  pub fn mem_reg_int_64_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => OperandX64::reg(self.reg_op(op)),
      IrOpKind::Constant => {
        let imm = self.int64_op(op);
        // int64_op 对 function 的只读借用已结束, 此刻经 `build_mut` 门面重建 build 的
        // 唯一可变借用发射 i64 立即数, 单线程降低无并发 build &mut 别名。
        self.build_mut().i64(imm)
      }
      IrOpKind::VmReg => luau_reg_value_int_64(vm_reg_op(op)),
      IrOpKind::VmConst => luau_constant_value(vm_const_op(op)),
      _ => {
        CODEGEN_ASSERT!(false, "Unsupported operand kind");
        OperandX64::reg(RegisterX64::NOREG)
      }
    }
  }

  pub fn mem_reg_int_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => OperandX64::reg(self.reg_op(op)),
      IrOpKind::Constant => OperandX64::imm(self.int_op(op)),
      IrOpKind::VmReg => luau_reg_value_int(vm_reg_op(op)),
      _ => {
        CODEGEN_ASSERT!(false);
        OperandX64::reg(RegisterX64::NOREG)
      }
    }
  }

  pub fn mem_reg_tag_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => OperandX64::reg(self.reg_op(op)),
      IrOpKind::VmReg => luau_reg_tag(vm_reg_op(op)),
      IrOpKind::VmConst => luau_constant_tag(vm_const_op(op)),
      _ => {
        CODEGEN_ASSERT!(false);
        OperandX64::reg(RegisterX64::NOREG)
      }
    }
  }

  pub fn mem_reg_uint_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => {
        let reg = self.reg_op(op);
        OperandX64::reg(reg)
      }
      IrOpKind::Constant => {
        let imm = self.int_op(op) as u32;
        OperandX64::imm(imm as i32)
      }
      IrOpKind::VmReg => {
        let ri = vm_reg_op(op);
        luau_reg_value_int(ri)
      }
      _ => {
        CODEGEN_ASSERT!(false);
        OperandX64::reg(RegisterX64::NOREG)
      }
    }
  }

  pub fn reg_op(&mut self, op: IrOp) -> RegisterX64 {
    // 指令视图需跨越 `restore(&mut regs)` 存活——function↔regs 共存别名窗口，
    // 经混窗元组门面一次派生两条借用（守则见 `function_regs_mut`）。
    let (function, regs) = self.function_regs_mut();
    let inst = function.inst_op(op);

    if inst.spilled || inst.needs_reload {
      regs.restore(inst, false);
    }

    CODEGEN_ASSERT!(inst.reg_x64 != RegisterX64::NOREG);
    inst.reg_x64
  }

  /// cpp `ScopedRegX64(IrRegAllocX64& owner, SizeX64 size)`（`IrRegAllocX64.h:122`）：
  /// 分配一个作用域寄存器，出作用域由 `ScopedRegX64::drop` 自动归还。
  pub fn alloc_scoped_reg(&mut self, size: SizeX64) -> ScopedRegX64 {
    ScopedRegX64::with_size(&mut self.regs, size)
  }

  /// cpp `ScopedRegX64(IrRegAllocX64& owner)`（`IrRegAllocX64.h:121`）：
  /// 先占位（reg = noreg），后续按条件 `alloc()` / `take()`。
  pub fn scoped_reg(&mut self) -> ScopedRegX64 {
    ScopedRegX64::new(&mut self.regs)
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

  pub fn store_float(&mut self, dst: OperandX64, src: IrOp) {
    // 各发射经 `build_mut` 发射门面派生借用、语句内即释（契约见宏与结构体注释）；
    // 调用前先取安全方法(alloc_scoped_reg/double_op/function_mut/reg_op)的读数。
    if src.kind() == IrOpKind::Constant {
      let tmp = self.alloc_scoped_reg(SizeX64::Xmmword);
      let float_val = self.double_op(src) as f32;
      let tmp_reg = OperandX64::reg(tmp.reg);
      let build = self.build_mut();
      let const_op = build.f32(float_val);
      build.vmovss_operand_x_64_operand_x_64(tmp_reg, const_op);
      build.vmovss_operand_x_64_operand_x_64(dst, tmp_reg);
    } else if src.kind() == IrOpKind::Inst {
      let cmd = self.function_mut().inst_op(src).cmd;
      CODEGEN_ASSERT!(get_cmd_value_kind(cmd) == IrValueKind::Float);
      let src_reg = OperandX64::reg(self.reg_op(src));
      self
        .build_mut()
        .vmovss_operand_x_64_operand_x_64(dst, src_reg);
    } else {
      unsupported_instruction_form();
    }
  }

  /// cpp `IrLoweringX64::tagOp` -> `function->tagOp(op)`（读取 tag 常量）
  ///
  /// 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
  pub fn tag_op(&self, op: IrOp) -> u8 {
    self.function_ref().tag_op(op)
  }

  /// cpp `IrLoweringX64::uintOp` -> `function->uintOp(op)`（读取 UInt 常量）
  ///
  /// 裸指针解引用收口在 `function_ref` 视图访问器（见 records 的 impl 注释契约）。
  pub fn uint_op(&self, op: IrOp) -> u32 {
    self.function_ref().uint_op(op)
  }

  pub fn vec_op(&mut self, op: IrOp, tmp: &mut ScopedRegX64) -> RegisterX64 {
    // 只读本条指令的 cmd（Copy），借用语句内即释；后续 reg_op 内部会再取同一条指令并
    // 可能触发 restore，与原「持裸指针视图跨调用」的读数序列逐位等价。
    let cmd = self.function_mut().inst_op(op).cmd;

    CODEGEN_ASSERT!(cmd != IrCmd::SUBSTITUTE);

    if cmd != IrCmd::LoadTvalue && cmd != IrCmd::GetUpvalue && cmd != IrCmd::TagVector {
      return self.reg_op(op);
    }

    tmp.alloc(SizeX64::Xmmword);
    let dst = OperandX64::reg(tmp.reg);
    let src1 = OperandX64::reg(self.reg_op(op));
    let src2 = self.vector_and_mask_op();
    // vandps 经 `build_mut` 发射门面派生唯一可变借用追加一条 SIMD 指令,
    // dst/src1/src2 均为已分配的寄存器与常量池操作数, 单线程降级无别名。
    self.build_mut().vandps(dst, src1, src2);
    tmp.reg
  }

  #[inline]
  pub(crate) fn vector_and_mask_op(&mut self) -> OperandX64 {
    // cpp IrLoweringX64.cpp:4203 `if (vectorAndMask.base == noreg)`：
    // 初值为 noreg（Rust 端 NOREG，bits=0x80），而非 0xFF
    if self.vector_and_mask.base == RegisterX64::NOREG {
      // `build_mut` 发射门面派生唯一可变借用发射常量向量, self 处于 &mut 独占、
      // 单线程降低无并发 build 别名。
      self.vector_and_mask = AssemblyBuilderX64::u32x4(self.build_mut(), !0u32, !0u32, !0u32, 0u32);
    }

    self.vector_and_mask
  }
}

const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;
