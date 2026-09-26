//! [findings S6] 双 Int-imm 算术折叠的决策回归（cpp `Sccp.cpp:240-262`）。
//!
//! cpp `SccpInterpreter::evaluateArith` 把任意数值常量对（Imm-Int 与
//! VmConst-Number 任意组合）经 `asNumber` 升为 double，折叠成 Number 型
//! VmConst 后落 `LOADK`；Rust 版仅对"双 Int-imm 且结果可整表示、落 i16
//! 射程"的组合有意改为整数折叠落 `LOADN`（保住 Lua 的 integer 语义，
//! `math.type` 不漂移）。本文件固化该收窄后的偏差边界，防止 sync-cpp 时
//! 被改回：**其余一切组合（超射程、DIV/POW 非整结果、除零 inf/NaN、
//! Imm×VmConst 数值混合对）必须与 cpp 同型落 double 通路折出 Number
//! VmConst**，不得退回"无条件放弃折叠"的旧形态。
//!
//! 另钉住 `JUMPIFEQ` 混合臂（VmConst×Imm）的折叠边界（cpp `Sccp.cpp:139-168`
//! `eq` 对异种类对返回 nullopt → `evaluateComparisonCondition` 落 Unknown）。

use ulua_bytecode::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  functions::{
    from_function_bytecode::from_function_bytecode, sccp_fold_constants::sccp_fold_constants,
  },
  records::{
    bc_function::BcFunction, bc_op::BcOp, bc_vm_const::BcVmConst,
    bytecode_builder::BytecodeBuilder, sccp::BcVmConstImpl, string_ref::StringRef,
  },
};
use ulua_common::enums::luau_opcode::LuauOpcode;

/// 折叠后可观察面摘要（`BcFunction` 借用 builder 的字符串表，无法逃出构造函数）。
struct Folded {
  has_loadk: bool,
  /// 源算术指令是否仍以未折叠形态存活。
  has_source_op: bool,
  /// 图中所有 `LOADN` 节点的 Int imm 载荷（升序）。
  loadn_imms: Vec<i32>,
}

/// `LOAD lhs→r0; LOAD rhs→r1; <op> r2=r0 op r1; RETURN r2`，建图后跑一遍 SCCP。
fn build_and_fold_operands(op: LuauOpcode, lhs: CmpOperand, rhs: CmpOperand) -> Folded {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  emit_operand_load(&mut bcb, 0, &lhs);
  emit_operand_load(&mut bcb, 1, &rhs);
  bcb.emit_abc(op, 2, 0, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 2, 2, 0);
  bcb.end_function(3, 0, 0, 0);

  let graph = fold_bcb(&mut bcb);

  let loadn_imms: Vec<i32> = graph
    .instructions
    .iter()
    .filter(|inst| inst.op == LuauOpcode::LOP_LOADN)
    .map(|inst| {
      let ops: Vec<BcOp> = inst.ops.as_slice().to_vec();
      assert_eq!(ops.len(), 1, "LOADN 须恰有 1 个输入");
      assert_eq!(ops[0].kind, BcOpKind::Imm);
      let imm = graph.immediates[ops[0].index as usize];
      assert_eq!(imm.kind(), BcImmKind::Int, "LOADN 输入须为 Int imm");
      imm.as_int()
    })
    .collect();

  Folded {
    has_loadk: graph
      .instructions
      .iter()
      .any(|inst| inst.op == LuauOpcode::LOP_LOADK),
    has_source_op: graph.instructions.iter().any(|inst| inst.op == op),
    loadn_imms: {
      let mut imms = loadn_imms;
      imms.sort_unstable();
      imms
    },
  }
}

/// 双 Int-imm 快捷形态。
fn build_and_fold(op: LuauOpcode, lhs: i16, rhs: i16) -> Folded {
  build_and_fold_operands(op, CmpOperand::Int(lhs), CmpOperand::Int(rhs))
}

/// 各 harness 共用：把 `bcb` 的函数 0 解析成图并跑一遍 SCCP 折叠（旧逐用例
/// 重复的 4 行尾部样板单源）。
fn fold_bcb<'a>(bcb: &'a mut BytecodeBuilder<'_>) -> BcFunction<'a> {
  let data = bcb.get_function_data(0);
  let strings = bcb.get_string_table();
  let mut graph = from_function_bytecode(&data, &strings).expect("自产函数块必须可解析");
  sccp_fold_constants(&mut graph, &BcVmConstImpl);
  graph
}

/// `<op>` 折叠后是否仍是**活跃指令**（在块 ops 里）：被折成恒真/恒假时指令
/// 经 `erase_op` 从所属块摘除（`instructions` 池仍残留对象，不能作判据）。
fn op_survived(graph: &BcFunction<'_>, op: LuauOpcode) -> bool {
  graph.blocks.iter().any(|block| {
    block
      .ops
      .iter()
      .any(|&i| graph.inst(i).operator_deref().op == op)
  })
}

/// 射程内的双 Int-imm 折叠：结果为 `LOADN`（Int imm 5），绝不落 `LOADK`/Number。
#[test]
fn int_imm_add_folds_to_loadn_not_loadk() {
  let folded = build_and_fold(LuauOpcode::LOP_ADD, 2, 3);

  // cpp 会折叠成 Number VmConst（LOADK 5.0）；Rust 决策为整数 LOADN 5
  assert!(
    !folded.has_loadk,
    "Int-imm 折叠不得产出 LOADK（Number），该路径会丢 integer 语义"
  );
  assert_eq!(
    folded.loadn_imms,
    vec![2, 3, 5],
    "ADD 须折叠为 LOADN 5（双操作数各自的 LOADN 保留）"
  );
}

/// 折叠结果超出 i16 射程（`LOADN` 载荷上界）时不得放弃：回落 cpp 同款
/// double 通路折出 Number VmConst（LOADK 60000）。旧版无条件 NotAConstant，
/// 折叠强度低于 oracle，系偏差范围失控——偏差只允许覆盖"可整表示且射程内"
/// 的组合。
#[test]
fn int_imm_add_out_of_loadn_range_folds_to_loadk() {
  let folded = build_and_fold(LuauOpcode::LOP_ADD, 30000, 30000);

  assert!(
    !folded.has_source_op,
    "60000 超出 LOADN 射程须落 LOADK 折叠（cpp 同型），不得保留 ADD"
  );
  assert!(folded.has_loadk, "超射程整对的折叠产物须是 Number VmConst");
  assert_eq!(
    folded.loadn_imms,
    vec![30000, 30000],
    "仅存两条源 LOADN，算术结果以 LOADK 形态存活"
  );
}

/// DIV 的整数结果并入 LOADN 形态（偏差载体）：6/2=3 可整表示且射程内。
#[test]
fn int_imm_div_integral_result_folds_to_loadn() {
  let folded = build_and_fold(LuauOpcode::LOP_DIV, 6, 2);

  assert!(!folded.has_loadk, "整值 DIV 结果须落 LOADN 3，不落 LOADK");
  assert!(!folded.has_source_op, "DIV 6/2 射程内必须折叠");
  let mut expected = [6, 2, 3];
  expected.sort_unstable();
  assert_eq!(folded.loadn_imms, expected, "DIV 6/2 须折叠为 LOADN 3");
}

/// DIV 的非整结果（7/2=3.5）不得并入 LOADN，须与 cpp 同型折出 Number
/// VmConst。旧版对 DIV 无条件放弃折叠。
#[test]
fn int_imm_div_fractional_result_folds_to_loadk() {
  let folded = build_and_fold(LuauOpcode::LOP_DIV, 7, 2);

  assert!(!folded.has_source_op, "7/2 须按 cpp 折叠摘除 DIV");
  assert!(folded.has_loadk, "3.5 的折叠产物须是 Number VmConst");
  assert_eq!(folded.loadn_imms, vec![2, 7], "仅存两条源 LOADN");
}

/// POW 同款边界：2^3=8 整值落 LOADN；2^0.5≈1.414 非整落 LOADK。
#[test]
fn int_imm_pow_follows_integrality_boundary() {
  let integral = build_and_fold(LuauOpcode::LOP_POW, 2, 3);
  assert!(!integral.has_loadk, "整值 POW 结果须落 LOADN 8");
  assert!(!integral.has_source_op, "2^3 射程内必须折叠");
  let mut expected = [2, 3, 8];
  expected.sort_unstable();
  assert_eq!(integral.loadn_imms, expected, "2^3 须折叠为 LOADN 8");

  let fractional = build_and_fold_operands(
    LuauOpcode::LOP_POW,
    CmpOperand::Int(2),
    CmpOperand::Num(0.5),
  );
  assert!(!fractional.has_source_op, "2^0.5 须按 cpp 折叠摘除 POW");
  assert!(fractional.has_loadk, "√2 的折叠产物须是 Number VmConst");
}

/// Int-imm 的 DIV 除零与 cpp 同型折 Number(+inf)（`1/0`，inf 格稳定照常
/// 折叠摘除）——旧版无条件放弃，oracle 会折。
#[test]
fn int_imm_div_by_zero_folds_to_inf() {
  let folded = build_and_fold(LuauOpcode::LOP_DIV, 1, 0);

  assert!(
    !folded.has_source_op,
    "+inf 常量格稳定，DIV 除零须照 cpp 折叠"
  );
  assert!(folded.has_loadk, "折叠产物须是 Number(+inf) VmConst");
}

/// Int-imm 的 IDIV 除零（`1//0`）：cpp `floor(1/0)`=+inf，格稳定照常折叠。
#[test]
fn int_imm_idiv_by_zero_folds_to_inf() {
  let folded = build_and_fold(LuauOpcode::LOP_IDIV, 1, 0);

  assert!(
    !folded.has_source_op,
    "floor(inf)=inf 格稳定，IDIV 除零须折叠"
  );
  assert!(folded.has_loadk, "折叠产物须是 Number(+inf) VmConst");
}

/// Int-imm 的 MOD 除零与 Number 路径同态：NaN 常量经 merge 退化为
/// NotAConstant，MOD 保留运行期判定，常量池残留 cpp 同款死 NaN 条目。
#[test]
fn int_imm_mod_by_zero_survives_lattice_degeneration() {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  bcb.emit_ad(LuauOpcode::LOP_LOADN, 0, 1);
  bcb.emit_ad(LuauOpcode::LOP_LOADN, 1, 0);
  bcb.emit_abc(LuauOpcode::LOP_MOD, 2, 0, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 2, 2, 0);
  bcb.end_function(3, 0, 0, 0);

  let graph = fold_bcb(&mut bcb);

  assert!(
    op_survived(&graph, LuauOpcode::LOP_MOD),
    "NaN 常量经 merge 退化为 NotAConstant，MOD 须保留运行期判定（cpp 同态）"
  );
  assert!(
    graph
      .constants
      .iter()
      .any(|c| matches!(c, BcVmConst::Number(v) if v.is_nan())),
    "evaluate 已执行：NaN 常量已按 findOrAddConst 语义追加（cpp 同款死条目）"
  );
}

/// Imm×VmConst 数值混合对（Int-imm 15 × Number 2.5）须按 cpp 折叠为
/// Number VmConst（17.5）：cpp `isNumber` 对 Imm-Int 与 VmConst-Number
/// 均为真，evaluateArith 任意组合统一升 double 折叠。旧版对混合臂无条件
/// NotAConstant，oracle 会做的折叠被静默丢弃。
#[test]
fn mixed_int_imm_and_number_vmconst_folds() {
  let folded = build_and_fold_operands(
    LuauOpcode::LOP_ADD,
    CmpOperand::Int(15),
    CmpOperand::Num(2.5),
  );

  assert!(!folded.has_source_op, "15+2.5 须按 cpp 折叠摘除 ADD");
  assert!(folded.has_loadk, "混合数值对的折叠产物须是 Number VmConst");
  assert_eq!(folded.loadn_imms, vec![15], "仅存源 Int-imm 的 LOADN");
}

/// 对照：混合对反向（Number × Int-imm）同样折叠——交换对称性。
#[test]
fn mixed_number_vmconst_and_int_imm_folds() {
  let folded = build_and_fold_operands(
    LuauOpcode::LOP_MUL,
    CmpOperand::Num(1.5),
    CmpOperand::Int(4),
  );

  assert!(!folded.has_source_op, "1.5*4 须按 cpp 折叠摘除 MUL");
  assert!(folded.has_loadk, "折叠产物须是 Number(6.0) VmConst");
}

/// 对照（折叠边界）：String VmConst × Int-imm 非数值对不得折叠——cpp
/// `isNumber` 对 String VmConst 为假，evaluate 返回 nullopt → NotAConstant。
#[test]
fn mixed_string_vmconst_and_int_imm_not_folded() {
  let folded = build_and_fold_operands(
    LuauOpcode::LOP_ADD,
    CmpOperand::Str(b"abc"),
    CmpOperand::Int(5),
  );

  assert!(
    folded.has_source_op,
    "String × Int-imm 无算术定义，ADD 必须保留（cpp 同型）"
  );
}

/// IDIV 双 Int-imm 折叠须满足 Lua 整除语义：向负无穷取整。`/` 向零截断，
/// 修正判据是"余数非零且操作数符号相异"（与 MOD 臂同款）——不得用
/// `quotient < 0`：截断商为 0 且余数非零、符号相异时（`-1 // 2`）该判据
/// 漏修，折叠结果错成 0（miscompile）。cpp oracle 的 `floor(a/b)`
/// （Sccp.cpp `evaluateNumberBinaryOp`）天然满足全部用例。
#[test]
fn int_imm_idiv_floors_toward_negative_infinity() {
  let cases: &[(i16, i16, i16)] = &[
    (-1, 2, -1), // 截断商 0：漏修重灾区，正确值为 -1 而非 0
    (1, -2, -1),
    (-2, 3, -1),
    (-7, 2, -4), // 截断商为负且余数非零
    (7, -2, -4),
    (-6, 2, -3), // 整除无余数：不修正
    (7, 2, 3),   // 同号：截断即向下
  ];
  for &(lhs, rhs, expected) in cases {
    let folded = build_and_fold(LuauOpcode::LOP_IDIV, lhs, rhs);
    assert!(
      !folded.has_loadk,
      "IDIV {lhs} // {rhs} 不得落 LOADK（Number），会丢 integer 语义"
    );
    assert!(!folded.has_source_op, "IDIV {lhs} // {rhs} 射程内必须折叠");
    let mut expected_imms = [i32::from(lhs), i32::from(rhs), i32::from(expected)];
    expected_imms.sort_unstable();
    assert_eq!(
      folded.loadn_imms, expected_imms,
      "IDIV {lhs} // {rhs} 须折叠为 LOADN {expected}（向负无穷取整）"
    );
  }
}

/// Number VmConst 的 MOD 除零**不折叠**（cpp 同态）：evaluate 无 `b == 0.0`
/// 守卫（Sccp.h `evaluateNumberBinaryOp` 的 `a - floor(a/b)*b` 对 b=0 落
/// NaN），但 `BcVmConst::operator==` 对 NaN 恒 false——findOrAddConst 每次
/// 求值追加新 NaN 常量，格在二次 visit 的 merge 处（常量不等）退化为
/// NotAConstant，MOD 保留运行期判定，仅常量池残留 cpp 同款的死 NaN 条目。
/// 旧实现的 `b == 0.0` 早退虽可观察面相同（同样不折叠），却是对 oracle 的
/// 无端文本偏差且掩盖了这条 NaN 退化机制，已删。
#[test]
fn number_vmconst_mod_by_zero_survives_lattice_degeneration() {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  let ka = bcb.add_constant_number(1.0);
  bcb.emit_ad(LuauOpcode::LOP_LOADK, 0, ka as i16);
  let kb = bcb.add_constant_number(0.0);
  bcb.emit_ad(LuauOpcode::LOP_LOADK, 1, kb as i16);
  bcb.emit_abc(LuauOpcode::LOP_MOD, 2, 0, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 2, 2, 0);
  bcb.end_function(3, 0, 0, 0);

  let graph = fold_bcb(&mut bcb);

  assert!(
    op_survived(&graph, LuauOpcode::LOP_MOD),
    "NaN 常量经 merge 退化为 NotAConstant，MOD 须保留运行期判定（cpp 同态）"
  );
  assert!(
    graph
      .constants
      .iter()
      .any(|c| matches!(c, BcVmConst::Number(v) if v.is_nan())),
    "evaluate 已执行：NaN 常量已按 findOrAddConst 语义追加（cpp 同款死条目）"
  );
}

/// 正对照：`1.0 / 0.0` 落 +inf，`inf == inf` 命中既有常量、格稳定，
/// DIV 须照常折叠为 `LOADK (+inf)`——证明上一条不是折叠整体失效。
#[test]
fn number_vmconst_div_by_zero_folds_to_inf() {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  let ka = bcb.add_constant_number(1.0);
  bcb.emit_ad(LuauOpcode::LOP_LOADK, 0, ka as i16);
  let kb = bcb.add_constant_number(0.0);
  bcb.emit_ad(LuauOpcode::LOP_LOADK, 1, kb as i16);
  bcb.emit_abc(LuauOpcode::LOP_DIV, 2, 0, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 2, 2, 0);
  bcb.end_function(3, 0, 0, 0);

  let graph = fold_bcb(&mut bcb);

  assert!(
    !op_survived(&graph, LuauOpcode::LOP_DIV),
    "+inf 常量格稳定，DIV 除零须照 cpp 折叠摘除"
  );
  assert!(
    graph
      .constants
      .iter()
      .any(|c| matches!(c, BcVmConst::Number(v) if v.is_infinite() && *v > 0.0)),
    "折叠产物须是 Number(+inf) 常量"
  );
}

/// MOD 快路径符号修正回归：Lua 取模结果符号随除数——负操作数组合与 IDIV
/// 的 floor 修正同判据（余数非零且符号相异），须逐例钉死。
#[test]
fn int_imm_mod_sign_follows_divisor() {
  let cases: &[(i16, i16, i16)] = &[
    (-7, 2, 1),  // 商负余负：+2 修正
    (7, -2, -1), // 商负余正：-2 修正
    (-6, 2, 0),  // 整除无余数：不修正
    (7, 2, 1),   // 同号：截断余数即结果
    (0, -5, 0),  // +0：MOD 的 0 恒为 +0.0（除数为负也不产生 -0.0）
  ];
  for &(lhs, rhs, expected) in cases {
    let folded = build_and_fold(LuauOpcode::LOP_MOD, lhs, rhs);
    assert!(
      !folded.has_loadk,
      "MOD {lhs}%{rhs} 不得落 LOADK（Number），会丢 integer 语义"
    );
    assert!(!folded.has_source_op, "MOD {lhs}%{rhs} 射程内必须折叠");
    let mut expected_imms = [i32::from(lhs), i32::from(rhs), i32::from(expected)];
    expected_imms.sort_unstable();
    assert_eq!(
      folded.loadn_imms, expected_imms,
      "MOD {lhs}%{rhs} 须折叠为 LOADN {expected}（符号随除数）"
    );
  }
}

/// 负零守卫：`0 / -2`（及 `0 * -2`、`0 // -2`）的真值是 -0.0，整数域不可
/// 表示——必须放弃 LOADN 快路径、落双精度通路折出 Number(-0.0)，否则
/// `tostring` "-0" 漂移成 "0"（可观察分歧）。
#[test]
fn negative_zero_results_fall_through_to_loadk() {
  for (op, name) in [
    (LuauOpcode::LOP_DIV, "DIV"),
    (LuauOpcode::LOP_MUL, "MUL"),
    (LuauOpcode::LOP_IDIV, "IDIV"),
  ] {
    let folded = build_and_fold(op, 0, -2);
    assert!(
      !folded.has_source_op,
      "{name} 0 op -2 须按 cpp 折叠摘除（落 Number(-0.0)）"
    );
    assert!(folded.has_loadk, "{name} -0.0 结果须落 LOADK，不得进 LOADN");
    assert_eq!(folded.loadn_imms, vec![-2, 0], "仅存两条源 LOADN");
  }
}

/// 负零折叠产物确实带负号（对照 `0 / 2` 须折 LOADN +0）。
#[test]
fn negative_zero_fold_product_is_actually_negative() {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  bcb.emit_ad(LuauOpcode::LOP_LOADN, 0, 0);
  bcb.emit_ad(LuauOpcode::LOP_LOADN, 1, -2);
  bcb.emit_abc(LuauOpcode::LOP_DIV, 2, 0, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 2, 2, 0);
  bcb.end_function(3, 0, 0, 0);

  let graph = fold_bcb(&mut bcb);

  assert!(
    graph
      .constants
      .iter()
      .any(|c| matches!(c, BcVmConst::Number(v) if *v == 0.0 && v.is_sign_negative())),
    "折叠产物须是 Number(-0.0)"
  );
  assert!(
    !graph
      .constants
      .iter()
      .any(|c| matches!(c, BcVmConst::Number(v) if *v == 0.0 && !v.is_sign_negative())),
    "不得产出 +0.0 常量（0/-2 的真值是 -0.0）"
  );
}

/// JUMPIFEQ 左操作数的构造来源。
enum CmpOperand {
  /// `LOADK r, "s"` → VmConst String
  Str(&'static [u8]),
  /// `LOADK r, n` → VmConst Number
  Num(f64),
  /// `LOADN r, v` → Int imm
  Int(i16),
  /// `LOADK r, <x,y,z,w>` → VmConst Vector
  Vec(f32, f32, f32, f32),
  /// `LOADK r, nil` → VmConst Nil
  Nil,
}

/// 按操作数种类发射对应的装载指令（字符串/数字走 LOADK，整数走 LOADN）。
fn emit_operand_load(bcb: &mut BytecodeBuilder, reg: u8, operand: &CmpOperand) {
  match *operand {
    CmpOperand::Str(s) => {
      let k = bcb.add_constant_string(StringRef::from_slice(s));
      bcb.emit_ad(LuauOpcode::LOP_LOADK, reg, k as i16);
    }
    CmpOperand::Num(n) => {
      let k = bcb.add_constant_number(n);
      bcb.emit_ad(LuauOpcode::LOP_LOADK, reg, k as i16);
    }
    CmpOperand::Int(v) => bcb.emit_ad(LuauOpcode::LOP_LOADN, reg, v),
    CmpOperand::Vec(x, y, z, w) => {
      let k = bcb.add_constant_vector(x, y, z, w);
      bcb.emit_ad(LuauOpcode::LOP_LOADK, reg, k as i16);
    }
    CmpOperand::Nil => {
      let k = bcb.add_constant_nil();
      bcb.emit_ad(LuauOpcode::LOP_LOADK, reg, k as i16);
    }
  }
}

/// `LOAD lhs→r0; LOAD rhs→r1; JUMPIFEQ r0, r1 L1; RETURN; L1: RETURN`，建图跑
/// SCCP 后返回跳转是否未被摘除（判据见 [`op_survived`]）。
fn jumpifeq_survives_fold(lhs: CmpOperand, rhs: CmpOperand) -> bool {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  emit_operand_load(&mut bcb, 0, &lhs);
  emit_operand_load(&mut bcb, 1, &rhs);
  bcb.emit_ad(LuauOpcode::LOP_JUMPIFEQ, 0, 1);
  bcb.emit_aux(1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 1, 1, 0);
  bcb.end_function(2, 0, 0, 0);

  op_survived(&fold_bcb(&mut bcb), LuauOpcode::LOP_JUMPIFEQ)
}

/// 异种类混合对 `"abc" == 5` 不得折叠：String VmConst × Int imm 无相等关系
/// （cpp `eq` 返回 nullopt → Unknown）。旧实现 `cmp_imm` 的 `_ => 0` 兜底
/// 把它当"相等"折叠成恒真，错误剪掉 fallthrough 路径。
#[test]
fn cross_kind_string_vs_int_imm_eq_not_folded() {
  assert!(
    jumpifeq_survives_fold(CmpOperand::Str(b"abc"), CmpOperand::Int(5)),
    "String × Int-imm 的 JUMPIFEQ 必须保留，不得按相等折叠"
  );
}

/// 对照 1（折叠机制在工作）：`5.0 == 5` 是可比较的 Number × Int-imm 混合对，
/// 应折叠为恒真并摘除 JUMPIFEQ——证明上一条不是"折叠整体失效"。
#[test]
fn number_vs_int_imm_eq_folds() {
  assert!(
    !jumpifeq_survives_fold(CmpOperand::Num(5.0), CmpOperand::Int(5)),
    "5.0 == 5 须折叠为恒真，JUMPIFEQ 应被摘除"
  );
}

/// 对照 2（折叠机制在工作）：双 Int-imm 等值比较照常折叠。
#[test]
fn int_imm_eq_folds() {
  assert!(
    !jumpifeq_survives_fold(CmpOperand::Int(5), CmpOperand::Int(5)),
    "5 == 5 须折叠为恒真，JUMPIFEQ 应被摘除"
  );
}

/// NaN 参与的 JUMPIFEQ 必须按**恒假**折叠（入口块唯一后继 = fallthrough 块1），
/// 不得折恒真跳目标（块2）：cpp `bcCompare` 对 NaN 的 eq/lt/le 均独立判 false
/// （`NaN == 5` 运行期为 false）。旧 `three_way` 的 PartialOrd 双假误返 0
/// （"相等"）→ JUMPIFEQ 折恒真，错剪 fallthrough（miscompile）。
#[test]
fn nan_eq_imm_folds_always_false_not_true() {
  // rhs 走 Int(5)→LOADN（词布局与 AUX 判据见 [`fold_nan_cmp`] 文档）：
  // AUX=1 比 r0×r1（NaN VmConst × Int imm，走 cmp_imm），AUX=0 会退化成
  // r0×r0 的 NaN×NaN（走 cmp_ops），LOADN r1 沦为死代码——非本用例要钉的路径。
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFEQ, CmpOperand::Int(5), true);
  assert!(
    !folded.jump_survived,
    "NaN 比较应被折叠（恒假摘除 JUMPIFEQ）"
  );
  assert_eq!(folded.succs.len(), 1, "折叠后只剩单后继");
  assert!(
    !folded.succ_has_filler[0],
    "唯一后继必须是 fallthrough 块（不含填充 LOADN），不得折恒真跳目标块"
  );
}

/// 折叠后可观察面：跳转指令是否仍活跃、入口块后继索引列表、
/// 各后继块是否含填充 LOADN（目标块独有标记）。
struct NanCmpFold {
  jump_survived: bool,
  succs: Vec<u32>,
  succ_has_filler: Vec<bool>,
}

/// 折叠后观察：`opcode` 是否存活 + 入口块后继及其是否含填充 LOADN。
fn observe_fold(graph: &BcFunction<'_>, opcode: LuauOpcode) -> NanCmpFold {
  let entry = &graph.blocks[0];
  let succs: Vec<u32> = entry.successors.iter().map(|e| e.target.index).collect();
  let succ_has_filler = succs
    .iter()
    .map(|&idx| {
      graph
        .block(BcOp::with(BcOpKind::Block, idx))
        .operator_deref()
        .ops
        .iter()
        .any(|&op| graph.inst(op).operator_deref().op == LuauOpcode::LOP_LOADN)
    })
    .collect();
  NanCmpFold {
    jump_survived: op_survived(graph, opcode),
    succs,
    succ_has_filler,
  }
}

/// `LOADK(NaN)` 与 `operand` 装载分置 r0/r1（`nan_on_lhs=false` 时交换寄存器，
/// 成 Imm×Vm 交换臂形态——73bad1b 所修 `-cmp` 翻转误折恒真的本形即此布局）；
/// 其后 `JUMPIF*(A=0,D=2) r0,r1; AUX(1); RETURN r0; LOADN r2(填充); RETURN r1`。
/// 布局与 `nan_eq_imm_folds_always_false_not_true` 相同：target = pc+d+1 =
/// word5（填充块），fallthrough = word4，两落点分属不同块——恒假折叠应只剩
/// fallthrough 后继，恒真折叠应只剩目标后继。
fn fold_nan_cmp(opcode: LuauOpcode, operand: CmpOperand, nan_on_lhs: bool) -> NanCmpFold {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  let k = bcb.add_constant_number(f64::NAN);
  if nan_on_lhs {
    bcb.emit_ad(LuauOpcode::LOP_LOADK, 0, k as i16);
    emit_operand_load(&mut bcb, 1, &operand);
  } else {
    emit_operand_load(&mut bcb, 0, &operand);
    bcb.emit_ad(LuauOpcode::LOP_LOADK, 1, k as i16);
  }
  bcb.emit_ad(opcode, 0, 2);
  bcb.emit_aux(1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
  bcb.emit_ad(LuauOpcode::LOP_LOADN, 2, 0);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 1, 1, 0);
  bcb.end_function(3, 0, 0, 0);
  observe_fold(&fold_bcb(&mut bcb), opcode)
}

/// 双 LOADK NaN（VmConst Number(NaN) × VmConst Number(NaN)，走 cmp_ops）的
/// JUMPIFEQ：`NaN == NaN` 运行期为 false → 恒假折叠走 fallthrough。rhs 取
/// 位样不同的 NaN：`add_constant_number` 按 `to_bits` 去重，两个 `f64::NAN`
/// 会合并为同一下标，无法形成两个独立 VmConst 节点。
#[test]
fn nan_nan_loadk_eq_folds_always_false() {
  let folded = fold_nan_cmp(
    LuauOpcode::LOP_JUMPIFEQ,
    CmpOperand::Num(f64::from_bits(0x7ff8_0000_0000_0001)),
    true,
  );
  assert!(
    !folded.jump_survived,
    "NaN==NaN 须折叠（恒假摘除 JUMPIFEQ），不得留运行期判定"
  );
  assert_eq!(folded.succs.len(), 1, "折叠后只剩单后继");
  assert!(
    !folded.succ_has_filler[0],
    "唯一后继必须是 fallthrough 块（不含填充 LOADN），不得折恒真跳目标块"
  );
}

/// NaN 参与的 JUMPIFLT（apply_cmp 的 `cmp < 0` 臂）：`NaN < 5` 运行期恒假 →
/// 恒假折叠走 fallthrough（NaN 哨兵 1 须满足 `1 < 0` 为假）。
#[test]
fn nan_lt_imm_folds_always_false() {
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFLT, CmpOperand::Int(5), true);
  assert!(
    !folded.jump_survived,
    "NaN<5 须折叠（恒假摘除 JUMPIFLT），不得留运行期判定"
  );
  assert_eq!(folded.succs.len(), 1, "折叠后只剩单后继");
  assert!(
    !folded.succ_has_filler[0],
    "唯一后继必须是 fallthrough 块，NaN<5 不得折恒真跳目标"
  );
}

/// NaN 参与的 JUMPIFLE（apply_cmp 的 `cmp <= 0` 臂）：`NaN <= 5` 运行期恒假 →
/// 恒假折叠走 fallthrough（NaN 哨兵 1 须满足 `1 <= 0` 为假——哨兵取 -1 或 0
/// 都会在此漏成恒真）。
#[test]
fn nan_le_imm_folds_always_false() {
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFLE, CmpOperand::Int(5), true);
  assert!(
    !folded.jump_survived,
    "NaN<=5 须折叠（恒假摘除 JUMPIFLE），不得留运行期判定"
  );
  assert_eq!(folded.succs.len(), 1, "折叠后只剩单后继");
  assert!(
    !folded.succ_has_filler[0],
    "唯一后继必须是 fallthrough 块，NaN<=5 不得折恒真跳目标"
  );
}

/// NaN 参与的 JUMPIFNOTEQ：`NaN ~= 5` 运行期恒**真** → 须折恒真跳目标块
/// （含填充 LOADN），钉住 NOT 变体的取反方向——基础 eq 条件恒假经
/// `conditional_targets(target_taken_on_true=false)` 取反后 target 边存活、
/// fallthrough 边死亡。若错把 NaN 当"相等"（旧 three_way 返 0）则会反向剪枝。
#[test]
fn nan_noteq_imm_folds_always_jump() {
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFNOTEQ, CmpOperand::Int(5), true);
  assert!(
    !folded.jump_survived,
    "NaN~=5 须折叠（恒真摘除 JUMPIFNOTEQ），不得留运行期判定"
  );
  assert_eq!(folded.succs.len(), 1, "折叠后只剩单后继");
  assert!(
    folded.succ_has_filler[0],
    "唯一后继必须是跳转目标块（含填充 LOADN）——NaN~=5 恒真，不得错剪 target 走 fallthrough"
  );
}

/// `5 < NaN`（Imm×Vm 交换臂）：运行期恒假 → 恒假折叠走 fallthrough。
/// 旧交换臂 `-cmp` 把哨兵 1 翻成 -1 会在此误折恒真跳目标（miscompile）。
#[test]
fn nan_rhs_lt_imm_lhs_folds_always_false() {
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFLT, CmpOperand::Int(5), false);
  assert!(!folded.jump_survived, "5<NaN 须折叠摘除 JUMPIFLT");
  assert_eq!(folded.succs.len(), 1, "折叠后只剩单后继");
  assert!(
    !folded.succ_has_filler[0],
    "5<NaN 须走 fallthrough，不得折恒真跳目标"
  );
}

/// `5 <= NaN`（交换臂 `cmp <= 0` 判定）：哨兵 1 须保持正值——被取反成 -1
/// 即误折恒真。
#[test]
fn nan_rhs_le_imm_lhs_folds_always_false() {
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFLE, CmpOperand::Int(5), false);
  assert!(!folded.jump_survived, "5<=NaN 须折叠摘除 JUMPIFLE");
  assert!(
    !folded.succ_has_filler[0],
    "5<=NaN 须走 fallthrough，不得折恒真跳目标"
  );
}

/// `NOTLT` 变体（交换臂取反方向）：基础 `5 < NaN` 为假 → NOT 后恒真必跳
/// target（填充块）。
#[test]
fn nan_rhs_notlt_imm_lhs_folds_always_true() {
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFNOTLT, CmpOperand::Int(5), false);
  assert!(!folded.jump_survived, "5 NOT< NaN 须折叠摘除 JUMPIFNOTLT");
  assert!(
    folded.succ_has_filler[0],
    "5 NOT< NaN 基础为假，取反后必跳 target（填充块）"
  );
}

/// `NOTLE` 变体：基础 `5 <= NaN` 为假 → 取反后恒真必跳 target。
#[test]
fn nan_rhs_notle_imm_lhs_folds_always_true() {
  let folded = fold_nan_cmp(LuauOpcode::LOP_JUMPIFNOTLE, CmpOperand::Int(5), false);
  assert!(!folded.jump_survived, "5 NOT<= NaN 须折叠摘除 JUMPIFNOTLE");
  assert!(
    folded.succ_has_filler[0],
    "5 NOT<= NaN 基础为假，取反后必跳 target（填充块）"
  );
}

/// 同种类 Vector×Vector 的 JUMPIFEQ 不得折叠：两个不同向量运行期必不相等，
/// cpp `eq` 对 Vector 对返回 nullopt → Unknown。旧 `cmp_ops` 的 `_ => 0`
/// 兜底把它们当"相等"折成恒真、错剪 fallthrough（miscompile）。
#[test]
fn same_kind_vector_eq_not_folded() {
  assert!(
    jumpifeq_survives_fold(
      CmpOperand::Vec(1.0, 2.0, 3.0, 4.0),
      CmpOperand::Vec(5.0, 6.0, 7.0, 8.0)
    ),
    "Vector × Vector 的 JUMPIFEQ 必须保留，不得按相等折叠"
  );
}

/// Nil×Nil 等值比较照常折叠恒真（cpp `eq` 对 Nil 对返回 true；
/// `cmp_ops` 的 Nil 臂返 Some(0)）——对照用例，证明上一条不是折叠整体失效。
#[test]
fn nil_nil_eq_folds() {
  assert!(
    !jumpifeq_survives_fold(CmpOperand::Nil, CmpOperand::Nil),
    "nil == nil 须折叠为恒真，JUMPIFEQ 应被摘除"
  );
}

/// `LOADN r0,5; JUMPIF r0`：cpp `evaluateCondition` 对 immConstant 统一走
/// `falsey`——整数 imm 非 falsey，条件恒真，折叠摘除 JUMPIF 并只剩 target
/// 后继。旧实现把 ImmConstant(Int) 落 Unknown 保留运行期分支（死分支未消除，
/// 与 oracle 输出不一致）。
#[test]
fn int_imm_condition_folds_always_true() {
  // word 布局：0=LOADN(5) 1=JUMPIF(d=1) 2=RETURN 3=LOADN(填充) 4=RETURN。
  // target = pc+d+1 = 3（填充块），fallthrough = 2（RET 块），两落点分属不同块。
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(0, false);
  bcb.emit_ad(LuauOpcode::LOP_LOADN, 0, 5);
  bcb.emit_ad(LuauOpcode::LOP_JUMPIF, 0, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
  bcb.emit_ad(LuauOpcode::LOP_LOADN, 2, 0);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 1, 1, 0);
  bcb.end_function(3, 0, 0, 0);

  let folded = observe_fold(&fold_bcb(&mut bcb), LuauOpcode::LOP_JUMPIF);
  assert!(!folded.jump_survived, "整数 imm 条件恒真，JUMPIF 应被摘除");
  assert_eq!(folded.succs.len(), 1, "折叠后只剩单后继");
  assert!(
    folded.succ_has_filler[0],
    "唯一后继必须是跳转目标块（含填充 LOADN）——整数 imm 恒真必跳 target"
  );
}
