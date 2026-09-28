// 待接入锚注（R69 跨 crate 审计立案）：本出口当前仅测试消费（ulua-unit-test
// fixture / 本 crate tests），生产管线（CLI/rt）尚未接线——属 [[vm-pure-rust]]
// 分阶段策略的在途面，勿按孤儿收窄；接线时连同 cpp 对应 TEST_CASE 一并启用。
use crate::records::{bc_function::BcFunction, bc_op::BcOp, call_inliner::CallInliner};

/// cpp `inlineCall(caller, target, callOp, targetProtoId, callerFbVecSize)`
/// （`Bytecode/include/Luau/BytecodeCallInliner.h:831`）。
///
/// `caller_fb_vec_size` 是调用方在 CALLFB 之前已分配的 feedback 槽位数（cpp 侧来自
/// `callerProto->feedbackvecsize`），用于把被内联函数体的 CALLFB 槽位整体后移；无
/// feedback 时传 0。
pub fn inline_call<'c, 't: 'c>(
  caller: &mut BcFunction<'c>,
  target: &mut BcFunction<'t>,
  call_op: BcOp,
  target_proto_id: u32,
  caller_fb_vec_size: u32,
) -> bool {
  let mut inliner = CallInliner::new(caller, target, call_op, caller_fb_vec_size);
  inliner.inline_target(target_proto_id)
}
