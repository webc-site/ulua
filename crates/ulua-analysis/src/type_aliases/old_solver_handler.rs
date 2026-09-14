use alloc::sync::Arc;

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  records::{scope::Scope, type_checker::TypeChecker, with_predicate::WithPredicate},
  type_aliases::type_pack_id::TypePackId,
};
/// 旧求解器回调函数指针（MagicFunction vtable 槽位，对应 C++ handleOldSolver）。
pub type OldSolverHandler = fn(
  &mut TypeChecker,
  &Arc<Scope>,
  &AstExprCall,
  WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>>;
