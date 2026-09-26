use alloc::vec::Vec;

use ulua_analysis::{
  enums::reduction::Reduction,
  functions::{follow_type, is_pending::is_pending, is_prim::is_number, is_string::is_string},
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

pub(crate) fn type_function_fixture_swap_reducer(
  _instance: TypeId,
  tys: &[TypeId],
  tps: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  LUAU_ASSERT!(tys.len() == 1);
  LUAU_ASSERT!(tps.is_empty());

  // 本 reducer 由类型检查器以 ReducerFunction 契约调用：分派点物化的独占借用
  // 仅帧内使用、不逃逸，此处降级为共享只读借用。
  let ctx_ref = &*ctx;
  let param = follow_type::follow(tys[0]);

  if is_string(param) {
    TypeFunctionReductionResult {
      result: Some(unsafe {
        // Safety: ctx_ref.builtins 为上下文档位 NonNull<BuiltinTypes>（fixture/检查器布线时指向存活 BuiltinTypes，非空），as_ptr() 读 number_type 拷出标量 TypeId，引用不逃逸帧。
        (*ctx_ref.builtins.as_ptr()).number_type
      }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  } else if is_number(param) {
    TypeFunctionReductionResult {
      result: Some(unsafe {
        // Safety: 同上：builtins NonNull 指向存活 BuiltinTypes，读 string_type 拷出标量即还，不物化长期借用。
        (*ctx_ref.builtins.as_ptr()).string_type
      }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  } else if unsafe {
    // Safety: is_pending 沿 cpp 检查器约定以 ctx_ref.solver（上下文档位布线的存活 ConstraintSolver 指针，本归约帧内有效）查询 pending 状态；只读、引用不逃逸帧，前置条件由调用帧契约满足。
    is_pending(param, ctx_ref.solver)
  } {
    TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: alloc::vec![param],
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  } else {
    TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
