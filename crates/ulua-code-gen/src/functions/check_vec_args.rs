use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// vector 内建操作数预检（dot/min_max/cross/clamp/magnitude/normalize/map1 系共用）：
/// `nparams == want`、`nresults <= 1` 且全部操作数非常量时，按 `ops` 原序逐参发
/// `load_and_check_tag(Vector)` 守卫并返回 true；任一不满足则不发任何 IR，由调用方
/// 回退 NONE_FALLBACK。判定皆为纯读（参数数/结果数/操作数 kind），短路次序无副作用；
/// `vm_exit` 是纯值构造，多参共用一个 exit 与逐参新建逐位等价（dot/min_max 原状即共用）。
pub(crate) fn check_vec_args(
  build: &mut IrBuilder,
  nparams: i32,
  want: i32,
  nresults: i32,
  pcpos: i32,
  ops: &[IrOp],
) -> bool {
  if nparams != want || nresults > 1 || ops.iter().any(|op| op.kind() == IrOpKind::Constant) {
    return false;
  }
  let exit = build.vm_exit(pcpos as u32);
  let tag = LuaType::Vector as u8;
  for op in ops {
    build.load_and_check_tag(*op, tag, exit);
  }
  true
}
