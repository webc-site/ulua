use crate::{
  functions::is_pending::is_pending,
  records::{
    extern_type::ExternType, find_user_type_function_blockers::FindUserTypeFunctionBlockers,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl FindUserTypeFunctionBlockers<'_> {
  /// 会话上下文以 `&'a mut` 独占借用持有（见 [`Self::ctx`] 的注），读取 `solver`
  /// 字段是普通的值拷贝，无需任何 unsafe。
  pub(crate) fn visit_type_id(&mut self, ty: TypeId) -> bool {
    let solver = self.ctx.solver;
    // Safety: 调用 unsafe fn is_pending；ty 为 arena 存活的 TypeId 句柄、solver 取自上方有效
    // context，is_pending 仅依该句柄读 pending 态，单线程内无并存可变借用。
    if unsafe { is_pending(ty, solver) } && !self.blocking_type_map.contains(&ty) {
      self.blocking_type_map.insert(ty);
      self.blocking_types.push(ty);
    }
    true
  }

  pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    true
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern: &ExternType) -> bool {
    false
  }
}
