use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

impl TypeChecker {
  pub fn error_recovery_type_pack_scope_ptr(&mut self, _scope: ScopePtr) -> TypePackId {
    // builtin_types 是 Handle（NonNull 编码非空，构造期从 Frontend 持有的全局
    // BuiltinTypes 接线、比 self 长寿）；get() 只读物化借用读 Copy 的
    // error_type_pack 常量字段，契约收拢于 arena_handle 模块。
    self.builtin_types.get().error_type_pack
  }

  pub fn error_recovery_type_pack_type_pack_id(&mut self, guess: TypePackId) -> TypePackId {
    // builtin_types 句柄目标会话期存活（见 arena_handle 契约）；
    // error_recovery_type_pack 取 &self（安全方法），get() 物化的共享借用只读、
    // 无在册可变别名；guess 由调用方传入的存活 TypePackId，本函数不解引用它。
    self.builtin_types.get().error_recovery_type_pack(guess)
  }
}
