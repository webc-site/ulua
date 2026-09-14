//! Port of `cpp/tests/Sccp.test.cpp`（462 行，7 个 TEST_CASE）。
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
//! - `BcImmValue` / `BcVmConstValue` 是 union，按 kind 读取活跃字段，
//!   `unsafe` 处均注明不变量。

use ulua_bytecode::{
  enums::{
    bc_block_edge_kind::BcBlockEdgeKind, bc_block_flag::BcBlockFlag, bc_imm_kind::BcImmKind,
    bc_op_kind::BcOpKind, bc_vm_const_kind::BcVmConstKind,
  },
  functions::sccp_fold_constants::sccp_fold_constants,
  records::{
    bc_block_edge::BcBlockEdge,
    bc_function::BcFunction,
    bc_imm::BcImmValue,
    bc_op::BcOp,
    bc_vm_const::{BcVmConst, BcVmConstValue},
    sccp::{BcVmConstImpl, Constness, Sccp, SccpState},
  },
  type_aliases::reg::Reg,
};
use ulua_common::{enums::luau_opcode::LuauOpcode, records::small_vector::SmallVector};

/// 追加 Boolean 立即数并写入值
fn add_bool_imm(func: &mut BcFunction, value: bool) -> BcOp {
  let op = func.add_imm(BcImmKind::Boolean);
  func.immediates[op.index as usize].value = BcImmValue {
    value_boolean: value,
  };
  op
}

/// 追加 Int 立即数并写入值
fn add_int_imm(func: &mut BcFunction, value: i32) -> BcOp {
  let op = func.add_imm(BcImmKind::Int);
  func.immediates[op.index as usize].value = BcImmValue { value_int: value };
  op
}

/// 追加指令并绑定 opcode、所属块与操作数
fn add_inst(func: &mut BcFunction, op: LuauOpcode, block: BcOp, ops: &[BcOp]) -> BcOp {
  let inst = func.add_inst();
  let slot = &mut func.instructions[inst.index as usize];
  slot.op = op;
  slot.block = block;
  slot.ops = SmallVector::from_iter(ops.iter().cloned());
  func.block_op(block).ops.push_back(inst);
  inst
}

/// 追加 number 类 VM 常量
fn add_number_const(func: &mut BcFunction, value: f64) -> BcOp {
  let c = BcVmConst {
    kind: BcVmConstKind::Number,
    value: BcVmConstValue {
      value_number: value,
    },
  };
  func.add_const(c)
}

/// 绑定寄存器（cpp `func.regs[op] = reg`）
fn bind_reg(func: &mut BcFunction, op: BcOp, reg: Reg) {
  func.regs.insert(op, reg);
}

/// 构造块间边
fn edge(kind: BcBlockEdgeKind, target: BcOp) -> BcBlockEdge {
  BcBlockEdge { kind, target }
}

/// 查询块是否带 Dead 标记
fn is_dead(func: &BcFunction, block: BcOp) -> bool {
  func.blocks[block.index as usize].flags & (BcBlockFlag::Dead as u8) != 0
}

/// entry/bTrue/bFalse/exit 四块骨架（cpp 各用例的公共前奏）
fn setup_branch_arms(func: &mut BcFunction) -> (BcOp, BcOp, BcOp, BcOp) {
  let entry = func.add_block();
  let b_true = func.add_block();
  let b_false = func.add_block();
  let exit = func.add_block();
  func.entry_block = entry;
  func.exit_block = exit;
  (entry, b_true, b_false, exit)
}

/// 在 block 尾追加 `RETURN value 1`
fn add_return(func: &mut BcFunction, block: BcOp, value: BcOp) {
  let ret_imm = add_int_imm(func, 1);
  add_inst(func, LuauOpcode::LOP_RETURN, block, &[value, ret_imm]);
}

/// 双向连接 `from --edge--> to`
fn link_edge(func: &mut BcFunction, from: BcOp, kind: BcBlockEdgeKind, to: BcOp) {
  func.block_op(from).successors.push_back(edge(kind, to));
  func.block_op(to).predecessors.push_back(edge(kind, from));
}

mod sccp_does_not_fold_boolean_ordering {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Sccp.test.cpp:13:sccp_does_not_fold_boolean_ordering`
  //! Source: `tests/Sccp.test.cpp:13-75`

  use super::*;

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
    let reg0 = BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmReg, 0);
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
}

mod sccp_folds_number_ordering {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Sccp.test.cpp:77:sccp_folds_number_ordering`
  //! Source: `tests/Sccp.test.cpp:77-148`

  use super::*;

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
    let reg0 = BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmReg, 0);
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
}

mod sccp_phi_filters_dead_predecessor {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Sccp.test.cpp:150:sccp_phi_filters_dead_predecessor`
  //! Source: `tests/Sccp.test.cpp:150-246`

  use super::*;

  #[test]
  fn sccp_phi_filters_dead_predecessor() {
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
      };
      sccp.propagate();

      // bFalse 因恒真条件而死：其操作数从未被访问、留在格顶，
      // phi 因此解析到活操作数的值（42）
      let lattice = sccp.state.op_constness.find(&phi0);
      assert!(lattice.is_some());
      let lattice = *lattice.unwrap();
      assert_eq!(lattice.kind, Constness::VmConstant);
      let vm_const = sccp.func.constants[lattice.vm_const.unwrap().index as usize];
      assert_eq!(vm_const.kind, BcVmConstKind::Number);
      // Safety: kind == Number 时 value_number 为活跃字段
      let value = unsafe { vm_const.value.value_number };
      assert_eq!(value, 42.0);

      sccp.rewrite();
    }

    assert!(is_dead(&func, b_false));
  }
}

mod sccp_erases_trivial_phi {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Sccp.test.cpp:248:sccp_erases_trivial_phi`
  //! Source: `tests/Sccp.test.cpp:248-295`

  use super::*;

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
}

mod sccp_loadk_mul_to_mulk {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Sccp.test.cpp:297:sccp_loadk_mul_to_mulk`
  //! Source: `tests/Sccp.test.cpp:297-349`

  use super::*;

  #[test]
  fn sccp_loadk_mul_to_mulk() {
    let mut func = BcFunction::default();
    let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

    let const42 = add_number_const(&mut func, 42.0);
    let loadk = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const42]);
    bind_reg(&mut func, loadk, 0);

    let upval_imm = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, 0);
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
}

mod sccp_loadk_div_to_divrk {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Sccp.test.cpp:351:sccp_loadk_div_to_divrk`
  //! Source: `tests/Sccp.test.cpp:351-405`

  use super::*;

  #[test]
  fn sccp_loadk_div_to_divrk() {
    let mut func = BcFunction::default();
    let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

    let const42 = add_number_const(&mut func, 42.0);
    let loadk = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const42]);
    bind_reg(&mut func, loadk, 0);

    let upval_imm = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, 0);
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
    assert_eq!(k.kind, BcVmConstKind::Number);
    // Safety: kind == Number 时 value_number 为活跃字段
    let value = unsafe { k.value.value_number };
    assert_eq!(value, 42.0);
  }
}

mod sccp_loadk_mul_to_zero {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Sccp.test.cpp:407:sccp_loadk_mul_to_zero`
  //! Source: `tests/Sccp.test.cpp:407-460`

  use super::*;

  #[test]
  fn sccp_loadk_mul_to_zero() {
    let mut func = BcFunction::default();
    let (entry, _b_true, _b_false, exit) = setup_branch_arms(&mut func);

    let const0 = add_number_const(&mut func, 0.0);
    let loadk = add_inst(&mut func, LuauOpcode::LOP_LOADK, entry, &[const0]);
    bind_reg(&mut func, loadk, 0);

    let upval_imm = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, 0);
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
    // Safety: kind == Int 时 value_int 为活跃字段
    let value = unsafe { imm.value.value_int };
    assert_eq!(value, 0);
  }
}
