// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。
use std::vec::Vec;

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_block::BcBlock, bc_function::BcFunction, bc_imm::BcImm, bc_inst::BcInst, bc_op::BcOp,
    bc_ref::BcRef,
  },
  type_aliases::reg::Reg,
};

pub trait BcInstHelperCreate {
  const OPCODE: LuauOpcode;
}

/// cpp `BcInstHelper`（`BytecodeOps.h:44-75`）。
///
/// 与 cpp 的 `BcInst*` 成员对应，这里只保存指令的 `BcOp` 下标 + 对图的**唯一**
/// 可变借用：可变访问一律经 `self.graph`（`BcFunction::inst_op` / 本类型的
/// `operator_deref_mut`）现取现用，不再从 `&Vec<T>` 伪造 `*mut T`（旧实现经
/// `BcRef::operator_arrow` 与 `graph as *mut BcFunction` 构成双 `&mut` 回环，
/// 属 Stacked Borrows UB，且 `instructions` 重分配后会悬垂）。
#[derive(Debug)]
pub struct BcInstHelper<'a, 'f> {
  pub(crate) graph: &'a mut BcFunction<'f>,
  pub(crate) inst: BcOp,
}

impl<'a, 'f> BcInstHelper<'a, 'f> {
  pub(crate) fn new(graph: &'a mut BcFunction<'f>, inst: BcOp) -> Self {
    Self { graph, inst }
  }

  /// cpp `BcInstHelper::create<T>(graph)`：新增一条 `T::OPCODE` 指令并返回持有图的
  /// 唯一可变借用的 helper。
  ///
  /// 旧实现在此用 `graph as *mut BcFunction` 把同一个图再借两次（`inst(op)` 的
  /// `&`、`BcInstHelper::new` 的 `&mut`），是 Stacked Borrows UB；现在 helper 只存
  /// `BcOp` 下标，无需二次借用。
  pub fn create<T>(graph: &'a mut BcFunction<'f>) -> Self
  where
    T: BcInstHelperCreate,
  {
    let op = graph.add_inst();
    graph.inst_op(op).op = T::OPCODE;
    BcInstHelper::new(graph, op)
  }

  /// cpp `BcInstHelper::op()`：本 helper 指向的指令的 `BcOp` 句柄。
  pub fn op(&self) -> BcOp {
    self.inst
  }

  /// cpp `BcInstHelper::operator*`：按下标从持有的图里现取指令，越界即 panic
  /// （旧实现经 `BcRef::operator_arrow` 从 `&Vec<BcInst>` 造 `*mut BcInst`，属 UB）。
  pub fn operator_deref(&self) -> &BcInst {
    LUAU_ASSERT!((self.inst.index as usize) < self.graph.instructions.len());
    &self.graph.instructions[self.inst.index as usize]
  }

  /// cpp `BcInstHelper::operator->`（`BytecodeOps.h`）的可变面：可变访问统一经持有
  /// `&mut BcFunction` 的 `self.graph`（`BcFunction::inst_op`）现取现用，下标越界时
  /// 是边界检查失败的可诊断 panic。
  ///
  /// 旧实现写在 `bc_inst_helper_set_vm_const.rs` 文件底部的 `impl BcRef` 里，对 `u32`
  /// 索引不做边界检查，直接 `&mut *(vec.as_ptr().add(index))` 越界写；现独立成文件
  /// 并改走边界检查路径。
  pub(crate) fn operator_deref_mut(&mut self) -> &mut BcInst {
    LUAU_ASSERT!((self.inst.index as usize) < self.graph.instructions.len());
    self.graph.inst_op(self.inst)
  }

  /// cpp `BcInstHelper::getBlock(inputIdx)`：第 `inputIdx` 个输入对应的块（只读视图）。
  pub fn get_block(&mut self, input_idx: u32) -> BcRef<'_, BcBlock> {
    let block_op = self.get_bc_op(input_idx);
    LUAU_ASSERT!(block_op.kind == BcOpKind::Block);
    self.graph.block(block_op)
  }

  /// cpp `BcInstHelper::getOutReg()`：指令结果寄存器取自图的 `regs` 映射。
  ///
  /// cpp 原址只用 `LUAU_ASSERT` 守映射命中，release 下解引用 `end()` 是 UB。
  /// 按 6e1992b 的裁定收敛为「断言 + 非 panic 读法」：未登记时回落寄存器 0，
  /// 调用方（内联器）随后在图重建处自然失效，不再让损坏输入触发越界读。
  pub(crate) fn get_out_reg(&self) -> Reg {
    LUAU_ASSERT!(self.graph.regs.contains_key(&self.inst));
    *self.graph.regs.get(&self.inst).unwrap_or(&0)
  }

  /// cpp `BcInstHelper::setOutReg(reg)`。
  pub(crate) fn set_out_reg(&mut self, out: Reg) {
    self.graph.regs.insert(self.inst, out);
  }

  pub(crate) fn get_bc_op(&mut self, input_idx: u32) -> BcOp {
    if (input_idx as usize) >= self.operator_deref().ops.len() {
      self.operator_deref_mut().ops.resize(input_idx + 1);
    }
    self.operator_deref().ops[input_idx as usize]
  }

  pub(crate) fn set_bc_op(&mut self, input_idx: u32, op: BcOp) {
    if input_idx >= self.operator_deref().ops.len() as u32 {
      self.operator_deref_mut().ops.resize(input_idx + 1);
    }
    self.operator_deref_mut().ops[input_idx as usize] = op;
  }

  pub(crate) fn int_imm_input(&mut self, input_idx: u32) -> i32 {
    let inst = self.operator_deref();
    LUAU_ASSERT!((input_idx as usize) < inst.ops.len());

    let op = inst.ops[input_idx as usize];
    let imm = self.graph.imm_op(op);

    // Safety:图构建阶段该操作数只可能由 `addImmInput(int32_t)` 写入（BcImmKind::Int）。
    imm.as_int()
  }

  pub(crate) fn set_imm_input(&mut self, input_idx: u32, value: i32) {
    if self.get_bc_op(input_idx).kind == BcOpKind::None {
      let imm_op = self.graph.add_imm(BcImmKind::Int);
      self.set_bc_op(input_idx, imm_op);
    }

    let op = self.get_bc_op(input_idx);
    let imm = self.graph.imm_op(op);

    LUAU_ASSERT!(imm.kind() == BcImmKind::Int);
    if let BcImm::Int(slot) = imm {
      *slot = value;
    }
  }

  /// cpp `BcInstHelper::setVMConst(inputIdx, cid)`：把第 `inputIdx` 个输入换成常量。
  pub(crate) fn set_vm_const(&mut self, input_idx: u32, cid: u32) {
    LUAU_ASSERT!(cid < self.graph.constants.len() as u32);
    self.set_bc_op(input_idx, BcOp::with(BcOpKind::VmConst, cid));
  }

  /// cpp `BcInstHelper::appendTo(block)`：改写指令的 `block` 并挂到块尾。
  pub(crate) fn append_to(&mut self, block: BcOp) {
    let op = self.inst;
    self.operator_deref_mut().block = block;
    self.graph.block_op(block).append_instruction(op);
  }

  /// cpp `BcInstHelper::prependTo(block)`：改写指令的 `block` 并挂到块首。
  pub(crate) fn prepend_to(&mut self, block: BcOp) {
    let op = self.inst;
    self.operator_deref_mut().block = block;
    self.graph.block_op(block).ops.push_front(op);
  }

  /// cpp `BcInstHelper::detach()`（BytecodeOps.h:64-71）：把指令从所属块的 `ops`
  /// 序列里摘掉并清空 `block`，指令本身仍留在 `instructions` 池中。
  pub(crate) fn detach(&mut self) {
    let block = self.operator_deref().block;
    if block.kind != BcOpKind::Block {
      return;
    }
    let op = self.inst;
    self
      .graph
      .block_op(block)
      .ops
      .retain(|existing| *existing != op);
    self.operator_deref_mut().block = BcOp::new();
  }

  pub(crate) fn slice_inputs(&self, start_from: u32) -> Vec<BcOp> {
    let ops = &self.operator_deref().ops;
    let start = start_from as usize;
    if start >= ops.len() {
      Vec::new()
    } else {
      ops[start..].to_vec()
    }
  }
}
