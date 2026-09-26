//! `Unifier` 上下文字段（`Handle` 句柄，见 `records/arena_handle.rs`）的
//! 访问器门面：`builtin_types` / `normalizer` / `shared_state` 由构造方以
//! `NonNull` 语义建立（C++ `NotNull<T>` / 引用同契约），unifier 存续期内恒有效。
//! 解引用安全性已收拢进 `Handle` 单点，本文件不再出现 `unsafe`。
use crate::records::{
  builtin_types::BuiltinTypes, normalizer::Normalizer, scope::Scope, type_arena::TypeArena,
  unifier::Unifier, unifier_shared_state::UnifierSharedState,
};
impl Unifier {
  /// C++ `NotNull<BuiltinTypes>`：内建类型表随前端上下文存活。
  pub fn builtin_types_ref(&self) -> &BuiltinTypes {
    self.builtin_types.get()
  }

  /// 归一化器：unifier 独占使用期间的可变访问（`try_normalize` 等带缓存写入）。
  pub fn normalizer_mut(&mut self) -> &mut Normalizer {
    self.normalizer.get_mut()
  }

  /// 类型 arena：unifier 独占期间的可变访问（新增 pack 等）。
  pub fn types_mut(&mut self) -> &mut TypeArena {
    self.types.get_mut()
  }

  /// C++ `NotNull<Scope*>`：当前 unifier 绑定的 scope，随求解上下文存活。
  pub fn scope_ref(&self) -> &Scope {
    self.scope.get()
  }

  /// C++ `UnifierSharedState&`：跨 unifier 共享的只读视图（缓存查找等）。
  pub fn shared_state_ref(&self) -> &UnifierSharedState {
    self.shared_state.get()
  }

  /// 共享状态的可变视图（计数器累加等）：与 C++ 相同，多个 unifier
  /// 顺序共享同一状态，单线程求解器内无并发别名。
  pub fn shared_state_mut(&mut self) -> &mut UnifierSharedState {
    self.shared_state.get_mut()
  }
}
