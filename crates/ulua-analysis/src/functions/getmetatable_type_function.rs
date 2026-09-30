use ulua_ast::records::location::Location;

use crate::{
  functions::{
    follow_type, get_type, getmetatable_helper::getmetatable_helper, is_pending::is_pending,
  },
  macros::check_arity,
  records::{
    intersection_type::IntersectionType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn getmetatable_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 1, "getmetatable");

  let location = if !ctx.constraint.is_null() {
    // Safety: constraint 是 *const 可空裸句柄，判空紧邻在先；非空时指向
    // solver 队列中 Box 持有、地址稳定的存活 Constraint，仅按值读 location。
    unsafe { (*ctx.constraint).location }
  } else {
    Location::default()
  };

  let target_ty = follow_type::follow(type_params[0]);

  // `is_pending` 为安全函数：solver 形参契约允许为空（内部判空折叠）。
  if is_pending(target_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![target_ty]);
  }

  if let Some(ut) = get_type::get::<UnionType>(target_ty) {
    let mut options: Vec<TypeId> = Vec::with_capacity(ut.options.len());
    for option in &ut.options {
      let result = getmetatable_helper(*option, &location, ctx);
      if result.result.is_none() {
        return result;
      }
      // 紧邻上方 is_none 分支已 return，result.result 至此必为 Some。
      options.push(
        *result
          .result
          .as_ref()
          .expect("紧邻 is_none 分支已早返，至此必为 Some"),
      );
    }

    // Safety: arena 是 ctx 构造期接线的 NonNull<TypeArena>（C++ NotNull），
    // 活过本 reduction；&mut 再借用窗口仅覆盖本次 add_type，单线程串行且
    // ut.options 的克隆值已入新节点，无并存别名；bump arena 地址稳定。
    return TypeFunctionReductionResult::reduction(unsafe {
      (*ctx.arena.as_ptr()).add_type(UnionType { options })
    });
  }

  if let Some(it) = get_type::get::<IntersectionType>(target_ty) {
    let mut parts: Vec<TypeId> = Vec::with_capacity(it.parts.len());
    let mut errored_with_unknown = false;

    for part in &it.parts {
      let result = getmetatable_helper(*part, &location, ctx);
      if result.result.is_none() {
        if get_type::get::<UnknownType>(follow_type::follow(*part)).is_some() {
          errored_with_unknown = true;
          continue;
        } else {
          return result;
        }
      }
      // 同上——is_none 早返路径排除后必为 Some。
      parts.push(
        *result
          .result
          .as_ref()
          .expect("紧邻 is_none 分支已早返，至此必为 Some"),
      );
    }

    if errored_with_unknown && parts.is_empty() {
      return TypeFunctionReductionResult::erroneous();
    }

    if parts.len() == 1 {
      return TypeFunctionReductionResult::reduction(parts[0]);
    }

    // Safety: 与 union 分支同一 arena NonNull 不变量——构造期接线非空，
    // add_type 的 &mut 借用止于返回，options→parts 均为已 follow 的存活句柄值。
    return TypeFunctionReductionResult::reduction(unsafe {
      (*ctx.arena.as_ptr()).add_type(IntersectionType { parts })
    });
  }

  getmetatable_helper(target_ty, &location, ctx)
}
