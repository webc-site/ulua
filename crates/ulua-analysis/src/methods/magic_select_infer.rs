use alloc::{string::ToString, vec::Vec};

use ulua_ast::{
  records::{
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
  },
  records::{
    generic_error::GenericError, magic_function_call_context::MagicFunctionCallContext,
    type_error::TypeError,
  },
  type_aliases::{
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_variant::TypePackVariant,
  },
};
pub fn magic_select_infer(context: &MagicFunctionCallContext) -> bool {
  // Safety: `context.solver` 是 `ConstraintSolver::run` 在派发本条约束时以
  // NotNull 语义写入 context 的求解器地址，指向当前调用栈内存活、生命周期覆盖
  // 整个 infer 调用的 `ConstraintSolver`，as_ref 只借用其只读字段。
  let solver = unsafe { context.solver.as_ref() };
  // Safety: `context.call_site` 同样由派发约束时注入，指向 parser 拥有、
  // 求解期间稳定存活的当前 `AstExprCall` 节点，本函数仅只读其 args/self 标志。
  let call_site = unsafe { context.call_site.as_ref() };

  if call_site.args.size == 0 {
    let error = TypeError::type_error_location_type_error_data(
      call_site.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "select should take 1 or more arguments".to_string(),
      )),
    );
    // Safety: `as_ptr` 取回构造期注入且仍在栈上独占存活的 solver 地址；
    // `report_error_type_error` 需 `&mut self`，此刻本调用同步持有 solver
    // 的独占访问，错误向量无并发别名。
    unsafe {
      (*context.solver.as_ptr()).report_error_type_error(error);
    }
    return false;
  }

  // args.size != 0 由上面的早退保证，[0] 恒在界内，故用安全切片读取元素指针。
  let arg1 = call_site.args.as_slice()[0];

  // Safety: `arg1` 是 arena 写入 AstExprCall.args 的存活表达式节点指针；
  // try_as_ptr 先判空再按 class_index 甄别，未命中返回 None 且从不解引用，命中即
  // repr(C) 基址重合的存活 AstExprConstantNumber 只读借用。
  if let Some(num) = unsafe { ast_node_try_as_ptr::<AstExprConstantNumber>(arg1) } {
    let (v, tail) = flatten_type_pack_id(context.arguments);

    // `num` 由 try_as_ptr 命中，指向存活 AstExprConstantNumber，读取 `value` 纯只读。
    let offset = num.value as i32;
    if offset > 0 {
      let offset_usize = offset as usize;
      if offset_usize < v.len() {
        let res: Vec<TypeId> = v[offset_usize..].to_vec();
        // Safety: `solver.arena` 由 solver 构造期指向其内嵌 `TypeArena`，
        // 本次 infer 内新增 TypePack 是唯一的 arena 可变借用路径。
        let arena = { &mut solver.arena.get_mut() };
        let res_type_pack = arena.add_type_pack_vector_type_id_optional_type_pack_id(res, tail);
        let result_mut = as_mutable_type_pack(context.result);
        // Safety: `context.result` 是 solver 为本约束分配、尚待定版的
        // arena TypePackVar；TypePackId 到 *mut 的转换对应 cpp `asMutable`
        // 的 const_cast，magic infer 在此独占写其 `ty`，此前无其他活动借用。
        unsafe {
          (*result_mut).ty = TypePackVariant::Bound(res_type_pack);
        }
      } else if let Some(tail) = tail {
        let result_mut = as_mutable_type_pack(context.result);
        // Safety: 同上——`context.result` 指向待定版的独占 arena TypePackVar，
        // 此处仅把其 `ty` 收敛为已有 tail 的 Bound 变体。
        unsafe {
          (*result_mut).ty = TypePackVariant::Bound(tail);
        }
      }

      return true;
    }

    return false;
  }

  // Safety: `arg1` 是存活表达式节点指针；try_as_ptr 判空+甄别，命中即存活
  // AstExprConstantString 只读借用（与 num 下转同源，只是命中类不同）。
  // size==1 保证 `as_slice()[0]` 落在 arena 成对写入的合法 c_char 区内。
  if let Some(str_expr) = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(arg1) }
    && str_expr.value.size == 1
    && str_expr.value.as_slice()[0] == b'#'
  {
    // Safety: `solver.arena` 是构造期注入且本次 infer 独占新增节点的目标
    // TypeArena（`add_type_pack_initializer_list_type_id` 需 `&mut`）。
    let arena = { &mut solver.arena.get_mut() };
    // Safety: `solver.builtin_types` 为 NotNull 语义的会话级内置类型表，
    // 比本次调用长寿且此处只读，借出 `number_type` 的 TypeId 值。
    let builtin_types = { &solver.builtin_types.get_mut() };
    let number_type_pack =
      arena.add_type_pack_initializer_list_type_id(&[builtin_types.number_type]);
    let result_mut = as_mutable_type_pack(context.result);
    // Safety: `context.result` 是待定版的 arena TypePackVar，const_cast 后
    // 由本次 infer 独占写 Bound；写点前后不再读取该位置的旧值。
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(number_type_pack);
    }
    return true;
  }

  false
}
