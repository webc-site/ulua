use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn error_recovery_type_scope_ptr(&mut self, _scope: &ScopePtr) -> TypeId {
    // builtin_types 是 Handle（NonNull 编码非空，构造期由 C++ NotNull 形参接线的
    // 进程/模块级 BuiltinTypes，比 type checker 长寿）；get() 只读物化借用读
    // error_type 的 TypeId 值，无写入、无别名冲突。
    self.builtin_types.get().error_type
  }

  pub fn error_recovery_type_type_id(&mut self, guess: TypeId) -> TypeId {
    // builtin_types 句柄目标非空且长寿（见 arena_handle 契约）；
    // error_recovery_type 走 &self 只读路径，get() 物化的借用唯一，
    // guess 仅透传给该 &self 方法，本次调用内借用唯一。
    self.builtin_types.get().error_recovery_type(guess)
  }
}
