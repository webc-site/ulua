use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_condition::IrCondition,
  },
  functions::{
    builtin_store_vector_result::builtin_store_vector_result, check_vec_args::check_vec_args,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};
pub fn translate_builtin_vector_clamp(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  fallback: IrOp,
  pcpos: i32,
) -> BuiltinImplResult {
  let arg1 = build.vm_reg(arg as u8);

  if !check_vec_args(build, nparams, 3, nresults, pcpos, &[arg1, args, arg3]) {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  // 三个 Internal 续块先于分量载入创建，块序与原 x/y/z 逐梯写法一致
  let blocks = [
    build.block(IrBlockKind::Internal),
    build.block(IrBlockKind::Internal),
    build.block(IrBlockKind::Internal),
  ];

  // x/y/z 三分量按偏移 0/4/8 逐梯：载入 (v,min,max) → min<=max 守卫 → 续块；
  // 每梯 const_int(off) 三次 LoadFloat 共用，与原 zero/four/eight 写法等价
  let mut comps = [(IrOp::default(), IrOp::default(), IrOp::default()); 3];
  for (i, (&blk, off)) in blocks.iter().zip([0, 4, 8]).enumerate() {
    let off_op = build.const_int(off);
    let v = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, off_op);
    let mn = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, args, off_op);
    let mx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg3, off_op);
    comps[i] = (v, mn, mx);

    let cond = build.cond(IrCondition::NotLessEqual);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::JumpCmpFloat,
      mn,
      mx,
      cond,
      fallback,
      blk,
    );

    build.begin_block(blk);
  }

  // clamp：max(min, v) 再与 max 取小；三分量 inst 次序与原六条展开一致
  let xyz = comps.map(|(v, mn, mx)| {
    let t = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxFloat, mn, v);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinFloat, mx, t)
  });

  builtin_store_vector_result(build, ra, xyz[0], xyz[1], xyz[2]);

  BuiltinImplResult::new(BuiltinImplType::UsesFallback, 1)
}
