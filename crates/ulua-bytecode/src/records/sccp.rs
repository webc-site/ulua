use alloc::{collections::VecDeque, vec::Vec};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  records::{
    dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault,
  },
};

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_block_edge::BcBlockEdge, bc_function::BcFunction, bc_imm::BcImm, bc_op::BcOp,
    bc_op_hash::BcOpHash,
  },
};

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

impl Constness {
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
  pub condition: ConditionState,
}

/// cpp `DenseHashSet<BcOp>` 的 DenseHashMap 值包装（块 → 到达它的块集合）。
#[derive(Debug)]
pub struct SccpBlockUses(pub DenseHashSet<BcOp, BcOpHash>);

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
  pub block_uses: DenseHashMap<u32, SccpBlockUses>,
  pub flow_worklist: VecDeque<BcOp>,
  pub flow_worklist_set: DenseHashSet<BcOp, BcOpHash>,
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
  pub fn operand_lattice(&mut self, op: BcOp) -> Constness {
    if matches!(
      op.kind,
      BcOpKind::Proj | BcOpKind::VmReg | BcOpKind::VmUpvalue
    ) {
      return Constness::NotAConstant;
    }
    *self.op_constness.get_or_insert(op)
  }

  /// 未解析条件的格：任一操作数为格底则格底，否则格顶。
  pub fn unknown_condition_constness(&mut self, ops: &[BcOp]) -> Constness {
    for &op in ops {
      if matches!(self.operand_lattice(op), Constness::NotAConstant) {
        return Constness::NotAConstant;
      }
    }
    Constness::Undetermined
  }

  /// 反向边记录：def 仅接受 Inst/Phi 消费者（对齐 cpp `recordUse`）。
  pub fn record_use(&mut self, def: BcOp, user: BcOp) {
    if matches!(def.kind, BcOpKind::Inst | BcOpKind::Phi) {
      self.op_uses.get_or_insert(def).push(user);
    }
  }

  /// 反向边解除：从 def 的使用列表移除 user（对齐 cpp `eraseUse`）。
  pub fn erase_use(&mut self, user: BcOp, def: BcOp) {
    if let Some(uses) = self.op_uses.find_mut(&def) {
      uses.retain(|u| *u != user);
    }
  }

  pub fn uses_of(&self, def: BcOp) -> &[BcOp] {
    self.op_uses.get(&def).map_or(&[], |v| v.as_slice())
  }
}

impl Default for SccpState {
  fn default() -> Self {
    Self::new()
  }
}

/// cpp `VmConstOps` 虚接口的 Rust 镜像。实现集封闭于本 crate（`BcVmConstImpl`），
/// 故以泛型静态分发替代 dyn；`func` 以参数传递而非持有引用，规避可变借用冲突。
pub trait VmConstOps {
  /// 折叠双常量算术；不支持时返回 None。
  fn evaluate(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp, op: LuauOpcode) -> Option<BcOp>;
  fn falsey(&self, func: &mut BcFunction, op: BcOp) -> bool;
  /// 三路比较：lhs < rhs 为 -1，相等为 0，大于为 1
  fn cmp_ops(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp) -> i32;
  fn cmp_imm(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcImm) -> i32;
  fn make_nil(&self, func: &mut BcFunction) -> BcOp;
  fn make_imm_bool(&self, value: bool) -> BcImm;
  fn make_imm_int(&self, value: i32) -> BcImm;
  /// 仅数值类常量（number/integer/string）支持排序比较
  fn is_orderable(&self, func: &mut BcFunction, op: BcOp) -> bool;
  fn kind_equals(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp) -> bool;
  /// 比较不受支持时返回 None
  fn eq_ops(&self, func: &mut BcFunction, lhs: BcOp, rhs: BcOp) -> Option<bool>;
  fn eq_bool(&self, func: &mut BcFunction, lhs: BcOp, rhs: bool) -> Option<bool>;
  fn eq_int(&self, func: &mut BcFunction, lhs: BcOp, rhs: i32) -> Option<bool>;
  /// 仅 LUA_TNUMBER
  fn is_arithmetic_constant(&self, func: &mut BcFunction, op: BcOp) -> bool;
  fn as_number(&self, func: &mut BcFunction, op: BcOp) -> f64;
  fn as_imm(&self, func: &mut BcFunction, op: BcOp) -> BcImm;
}

/// cpp `BcVmConstImpl`：唯一实现，无状态。
#[derive(Debug, Clone, Copy, Default)]
pub struct BcVmConstImpl;

/// cpp `Sccp<VmConst>` 主结构。泛型参数为常量求值器实现。
pub struct Sccp<'f, 'i, I: ?Sized> {
  pub func: &'f mut BcFunction,
  pub vm_ops: &'i I,
  pub state: SccpState,
  /// 每块遍历前把块内容拷进 scratch，避免 `visit_*(&mut self)` 与块借用
  /// 冲突而整块堆 clone；缓冲跨块复用，只拷贝不分配。
  pub scratch: SccpScratch,
}

/// `propagate` 逐块复用的遍历缓冲（BcOp/BcBlockEdge 均为 Copy）。
#[derive(Default)]
pub struct SccpScratch {
  pub phis: Vec<BcOp>,
  pub ops: Vec<BcOp>,
  pub successors: Vec<BcBlockEdge>,
}
