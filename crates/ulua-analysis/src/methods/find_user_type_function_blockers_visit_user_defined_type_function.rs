use crate::{
  functions::is_pending::is_pending,
  records::find_user_type_function_blockers::FindUserTypeFunctionBlockers,
  type_aliases::type_id::TypeId,
};

impl FindUserTypeFunctionBlockers {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_type_id(&mut self, ty: TypeId) -> bool {
    let solver = unsafe { self.ctx.as_ref().solver };
    if unsafe { is_pending(ty, solver) } && !self.blocking_type_map.contains(&ty) {
      self.blocking_type_map.insert(ty);
      self.blocking_types.push(ty);
    }
    true
  }
}
