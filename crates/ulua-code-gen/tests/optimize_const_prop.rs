//! OptimizeConstProp buffer store→load 转发矩阵回归（审计轮 code-gen T1）。
//!
//! 覆盖 cpp `OptimizeConstProp.cpp:1075`（forwardBufferStoreToLoad）与
//! `:1147`（substituteOrRecordBufferLoad）中决定 JIT 常量替换值的分支：
//! i8/u8/i16/u16/i32（dirty-high 源插 TruncateUint）/f32/f64/i64 ×
//! 常量值与 Inst 值 × 常量偏移与动态偏移的区间失效（含 swap-remove 与
//! NewUserdata 独立分配豁免）。
//!
//! 入口为纯 IR 级：手工构造单 Internal 块函数后直接驱动
//! `const_prop_in_block_chains`，断言折叠后的 IrCmd 与替换常量。

use ulua_code_gen::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    const_prop_in_block_chains::const_prop_in_block_chains, update_use_counts::update_use_counts,
  },
  records::{host_ir_hooks::HostIrHooks, ir_builder::IrBuilder, ir_op::IrOp},
};

/// buffer 读写共用 tag（cpp 侧为 LUA_TBUFFER；转发仅要求 store/load 两侧一致）
const TAG: u8 = 8;

/// 空 hook 集合；泄漏以保证 IrBuilder 内部裸指针在整个测试期间地址稳定
fn env() -> IrBuilder {
  let hooks: &HostIrHooks = Box::leak(Box::new(HostIrHooks::default()));
  let mut build = IrBuilder::ir_builder_ir_builder(hooks);
  let entry = build.block(IrBlockKind::Internal);
  build.begin_block(entry);
  build
}

/// 以 RETURN 收尾并驱动 const prop
fn optimize(build: &mut IrBuilder) {
  build.inst_ir_cmd(IrCmd::RETURN);
  // 手工构造的 IR 不经 IrBuilder 的 addUse 记账，先重建 use_count 再优化
  update_use_counts(&mut build.function);
  const_prop_in_block_chains(build);
}

fn read(build: &mut IrBuilder, cmd: IrCmd, addr: IrOp, off: IrOp) -> IrOp {
  let tag = build.const_tag(TAG);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, addr, off, tag)
}

fn write(build: &mut IrBuilder, cmd: IrCmd, addr: IrOp, off: IrOp, val: IrOp) {
  let tag = build.const_tag(TAG);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(cmd, addr, off, val, tag);
}

/// 断言 idx 处指令为 SUBSTITUTE 且携带常量 `v`（i32 视角）
fn assert_substituted_int(build: &IrBuilder, idx: u32, v: i32) {
  let inst = &build.function.instructions[idx as usize];
  assert_eq!(inst.cmd, IrCmd::SUBSTITUTE, "指令 {idx} 应被常量替换");
  assert_eq!(inst.ops[0].kind(), IrOpKind::Constant);
  assert_eq!(build.function.int_op(inst.ops[0]), v);
}

fn assert_cmd(build: &IrBuilder, idx: u32, cmd: IrCmd) {
  assert_eq!(build.function.instructions[idx as usize].cmd, cmd);
}

#[test]
fn writei8_forwards_to_readi8_with_int8_truncated_constant() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(4);
  // int8_t 截断：-3 保持 -3
  let val = b.const_int(-3);
  write(&mut b, IrCmd::BufferWritei8, addr, off, val);
  let rd = read(&mut b, IrCmd::BufferReadi8, addr, off);
  optimize(&mut b);
  assert_substituted_int(&b, rd.index(), -3);
}

#[test]
fn writei8_forwards_to_readu8_with_zero_extended_constant() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(4);
  // store 侧 int8_t(-3) = -3，load 侧再 as u8 → 253
  let val = b.const_int(-3);
  write(&mut b, IrCmd::BufferWritei8, addr, off, val);
  let rd = read(&mut b, IrCmd::BufferReadu8, addr, off);
  optimize(&mut b);
  assert_substituted_int(&b, rd.index(), 253);
}

#[test]
fn writei8_dynamic_value_inserts_sext_and_bitand_on_reads() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(4);
  let reg = b.vm_reg(0);
  let src = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, reg);
  let sext = b.inst_ir_cmd_ir_op(IrCmd::Sexti8Int, src);
  // forwardBufferIntStore(writei8) 将 Sexti8Int(x) 逆还原为 x；
  // load 侧再按读取类型补回 Sexti8Int / BitandUint(0xff)
  write(&mut b, IrCmd::BufferWritei8, addr, off, sext);
  let rd_i8 = read(&mut b, IrCmd::BufferReadi8, addr, off);
  let rd_u8 = read(&mut b, IrCmd::BufferReadu8, addr, off);
  optimize(&mut b);

  assert_cmd(&b, rd_i8.index(), IrCmd::Sexti8Int);
  let i8ops = &b.function.instructions[rd_i8.index() as usize].ops;
  assert_eq!(i8ops[0], src);

  assert_cmd(&b, rd_u8.index(), IrCmd::BitandUint);
  let u8ops = &b.function.instructions[rd_u8.index() as usize].ops;
  assert_eq!(u8ops[0], src);
  assert_eq!(
    b.function.int_op(u8ops[1]),
    0xff,
    "readu8 掩码常量必须为 0xff"
  );
}

#[test]
fn writei16_forwards_to_readu16_with_zero_extended_constant() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(0);
  // store 侧 int16_t(-2) = -2，load 侧 as u16 → 65534
  let val = b.const_int(-2);
  write(&mut b, IrCmd::BufferWritei16, addr, off, val);
  let rd = read(&mut b, IrCmd::BufferReadu16, addr, off);
  optimize(&mut b);
  assert_substituted_int(&b, rd.index(), 65534);
}

#[test]
fn writei32_from_dirty_source_inserts_truncate_uint() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(8);
  let reg = b.vm_reg(1);
  let src = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, reg);
  // NumToUint 在 produces_dirty_high_register_bits 名单内（高 32 位为垃圾）
  let dirty = b.inst_ir_cmd_ir_op(IrCmd::NumToUint, src);
  write(&mut b, IrCmd::BufferWritei32, addr, off, dirty);
  let rd = read(&mut b, IrCmd::BufferReadi32, addr, off);
  optimize(&mut b);

  assert_cmd(&b, rd.index(), IrCmd::TruncateUint);
  let ops = &b.function.instructions[rd.index() as usize].ops;
  assert_eq!(ops[0], dirty);
}

#[test]
fn writei32_from_clean_source_substitutes_directly() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(8);
  let reg = b.vm_reg(1);
  let src = b.inst_ir_cmd_ir_op(IrCmd::LoadInt, reg);
  write(&mut b, IrCmd::BufferWritei32, addr, off, src);
  let rd = read(&mut b, IrCmd::BufferReadi32, addr, off);
  optimize(&mut b);

  let inst = &b.function.instructions[rd.index() as usize];
  assert_eq!(inst.cmd, IrCmd::SUBSTITUTE);
  assert_eq!(inst.ops[0], src);
}

#[test]
fn writef64_and_writei64_forward_constants() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(0);

  let d = b.const_double(1.5);
  write(&mut b, IrCmd::BufferWritef64, addr, off, d);
  let rd = read(&mut b, IrCmd::BufferReadf64, addr, off);

  let v = b.const_int_64(-1_000_000_000_000);
  write(&mut b, IrCmd::BufferWritei64, addr, off, v);
  let ri = read(&mut b, IrCmd::BufferReadi64, addr, off);
  optimize(&mut b);

  // f64 读回读命中 f64 记录，i64 读回读命中 i64 记录（无截断分支）
  let inst = &b.function.instructions[rd.index() as usize];
  assert_eq!(inst.cmd, IrCmd::SUBSTITUTE);
  assert_eq!(b.function.double_op(inst.ops[0]), 1.5);
  let inst = &b.function.instructions[ri.index() as usize];
  assert_eq!(inst.cmd, IrCmd::SUBSTITUTE);
  assert_eq!(b.function.int64_op(inst.ops[0]), -1_000_000_000_000);
}

#[test]
fn writef32_truncates_constant_to_f32_before_forwarding() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(0);
  let d = b.const_double(0.1);
  write(&mut b, IrCmd::BufferWritef32, addr, off, d);
  let rd = read(&mut b, IrCmd::BufferReadf32, addr, off);
  optimize(&mut b);

  let inst = &b.function.instructions[rd.index() as usize];
  assert_eq!(inst.cmd, IrCmd::SUBSTITUTE);
  assert_eq!(
    b.function.double_op(inst.ops[0]),
    0.1f32 as f64,
    "store 侧必须先把 double 常量收窄为 f32"
  );
}

#[test]
fn dynamic_offset_write_invalidates_same_tag_records() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(4);
  let val = b.const_int(7);
  write(&mut b, IrCmd::BufferWritei8, addr, off, val);
  // 未知偏移写：同 tag 的记录全部清空
  let dyn_off = b.vm_reg(2);
  let tag = b.const_tag(TAG);
  b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritei8, addr, dyn_off, val, tag);
  let rd = read(&mut b, IrCmd::BufferReadi8, addr, off);
  optimize(&mut b);
  assert_cmd(&b, rd.index(), IrCmd::BufferReadi8);
}

#[test]
fn non_intersecting_offset_record_survives() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  // 写入 [4,8)
  let off4 = b.const_int(4);
  let val9 = b.const_int(9);
  write(&mut b, IrCmd::BufferWritei32, addr, off4, val9);
  // 写入 [8,12)：与已记录区间不相交，swap-remove 不得误删
  let off8 = b.const_int(8);
  let val11 = b.const_int(11);
  write(&mut b, IrCmd::BufferWritei32, addr, off8, val11);
  let rd = read(&mut b, IrCmd::BufferReadi32, addr, off4);
  optimize(&mut b);
  assert_substituted_int(&b, rd.index(), 9);
}

#[test]
fn intersecting_write_at_other_newuserdata_keeps_record() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr_a = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let addr_b = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(4);

  let rd1 = read(&mut b, IrCmd::BufferReadi8, addr_a, off);
  // A 记录（from_store=false）与 B 的写区间相交，但两侧指针均为
  // NewUserdata 且不同 → 独立分配豁免，A 记录保留并可 CSE 第二次读
  let val1 = b.const_int(1);
  write(&mut b, IrCmd::BufferWritei8, addr_b, off, val1);
  let rd2 = read(&mut b, IrCmd::BufferReadi8, addr_a, off);
  optimize(&mut b);

  let inst = &b.function.instructions[rd2.index() as usize];
  assert_eq!(inst.cmd, IrCmd::SUBSTITUTE, "A 上的首次 load 记录应幸存");
  assert_eq!(inst.ops[0], rd1);
}

#[test]
fn repeated_load_without_store_is_csed() {
  let mut b = env();
  let size = b.const_uint(64);
  let addr = b.inst_ir_cmd_ir_op(IrCmd::NewUserdata, size);
  let off = b.const_int(2);
  let rd1 = read(&mut b, IrCmd::BufferReadu8, addr, off);
  let rd2 = read(&mut b, IrCmd::BufferReadu8, addr, off);
  optimize(&mut b);

  // 无 store 时：首次 load 被记录，二次同址同偏移同 tag 的 load 命中 load-CSE
  assert_cmd(&b, rd1.index(), IrCmd::BufferReadu8);
  let inst = &b.function.instructions[rd2.index() as usize];
  assert_eq!(inst.cmd, IrCmd::SUBSTITUTE);
  assert_eq!(inst.ops[0], rd1);
}
