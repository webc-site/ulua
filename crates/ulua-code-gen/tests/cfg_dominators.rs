//! CFG 支配树（computeCfgImmediateDominators / findCommonDominator）纯 IR 级回归
//! （审计轮 code-gen T2，对齐 cpp `IrAnalysis.cpp:685,705`）。
//!
//! 手工构造 2-5 块的菱形/回边/不可达 CFG，驱动
//! `compute_cfg_block_edges + compute_cfg_immediate_dominators`，
//! 直接断言 idoms 数组（入口按 cpp 惯例收尾置回 INVALID）。

use ulua_code_gen::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  functions::{
    compute_cfg_block_edges::compute_cfg_block_edges,
    compute_cfg_immediate_dominators::compute_cfg_immediate_dominators,
    update_use_counts::update_use_counts,
  },
  records::{host_ir_hooks::HostIrHooks, ir_builder::IrBuilder},
};

const INVALID: u32 = !0u32;

fn env() -> IrBuilder {
  let hooks: &HostIrHooks = Box::leak(Box::new(HostIrHooks::default()));
  IrBuilder::ir_builder_ir_builder(hooks)
}

/// 重建 use_count → CFG 边 → 立即支配者，返回 idoms
fn compute_idoms(build: &mut IrBuilder) -> Vec<u32> {
  update_use_counts(&mut build.function);
  compute_cfg_block_edges(&mut build.function);
  compute_cfg_immediate_dominators(&mut build.function);
  build.function.cfg.idoms.clone()
}

fn cond(build: &mut IrBuilder, reg: u8) -> u32 {
  let r = build.vm_reg(reg);
  build.inst_ir_cmd_ir_op(IrCmd::LoadInt, r).index()
}

/// 菱形：idom(B1)=idom(B2)=idom(B3)=B0
#[test]
fn diamond_cfg_places_merge_under_entry() {
  let mut b = env();
  let b0 = b.block(IrBlockKind::Internal);
  let b1 = b.block(IrBlockKind::Internal);
  let b2 = b.block(IrBlockKind::Internal);
  let b3 = b.block(IrBlockKind::Internal);

  b.begin_block(b0);
  let c = cond(&mut b, 0);
  let cval = b.function.instructions[c as usize].ops[0];
  b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, cval, b1, b2);
  b.begin_block(b1);
  b.inst_ir_cmd_ir_op(IrCmd::JUMP, b3);
  b.begin_block(b2);
  b.inst_ir_cmd_ir_op(IrCmd::JUMP, b3);
  b.begin_block(b3);
  b.inst_ir_cmd(IrCmd::RETURN);

  let idoms = compute_idoms(&mut b);
  assert_eq!(
    idoms,
    vec![INVALID, 0, 0, 0],
    "汇合块的立即支配者应为入口块"
  );
}

/// 回边循环：B1 跳回 B0 不改变 idom(B1)=idom(B2)=B0
#[test]
fn loop_back_edge_does_not_corrupt_dominators() {
  let mut b = env();
  let b0 = b.block(IrBlockKind::Internal);
  let b1 = b.block(IrBlockKind::Internal);
  let b2 = b.block(IrBlockKind::Internal);

  b.begin_block(b0);
  let c = cond(&mut b, 0);
  let cval = b.function.instructions[c as usize].ops[0];
  b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, cval, b1, b2);
  b.begin_block(b1);
  b.inst_ir_cmd_ir_op(IrCmd::JUMP, b0); // 回边
  b.begin_block(b2);
  b.inst_ir_cmd(IrCmd::RETURN);

  let idoms = compute_idoms(&mut b);
  assert_eq!(idoms, vec![INVALID, 0, 0]);
}

/// 不可达块：idom 保持 INVALID（不动点迭代不触及）
#[test]
fn unreachable_block_keeps_invalid_idom() {
  let mut b = env();
  let b0 = b.block(IrBlockKind::Internal);
  let b1 = b.block(IrBlockKind::Internal);

  b.begin_block(b0);
  b.inst_ir_cmd(IrCmd::RETURN);
  b.begin_block(b1); // 无任何前驱边
  b.inst_ir_cmd(IrCmd::RETURN);

  let idoms = compute_idoms(&mut b);
  assert_eq!(idoms, vec![INVALID, INVALID]);
}

/// 链式支配：B1 支配 B3；B4 的前驱为 B3 与 B1 ⇒ idom(B4)=B1、idom(B3)=B1
#[test]
fn partially_dominating_predecessors_pick_closer_dominator() {
  let mut b = env();
  let b0 = b.block(IrBlockKind::Internal);
  let b1 = b.block(IrBlockKind::Internal);
  let b2 = b.block(IrBlockKind::Internal);
  let b3 = b.block(IrBlockKind::Internal);
  let b4 = b.block(IrBlockKind::Internal);

  b.begin_block(b0);
  let c0 = cond(&mut b, 0);
  let c0val = b.function.instructions[c0 as usize].ops[0];
  b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, c0val, b1, b2);
  b.begin_block(b1);
  let c1 = cond(&mut b, 1);
  let c1val = b.function.instructions[c1 as usize].ops[0];
  b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, c1val, b3, b4);
  b.begin_block(b2);
  b.inst_ir_cmd(IrCmd::RETURN);
  b.begin_block(b3);
  b.inst_ir_cmd_ir_op(IrCmd::JUMP, b4);
  b.begin_block(b4);
  b.inst_ir_cmd(IrCmd::RETURN);

  let idoms = compute_idoms(&mut b);
  assert_eq!(
    idoms,
    vec![INVALID, 0, 0, 1, 1],
    "find_common_dominator(B3,B1) 必须返回支配更近的 B1"
  );
}
