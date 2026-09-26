//! Faithful Rust port of the test-only `struct MagicInstanceIsA : MagicFunction`
//! from `tests/TypeInfer.refinements.test.cpp` (lines 21-74).
//!
//! In C++ this is a `MagicFunction` subclass overriding `handleOldSolver`,
//! `infer`, and `refine`. In this port a `MagicFunction` *is* its vtable (a set
//! of function pointers), so the "subclass" is simply a `MagicFunction` built
//! from the three handler functions below. `make_magic_instance_is_a` returns
//! the shared instance that the refinement fixture attaches to the `IsA`
//! function type.

use alloc::{string::String, sync::Arc};

/// C++ `MagicInstanceIsA::handleOldSolver` (old type checker path).
use ulua_analysis::{
  functions::{as_mutable_type::as_mutable_type_id, try_get_l_value::try_get_l_value},
  records::{
    is_a_predicate::IsAPredicate, magic_function::MagicFunction,
    magic_function_call_context::MagicFunctionCallContext,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    magic_refinement_context::MagicRefinementContext, scope::Scope, type_checker::TypeChecker,
    with_predicate::WithPredicate,
  },
  type_aliases::{predicate::Predicate, type_pack_id::TypePackId, type_variant::TypeVariant},
};
use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
  ast_expr_index_name::AstExprIndexName,
};

use crate::functions::{
  ast_node_ref::{NodePtr, PtrRef},
  raw_handle::raw_handle,
};
fn magic_instance_is_a_handle_old_solver(
  type_checker: &mut TypeChecker,
  scope: &Arc<Scope>,
  expr: &AstExprCall,
  _with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  if expr.args.size != 1 {
    return None;
  }

  let args = expr.args.as_slice();
  // 门面一步「上转+判型+判空」：安全 `NodePtr::as_node`（生命周期由 slot 借用给出），
  // 未命中与 null 折叠为同一 None 早退，调用点零 unsafe。
  let index = expr.func.as_node::<AstExprIndexName>();
  let str_node = args
    .first()
    .and_then(|arg| arg.as_node::<AstExprConstantString>());
  let (Some(index), Some(str_node)) = (index, str_node) else {
    return None;
  };

  // 指针槽读法收口在夹具门面 PtrRef::as_ref_opt（null → None），调用点零 unsafe。
  let lvalue = index.expr.as_ref_opt().and_then(try_get_l_value)?;
  let name = String::from(str_node.value.as_str().unwrap_or(""));
  let tfun = scope.lookup_type(&name)?;

  let module_arc = type_checker.current_module.clone()?;
  let module_ptr = raw_handle(&module_arc);
  let boolean_pack = unsafe {
    // Safety: module_ptr 为 raw_handle(&module_arc)——resolver ModulePtr 强引用克隆的本帧存活块地址（非空）；对应 cpp module->internalTypes.addTypePack(...)，写入落在模块 arena，单线程顺序、调用即还，无第二 &mut。
    (*module_ptr)
      .internal_types
      .add_type_pack_initializer_list_type_id(&[type_checker.boolean_type])
  };

  Some(WithPredicate::with_predicate_t_predicate_vec(
    boolean_pack,
    alloc::vec![Predicate::IsA(IsAPredicate {
      lvalue,
      location: expr.base.base.location,
      ty: tfun.r#type(),
    })],
  ))
}

/// C++ `MagicInstanceIsA::infer` — returns false (this magic does not infer).
fn magic_instance_is_a_infer(_context: &MagicFunctionCallContext) -> bool {
  false
}

/// C++ `MagicInstanceIsA::refine` (new constraint-solver path).
fn magic_instance_is_a_refine(ctx: &MagicRefinementContext) {
  let call_site = ctx
    .call_site
    .as_ref_opt()
    .expect("MagicRefinementContext 约定 call_site 非空");

  if call_site.args.size != 1 || ctx.discriminant_types.is_empty() {
    return;
  }

  // 安全门面 as_node：slot 借用（&call_site）给出返回引用寿命，判型未命中折叠为 None 早退。
  let args = call_site.args.as_slice();
  let index = call_site.func.as_node::<AstExprIndexName>();
  let str_node = args
    .first()
    .and_then(|arg| arg.as_node::<AstExprConstantString>());
  let (Some(_index), Some(str_node)) = (index, str_node) else {
    return;
  };

  let discriminant_ty = match ctx.discriminant_types[0] {
    Some(ty) => ty,
    None => return,
  };

  let name = String::from(str_node.value.as_str().unwrap_or(""));
  let scope = ctx
    .scope
    .as_ref_opt()
    .expect("MagicRefinementContext 约定 scope 非空");
  let tfun = match scope.lookup_type(&name) {
    Some(tfun) => tfun,
    None => return,
  };

  // C++: LUAU_ASSERT(get<BlockedType>(*discriminant_ty));
  //      asMutable(*discriminant_ty)->ty.emplace<BoundType>(tfun->type);
  let mutable_ty = as_mutable_type_id(discriminant_ty);
  // Safety: mutable_ty = as_mutable_type_id(discriminant_ty) 为求解器 arena 中该 BlockedType 的非空可写地址（cpp asMutable(*discriminantTy)->ty.emplace<BoundType> 同款写入）；此刻该类型被本回调独占求解，写入 Bound(tfun.type) 满足 cpp 类型不变式，单线程无别名。
  unsafe {
    (*mutable_ty).ty = TypeVariant::Bound(tfun.r#type());
  }
}

fn magic_instance_is_a_type_check(_context: &MagicFunctionTypeCheckContext) -> bool {
  false
}

/// Build the shared `MagicInstanceIsA` instance (C++ `std::make_shared<MagicInstanceIsA>()`).
pub fn make_magic_instance_is_a() -> Arc<MagicFunction> {
  Arc::new(MagicFunction::from_handlers(
    magic_instance_is_a_handle_old_solver,
    magic_instance_is_a_infer,
    magic_instance_is_a_refine,
    magic_instance_is_a_type_check,
  ))
}
