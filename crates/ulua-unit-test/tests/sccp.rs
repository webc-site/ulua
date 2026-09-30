//! Port of `cpp/tests/Sccp.test.cpp`（517 行，12 个 TEST_CASE）。
//!
//! 被测对象：`ulua_bytecode` 的 SCCP 常量传播
//! （对应 C++ `Compiler/src/Sccp.cpp`，Rust 实现
//! `ulua_bytecode::records::sccp` + `functions::sccp_fold_constants`）。
//!
//! 移植说明：
//! - C++ 直接手工搭 `BcFunction<BcVmConst>` IR 再跑 `Sccp::propagate/rewrite`；
//!   Rust 侧同样手工搭 `BcFunction`，`propagate+rewrite` 的封装是
//!   `sccp_fold_constants`；用例 3 需要在 propagate 与 rewrite 之间读格值，
//!   与 C++ 一样直接展开 `Sccp` 结构。
//! - `BcImm` / `BcVmConst` 为带载荷 enum（原 cpp tag+union 的 Rust 建模），
//!   构造与读取均经变体，无 unsafe。

use ulua_bytecode::{
  enums::{
    bc_block_edge_kind::BcBlockEdgeKind, bc_block_flag::BcBlockFlag, bc_imm_kind::BcImmKind,
    bc_op_kind::BcOpKind, bc_vm_const_kind::BcVmConstKind,
  },
  functions::sccp_fold_constants::sccp_fold_constants,
  records::{
    bc_block_edge::BcBlockEdge,
    bc_function::BcFunction,
    bc_imm::BcImm,
    bc_op::BcOp,
    bc_vm_const::BcVmConst,
    sccp::{BcVmConstImpl, Constness, Sccp, SccpState},
  },
  type_aliases::reg::Reg,
};
use ulua_common::{enums::luau_opcode::LuauOpcode, records::small_vector::SmallVector};

/// 追加 Boolean 立即数并写入值
fn add_bool_imm(func: &mut BcFunction<'_>, value: bool) -> BcOp {
  let op = func.add_imm(BcImmKind::Boolean);
  func.immediates[op.index as usize] = BcImm::Boolean(value);
  op
}

/// 追加 Int 立即数并写入值
fn add_int_imm(func: &mut BcFunction<'_>, value: i32) -> BcOp {
  let op = func.add_imm(BcImmKind::Int);
  func.immediates[op.index as usize] = BcImm::Int(value);
  op
}

/// 追加指令并绑定 opcode、所属块与操作数
fn add_inst(func: &mut BcFunction<'_>, op: LuauOpcode, block: BcOp, ops: &[BcOp]) -> BcOp {
  let inst = func.add_inst();
  let slot = &mut func.instructions[inst.index as usize];
  slot.op = op;
  slot.block = block;
  slot.ops = SmallVector::from_iter(ops.iter().cloned());
  func.block_op(block).ops.push_back(inst);
  inst
}

/// 追加 number 类 VM 常量
fn add_number_const(func: &mut BcFunction<'_>, value: f64) -> BcOp {
  let c = BcVmConst::Number(value);
  func.add_const(c)
}

/// 绑定寄存器（cpp `func.regs[op] = reg`）
fn bind_reg(func: &mut BcFunction<'_>, op: BcOp, reg: Reg) {
  func.regs.insert(op, reg);
}

/// 构造块间边
fn edge(kind: BcBlockEdgeKind, target: BcOp) -> BcBlockEdge {
  BcBlockEdge { kind, target }
}

/// 查询块是否带 Dead 标记
fn is_dead(func: &BcFunction<'_>, block: BcOp) -> bool {
  func.blocks[block.index as usize].flags & (BcBlockFlag::Dead as u8) != 0
}

/// entry/bTrue/bFalse/exit 四块骨架（cpp 各用例的公共前奏）
fn setup_branch_arms(func: &mut BcFunction<'_>) -> (BcOp, BcOp, BcOp, BcOp) {
  let entry = func.add_block();
  let b_true = func.add_block();
  let b_false = func.add_block();
  let exit = func.add_block();
  func.entry_block = entry;
  func.exit_block = exit;
  (entry, b_true, b_false, exit)
}

/// 在 block 尾追加 `RETURN value 1`
fn add_return(func: &mut BcFunction<'_>, block: BcOp, value: BcOp) {
  let ret_imm = add_int_imm(func, 1);
  add_inst(func, LuauOpcode::LOP_RETURN, block, &[value, ret_imm]);
}

/// 双向连接 `from --edge--> to`
fn link_edge(func: &mut BcFunction<'_>, from: BcOp, kind: BcBlockEdgeKind, to: BcOp) {
  func.block_op(from).successors.push_back(edge(kind, to));
  func.block_op(to).predecessors.push_back(edge(kind, from));
}

// Source: `tests/Sccp.test.cpp:14-66`
#[test]
fn sccp_does_not_fold_boolean_ordering() {
  let mut func = BcFunction::default();
  let (entry, b_true, b_false, exit) = setup_branch_arms(&mut func);

  // entry: LOADB R0 true
  let imm_true = add_bool_imm(&mut func, true);
  let loadb = add_inst(&mut func, LuauOpcode::LOP_LOADB, entry, &[imm_true]);
  bind_reg(&mut func, loadb, 0);

  // entry: JUMPIFLT loadb loadb bTrue
  add_inst(
    &mut func,
    LuauOpcode::LOP_JUMPIFLT,
    entry,
    &[loadb, loadb, b_true],
  );
  link_edge(&mut func, entry, BcBlockEdgeKind::Branch, b_true);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, b_false);

  // bTrue: RETURN R0 1
  let reg0 = BcOp::with(BcOpKind::VmReg, 0);
  add_return(&mut func, b_true, reg0);
  link_edge(&mut func, b_true, BcBlockEdgeKind::Fallthrough, exit);

  // bFalse: RETURN R0 1
  add_return(&mut func, b_false, reg0);
  link_edge(&mut func, b_false, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  // 布尔无序，JUMPIFLT 不得折叠，两臂均保持可达
  assert!(!is_dead(&func, b_true));
  assert!(!is_dead(&func, b_false));
}

// Source: `tests/Sccp.test.cpp:67-115`
#[test]
fn sccp_folds_number_ordering() {
  let mut func = BcFunction::default();
  let (entry, b_true, b_false, exit) = setup_branch_arms(&mut func);

  // entry: LOADN R0 1, LOADN R1 2
  let imm1 = add_int_imm(&mut func, 1);
  let load1 = add_inst(&mut func, LuauOpcode::LOP_LOADN, entry, &[imm1]);
  bind_reg(&mut func, load1, 0);
  let imm2 = add_int_imm(&mut func, 2);
  let load2 = add_inst(&mut func, LuauOpcode::LOP_LOADN, entry, &[imm2]);
  bind_reg(&mut func, load2, 1);

  // entry: JUMPIFLT load1 load2 bTrue（1 < 2 恒真）
  add_inst(
    &mut func,
    LuauOpcode::LOP_JUMPIFLT,
    entry,
    &[load1, load2, b_true],
  );
  link_edge(&mut func, entry, BcBlockEdgeKind::Branch, b_true);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, b_false);

  // bTrue: RETURN R0 1
  let reg0 = BcOp::with(BcOpKind::VmReg, 0);
  add_return(&mut func, b_true, reg0);
  link_edge(&mut func, b_true, BcBlockEdgeKind::Fallthrough, exit);

  // bFalse: RETURN R0 1
  add_return(&mut func, b_false, reg0);
  link_edge(&mut func, b_false, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  // 1 < 2 恒真：bTrue 活，bFalse 死
  assert!(!is_dead(&func, b_true));
  assert!(is_dead(&func, b_false));
}

// Source: `tests/Sccp.test.cpp:116-182`
#[test]
fn sccp_phi_filters_dead_predecessor() {
  use ulua_bytecode::records::sccp::SccpScratch;

  let mut func = BcFunction::default();
  let (entry, b_true, b_false, exit) = setup_branch_arms(&mut func);
  let merge = func.add_block();

  // entry: LOADB R0 true
  let imm_true = add_bool_imm(&mut func, true);
  let loadb = add_inst(&mut func, LuauOpcode::LOP_LOADB, entry, &[imm_true]);
  bind_reg(&mut func, loadb, 0);

  // entry: JUMPIF loadb bTrue（恒真，bFalse 的 fallthrough 死）
  add_inst(&mut func, LuauOpcode::LOP_JUMPIF, entry, &[loadb, b_true]);
  link_edge(&mut func, entry, BcBlockEdgeKind::Branch, b_true);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, b_false);

  // bTrue: LOADK R1 const 42
  let const42 = add_number_const(&mut func, 42.0);
  let loadk1 = add_inst(&mut func, LuauOpcode::LOP_LOADK, b_true, &[const42]);
  bind_reg(&mut func, loadk1, 1);

  // bFalse: LOADK R1 const 99
  let const99 = add_number_const(&mut func, 99.0);
  let loadk2 = add_inst(&mut func, LuauOpcode::LOP_LOADK, b_false, &[const99]);
  bind_reg(&mut func, loadk2, 1);

  // bTrue -> merge, bFalse -> merge
  link_edge(&mut func, b_true, BcBlockEdgeKind::Fallthrough, merge);
  link_edge(&mut func, b_false, BcBlockEdgeKind::Fallthrough, merge);

  // merge: phi(R1) = {loadk1, loadk2}
  let phi0 = func.add_phi();
  {
    let phi = func.phi_op(phi0);
    phi.ops = SmallVector::from_iter([loadk1, loadk2]);
  }
  func.block_op(merge).phis.push(phi0);
  bind_reg(&mut func, phi0, 1);

  // merge: RETURN R1 1
  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, merge, &[phi0, ret_imm]);
  link_edge(&mut func, merge, BcBlockEdgeKind::Fallthrough, exit);

  let vm_ops = BcVmConstImpl;
  {
    let mut sccp = Sccp {
      func: &mut func,
      vm_ops: &vm_ops,
      state: SccpState::new(),
      scratch: SccpScratch::default(),
    };
    sccp.propagate();

    // bFalse 因恒真条件而死：其操作数从未被访问、留在格顶，
    // phi 因此解析到活操作数的值（42）
    let Constness::VmConstant(phi_const) = *sccp.state.op_constness.find(&phi0).unwrap() else {
      panic!("phi 应解析为 VM 常量格值");
    };
    let vm_const = sccp.func.constants[phi_const.index as usize];
    assert_eq!(vm_const.kind(), BcVmConstKind::Number);
    let value = vm_const.as_number();
    assert_eq!(value, 42.0);

    sccp.rewrite();
  }

  assert!(is_dead(&func, b_false));
}

// Source: `tests/Sccp.test.cpp:183-214`
#[test]
fn sccp_erases_trivial_phi() {
  let mut func = BcFunction::default();
  let entry = func.add_block();
  let exit = func.add_block();
  func.entry_block = entry;
  func.exit_block = exit;

  // entry: LOADK R0 const 42
  let const42 = add_number_const(&mut func, 42.0);
  let loadk = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const42]);
  bind_reg(&mut func, loadk, 0);

  // entry -> exit
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, exit);

  // exit: phi(R0) = {loadk, loadk}
  let phi0 = func.add_phi();
  {
    let phi = func.phi_op(phi0);
    phi.ops = SmallVector::from_iter([loadk, loadk]);
  }
  func.block_op(exit).phis.push(phi0);
  bind_reg(&mut func, phi0, 0);

  // exit: RETURN phi0 1
  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, exit, &[phi0, ret_imm]);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  // simplifyPhis 后平凡 phi 应从 exit 块移除
  assert!(func.block_op(exit).phis.is_empty());
}

// Source: `tests/Sccp.test.cpp:215-247`
#[test]
fn sccp_loadk_mul_to_mulk() {
  let mut func = BcFunction::default();
  let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

  let const42 = add_number_const(&mut func, 42.0);
  let loadk = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const42]);
  bind_reg(&mut func, loadk, 0);

  let upval_imm = BcOp::with(BcOpKind::Imm, 0);
  let upval = add_inst(&mut func, LuauOpcode::LOP_GETUPVAL, entry, &[upval_imm]);
  bind_reg(&mut func, upval, 1);

  let mul = add_inst(&mut func, LuauOpcode::LOP_MUL, entry, &[loadk, upval]);
  bind_reg(&mut func, mul, 2);

  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, entry, &[mul, ret_imm]);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  assert_eq!(func.block_op(entry).ops.len(), 3);
  assert_eq!(
    func.instructions[mul.index as usize].op,
    LuauOpcode::LOP_MULK
  );
}

// Source: `tests/Sccp.test.cpp:282-314`
#[test]
fn sccp_loadk_div_to_divrk() {
  let mut func = BcFunction::default();
  let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

  let const42 = add_number_const(&mut func, 42.0);
  let loadk = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const42]);
  bind_reg(&mut func, loadk, 0);

  let upval_imm = BcOp::with(BcOpKind::Imm, 0);
  let upval = add_inst(&mut func, LuauOpcode::LOP_GETUPVAL, entry, &[upval_imm]);
  bind_reg(&mut func, upval, 1);

  let div = add_inst(&mut func, LuauOpcode::LOP_DIV, entry, &[loadk, upval]);
  bind_reg(&mut func, div, 2);

  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, entry, &[div, ret_imm]);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  assert_eq!(func.block_op(entry).ops.len(), 3);
  let div_inst = &func.instructions[div.index as usize];
  assert_eq!(div_inst.op, LuauOpcode::LOP_DIVRK);
  let k = &func.constants[div_inst.ops.as_slice()[0].index as usize];
  assert_eq!(k.kind(), BcVmConstKind::Number);
  let value = k.as_number();
  assert_eq!(value, 42.0);
}

// 钉住 oracle 源码行为：`cpp/Bytecode/include/Luau/Sccp.h:843-947`（`Sccp::arithToK`
// 的 `valueNumber == 0` 分支，MUL 臂见 :925-932——一侧为 number VmConst 0、另一侧
// NotAConstant 时改写为 `LOADN 0`）。cpp 侧无同名 doctest 用例（`-ltc
// --test-suite=Sccp` 的 12 个用例里没有本情形），故此测试的依据是 oracle 源码，
// 不是 `tests/Sccp.test.cpp` 的行号（原引用 :407-460 实为 nan_compare 用例，属笔误）。
#[test]
fn sccp_loadk_mul_to_zero() {
  let mut func = BcFunction::default();
  let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

  let const0 = add_number_const(&mut func, 0.0);
  let loadk = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const0]);
  bind_reg(&mut func, loadk, 0);

  let upval_imm = BcOp::with(BcOpKind::Imm, 0);
  let upval = add_inst(&mut func, LuauOpcode::LOP_GETUPVAL, entry, &[upval_imm]);
  bind_reg(&mut func, upval, 1);

  let mul = add_inst(&mut func, LuauOpcode::LOP_MUL, entry, &[loadk, upval]);
  bind_reg(&mut func, mul, 2);

  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, entry, &[mul, ret_imm]);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  assert_eq!(func.block_op(entry).ops.len(), 3);
  let mul_inst = &func.instructions[mul.index as usize];
  assert_eq!(mul_inst.op, LuauOpcode::LOP_LOADN);
  let imm = &func.immediates[mul_inst.ops.as_slice()[0].index as usize];
  let value = imm.as_int();
  assert_eq!(value, 0);
}

// 循环回边场景（第 2 轮 sccp_seed_uses 补丁的回归用例）：
// phi 必须经 use 反向边收敛到 NotAConstant，循环累加不得被陈旧格值错误折叠。
// cpp 的 uses 由 GraphParser 增量维护（addUse），Rust 在 SCCP 入口一次性播种，两者等价；
// 播种缺失时 phi 停留格顶陈旧常量，`add` 被错误折叠为 LOADN。
#[test]
fn sccp_loop_carried_phi_stays_not_constant() {
  let mut func = BcFunction::default();
  let entry = func.add_block();
  let b_loop = func.add_block();
  let exit = func.add_block();
  func.entry_block = entry;
  func.exit_block = exit;

  // entry: LOADN R0 0
  let imm0 = add_int_imm(&mut func, 0);
  let load0 = add_inst(&mut func, LuauOpcode::LOP_LOADN, entry, &[imm0]);
  bind_reg(&mut func, load0, 0);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, b_loop);

  // b_loop: ADD R1 = phi + 2（操作数先于 phi 建好，phi 输入引用它）
  let const2 = add_number_const(&mut func, 2.0);
  let phi0 = func.add_phi();
  let add = add_inst(&mut func, LuauOpcode::LOP_ADD, b_loop, &[phi0, const2]);
  bind_reg(&mut func, add, 1);

  // 回边与出口（predecessors 序：entry、b_loop 自身，与 phi 输入序对应）
  link_edge(&mut func, b_loop, BcBlockEdgeKind::Loop, b_loop);
  link_edge(&mut func, b_loop, BcBlockEdgeKind::Fallthrough, exit);

  {
    let phi = func.phi_op(phi0);
    phi.ops = SmallVector::from_iter([load0, add]);
  }
  func.block_op(b_loop).phis.push(phi0);
  bind_reg(&mut func, phi0, 0);

  // exit: RETURN phi 1
  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, exit, &[phi0, ret_imm]);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  // 循环体不被误标死块
  assert!(!is_dead(&func, b_loop));

  // add 保持 ADD 或被合法改写为 ADDK（第一操作数仍是 phi），
  // 不得被陈旧格值折叠成 LOADN 常量
  let inst = &func.instructions[add.index as usize];
  assert!(
    inst.op == LuauOpcode::LOP_ADD
      || (inst.op == LuauOpcode::LOP_ADDK && inst.ops.as_slice().first() == Some(&phi0)),
    "循环累加不得被错误折叠: op={:?} ops={:?}",
    inst.op,
    inst.ops
  );

  // phi 一常量一非常量，非平凡，不得被 simplifyPhis 删除
  assert!(func.block_op(b_loop).phis.contains(&phi0));
}

// Source: `tests/Sccp.test.cpp:248-281`（LOADN 整数 imm 不得被提升为 K 形态）
#[test]
fn sccp_loadn_does_not_promote_to_k() {
  let mut func = BcFunction::default();
  let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

  let imm2 = add_int_imm(&mut func, 2);
  let loadn = add_inst(&mut func, LuauOpcode::LOP_LOADN, entry, &[imm2]);
  bind_reg(&mut func, loadn, 0);

  let upval_imm = BcOp::with(BcOpKind::Imm, 0);
  let upval = add_inst(&mut func, LuauOpcode::LOP_GETUPVAL, entry, &[upval_imm]);
  bind_reg(&mut func, upval, 1);

  // MUL 操作数序：upvalue, loadn（与 cpp 用例一致）
  let mul = add_inst(&mut func, LuauOpcode::LOP_MUL, entry, &[upval, loadn]);
  bind_reg(&mut func, mul, 2);

  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, entry, &[mul, ret_imm]);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  // upvalue 非常量 → MUL 不折叠；LOADN 保持 LOADN，不得改写为 MULK/LOADK
  let mul_inst = &func.instructions[mul.index as usize];
  assert_eq!(mul_inst.op, LuauOpcode::LOP_MUL);
  let loadn_inst = &func.instructions[loadn.index as usize];
  assert_eq!(loadn_inst.op, LuauOpcode::LOP_LOADN);
  assert_eq!(func.block_op(entry).ops.len(), 4);
}

// Source: `tests/Sccp.test.cpp:315-348`（`1 // -2` 必须向负无穷折叠为 -1）
//
// cpp 落点为 `LOADK K0 (-1)`；本端口按已记录的 DELIBERATE DEVIATION
// （`sccp_evaluate_arith.rs` 头注 + tests/sccp_int_imm_fold.rs）把双 Int-imm
// 折叠落 `LOADN` 以保住 integer 子类型。行为 oracle 取 -1 的值本身，
// 表示形态按偏差钉为 LOADN(-1)。
#[test]
fn sccp_immediate_floor_division_toward_zero() {
  let mut func = BcFunction::default();
  let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

  let imm1 = add_int_imm(&mut func, 1);
  let lhs = add_inst(&mut func, LuauOpcode::LOP_LOADN, entry, &[imm1]);
  bind_reg(&mut func, lhs, 0);
  let immneg2 = add_int_imm(&mut func, -2);
  let rhs = add_inst(&mut func, LuauOpcode::LOP_LOADN, entry, &[immneg2]);
  bind_reg(&mut func, rhs, 1);

  let idiv = add_inst(&mut func, LuauOpcode::LOP_IDIV, entry, &[lhs, rhs]);
  bind_reg(&mut func, idiv, 2);

  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, entry, &[idiv, ret_imm]);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  // 1 // -2 = -1（floor，非截断的 0）
  let inst = &func.instructions[idiv.index as usize];
  match inst.op {
    LuauOpcode::LOP_LOADN => {
      let imm = &func.immediates[inst.ops.as_slice()[0].index as usize];
      assert_eq!(imm.as_int(), -1);
    }
    LuauOpcode::LOP_LOADK => {
      let c = &func.constants[inst.ops.as_slice()[0].index as usize];
      assert_eq!(c.kind(), BcVmConstKind::Number);
      assert_eq!(c.as_number(), -1.0);
    }
    other => panic!("IDIV 未折叠: op={other:?}"),
  }
}

mod sccp_nan_compare {
  //! Source: `tests/Sccp.test.cpp:349-482`（NaN 三比较：==、<=、NOTLE(1,NaN)）

  use super::*;

  /// JUMPIFEQ nan,nan：NaN == NaN 为假 → 分支不跳，bTrue 死、bFalse 活
  #[test]
  fn sccp_nan_compare_check1() {
    let mut func = BcFunction::default();
    let (entry, b_true, b_false, exit) = setup_branch_arms(&mut func);

    let nan_const = add_number_const(&mut func, f64::NAN);
    let nan = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[nan_const]);
    bind_reg(&mut func, nan, 0);

    add_inst(
      &mut func,
      LuauOpcode::LOP_JUMPIFEQ,
      entry,
      &[nan, nan, b_true],
    );
    link_edge(&mut func, entry, BcBlockEdgeKind::Branch, b_true);
    link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, b_false);

    add_return(&mut func, b_true, BcOp::with(BcOpKind::VmReg, 0));
    link_edge(&mut func, b_true, BcBlockEdgeKind::Fallthrough, exit);
    add_return(&mut func, b_false, BcOp::with(BcOpKind::VmReg, 0));
    link_edge(&mut func, b_false, BcBlockEdgeKind::Fallthrough, exit);

    sccp_fold_constants(&mut func, &BcVmConstImpl);

    assert!(is_dead(&func, b_true), "NaN == NaN 恒假，bTrue 应死");
    assert!(!is_dead(&func, b_false));
  }

  /// JUMPIFLE nan,nan：NaN <= NaN 为假 → 同样 bTrue 死
  #[test]
  fn sccp_nan_compare_check2() {
    let mut func = BcFunction::default();
    let (entry, b_true, b_false, exit) = setup_branch_arms(&mut func);

    let nan_const = add_number_const(&mut func, f64::NAN);
    let nan = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[nan_const]);
    bind_reg(&mut func, nan, 0);

    add_inst(
      &mut func,
      LuauOpcode::LOP_JUMPIFLE,
      entry,
      &[nan, nan, b_true],
    );
    link_edge(&mut func, entry, BcBlockEdgeKind::Branch, b_true);
    link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, b_false);

    add_return(&mut func, b_true, BcOp::with(BcOpKind::VmReg, 0));
    link_edge(&mut func, b_true, BcBlockEdgeKind::Fallthrough, exit);
    add_return(&mut func, b_false, BcOp::with(BcOpKind::VmReg, 0));
    link_edge(&mut func, b_false, BcBlockEdgeKind::Fallthrough, exit);

    sccp_fold_constants(&mut func, &BcVmConstImpl);

    assert!(is_dead(&func, b_true), "NaN <= NaN 恒假，bTrue 应死");
    assert!(!is_dead(&func, b_false));
  }

  /// JUMPIFNOTLE 1,nan：!(1 <= NaN) 为真 → 跳 bTrue，bFalse 死
  #[test]
  fn sccp_nan_compare_check3() {
    let mut func = BcFunction::default();
    let (entry, b_true, b_false, exit) = setup_branch_arms(&mut func);

    let one_const = add_number_const(&mut func, 1.0);
    let one = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[one_const]);
    bind_reg(&mut func, one, 0);
    let nan_const = add_number_const(&mut func, f64::NAN);
    let nan = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[nan_const]);
    bind_reg(&mut func, nan, 1);

    add_inst(
      &mut func,
      LuauOpcode::LOP_JUMPIFNOTLE,
      entry,
      &[one, nan, b_true],
    );
    link_edge(&mut func, entry, BcBlockEdgeKind::Branch, b_true);
    link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, b_false);

    add_return(&mut func, b_true, BcOp::with(BcOpKind::VmReg, 0));
    link_edge(&mut func, b_true, BcBlockEdgeKind::Fallthrough, exit);
    add_return(&mut func, b_false, BcOp::with(BcOpKind::VmReg, 0));
    link_edge(&mut func, b_false, BcBlockEdgeKind::Fallthrough, exit);

    sccp_fold_constants(&mut func, &BcVmConstImpl);

    assert!(!is_dead(&func, b_true));
    assert!(is_dead(&func, b_false), "!(1 <= NaN) 恒真，bFalse 应死");
  }
}

// Source: `tests/Sccp.test.cpp:483-516`（LOADN imm × LOADK vm 混合折叠）
//
// cpp 把 `1 + 2.0` 折叠为常量 `LOADK K1 (3)`；本端口 oracle 断言同值。
#[test]
fn sccp_loadn_plus_loadk_fold() {
  let mut func = BcFunction::default();
  let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

  let imm1 = add_int_imm(&mut func, 1);
  let lhs = add_inst(&mut func, LuauOpcode::LOP_LOADN, entry, &[imm1]);
  bind_reg(&mut func, lhs, 0);
  let const2 = add_number_const(&mut func, 2.0);
  let rhs = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const2]);
  bind_reg(&mut func, rhs, 1);

  let add = add_inst(&mut func, LuauOpcode::LOP_ADD, entry, &[lhs, rhs]);
  bind_reg(&mut func, add, 2);

  let ret_imm = add_int_imm(&mut func, 1);
  add_inst(&mut func, LuauOpcode::LOP_RETURN, entry, &[add, ret_imm]);
  link_edge(&mut func, entry, BcBlockEdgeKind::Fallthrough, exit);

  sccp_fold_constants(&mut func, &BcVmConstImpl);

  // cpp：混合 Imm×Vm 操作数升 double 折叠为常量 3.0
  let inst = &func.instructions[add.index as usize];
  assert_eq!(
    inst.op,
    LuauOpcode::LOP_LOADK,
    "LOADN×LOADK 混合加法应折叠为 LOADK（cpp Sccp.test.cpp:483）"
  );
  let c = &func.constants[inst.ops.as_slice()[0].index as usize];
  assert_eq!(c.kind(), BcVmConstKind::Number);
  assert_eq!(c.as_number(), 3.0);
}
