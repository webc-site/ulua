use alloc::{string::ToString, vec::Vec};

use ulua_ast::{
  records::{
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, node_handle::OptNode,
  },
  rtti::ast_node_try_as,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
  },
  records::{
    arena_handle::{alias, alias_ref},
    generic_error::GenericError,
    magic_function_call_context::MagicFunctionCallContext,
    type_error::TypeError,
  },
  type_aliases::{
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_variant::TypePackVariant,
  },
};
pub fn magic_select_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = alias_ref(context.solver.as_ptr());
  let call_site = alias_ref(context.call_site.as_ptr());

  if call_site.args.size == 0 {
    let error = TypeError::type_error_location_type_error_data(
      call_site.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "select should take 1 or more arguments".to_string(),
      )),
    );
    alias(context.solver.as_ptr()).report_error_type_error(error);
    return false;
  }

  // args.size != 0 由上面的早退保证，[0] 恒在界内，故用安全切片读取元素指针。
  let arg1 = call_site.args.as_slice()[0];

  // `args` 槽位仍是裸指针：经句柄门面 `OptNode::from_ptr` 借出基类引用，判型
  // 下转走生命周期正确的 [`ast_node_try_as`]（未命中折叠 None 不解引用），借用
  // 半径由本函数局部句柄供给，不锻造假 'static。
  let arg1_node = OptNode::from_ptr(arg1);
  if let Some(num) = arg1_node
    .get()
    .and_then(|a| ast_node_try_as::<AstExprConstantNumber>(a))
  {
    let (v, tail) = flatten_type_pack_id(context.arguments);

    // `num` 由 try_as 命中，指向存活 AstExprConstantNumber，读取 `value` 纯只读。
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
        alias(result_mut).ty = TypePackVariant::Bound(res_type_pack);
      } else if let Some(tail) = tail {
        let result_mut = as_mutable_type_pack(context.result);
        alias(result_mut).ty = TypePackVariant::Bound(tail);
      }

      return true;
    }

    return false;
  }

  // 同一 `arg1_node` 句柄再下转（与 num 分支同源，只是命中类不同）。
  // size==1 保证 `as_slice()[0]` 落在 arena 成对写入的合法 c_char 区内。
  if let Some(str_expr) = arg1_node
    .get()
    .and_then(|a| ast_node_try_as::<AstExprConstantString>(a))
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
    alias(result_mut).ty = TypePackVariant::Bound(number_type_pack);
    return true;
  }

  false
}
