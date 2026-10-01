//! `Sccp::arithToK` 单侧常量改写回归：钉死算术 opcode → K 变体的编译期定表
//! (`const ARITH_TO_K`, 由旧 6 臂 `match` 收成)。cpp `Sccp.h arithToK` 对
//! 「一侧 NotAConstant、另一侧 Number VmConst」的算术指令改写为 K 形态。
//!
//! 现有 `sccp_int_imm_fold.rs` 只覆盖「双侧常量整体折叠」，不触达此改写臂，
//! 故单独锁定：6 个算术 opcode 命中各自 K 变体、非表内 opcode(AND)保持原样。
//!
//! 未知一侧用函数参数：`rebuildGraph` 把入口参数映射为裸 `BcOp::VmReg` 生产者，
//! `operand_lattice` 对 `VmReg` 恒返 `NotAConstant`（GETUPVAL 指令 def 经 propagate
//! 可能仍为 `Undetermined`，不满足改写守卫）。

use ulua_bytecode::{
  functions::{
    from_function_bytecode::from_function_bytecode, sccp_fold_constants::sccp_fold_constants,
  },
  records::{bc_function::BcFunction, bytecode_builder::BytecodeBuilder, sccp::BcVmConstImpl},
};
use ulua_common::enums::luau_opcode::LuauOpcode;

/// 构 `r0=参数(未知)`; `r1=LOADK 2.0`; `r2 = r0 <op> r1`; `RETURN`，跑 SCCP，
/// 返回改写后仍活跃于块序列的全部 opcode。
///
/// 常量取 2.0：`arith_to_k` 对 0.0/1.0 各有专门的代数折叠分支（加零、乘零/一、
/// 零/一次幂等），须避开才能命中末尾 `else` 的 K 形态改写臂。
fn fold_and_collect_alive(op: LuauOpcode) -> Vec<LuauOpcode> {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.begin_function(1, false);
  let k = bcb.add_constant_number(2.0);
  bcb.emit_ad(LuauOpcode::LOP_LOADK, 1, k as i16);
  bcb.emit_abc(op, 2, 0, 1);
  bcb.emit_abc(LuauOpcode::LOP_RETURN, 2, 2, 0);
  bcb.end_function(3, 0, 0, 0);

  let data = bcb.get_function_data(0);
  let strings = bcb.get_string_table();
  let mut graph = from_function_bytecode(&data, &strings).expect("自产函数块必须可解析");
  sccp_fold_constants(&mut graph, &BcVmConstImpl);
  alive_ops(&graph)
}

/// 块序列里仍活跃的 opcode 集合。
fn alive_ops(graph: &BcFunction<'_>) -> Vec<LuauOpcode> {
  graph
    .blocks
    .iter()
    .flat_map(|b| b.ops.iter())
    .map(|&i| graph.inst(i).operator_deref().op)
    .collect()
}

/// 6 个算术 opcode 的「单侧常量」改写：源 opcode 摘除、对应 K 变体存活。
#[test]
fn arith_one_side_const_rewrites_to_k_variant() {
  for (op, k_op) in [
    (LuauOpcode::LOP_ADD, LuauOpcode::LOP_ADDK),
    (LuauOpcode::LOP_SUB, LuauOpcode::LOP_SUBK),
    (LuauOpcode::LOP_MUL, LuauOpcode::LOP_MULK),
    (LuauOpcode::LOP_DIV, LuauOpcode::LOP_DIVK),
    (LuauOpcode::LOP_MOD, LuauOpcode::LOP_MODK),
    (LuauOpcode::LOP_POW, LuauOpcode::LOP_POWK),
  ] {
    let alive = fold_and_collect_alive(op);
    assert!(
      alive.contains(&k_op),
      "{op:?} 单侧常量须改写为 {k_op:?}（const ARITH_TO_K 定表命中）；实得 {alive:?}"
    );
    assert!(
      !alive.contains(&op),
      "{op:?} 改写后原 opcode 不得仍活跃；实得 {alive:?}"
    );
  }
}

/// 非表内 opcode（AND 无 K 映射臂）：一侧常量、一侧未知时不改写、不折叠，原样存活。
#[test]
fn non_arith_opcode_is_not_rewritten() {
  let alive = fold_and_collect_alive(LuauOpcode::LOP_AND);
  assert!(
    alive.contains(&LuauOpcode::LOP_AND),
    "AND 不在 ARITH_TO_K 表内, 须原样存活（等价旧 _ => None 臂）；实得 {alive:?}"
  );
  assert!(
    !alive.contains(&LuauOpcode::LOP_ANDK),
    "AND 不得被改写成 ANDK；实得 {alive:?}"
  );
}
