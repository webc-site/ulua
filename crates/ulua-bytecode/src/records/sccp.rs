use core::mem;
use std::{collections::VecDeque, vec::Vec};

use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  functions::is_jump_d::is_jump_d,
  macros::luau_assert::LUAU_ASSERT,
  records::{
    dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault,
    small_vector::SmallVector,
  },
};

use crate::{
  enums::{
    bc_block_edge_kind::BcBlockEdgeKind, bc_block_flag::BcBlockFlag, bc_imm_kind::BcImmKind,
    bc_op_kind::BcOpKind, bc_vm_const_kind::BcVmConstKind,
  },
  records::{
    bc_block_edge::BcBlockEdge, bc_function::BcFunction, bc_imm::BcImm, bc_op::BcOp,
    bc_op_hash::BcOpHash, bc_vm_const::BcVmConst,
  },
  type_aliases::bc_edges::BcEdges,
};

// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。

/// 常量格上的取值：cpp `Constness` 标签枚举与 `ConstnessLattice`（kind + 两个
/// `std::optional` 模拟联合体）合一，标签即数据，非法组合（如 kind 为 VmConstant
/// 却不带常量）在类型上不可表示，故消费侧无需 `unwrap`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constness {
  /// 格顶
  Undetermined,
  /// 格底
  NotAConstant,
  /// VM 常量（cpp `vmConst`）
  VmConstant(BcOp),
  /// 立即数常量（cpp `immConst`）
  ImmConstant(BcImm),
}

/// 常量格值的按值视图（cpp `Constness::constant` 是单个 `BcOp`，Rust 格中
/// Imm 侧只携带立即数值、无池句柄，故以此枚举统一传值口径）
#[derive(Debug, Clone, Copy)]
pub enum ConstValue {
  /// 常量池句柄（`BcOpKind::VmConst`）
  Vm(BcOp),
  /// 立即数值本身
  Imm(BcImm),
}

impl Constness {
  /// cpp `Constness::constant`（BcOp）的值侧视图：Imm 格携带立即数值本身，
  /// VmConst 格携带常量池句柄（读值需 func 解引用）。
  pub(crate) fn const_value(&self) -> Option<ConstValue> {
    match self {
      Self::ImmConstant(imm) => Some(ConstValue::Imm(*imm)),
      Self::VmConstant(op) => Some(ConstValue::Vm(*op)),
      _ => None,
    }
  }

  /// cpp `ConstnessLattice::merge`：Undetermined 是格顶；两个相等常量 meet 到自身，其余落到格底。
  pub fn merge(&self, other: &Self) -> Self {
    if matches!(self, Self::Undetermined) {
      return *other;
    }
    if matches!(other, Self::Undetermined) {
      return *self;
    }
    if self == other {
      return *self;
    }
    Self::NotAConstant
  }
}

impl DenseDefault for Constness {
  fn dense_default() -> Self {
    Self::Undetermined
  }
}

/// cpp `ConditionState`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionState {
  AlwaysFalse,
  AlwaysTrue,
  Unknown,
}

/// cpp `JumpTarget`：分支的一个落点；dead 表示条件解析后该路径不可达。
#[derive(Debug, Clone, Copy)]
pub struct JumpTarget {
  pub dead: bool,
  pub block_op: BcOp,
  /// cpp 结构体镜像保留：cpp 侧同样只写不读；Rust 消费端用 dead/block_op 即足
  pub condition: ConditionState,
}

/// cpp `DenseHashSet<BcOp>` 的 DenseHashMap 值包装（块 → 到达它的块集合）。
#[derive(Debug)]
pub(crate) struct SccpBlockUses(pub DenseHashSet<BcOp, BcOpHash>);

impl DenseDefault for SccpBlockUses {
  fn dense_default() -> Self {
    Self(DenseHashSet::new(BcOp::new()))
  }
}

impl SccpBlockUses {
  pub fn insert(&mut self, op: BcOp) {
    self.0.insert(op);
  }

  pub fn contains(&self, op: &BcOp) -> bool {
    self.0.contains(op)
  }

  pub fn len(&self) -> usize {
    self.0.size()
  }

  pub fn is_empty(&self) -> bool {
    self.0.empty()
  }
}

/// Sccp 全程状态：cpp 中分散在 `SccpState` + `Sccp` 字段 + `BcInst::uses`。
/// def→uses 反向边在此自建（从图一次性推导，随重写同步），语义与 cpp `BcInst::uses` 恒等。
#[derive(Debug)]
pub struct SccpState {
  pub op_constness: DenseHashMap<BcOp, Constness>,
  pub op_uses: DenseHashMap<BcOp, Vec<BcOp>>,
  pub(crate) block_uses: DenseHashMap<u32, SccpBlockUses>,
  pub flow_worklist: VecDeque<BcOp>,
  pub(crate) flow_worklist_set: DenseHashSet<BcOp, BcOpHash>,
  pub ssa_worklist: VecDeque<BcOp>,
}

impl SccpState {
  pub fn new() -> Self {
    Self {
      op_constness: DenseHashMap::new(BcOp::new()),
      op_uses: DenseHashMap::new(BcOp::new()),
      block_uses: DenseHashMap::new(u32::MAX),
      flow_worklist: VecDeque::new(),
      flow_worklist_set: DenseHashSet::new(BcOp::new()),
      ssa_worklist: VecDeque::new(),
    }
  }

  /// cpp `SccpState::operandLattice`：寄存器类操作数恒为格底，其余查格。
  pub(crate) fn operand_lattice(&mut self, op: BcOp) -> Constness {
    if matches!(
      op.kind,
      BcOpKind::Proj | BcOpKind::VmReg | BcOpKind::VmUpvalue
    ) {
      return Constness::NotAConstant;
    }
    *self.op_constness.get_or_insert(op)
  }

  /// 未解析条件的格：任一操作数为格底则格底，否则格顶。
  pub(crate) fn unknown_condition_constness(&mut self, ops: &[BcOp]) -> Constness {
    for &op in ops {
      if matches!(self.operand_lattice(op), Constness::NotAConstant) {
        return Constness::NotAConstant;
      }
    }
    Constness::Undetermined
  }

  /// 反向边记录：def 仅接受 Inst/Phi 消费者（对齐 cpp `recordUse`）。
  pub(crate) fn record_use(&mut self, def: BcOp, user: BcOp) {
    if matches!(def.kind, BcOpKind::Inst | BcOpKind::Phi) {
      self.op_uses.get_or_insert(def).push(user);
    }
  }

  /// 反向边解除：从 def 的使用列表移除 user（对齐 cpp `eraseUse`）。
  pub(crate) fn erase_use(&mut self, user: BcOp, def: BcOp) {
    if let Some(uses) = self.op_uses.find_mut(&def) {
      uses.retain(|u| *u != user);
    }
  }

  pub(crate) fn uses_of(&self, def: BcOp) -> &[BcOp] {
    self.op_uses.get(&def).map_or(&[], |v| v.as_slice())
  }

  /// 把 def 的全部使用点压入 SSA 工作表。
  ///
  /// 字段级拆分借用（op_uses 只读出、ssa_worklist 写入），调用方无需为绕开
  /// 借用冲突而 `to_vec` 快照——visit 阶段每次格值变化都要走这里，属热路径。
  pub(crate) fn defer_uses_to_ssa(&mut self, def: BcOp) {
    if let Some(uses) = self.op_uses.find_mut(&def) {
      self.ssa_worklist.extend(uses.iter().copied());
    }
  }
}

impl Default for SccpState {
  fn default() -> Self {
    Self::new()
  }
}

/// 三路比较：lhs < rhs 为 -1，相等为 0，大于为 1。
/// 供 VmConst 实现与 imm 比较臂共用，避免各写一份 `from(lv > rv) - from(lv < rv)`。
pub(crate) fn three_way<T: PartialOrd>(a: T, b: T) -> i32 {
  i32::from(a > b) - i32::from(a < b)
}

/// cpp `VmConstOps` 虚接口的 Rust 镜像。实现集封闭于本 crate（`BcVmConstImpl`），
/// 故以泛型静态分发替代 dyn；`func` 以参数传递而非持有引用，规避可变借用冲突。
pub trait VmConstOps {
  /// 折叠双常量算术（cpp `isNumber`：Imm-Int 与 VmConst-Number 任意组合升
  /// double 折叠为 Number 常量）；任一侧非数值或不支持的 opcode 返回 None
  fn evaluate(
    &self,
    func: &mut BcFunction,
    lhs: ConstValue,
    rhs: ConstValue,
    op: LuauOpcode,
  ) -> Option<BcOp>;
  fn falsey(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool;
  /// VmConst×VmConst 三路比较：lhs < rhs 为 -1，相等为 0，大于为 1；
  /// 种类对无定义关系时返回 None，调用方须视为 Unknown 而非相等
  /// （cpp eq 对 Vector/Table 等返回 nullopt，compare 仅在 isOrderable 守卫内可达）
  fn cmp_ops(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> Option<i32>;
  /// VmConst×Imm 三路比较；种类不可比较（如 String×Int-imm）时返回 None，
  /// 调用方须视为 Unknown 而非相等（cpp `eq` 对此类对返回 nullopt）
  fn cmp_imm(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcImm) -> Option<i32>;
  fn make_nil(&self, func: &mut BcFunction<'_>) -> BcOp;
  fn make_imm_bool(&self, value: bool) -> BcImm;
  fn make_imm_int(&self, value: i32) -> BcImm;
  /// 仅数值类常量（number/integer/string）支持排序比较
  fn is_orderable(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool;
  fn kind_equals(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> bool;
  /// 比较不受支持时返回 None
  fn eq_ops(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> Option<bool>;
  /// 该 VmConst 是否为 NaN（仅 Number kind 可能为 true；imm 侧无 NaN）
  fn is_nan_const(&self, func: &mut BcFunction<'_>, vm: BcOp) -> bool;
  fn eq_int(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: i32) -> Option<bool>;
  /// 仅 LUA_TNUMBER
  fn is_arithmetic_constant(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool;
  fn as_number(&self, func: &mut BcFunction<'_>, op: BcOp) -> f64;
  fn as_imm(&self, func: &mut BcFunction<'_>, op: BcOp) -> BcImm;
}

/// cpp `BcVmConstImpl`：唯一实现，无状态。
#[derive(Debug, Clone, Copy, Default)]
pub struct BcVmConstImpl;

/// cpp `Sccp<VmConst>` 主结构。泛型参数为常量求值器实现。
pub struct Sccp<'f, 'c, 'i, I: ?Sized> {
  pub func: &'f mut BcFunction<'c>,
  pub vm_ops: &'i I,
  pub state: SccpState,
  /// 每块遍历前把块内容拷进 scratch，避免 `visit_*(&mut self)` 与块借用
  /// 冲突而整块堆 clone；缓冲跨块复用，只拷贝不分配。
  pub scratch: SccpScratch,
}

/// `propagate`/`arith_to_k` 逐块复用的遍历缓冲（BcOp/BcBlockEdge 均为 Copy）。
#[derive(Default)]
pub struct SccpScratch {
  pub phis: Vec<BcOp>,
  pub ops: Vec<BcOp>,
  pub successors: Vec<BcBlockEdge>,
  /// `arith_to_k` 每块收集的待删纯生产者，跨块 clear 复用
  pub to_erase: Vec<BcOp>,
}

// ── abs-r139：并自 `methods/sccp_arith_to_k.rs` ──
/// cpp `Sccp::arithToKOpcode` 的编译期定表：算术 opcode → 对应 K 变体。
/// `LuauOpcode` 为 `#[repr(u8)]`，以其判别式为下标；未命中项保持 `None`
/// （等价旧实现的 `_ => None`，含 `LopCount` 及一切非算术 opcode）。
const ARITH_TO_K: [Option<LuauOpcode>; LuauOpcode::LOP__COUNT as usize] = {
  let mut table = [None; LuauOpcode::LOP__COUNT as usize];
  table[LuauOpcode::LOP_ADD as usize] = Some(LuauOpcode::LOP_ADDK);
  table[LuauOpcode::LOP_SUB as usize] = Some(LuauOpcode::LOP_SUBK);
  table[LuauOpcode::LOP_MUL as usize] = Some(LuauOpcode::LOP_MULK);
  table[LuauOpcode::LOP_DIV as usize] = Some(LuauOpcode::LOP_DIVK);
  table[LuauOpcode::LOP_MOD as usize] = Some(LuauOpcode::LOP_MODK);
  table[LuauOpcode::LOP_POW as usize] = Some(LuauOpcode::LOP_POWK);
  table
};

impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp `Sccp::arithToKOpcode`：算术 opcode 对应的 K 变体。
  fn arith_to_k_opcode(op: LuauOpcode) -> Option<LuauOpcode> {
    ARITH_TO_K.get(op as usize).copied().flatten()
  }

  /// cpp `Sccp::isPureProducer`：无使用即可删除的纯值生产者。
  fn is_pure_producer(op: LuauOpcode) -> bool {
    matches!(
      op,
      LuauOpcode::LOP_LOADK
        | LuauOpcode::LOP_LOADKX
        | LuauOpcode::LOP_LOADN
        | LuauOpcode::LOP_LOADB
        | LuauOpcode::LOP_LOADNIL
        | LuauOpcode::LOP_GETUPVAL
    )
  }

  /// 整型立即数快捷构造（对应 cpp `BcImm{BcImmKind::Int}` 字面量）
  fn int_imm(value: i32) -> BcImm {
    BcImm::Int(value)
  }

  /// cpp `Sccp::setOps`：替换指令操作数，同步 def→use 反向边。
  fn set_ops(&mut self, op: BcOp, new_ops: &[BcOp]) {
    // 拆分借用：旧 ops 的遍历只读 func，erase_use/record_use 只写 state，
    // 两字段不相交，无需旧实现的 to_vec 快照
    let Sccp { func, state, .. } = self;
    {
      let inst = func.inst(op);
      for old in inst.operator_deref().ops.iter().copied() {
        state.erase_use(op, old);
      }
    }
    let inst = func.inst_op(op);
    inst.ops.clear();
    inst.ops.extend(new_ops.iter().copied());
    for &new_op in new_ops {
      state.record_use(new_op, op);
    }
  }

  /// 一侧为算术常量（number 类 VM 常量）时返回该常量 op（cpp `isConstNumber`）。
  fn const_number(&mut self, lattice: &Constness) -> Option<BcOp> {
    let Constness::VmConstant(const_op) = *lattice else {
      return None;
    };
    self
      .vm_ops
      .is_arithmetic_constant(self.func, const_op)
      .then_some(const_op)
  }

  /// cpp `Sccp::eraseDeadProducer`：删除无引用的纯值生产者。
  fn erase_dead_producer(&mut self, op: BcOp) {
    if op.kind != BcOpKind::Inst {
      return;
    }
    let (opcode, block) = {
      let inst = self.func.inst(op);
      (inst.operator_deref().op, inst.operator_deref().block)
    };
    if !Self::is_pure_producer(opcode) {
      return;
    }
    if !self.state.uses_of(op).is_empty() {
      return;
    }
    self.func.block_op(block).ops.retain(|x| *x != op);
  }

  /// cpp `Sccp::arithToK`：算术一侧为已知常量时改写为 K/RK 变体，并顺带做代数折叠。
  pub(crate) fn arith_to_k(&mut self) {
    // 快照缓冲整体换出 self（O(1) 挪指针），整轮复用同一容量；局部持有后
    // 可按引用迭代，`&mut self` 调用不再与缓冲的共享借用冲突。
    let mut scratch = mem::take(&mut self.scratch);
    // block_idx 是块号：既当作 `state.block_uses` 的 u32 键（数据语义），又要现取
    // `blocks[block_idx]` 的 ops 快照；循环体经 set_ops/inst_op 回调 &mut self，
    // 无法持有 blocks 的迭代借用，故保留下标遍历。
    for block_idx in 0..self.func.blocks.len() {
      if self
        .state
        .block_uses
        .get_or_insert(block_idx as u32)
        .is_empty()
      {
        continue;
      }

      // 本块的改写会 retain/clear 块的 ops，需先快照（同 sccp_visit 的缓冲复用约定）。
      scratch.ops.clear();
      scratch
        .ops
        .extend(self.func.blocks[block_idx].ops.iter().copied());

      // 待删纯生产者清单亦走 scratch 缓冲：每块 clear 复用，不再逐块堆分配
      scratch.to_erase.clear();

      for &op in &scratch.ops {
        let opcode = self.func.inst(op).operator_deref().op;
        let Some(mut k_opcode) = Self::arith_to_k_opcode(opcode) else {
          continue;
        };
        // cpp（Sccp.h arithToK）：先验证双操作数形状，再解引用操作数
        let (lhs, rhs) = {
          let inst = self.func.inst(op);
          let ops = inst.operator_deref().ops.as_slice();
          if ops.len() != 2 {
            continue;
          }
          (ops[0], ops[1])
        };

        let lhs_lat = self.state.operand_lattice(lhs);
        let rhs_lat = self.state.operand_lattice(rhs);

        // 至多一侧为 VmConstant：双侧恒常时算术已被整体折叠
        let (non_constant_op, constant_op, rk) = if let Some(k) = self.const_number(&rhs_lat)
          && matches!(lhs_lat, Constness::NotAConstant)
        {
          (lhs, k, false)
        } else if let Some(k) = self.const_number(&lhs_lat)
          && matches!(rhs_lat, Constness::NotAConstant)
        {
          if !matches!(
            opcode,
            LuauOpcode::LOP_ADD | LuauOpcode::LOP_MUL | LuauOpcode::LOP_SUB | LuauOpcode::LOP_DIV
          ) {
            continue;
          }
          if opcode == LuauOpcode::LOP_SUB {
            k_opcode = LuauOpcode::LOP_SUBRK;
          } else if opcode == LuauOpcode::LOP_DIV {
            k_opcode = LuauOpcode::LOP_DIVRK;
          }
          let rk = matches!(k_opcode, LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK);
          (rhs, k, rk)
        } else {
          continue;
        };

        let prev_const_operand = if non_constant_op == lhs { rhs } else { lhs };
        let constant_is_rhs = non_constant_op == lhs;

        // 一侧恒常时代数折叠：加零、乘零/一、零/一次幂等
        let value_number = self.vm_ops.as_number(self.func, constant_op);
        if value_number == 0.0 {
          if opcode == LuauOpcode::LOP_ADD || (opcode == LuauOpcode::LOP_SUB && constant_is_rhs) {
            self.func.inst_op(op).op = LuauOpcode::LOP_MOVE;
            self.set_ops(op, &[non_constant_op]);
          } else if opcode == LuauOpcode::LOP_MUL {
            self.func.inst_op(op).op = LuauOpcode::LOP_LOADN;
            let imm_op = self.func.add_imm_value(Self::int_imm(0));
            self.set_ops(op, &[imm_op]);
          } else if opcode == LuauOpcode::LOP_POW && constant_is_rhs {
            // x ^ 0 == 1（0 ^ x 不折叠：x != 0 时为 0）
            self.func.inst_op(op).op = LuauOpcode::LOP_LOADN;
            let imm_op = self.func.add_imm_value(Self::int_imm(1));
            self.set_ops(op, &[imm_op]);
          }
        } else if value_number == 1.0 {
          if opcode == LuauOpcode::LOP_MUL
            || (opcode == LuauOpcode::LOP_POW && constant_is_rhs)
            || (opcode == LuauOpcode::LOP_DIV && constant_is_rhs)
          {
            self.func.inst_op(op).op = LuauOpcode::LOP_MOVE;
            self.set_ops(op, &[non_constant_op]);
          }
        } else {
          self.func.inst_op(op).op = k_opcode;
          if !rk {
            self.set_ops(op, &[non_constant_op, constant_op]);
          } else {
            // SUBRK/DIVRK 期望 B 为常量表索引
            self.set_ops(op, &[constant_op, non_constant_op]);
          }
        }

        scratch.to_erase.push(prev_const_operand);
      }

      // drain 保序清空且保留容量，供下一块复用
      for op in scratch.to_erase.drain(..) {
        self.erase_dead_producer(op);
      }
    }
    // 归还缓冲，保留已分配的容量供后续阶段复用。
    self.scratch = scratch;
  }
}

// ── abs-r139：并自 `methods/sccp_bc_vm_const_impl.rs` ──
/// 按值取出 VM 常量（避开 const_op 的可变借用）
fn const_at<'f>(func: &BcFunction<'f>, op: BcOp) -> BcVmConst<'f> {
  LUAU_ASSERT!(op.kind == BcOpKind::VmConst);
  func.constants[op.index as usize]
}

/// 按值取出立即数
fn imm_at(func: &BcFunction<'_>, op: BcOp) -> BcImm {
  LUAU_ASSERT!(op.kind == BcOpKind::Imm);
  func.immediates[op.index as usize]
}

/// cpp `isNumber`+`asNumber` 合一：数值常量（Imm-Int / VmConst-Number）取
/// double 值，非数值返回 None（cpp 先 isNumber 守卫再断言取值，此处以
/// Option 收敛同一契约）
fn number_of(func: &BcFunction<'_>, value: ConstValue) -> Option<f64> {
  match value {
    ConstValue::Imm(imm) => (imm.kind() == BcImmKind::Int).then(|| f64::from(imm.as_int())),
    ConstValue::Vm(op) => {
      let c = const_at(func, op);
      (c.kind() == BcVmConstKind::Number).then(|| c.as_number())
    }
  }
}

impl BcVmConstImpl {
  /// cpp `findOrAddConst`：复用已存在的相等常量，否则追加。
  fn find_or_add_const<'f>(func: &mut BcFunction<'f>, value: BcVmConst<'f>) -> BcOp {
    match func
      .constants
      .iter()
      .position(|existing| *existing == value)
    {
      Some(idx) => BcOp::with(BcOpKind::VmConst, idx as u32),
      None => func.add_const(value),
    }
  }
}

impl VmConstOps for BcVmConstImpl {
  fn evaluate(
    &self,
    func: &mut BcFunction<'_>,
    lhs: ConstValue,
    rhs: ConstValue,
    op: LuauOpcode,
  ) -> Option<BcOp> {
    // cpp `isNumber`+`asNumber`（Sccp.cpp:189-227）：Imm-Int 与 VmConst-Number
    // 任意组合均为数值常量，升 double 折叠；其余种类（含 VmConst-Integer/
    // String/Boolean 与 Imm-Boolean）`isNumber` 为假 → nullopt，静默放弃折叠。
    let a = number_of(func, lhs)?;
    let b = number_of(func, rhs)?;
    let r = match op {
      LuauOpcode::LOP_ADD => a + b,
      LuauOpcode::LOP_SUB => a - b,
      LuauOpcode::LOP_MUL => a * b,
      LuauOpcode::LOP_DIV => a / b,
      // cpp 无 b==0 守卫（Sccp.h evaluateNumberBinaryOp）：除零求值落 NaN。
      // 注意 NaN 并不会真的折叠：BcVmConst::operator== 对 NaN 恒 false，
      // find_or_add_const 每次求值都追加新 NaN 条目，格在二次 visit 的
      // merge 处退化为 NotAConstant——MOD 与 cpp 同态地保留运行期判定，
      // 仅常量池残留死 NaN 条目（对照：DIV/IDIV 除零落 ±inf，inf==inf
      // 命中既有常量、格稳定，照常折叠）。单独拦除零反而是对 oracle 的
      // 无端偏差，见 tests/sccp_int_imm_fold.rs 的 MOD/DIV 除零回归。
      LuauOpcode::LOP_MOD => a - (a / b).floor() * b,
      LuauOpcode::LOP_POW => a.powf(b),
      LuauOpcode::LOP_IDIV => (a / b).floor(),
      _ => return None,
    };

    Some(Self::find_or_add_const(func, BcVmConst::Number(r)))
  }

  fn falsey(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool {
    match op.kind {
      BcOpKind::VmConst => {
        let vm_const = const_at(func, op);
        vm_const.kind() == BcVmConstKind::Nil
          || (vm_const.kind() == BcVmConstKind::Boolean && !vm_const.as_boolean())
      }
      BcOpKind::Imm => {
        let imm = imm_at(func, op);
        imm.kind() == BcImmKind::Boolean && !imm.as_boolean()
      }
      _ => false,
    }
  }

  fn cmp_ops(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> Option<i32> {
    let lhs_const = const_at(func, lhs);
    let rhs_const = const_at(func, rhs);
    LUAU_ASSERT!(lhs_const.kind() == rhs_const.kind());
    match lhs_const.kind() {
      // NaN 与任何数（含 NaN）的 eq/lt/le 均为 false（cpp bcCompare 各算符
      // 独立判 false）。此哨兵是直接臂（Vm×Vm）的主语义，交换臂不经过此处；
      // three_way 的 PartialOrd 双假会误返 0（"相等"），故须先拦
      BcVmConstKind::Number => {
        let (a, b) = (lhs_const.as_number(), rhs_const.as_number());
        if a.is_nan() || b.is_nan() {
          Some(1)
        } else {
          Some(three_way(a, b))
        }
      }
      BcVmConstKind::Integer => Some(three_way(lhs_const.as_integer(), rhs_const.as_integer())),
      BcVmConstKind::Boolean => Some(i32::from(lhs_const.as_boolean() != rhs_const.as_boolean())),
      BcVmConstKind::String => Some(three_way(lhs_const.as_string(), rhs_const.as_string())),
      // Nil×Nil 相等（cpp eq 返回 true）；Vector/Vectord/Import/Table/
      // Closure/ClassShape 同种类对既无序也不可判等（cpp eq 返回 nullopt），
      // 返回 None 落 Unknown——旧实现 `_ => 0` 会把两个不同 Vector/不同
      // proto 的 Closure 折成"相等"，恒真/恒假跳转是 miscompile
      BcVmConstKind::Nil => Some(0),
      _ => None,
    }
  }

  fn is_nan_const(&self, func: &mut BcFunction<'_>, vm: BcOp) -> bool {
    let c = const_at(func, vm);
    c.kind() == BcVmConstKind::Number && c.as_number().is_nan()
  }

  fn cmp_imm(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcImm) -> Option<i32> {
    let lhs_const = const_at(func, lhs);
    match (lhs_const.kind(), rhs.kind()) {
      (BcVmConstKind::Number, BcImmKind::Int) => {
        // 直接臂（Vm×Int-imm）的主语义：NaN 返 1 令 eq/lt/le 全 false；
        // 交换臂（Imm×Vm）由调用方按 is_nan_const 跳过取反，勿改返回约定。
        // three_way 的 PartialOrd 双假会误返 0（"相等"）
        let a = lhs_const.as_number();
        if a.is_nan() {
          Some(1)
        } else {
          Some(three_way(a, f64::from(rhs.as_int())))
        }
      }
      (BcVmConstKind::Integer, BcImmKind::Int) => {
        Some(three_way(lhs_const.as_integer(), i64::from(rhs.as_int())))
      }
      (BcVmConstKind::Boolean, BcImmKind::Boolean) => {
        Some(i32::from(lhs_const.as_boolean() != rhs.as_boolean()))
      }
      // String/Nil/… × Int-imm 等异种类对无相等/序关系，必须返回 None 走
      // Unknown；旧实现 `_ => 0` 会把它们当"相等"折叠成恒真/恒假跳转
      // （cpp Sccp.cpp `eq` 对此类对返回 nullopt）
      _ => None,
    }
  }

  fn make_nil(&self, func: &mut BcFunction<'_>) -> BcOp {
    Self::find_or_add_const(func, BcVmConst::new())
  }

  fn make_imm_bool(&self, value: bool) -> BcImm {
    BcImm::Boolean(value)
  }

  fn make_imm_int(&self, value: i32) -> BcImm {
    BcImm::Int(value)
  }

  fn is_orderable(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool {
    matches!(
      const_at(func, op).kind(),
      BcVmConstKind::Number | BcVmConstKind::Integer | BcVmConstKind::String
    )
  }

  fn kind_equals(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> bool {
    const_at(func, lhs).kind() == const_at(func, rhs).kind()
  }

  fn eq_ops(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: BcOp) -> Option<bool> {
    match (lhs.kind, rhs.kind) {
      (BcOpKind::VmConst, BcOpKind::VmConst) => {
        let lhs_const = const_at(func, lhs);
        let rhs_const = const_at(func, rhs);
        match (lhs_const.kind(), rhs_const.kind()) {
          (BcVmConstKind::Number, BcVmConstKind::Number) => {
            Some(lhs_const.as_number() == rhs_const.as_number())
          }
          (BcVmConstKind::Integer, BcVmConstKind::Integer) => {
            Some(lhs_const.as_integer() == rhs_const.as_integer())
          }
          (BcVmConstKind::Number, BcVmConstKind::Integer) => {
            Some(lhs_const.as_number() == rhs_const.as_integer() as f64)
          }
          (BcVmConstKind::Integer, BcVmConstKind::Number) => {
            Some(lhs_const.as_integer() as f64 == rhs_const.as_number())
          }
          (BcVmConstKind::String, BcVmConstKind::String) => {
            Some(lhs_const.as_string() == rhs_const.as_string())
          }
          _ => None,
        }
      }
      (BcOpKind::VmConst, BcOpKind::Imm) => {
        let lhs_const = const_at(func, lhs);
        let rhs_imm = imm_at(func, rhs);
        if lhs_const.kind() == BcVmConstKind::Boolean && rhs_imm.kind() == BcImmKind::Boolean {
          Some(lhs_const.as_boolean() == rhs_imm.as_boolean())
        } else {
          None
        }
      }
      // (Imm, Imm) 与 (Imm, VmConst) 臂已删：仅有的两个调用点
      // （evaluate_xeqk_condition 的 JUMPXEQKB/KN/KS）lhs 恒为 VmConstant
      _ => None,
    }
  }

  fn eq_int(&self, func: &mut BcFunction<'_>, lhs: BcOp, rhs: i32) -> Option<bool> {
    let lhs_const = const_at(func, lhs);
    match lhs_const.kind() {
      BcVmConstKind::Number => Some(f64::from(rhs) == lhs_const.as_number()),
      BcVmConstKind::Integer => Some(i64::from(rhs) == lhs_const.as_integer()),
      _ => None,
    }
  }

  fn is_arithmetic_constant(&self, func: &mut BcFunction<'_>, op: BcOp) -> bool {
    const_at(func, op).kind() == BcVmConstKind::Number
  }

  fn as_number(&self, func: &mut BcFunction<'_>, op: BcOp) -> f64 {
    let vm_const = const_at(func, op);
    LUAU_ASSERT!(vm_const.kind() == BcVmConstKind::Number);
    vm_const.as_number()
  }

  fn as_imm(&self, func: &mut BcFunction<'_>, op: BcOp) -> BcImm {
    imm_at(func, op)
  }
}

// ── abs-r139：并自 `methods/sccp_evaluate_arith.rs` ──
/// cpp 注释：LOADN 载荷为 16 位有符号，replaceUses 阶段以 LOADN 重写
const K_LOADN_MIN: i64 = i16::MIN as i64;
const K_LOADN_MAX: i64 = i16::MAX as i64;

/// DIV/POW 的 double 结果仅当为有限整数时方可并入 LOADN 形态；inf/NaN/
/// 带小数部分一律返回 None（回落 Number VmConst 通路，oracle 同型）。
/// cast 饱和 + 射程过滤，超大整值（如 1e30）自然落 None。
fn int_if_representable(value: f64) -> Option<i64> {
  (value.is_finite() && value.fract() == 0.0).then_some(value as i64)
}

impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp `SccpInterpreter::evaluateArith`：任意两个常量（Imm/VmConst 组合）
  /// 统一交由 `impl->evaluate` 升 double 折叠成 Number 型 VmConst
  /// （cpp/Bytecode/src/Sccp.cpp:240-262），并无整数折叠分支。
  ///
  /// DELIBERATE DEVIATION：Rust 版对双 Int-imm 且结果可整表示、落在 `LOADN`
  /// i16 载荷射程内的组合，另走整数折叠落 `LOADN`（而非 Number `LOADK`），
  /// 保住 Lua 的 integer 语义（`math.type` 不漂移）；该决策回归钉在
  /// tests/sccp_int_imm_fold.rs，防 sync-cpp 时被改回。其余一切常量对
  /// （超射程整对、DIV/POW 非整结果、除零 inf/NaN、Imm×VmConst 数值混合对）
  /// 与 cpp 同型走 `vm_ops.evaluate` 双精度通路——旧版对这些无条件
  /// NotAConstant，折叠强度低于 oracle，系偏差范围失控而非决策本身。
  pub(crate) fn evaluate_arith(&mut self, op: LuauOpcode, inst_op: BcOp) -> Constness {
    // 双操作数均为 Copy：块作用域借切片拷出即释放 func 的共享借用，
    // 后续 &mut self.state 调用不再冲突，无需整条 ops clone
    let (lhs, rhs) = {
      let inst = self.func.inst(inst_op);
      let ops = inst.operator_deref().ops.as_slice();
      (ops[0], ops[1])
    };

    let lhs_constness = self.state.operand_lattice(lhs);
    let rhs_constness = self.state.operand_lattice(rhs);

    // 整数快路径只认双 Int-imm；落不进 LOADN 的组合不放弃，继续走下方
    // 双精度通路（oracle 对超射程整对照折 Number VmConst）
    if let (Constness::ImmConstant(lhs_imm), Constness::ImmConstant(rhs_imm)) =
      (&lhs_constness, &rhs_constness)
      && lhs_imm.kind() == BcImmKind::Int
      && rhs_imm.kind() == BcImmKind::Int
      && let Some(imm) = self.fold_int_imm(op, lhs_imm.as_int(), rhs_imm.as_int())
    {
      return Constness::ImmConstant(imm);
    }

    match (lhs_constness.const_value(), rhs_constness.const_value()) {
      // cpp：双侧常量（任意 Imm/VmConst 组合）统一 evaluate，nullopt 落格底
      (Some(lhs_const), Some(rhs_const)) => {
        match self.vm_ops.evaluate(self.func, lhs_const, rhs_const, op) {
          Some(vm_const) => Constness::VmConstant(vm_const),
          None => Constness::NotAConstant,
        }
      }
      // cpp：双侧 Undetermined 才延续格顶
      (None, None)
        if matches!(lhs_constness, Constness::Undetermined)
          && matches!(rhs_constness, Constness::Undetermined) =>
      {
        Constness::Undetermined
      }
      _ => Constness::NotAConstant,
    }
  }

  /// 双 Int-imm 整数快路径（DELIBERATE DEVIATION 载体）：结果可整表示且落
  /// i16 射程 → Int imm；否则 None 由调用方回落双精度通路。ADD/SUB/MUL/
  /// MOD/IDIV 用 i64 精确整数域（操作数 |v|≤2^31，积 ≤2^62 不溢出）；
  /// DIV/POW 本征浮点，仅当 double 结果恰为有限整数时并入 LOADN 形态，
  /// 数值与 oracle 的 double 折叠一致，只是编码不同。
  fn fold_int_imm(&self, op: LuauOpcode, lv: i32, rv: i32) -> Option<BcImm> {
    let (l, r) = (i64::from(lv), i64::from(rv));
    // 负零守卫：整数域无法表示 -0.0，凡 double 真值为 -0.0 的组合必须落
    // 双精度通路（Number(-0.0)→LOADK），否则 `tostring` "-0" 漂移成 "0"，
    // 且后续 1/x 的 ±inf 方向翻转。-0.0 仅产自 MUL/DIV/IDIV 的 (0, 负)
    // 组合；ADD/SUB/MOD 的整值 0 恒为 +0.0（MOD 符号随除数，0%x = +0）
    let negative_zero = (l == 0 || r == 0) && (l < 0) != (r < 0);
    // MOD/IDIV 除零在整数域无定义，直接落双精度通路（NaN/inf，cpp 同款）
    let exact: Option<i64> = match op {
      LuauOpcode::LOP_ADD => Some(l + r),
      LuauOpcode::LOP_SUB => Some(l - r),
      LuauOpcode::LOP_MUL if negative_zero => None,
      LuauOpcode::LOP_MUL => Some(l * r),
      LuauOpcode::LOP_MOD => {
        // Lua 取模：结果符号随除数
        (r != 0).then(|| {
          let mut remainder = l % r;
          if remainder != 0 && (l < 0) != (r < 0) {
            remainder += r;
          }
          remainder
        })
      }
      LuauOpcode::LOP_IDIV => {
        // Lua 整除：向负无穷取整。`/` 向零截断，需 -1 修正的判据是
        // 余数非零且操作数符号相异（真实商为负）——不能用 `quotient < 0`：
        // 截断商为 0 时（如 -1 // 2 == -1）会漏修成 0
        if l == 0 && r < 0 {
          return None; // floor(-0.0) = -0.0，整数域不可表示
        }
        (r != 0).then(|| {
          let mut quotient = l / r;
          if l % r != 0 && (l < 0) != (r < 0) {
            quotient -= 1;
          }
          quotient
        })
      }
      LuauOpcode::LOP_DIV if l == 0 && r < 0 => None,
      LuauOpcode::LOP_DIV => int_if_representable(f64::from(lv) / f64::from(rv)),
      LuauOpcode::LOP_POW => int_if_representable(f64::from(lv).powf(f64::from(rv))),
      _ => {
        LUAU_ASSERT!(false, "Unhandled opcode");
        None
      }
    };

    exact
      .filter(|result| (K_LOADN_MIN..=K_LOADN_MAX).contains(result))
      .map(|result| self.vm_ops.make_imm_int(result as i32))
  }

  /// cpp `SccpInterpreter::evaluate`：单条指令的常量求值。
  pub(crate) fn evaluate(&mut self, op: LuauOpcode, inst_op: BcOp) -> Constness {
    let (op0, op1) = {
      let inst = self.func.inst(inst_op);
      let s = inst.operator_deref().ops.as_slice();
      (
        s.first().copied().unwrap_or_default(),
        s.get(1).copied().unwrap_or_default(),
      )
    };

    match op {
      LuauOpcode::LOP_LOADK | LuauOpcode::LOP_LOADKX => {
        LUAU_ASSERT!(op0.kind == BcOpKind::VmConst);
        Constness::VmConstant(op0)
      }
      LuauOpcode::LOP_LOADB | LuauOpcode::LOP_LOADN => {
        LUAU_ASSERT!(op0.kind == BcOpKind::Imm);
        Constness::ImmConstant(self.vm_ops.as_imm(self.func, op0))
      }
      LuauOpcode::LOP_LOADNIL => {
        let nil_const = self.vm_ops.make_nil(self.func);
        Constness::VmConstant(nil_const)
      }
      LuauOpcode::LOP_ADD
      | LuauOpcode::LOP_SUB
      | LuauOpcode::LOP_MUL
      | LuauOpcode::LOP_DIV
      | LuauOpcode::LOP_MOD
      | LuauOpcode::LOP_POW
      | LuauOpcode::LOP_IDIV => self.evaluate_arith(op, inst_op),
      LuauOpcode::LOP_MOVE => self.state.operand_lattice(op0),
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        let cond = self.evaluate_condition(op0);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[op0]);
        }
        let jumps_on_true = op == LuauOpcode::LOP_JUMPIF;
        let takes_jump = (cond == ConditionState::AlwaysTrue) == jumps_on_true;
        Constness::ImmConstant(self.vm_ops.make_imm_bool(takes_jump))
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        let cond = self.evaluate_comparison_condition(op, op0, op1);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[op0, op1]);
        }
        let negated = matches!(
          op,
          LuauOpcode::LOP_JUMPIFNOTEQ | LuauOpcode::LOP_JUMPIFNOTLE | LuauOpcode::LOP_JUMPIFNOTLT
        );
        let takes_jump = (cond == ConditionState::AlwaysTrue) != negated;
        Constness::ImmConstant(self.vm_ops.make_imm_bool(takes_jump))
      }
      LuauOpcode::LOP_JUMPXEQKNIL
      | LuauOpcode::LOP_JUMPXEQKB
      | LuauOpcode::LOP_JUMPXEQKN
      | LuauOpcode::LOP_JUMPXEQKS => {
        let cond = self.evaluate_xeqk_condition(inst_op);
        if cond == ConditionState::Unknown {
          return self.state.unknown_condition_constness(&[op0]);
        }
        // 取反位以 Imm(bool) 编码，falsey 即其真值
        let negated = !self.vm_ops.falsey(self.func, op1);
        let takes_jump = (cond == ConditionState::AlwaysTrue) != negated;
        Constness::ImmConstant(self.vm_ops.make_imm_bool(takes_jump))
      }
      _ => Constness::NotAConstant,
    }
  }
}

// ── abs-r139：并自 `methods/sccp_evaluate_condition.rs` ──
impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// 布尔结果映射到格态（cpp `condTrue ? AlwaysTrue : AlwaysFalse`）。
  fn resolved(cond: bool) -> ConditionState {
    if cond {
      ConditionState::AlwaysTrue
    } else {
      ConditionState::AlwaysFalse
    }
  }

  /// cpp `SccpInterpreter::evaluateCondition`：JUMPIF* 条件的真值。
  /// cpp 对一切 Constant（vmConst/immConst）统一走 `falsey`：仅 nil 与
  /// false 为假，imm 侧无 nil，故 Boolean imm 取自身真值，其余恒真
  /// （`LOADN r,0; JUMPIF r` 的整数 imm 折 AlwaysTrue，与 cpp 一致）。
  pub(crate) fn evaluate_condition(&mut self, op: BcOp) -> ConditionState {
    let lattice = self.state.operand_lattice(op);
    match lattice {
      Constness::VmConstant(vm_const) => Self::resolved(!self.vm_ops.falsey(self.func, vm_const)),
      Constness::ImmConstant(imm) => Self::resolved(match imm.kind() {
        BcImmKind::Boolean => imm.as_boolean(),
        _ => true,
      }),
      _ => ConditionState::Unknown,
    }
  }

  /// cpp `applyOp`：三路比较结果套用比较算符。
  fn apply_cmp(cmp: i32, op: LuauOpcode) -> bool {
    match op {
      LuauOpcode::LOP_JUMPIFEQ | LuauOpcode::LOP_JUMPIFNOTEQ => cmp == 0,
      LuauOpcode::LOP_JUMPIFLT | LuauOpcode::LOP_JUMPIFNOTLT => cmp < 0,
      LuauOpcode::LOP_JUMPIFLE | LuauOpcode::LOP_JUMPIFNOTLE => cmp <= 0,
      _ => {
        LUAU_ASSERT!(false, "Unhandled comparison opcode");
        false
      }
    }
  }

  /// cpp `SccpInterpreter::evaluateComparisonCondition`。
  pub(crate) fn evaluate_comparison_condition(
    &mut self,
    op: LuauOpcode,
    lhs: BcOp,
    rhs: BcOp,
  ) -> ConditionState {
    let lhs_const = self.state.operand_lattice(lhs);
    let rhs_const = self.state.operand_lattice(rhs);

    let is_ordering_op = matches!(
      op,
      LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFNOTLT
        | LuauOpcode::LOP_JUMPIFNOTLE
    );

    let orderable = |lat: Constness, sccp: &mut Self| match lat {
      Constness::VmConstant(vm_const) => sccp.vm_ops.is_orderable(sccp.func, vm_const),
      Constness::ImmConstant(imm) => imm.kind() == BcImmKind::Int,
      _ => false,
    };

    if is_ordering_op && (!orderable(lhs_const, self) || !orderable(rhs_const, self)) {
      return ConditionState::Unknown;
    }

    // VM 常量种类不匹配：序无定义（cpp compare 不可达），但等值有定义——
    // cpp `eq` 对异种类 VmConst 对直接返回 false（Sccp.cpp:157-158），故
    // 等值算符折 AlwaysFalse、序算符落 Unknown
    if let (Constness::VmConstant(lhs_vm), Constness::VmConstant(rhs_vm)) = (lhs_const, rhs_const)
      && !self.vm_ops.kind_equals(self.func, lhs_vm, rhs_vm)
    {
      return if is_ordering_op {
        ConditionState::Unknown
      } else {
        ConditionState::AlwaysFalse
      };
    }

    match (lhs_const, rhs_const) {
      (Constness::VmConstant(lhs_vm), Constness::VmConstant(rhs_vm)) => {
        // 同种类但无定义关系的对（Vector/Table/Closure…）返回 None 落
        // Unknown，不得当"相等"折叠（cpp eq 返回 nullopt）
        let Some(cmp) = self.vm_ops.cmp_ops(self.func, lhs_vm, rhs_vm) else {
          return ConditionState::Unknown;
        };
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant(lhs_imm), Constness::ImmConstant(rhs_imm)) => {
        if lhs_imm.kind() == BcImmKind::Int && rhs_imm.kind() == BcImmKind::Int {
          let (lv, rv) = (lhs_imm.as_int(), rhs_imm.as_int());
          let cmp = three_way(lv, rv);
          Self::resolved(Self::apply_cmp(cmp, op))
        } else if lhs_imm.kind() == BcImmKind::Boolean && rhs_imm.kind() == BcImmKind::Boolean {
          let (lb, rb) = (lhs_imm.as_boolean(), rhs_imm.as_boolean());
          let cmp = i32::from(lb != rb);
          Self::resolved(Self::apply_cmp(cmp, op))
        } else {
          ConditionState::Unknown
        }
      }
      // 混合 VmConst×Imm 臂：cpp `eq` 对不可比较种类对（String×Int-imm 等）
      // 返回 nullopt 落 Unknown；cmp_imm 返回 None 时同样不得默认"相等"
      (Constness::VmConstant(vm_const), Constness::ImmConstant(imm)) => {
        let Some(cmp) = self.vm_ops.cmp_imm(self.func, vm_const, imm) else {
          return ConditionState::Unknown;
        };
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      (Constness::ImmConstant(imm), Constness::VmConstant(vm_const)) => {
        let Some(cmp) = self.vm_ops.cmp_imm(self.func, vm_const, imm) else {
          return ConditionState::Unknown;
        };
        // NaN 参与时 cmp_imm 返回哨兵 1（eq/lt/le 基础真值全 false，cpp
        // bcCompare 逐算符独立判 false）；此处不得取反——`-1` 会使
        // `imm < NaN` / `imm <= NaN` 误折恒真（miscompile）
        let cmp = if self.vm_ops.is_nan_const(self.func, vm_const) {
          cmp
        } else {
          -cmp
        };
        Self::resolved(Self::apply_cmp(cmp, op))
      }
      _ => ConditionState::Unknown,
    }
  }

  /// cpp `SccpInterpreter::evaluateXeqkCondition`。
  pub(crate) fn evaluate_xeqk_condition(&mut self, inst_op: BcOp) -> ConditionState {
    let inst = self.func.inst(inst_op);
    let opcode = inst.operator_deref().op;
    let ops: &[BcOp] = inst.operator_deref().ops.as_slice();

    let val_const = self.state.operand_lattice(ops[0]);

    match opcode {
      LuauOpcode::LOP_JUMPXEQKNIL => {
        if let Constness::VmConstant(vm) = val_const {
          let nil = self.vm_ops.make_nil(self.func);
          // 既 falsey 又与 nil 同类：条件恒真
          if self.vm_ops.falsey(self.func, vm) && self.vm_ops.kind_equals(self.func, vm, nil) {
            return ConditionState::AlwaysTrue;
          }
          // 与 nil 不同类：条件恒假
          if !self.vm_ops.kind_equals(self.func, vm, nil) {
            return ConditionState::AlwaysFalse;
          }
        } else if matches!(val_const, Constness::ImmConstant(_)) {
          return ConditionState::AlwaysFalse;
        }
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        let cmp_imm_op = ops[3];
        LUAU_ASSERT!(cmp_imm_op.kind == BcOpKind::Imm);
        match val_const {
          Constness::ImmConstant(imm) if imm.kind() == BcImmKind::Boolean => {
            let lhs_bool = imm.as_boolean();
            let rhs_imm = self.vm_ops.as_imm(self.func, cmp_imm_op);
            // JUMPXEQKB 的比较操作数恒为 Boolean imm（图布局约定）
            let rhs_bool = rhs_imm.as_boolean();
            return Self::resolved(lhs_bool == rhs_bool);
          }
          Constness::VmConstant(vm) => {
            if let Some(eq) = self.vm_ops.eq_ops(self.func, vm, cmp_imm_op) {
              return Self::resolved(eq);
            }
          }
          _ => {}
        }
      }
      LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
        let cmp_const_op = ops[3];
        LUAU_ASSERT!(cmp_const_op.kind == BcOpKind::VmConst);
        match val_const {
          Constness::VmConstant(vm) => {
            if let Some(eq) = self.vm_ops.eq_ops(self.func, vm, cmp_const_op) {
              return Self::resolved(eq);
            }
          }
          // 整数 imm 只与 JUMPXEQKN 比较；字符串走 VmConstant 分支
          Constness::ImmConstant(imm)
            if opcode == LuauOpcode::LOP_JUMPXEQKN && imm.kind() == BcImmKind::Int =>
          {
            let value = imm.as_int();
            if let Some(eq) = self.vm_ops.eq_int(self.func, cmp_const_op, value) {
              return Self::resolved(eq);
            }
          }
          _ => {}
        }
      }
      _ => {}
    }

    ConditionState::Unknown
  }
}

// ── abs-r139：并自 `methods/sccp_jump_targets.rs` ──
impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp `Sccp::getFallthrough`：取块的唯一 fallthrough 后继；多个则违反 CFG 不变量。
  pub(crate) fn get_fallthrough(&self, block_op: BcOp) -> Option<BcOp> {
    let mut fallthrough = None;
    for succ in self
      .func
      .block(block_op)
      .operator_deref()
      .successors
      .as_slice()
    {
      if succ.kind == BcBlockEdgeKind::Fallthrough {
        if fallthrough.is_some() {
          LUAU_ASSERT!(false, "Multiple fallthroughs");
          return None;
        }
        fallthrough = Some(succ.target);
      }
    }
    fallthrough
  }

  /// cpp `Sccp::conditionalTargets`：构造两路分支的 target/fallthrough 落点对。
  /// `target_taken_on_true` 指出条件成立时走哪条边；条件已解析时另一边标记 dead。
  pub(crate) fn conditional_targets(
    &self,
    inst_op: BcOp,
    target: BcOp,
    cond: ConditionState,
    target_taken_on_true: bool,
  ) -> SmallVector<JumpTarget, 2> {
    LUAU_ASSERT!(target.kind == BcOpKind::Block);
    let inst_block = self.func.inst(inst_op).operator_deref().block;
    let fallthrough = self.get_fallthrough(inst_block);
    LUAU_ASSERT!(fallthrough.is_some());

    let (target_dead, fallthrough_dead) = match cond {
      ConditionState::AlwaysTrue => (!target_taken_on_true, target_taken_on_true),
      ConditionState::AlwaysFalse => (target_taken_on_true, !target_taken_on_true),
      ConditionState::Unknown => (false, false),
    };

    [
      JumpTarget {
        dead: target_dead,
        block_op: target,
        condition: cond,
      },
      JumpTarget {
        dead: fallthrough_dead,
        // cpp release 下此处用空 `BcRef` 构造 `BcOp(nullptr)`，Rust 用 `BcOp::new()`
        // 表示同一空操作数（6e1992b 裁定的断言 + 非 panic 读法），不再解包 panic。
        block_op: fallthrough.unwrap_or_default(),
        condition: cond,
      },
    ]
    .into_iter()
    .collect()
  }

  /// cpp `Sccp::jumpTargets`：按指令的跳转语义给出全部落点；空结果表示非分支指令。
  pub(crate) fn jump_targets(&mut self, inst_op: BcOp) -> SmallVector<JumpTarget, 2> {
    // 快照进 SmallVector：常规分支 ≤4 个操作数时栈内联，不走堆
    let (opcode, ops): (LuauOpcode, SmallVector<BcOp, 4>) = {
      let inst = self.func.inst(inst_op);
      (inst.operator_deref().op, inst.operator_deref().ops.clone())
    };
    let ops = ops.as_slice();

    match opcode {
      LuauOpcode::LOP_JUMP | LuauOpcode::LOP_JUMPBACK => {
        LUAU_ASSERT!(ops[0].kind == BcOpKind::Block);
        [JumpTarget {
          dead: false,
          block_op: ops[0],
          condition: ConditionState::AlwaysTrue,
        }]
        .into_iter()
        .collect()
      }
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        let cond = self.evaluate_condition(ops[0]);
        self.conditional_targets(inst_op, ops[1], cond, opcode == LuauOpcode::LOP_JUMPIF)
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        let cond = self.evaluate_comparison_condition(opcode, ops[0], ops[1]);
        let negated = matches!(
          opcode,
          LuauOpcode::LOP_JUMPIFNOTEQ | LuauOpcode::LOP_JUMPIFNOTLE | LuauOpcode::LOP_JUMPIFNOTLT
        );
        self.conditional_targets(inst_op, ops[2], cond, !negated)
      }
      LuauOpcode::LOP_JUMPXEQKNIL
      | LuauOpcode::LOP_JUMPXEQKB
      | LuauOpcode::LOP_JUMPXEQKN
      | LuauOpcode::LOP_JUMPXEQKS => {
        let cond = self.evaluate_xeqk_condition(inst_op);
        let neg_imm = *self.func.imm(ops[1]).operator_deref();
        LUAU_ASSERT!(neg_imm.kind() == BcImmKind::Boolean);
        let negated = neg_imm.as_boolean();
        self.conditional_targets(inst_op, ops[2], cond, !negated)
      }
      LuauOpcode::LOP_FORNPREP
      | LuauOpcode::LOP_FORNLOOP
      | LuauOpcode::LOP_FORGPREP
      | LuauOpcode::LOP_FORGPREP_NEXT
      | LuauOpcode::LOP_FORGPREP_INEXT => {
        self.conditional_targets(inst_op, ops[3], ConditionState::Unknown, true)
      }
      // FORGLOOP 比 FORGPREP* 多两个 imm 前导操作数，target 在 ops[5]
      LuauOpcode::LOP_FORGLOOP => {
        self.conditional_targets(inst_op, ops[5], ConditionState::Unknown, true)
      }
      LuauOpcode::LOP_CMPPROTO => {
        self.conditional_targets(inst_op, ops[2], ConditionState::Unknown, true)
      }
      LuauOpcode::LOP_JUMPX => {
        LUAU_ASSERT!(false, "Should have never parsed this");
        SmallVector::new()
      }
      _ => SmallVector::new(),
    }
  }
}

// ── abs-r139：并自 `methods/sccp_rewrite.rs` ──
impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp `Sccp::rewrite`：常量传播完成后的图重写四部曲。
  pub fn rewrite(&mut self) {
    self.arith_to_k();
    self.replace_uses();
    self.simplify_phis();
    self.update_block_uses();
  }

  /// cpp `Sccp::isLoadInst`：已是载入指令者不再改写。
  fn is_load_inst(&self, op: BcOp) -> bool {
    if op.kind != BcOpKind::Inst {
      return false;
    }
    matches!(
      self.func.inst(op).operator_deref().op,
      LuauOpcode::LOP_LOADK
        | LuauOpcode::LOP_LOADKX
        | LuauOpcode::LOP_LOADN
        | LuauOpcode::LOP_LOADB
        | LuauOpcode::LOP_LOADNIL
    )
  }

  /// cpp `Sccp::replaceUses`：常量指令改写为载入或删除死跳转。
  fn replace_uses(&mut self) {
    let entries: Vec<(BcOp, Constness)> = self
      .state
      .op_constness
      .iter()
      .map(|(op, lattice)| (*op, *lattice))
      .collect();
    for (op, lattice) in entries {
      if !matches!(
        lattice,
        Constness::ImmConstant(_) | Constness::VmConstant(_)
      ) {
        continue;
      }
      if op.kind != BcOpKind::Inst {
        continue;
      }
      if self.is_load_inst(op) {
        continue;
      }

      let opcode = self.func.inst(op).operator_deref().op;
      LUAU_ASSERT!(opcode != LuauOpcode::LOP_JUMPX);

      // JUMPX 不被 GraphParser 解析，is_jump_d 在此安全
      if is_jump_d(opcode) {
        self.remove_dead_edges(op);
        self.erase_op(op);
      } else {
        self.rewrite_to_load(op, lattice);
      }
    }
  }

  /// cpp `BcFunction::eraseOp`：从所属块移除指令，并解除其作为消费者的全部反向边。
  fn erase_op(&mut self, op: BcOp) {
    // 拆分借用（同 set_ops）：遍历旧 ops 只读 func，erase_use 只写 state
    let Sccp { func, state, .. } = self;
    let block = func.inst(op).operator_deref().block;
    {
      let inst = func.inst(op);
      for used in inst.operator_deref().ops.iter().copied() {
        state.erase_use(op, used);
      }
    }
    func.block_op(block).ops.retain(|x| *x != op);
  }

  /// cpp `Sccp::rewriteToLoad`：原位改写为载入指令，op 身份不变，既有使用点全部有效。
  fn rewrite_to_load(&mut self, op: BcOp, lattice: Constness) {
    // 拆分借用：先只读遍历旧 ops 解除反向边，再独占可变改写
    let Sccp { func, state, .. } = self;
    {
      let inst = func.inst(op);
      for used in inst.operator_deref().ops.iter().copied() {
        state.erase_use(op, used);
      }
    }

    // 调用方 replaceUses 已过滤掉非常量格值，故两个顶/底态分支不可达
    match lattice {
      Constness::VmConstant(const_op) => {
        let inst = func.inst_op(op);
        inst.op = LuauOpcode::LOP_LOADK;
        inst.ops.clear();
        inst.ops.push_back(const_op);
      }
      Constness::ImmConstant(imm) => {
        let imm_op = func.add_imm_value(imm);
        let inst = func.inst_op(op);
        inst.op = if imm.kind() == BcImmKind::Boolean {
          LuauOpcode::LOP_LOADB
        } else {
          LuauOpcode::LOP_LOADN
        };
        inst.ops.clear();
        inst.ops.push_back(imm_op);
      }
      Constness::Undetermined | Constness::NotAConstant => {}
    }
  }

  /// cpp `Sccp::removeDeadEdges`：剔除死边，保证活目标持有 fallthrough 边。
  fn remove_dead_edges(&mut self, op: BcOp) {
    let targets = self.jump_targets(op);
    let block_op = self.func.inst(op).operator_deref().block;

    // 收集死目标，同时锁定唯一活目标（最后一个非死项）
    let dead_targets: Vec<BcOp> = targets
      .as_slice()
      .iter()
      .filter(|target| target.dead)
      .map(|target| target.block_op)
      .collect();
    let Some(live_target) = targets
      .as_slice()
      .iter()
      .rev()
      .find(|target| !target.dead)
      .map(|target| target.block_op)
    else {
      return;
    };

    if !dead_targets.is_empty() {
      let kept: BcEdges = self
        .func
        .block_op(block_op)
        .successors
        .as_slice()
        .iter()
        .filter(|edge| !dead_targets.contains(&edge.target))
        .copied()
        .collect();
      self.func.block_op(block_op).successors = kept;
    }

    // 活目标必须有 fallthrough 边
    let block = self.func.block_op(block_op);
    if let Some(edge) = block
      .successors
      .as_mut_slice()
      .iter_mut()
      .find(|edge| edge.target == live_target)
    {
      edge.kind = BcBlockEdgeKind::Fallthrough;
    } else {
      block.successors.push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: live_target,
      });
    }
  }
}

// ── abs-r139：并自 `methods/sccp_seed_uses.rs` ──
/// 播种一个函数的全部 def→use 反向边。
///
/// 拆成自由函数是为了一次性拿到 `&BcFunction` 与 `&mut SccpState` 两个**不相交**
/// 借用：成员视图只读，写侧仅落在 state，迭代器可安全跨 `record_use` 存活，
/// 不再需要旧实现「把每条 ops 拷成 Vec 绕借用检查」的热路径堆分配。
fn seed_block_uses(func: &BcFunction<'_>, state: &mut SccpState) {
  for block in &func.blocks {
    for phi_op in block.phis.iter().copied() {
      LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);
      let phi = func.phi(phi_op);
      for phi_operand in phi.operator_deref().ops.iter().copied() {
        state.record_use(phi_operand, phi_op);
      }
    }

    for inst_op in block.ops.iter().copied() {
      LUAU_ASSERT!(inst_op.kind == BcOpKind::Inst);
      let inst = func.inst(inst_op);
      for inst_operand in inst.operator_deref().ops.iter().copied() {
        state.record_use(inst_operand, inst_op);
      }
    }
  }
}

impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp 侧 def→use 边由 GraphParser 的 `addUse` 建图时同步写入 `BcInst::uses`/
  /// `BcPhi::uses`，内联器经 `addUse`/`eraseUse` 增量维护；Rust 侧反向边收敛于
  /// `SccpState.op_uses`，进入传播前必须先从最终图一次性播种，否则 SSA 工作表
  /// 永远为空，visit 检测到格值变化后无法重估消费者（循环回边场景会把陈旧常量
  /// 错误折叠），与 cpp 的不动点语义不等价。
  pub(crate) fn seed_uses(&mut self) {
    // 字段级拆分借用：func 只读重借用、state 独占可变，二者不相交
    let Sccp { func, state, .. } = self;
    seed_block_uses(func, state);
  }
}

// ── abs-r139：并自 `methods/sccp_simplify.rs` ──
impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp `Sccp::simplifyPhis`：消解平凡 phi（全部操作数相同），使用点重定向到唯一操作数。
  pub(crate) fn simplify_phis(&mut self) {
    let visited_blocks: Vec<BcOp> = self.state.flow_worklist_set.iter().copied().collect();

    for block_op in visited_blocks {
      LUAU_ASSERT!(block_op.kind == BcOpKind::Block);
      let mut phi_idx = 0;
      while let Some(&phi_op) = self.func.block(block_op).operator_deref().phis.get(phi_idx) {
        LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);

        let phi_ops: SmallVector<_, 4> = self.func.phi(phi_op).operator_deref().ops.clone();
        let Some(&unique) = phi_ops.first() else {
          phi_idx += 1;
          continue;
        };
        if !phi_ops.as_slice().iter().all(|&op| op == unique) {
          phi_idx += 1;
          continue;
        }

        // 把使用点重定向到唯一操作数
        let redirect = |ops: &mut SmallVector<BcOp, 4>| {
          for op in ops.as_mut_slice() {
            if *op == phi_op {
              *op = unique;
            }
          }
        };

        for use_op in self.state.uses_of(phi_op).to_vec() {
          match use_op.kind {
            BcOpKind::Inst => redirect(&mut self.func.inst_op(use_op).ops),
            BcOpKind::Phi => redirect(&mut self.func.phi_op(use_op).ops),
            _ => {}
          }
          self.state.record_use(unique, use_op);
        }

        // cpp `Sccp.h:733-740`：`phi->uses.clear()` → 逐操作数 `eraseUse(phi, operand)`
        // → `phi->ops.clear()` → 从块中摘除。反向边里不得残留已消解 phi 的正向引用，
        // 否则后续多轮迭代（cpp 亦是）会读到陈旧反向边。
        self.state.op_uses.insert(phi_op, Vec::new());
        for &operand in phi_ops.as_slice() {
          self.state.erase_use(phi_op, operand);
        }
        self.func.phi_op(phi_op).ops.clear();
        // phi 已消解：出块（remove 后同位下个 phi 前移，不递增）
        self.func.block_op(block_op).phis.remove(phi_idx);
      }
    }
  }

  /// cpp `Sccp::updateBlockUses`：按 entry 前向可达性标记死块，并回填 useCount。
  pub(crate) fn update_block_uses(&mut self) {
    // 以 entry 出发的前向可达性（而非 blockUses，其结果依赖工作表顺序）标记死块
    let mut reachable: DenseHashSet<u32> = DenseHashSet::new(u32::MAX);
    let entry_idx = self.func.entry_block.index;
    let exit_idx = self.func.exit_block.index;
    reachable.insert(entry_idx);
    reachable.insert(exit_idx);

    let mut worklist: Vec<u32> = vec![entry_idx];
    while let Some(idx) = worklist.pop() {
      for edge in self.func.blocks[idx as usize].successors.as_slice() {
        let succ_idx = edge.target.index;
        LUAU_ASSERT!(edge.target.kind == BcOpKind::Block);
        if !reachable.contains(&succ_idx) {
          reachable.insert(succ_idx);
          worklist.push(succ_idx);
        }
      }
    }

    let Sccp { func, state, .. } = self;
    for (idx, block) in func.blocks.iter_mut().enumerate() {
      let idx = idx as u32;
      block.use_count = state.block_uses.get_or_insert(idx).len() as u32;
      if !reachable.contains(&idx) {
        block.flags |= BcBlockFlag::Dead;
      }
    }
  }
}

// ── abs-r139：并自 `methods/sccp_visit.rs` ──
impl<'c, I: VmConstOps + ?Sized> Sccp<'_, 'c, '_, I> {
  /// cpp `Sccp::visitPhi`：合并各操作数格值，变化时把使用点投入 SSA 工作表。
  pub(crate) fn visit_phi(&mut self, phi_op: BcOp) {
    LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);

    // 借用拆分：phi 操作数只读借 `self.func`，`operand_lattice` 写 `self.state`，
    // 两者是 Sccp 的不相交字段，无需快照整份操作数表。
    let mut fold = Constness::Undetermined;
    {
      let phi_ref = self.func.phi(phi_op);
      let phi_ops = phi_ref.operator_deref().ops.as_slice();
      LUAU_ASSERT!(!phi_ops.is_empty());
      for &op in phi_ops {
        fold = self.state.operand_lattice(op).merge(&fold);
      }
    }

    let prev_lattice = *self.state.op_constness.get_or_insert(phi_op);
    if fold != prev_lattice {
      self.state.defer_uses_to_ssa(phi_op);
    }
    self.state.op_constness.insert(phi_op, fold);
  }

  /// cpp `Sccp::visitInst`：求值、对比旧格值、登记跳转落点。
  pub(crate) fn visit_inst(&mut self, inst_op: BcOp) {
    LUAU_ASSERT!(inst_op.kind == BcOpKind::Inst);

    // 借用拆分：本块只读借 `self.func`，跨后续 `&mut self` 调用（evaluate 等）
    // 仅携带 Copy 的标量与 BcOp，不持有任何切片，免整份 ops 快照。
    let (opcode, inst_block, capture_ref_src) = {
      let inst_ref = self.func.inst(inst_op);
      let inst_repr = inst_ref.operator_deref();
      let opcode = inst_repr.op;
      let inst_block = inst_repr.block;

      // CAPTURE REF 的源可经 SETUPVAL 外部修改，SSA 图不建模该别名，
      // 将源标记为非常量以避免折叠出陈旧值
      let mut capture_ref_src = None;
      if opcode == LuauOpcode::LOP_CAPTURE && inst_repr.ops.len() >= 2 {
        let capture_type_op = inst_repr.ops[0];
        // 先把源 BcOp（Copy）取出，结束对 self.func 的不可变借用，
        // 随后 imm_op 以 &mut 借用才能成立（否则与 inst_ref 生命周期冲突 E0502）。
        let src_op = inst_repr.ops[1];
        LUAU_ASSERT!(capture_type_op.kind == BcOpKind::Imm);
        let capture_imm = *self.func.imm_op(capture_type_op);
        let is_ref = capture_imm.kind() == BcImmKind::Int
          && capture_imm.as_int() == (LuauCaptureType::LCT_REF as u32) as i32;
        if is_ref {
          capture_ref_src = Some(src_op);
        }
      }
      (opcode, inst_block, capture_ref_src)
    };

    if let Some(src_op) = capture_ref_src {
      let prev = *self.state.op_constness.get_or_insert(src_op);
      if !matches!(prev, Constness::NotAConstant)
        && matches!(src_op.kind, BcOpKind::Inst | BcOpKind::Phi)
      {
        self
          .state
          .op_constness
          .insert(src_op, Constness::NotAConstant);
        self.state.defer_uses_to_ssa(src_op);
      }
    }

    let lattice = self.evaluate(opcode, inst_op);
    let prev_lattice = *self.state.op_constness.get_or_insert(inst_op);

    let new_val = lattice.merge(&prev_lattice);
    if new_val != prev_lattice {
      self.state.defer_uses_to_ssa(inst_op);
    }

    for target in self.jump_targets(inst_op).as_slice() {
      LUAU_ASSERT!(target.block_op.kind == BcOpKind::Block);
      let block_idx = target.block_op.index;
      if !target.dead {
        self
          .state
          .block_uses
          .get_or_insert(block_idx)
          .insert(inst_block);
        if !self.state.flow_worklist_set.contains(&target.block_op) {
          self.state.flow_worklist.push_back(target.block_op);
        }
      }
    }

    self.state.op_constness.insert(inst_op, new_val);
  }

  /// cpp `Sccp::propagate`：flow 工作表驱动块遍历，SSA 工作表驱动格值收敛。
  pub fn propagate(&mut self) {
    let entry_op = self.func.entry_block;
    let exit_op = self.func.exit_block;
    // entry/exit 块恒为活块（exit 因序列化要求）
    self
      .state
      .block_uses
      .get_or_insert(entry_op.index)
      .insert(entry_op);
    self
      .state
      .block_uses
      .get_or_insert(exit_op.index)
      .insert(exit_op);

    self.state.flow_worklist.push_back(entry_op);

    while !self.state.flow_worklist.is_empty() || !self.state.ssa_worklist.is_empty() {
      while let Some(block_op) = self.state.flow_worklist.pop_front() {
        if self.state.flow_worklist_set.contains(&block_op) {
          continue;
        }

        // 把整份缓冲从 self 里换出（O(1) 挪指针，容量随之带走复用）：局部持有后
        // 可按引用迭代，`visit_*` 的 `&mut self` 不再与缓冲的共享借用冲突。
        let mut scratch = mem::take(&mut self.scratch);
        {
          let block_ref = self.func.block(block_op);
          let block = block_ref.operator_deref();
          scratch.phis.clear();
          scratch.phis.extend_from_slice(block.phis.as_slice());
          scratch.ops.clear();
          scratch.ops.extend(block.ops.iter().copied());
          scratch.successors.clear();
          scratch
            .successors
            .extend_from_slice(block.successors.as_slice());
        }

        for &phi_op in &scratch.phis {
          LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);
          self.visit_phi(phi_op);
        }

        for &op in &scratch.ops {
          LUAU_ASSERT!(op.kind == BcOpKind::Inst);
          self.visit_inst(op);
        }

        let block_ends_with_branch = scratch
          .ops
          .last()
          .copied()
          .is_some_and(|last| last.kind == BcOpKind::Inst && !self.jump_targets(last).is_empty());

        for &succ_edge in &scratch.successors {
          if succ_edge.kind == BcBlockEdgeKind::Fallthrough {
            let succ_idx = succ_edge.target.index;
            // 分支已覆盖该后继时跳过，避免重复入队
            if block_ends_with_branch
              && !self
                .state
                .block_uses
                .get_or_insert(succ_idx)
                .contains(&block_op)
            {
              continue;
            }
            self
              .state
              .block_uses
              .get_or_insert(succ_idx)
              .insert(block_op);
            if !self.state.flow_worklist_set.contains(&succ_edge.target) {
              self.state.flow_worklist.push_back(succ_edge.target);
            }
          }
        }

        self.scratch = scratch;
        self.state.flow_worklist_set.insert(block_op);
      }

      while let Some(op) = self.state.ssa_worklist.pop_front() {
        match op.kind {
          BcOpKind::Inst => self.visit_inst(op),
          BcOpKind::Phi => self.visit_phi(op),
          _ => {}
        }
      }
    }
  }
}
