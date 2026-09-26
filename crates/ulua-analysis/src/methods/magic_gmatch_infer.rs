use ulua_ast::{
  records::ast_expr_constant_string::AstExprConstantString, rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    function_type::FunctionType, magic_function_call_context::MagicFunctionCallContext,
    magic_gmatch::MagicGmatch, type_pack::TypePack,
  },
  type_aliases::type_pack_variant::TypePackVariant,
};
impl MagicGmatch {
  pub fn infer(&self, context: &MagicFunctionCallContext) -> bool {
    let (params, _tail) = flatten_type_pack_id(context.arguments);

    if params.len() != 2 {
      return false;
    }

    // Safety: context.solver 是 MagicFunctionCallContext 构造期接线的
    // NonNull<ConstraintSolver>，非空且活过整个 magic 调用；此处只取共享只读借用。
    let solver = unsafe { context.solver.as_ref() };
    // Safety: context.call_site 同为构造期接线的 NonNull，指向本模块 parse arena
    // 持有的存活 AstExprCall 节点（AST 活过整个约束生成期），只读借用。
    let call_site = unsafe { context.call_site.as_ref() };

    let index = if call_site.self_ { 0 } else { 1 };
    let pattern = if call_site.args.size > index {
      // as_slice 为安全方法：args.data/size 由 parser arena 保证有效区间。
      let expr = call_site.args.as_slice()[index];
      // Safety: expr 是 parser 构造的非空实参节点（parser 子节点非空不变量）；
      // try_as_ptr 自带判空，class index 命中即 repr(C) 首字段基址重合的真实
      // AstExprConstantString 只读借用，未命中返回 None、从不解引用。
      unsafe { ast_node_try_as_ptr::<AstExprConstantString>(expr) }
    } else {
      None
    };

    let Some(pattern) = pattern else {
      return false;
    };

    let return_types =
      parse_pattern_string_bytes(solver.builtin_types.as_nonnull(), pattern.value.as_bytes());

    if return_types.is_empty() {
      return false;
    }

    // Safety: `&*solver.builtin_types` 只读借出构造期接线、此后只读不变的非空
    // BuiltinTypes 单例（借用挂在裸指针点物上，独立于 solver）；
    // `(*context.solver.as_ptr())` 把同一 NonNull 再借成 &mut ConstraintSolver，
    // 此刻 solver 的共享借用已止于上一条字段读取，unify 内部对 arena 的可变
    // 借用随其返回结束——单线程串行窗口，无并存别名；constraint 仅作身份指针透传。
    unsafe {
      let builtin_types = &solver.builtin_types.get_mut();
      (*context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[0],
        builtin_types.string_type,
      );
    }

    // Safety: unify 的 &mut 借用已随其返回结束；此处从构造期接线、非空且比
    // solver 长寿的 arena 裸指针字段一次性重建唯一 &mut TypeArena（借用窗口
    // 串行、无并存别名），后续 add_type/add_type_pack_t 返回 bump arena 存活
    // 节点句柄，地址稳定且活过 infer。
    let arena = unsafe { &mut (context.solver.as_ref().arena).get_mut() };

    let empty_pack = arena.add_type_pack_t(TypePack::empty());
    let return_list = arena.add_type_pack_t(TypePack::from_vec(return_types));
    let iterator_type = arena.add_type(FunctionType::function_type_new(
      empty_pack,
      return_list,
      None,
      false,
    ));
    let res_type_pack = arena.add_type_pack_t(TypePack::single(iterator_type));

    let result_mut = as_mutable_type_pack(context.result);
    // Safety: context.result 是本次 magic 调用的结果 TypePackId，指向类型 arena
    // 中存活的 TypePackVar 节点（bump arena 地址稳定）；此刻除该裸句柄外无其它
    // 活跃借用，单线程下原地写 `.ty` 等价 C++ `*asMutable(result) = ...`。
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(res_type_pack);
    }

    true
  }
}
