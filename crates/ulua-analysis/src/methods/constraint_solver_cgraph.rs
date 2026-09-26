//! `ConstraintSolver` 上下文裸指针字段的访问器收口：`cgraph` / `builtin_types` 等
//! 由构造方以 NonNull 语义建立、solver 存续期内恒有效（C++ `NotNull<T>` 同契约），
//! 解引用统一收进这里，调用点不再出现 `unsafe { (*self.x) }` 样板。
use crate::records::{
  builtin_types::BuiltinTypes, constraint_graph::ConstraintGraph,
  constraint_solver::ConstraintSolver, type_arena::TypeArena,
};
impl ConstraintSolver {
  /// `cgraph` 由构造方以 NonNull 语义建立（见 `check_frontend` 的 ncgraph 存储），
  /// solver 存续期内恒有效且非空，等价 C++ `NotNull<ConstraintGraph>`。
  pub fn cgraph_ref(&self) -> &ConstraintGraph {
    // SAFETY: 见函数注释；非空性由构造契约保证。
    unsafe { &*self.cgraph }
  }

  /// [`cgraph_ref`](Self::cgraph_ref) 的可变形态：solver 独占 graph，无并发别名。
  pub fn cgraph_mut(&mut self) -> &mut ConstraintGraph {
    // SAFETY: 见函数注释；独占性由 `&mut self` 保证。
    unsafe { &mut *self.cgraph }
  }

  /// C++ `NotNull<BuiltinTypes>`：内建类型表随前端上下文存活，solver 存续期内恒有效。
  pub fn builtin_types_ref(&self) -> &BuiltinTypes {
    self.builtin_types.get()
  }

  /// [`cgraph_ref`](Self::cgraph_ref) 同契约的 arena 可变访问；与其它字段的
  /// 混合借用场景仍需经裸指针拆分，请优先在单一字段访问处使用本方法。
  pub fn arena_mut(&mut self) -> &mut TypeArena {
    self.arena.get_mut()
  }
}
