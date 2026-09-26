use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, first::first, flatten_type_pack::flatten_type_pack_id, get_type,
  },
  records::{
    never_type::NeverType, type_checker::TypeChecker, type_pack::TypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
pub fn magic_assert_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  _expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;
  let predicates = with_predicate.predicates;
  let module = typechecker.current_module.as_ref()?;
  // Safety: `arc_as_mut(module)` 把 `typechecker.current_module` 这个 `Arc<Module>` 的
  // `Arc::as_ptr` 当作写入句柄（仓库既有的 arc_as_mut 惯用法）：被检查模块在检查期内由
  // checker 独占，本函数单线程串行执行，从取出 `&mut internal_types` 到末尾
  // `add_type_pack_t` 之间，链路上不存在指向同一 `Module` 的其它活引用。
  // `internal_types` 是 Module 的自有字段，地址随 Module 稳定；TypeArena 由 bump 块构成，
  // 块地址不移动，故其产出的 TypePackId 在 arena 存活期内一直有效。
  let arena = unsafe { &mut (*(arc_as_mut(module))).internal_types };

  let (mut head, tail) = flatten_type_pack_id(param_pack);
  if head.is_empty()
    && let Some(t) = tail
  {
    let fst = first(t, false);
    if let Some(fst) = fst {
      head.push(fst);
    } else {
      return Some(WithPredicate::with_predicate_t_predicate_vec(
        param_pack, predicates,
      ));
    }
  }

  typechecker.resolve_predicate_vec_scope_ptr_bool(&predicates, scope, true);

  if !head.is_empty() {
    let (ty, _ok) = typechecker.pick_types_from_sense(
      head[0],
      true,
      // 契约：`typechecker.builtin_types` 是 checker 构造期接线的
      // `Handle<BuiltinTypes>`（NotNull 语义），非空且指向会话级长寿单例；这里在
      // 实参位置一次性拷贝 `nil_type` 这个 `Copy` 句柄，不保留 `&BuiltinTypes`
      // 借用，因此与随后 `&mut typechecker` 的调用不冲突。
      typechecker.builtin_types.get().nil_type,
    );

    if let Some(ty) = ty {
      if get_type::get::<NeverType>(ty).is_some() {
        head = vec![ty];
      } else {
        head[0] = ty;
      }
    }
  }

  let new_tp_id = arena.add_type_pack_t(TypePack::new(head, tail));
  Some(WithPredicate::with_predicate_t_predicate_vec(
    new_tp_id, predicates,
  ))
}
