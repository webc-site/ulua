use crate::{
  functions::get_type_utils::get_optional_ty,
  records::{
    extern_type::ExternType, function_type::FunctionType, replace_generics::ReplaceGenerics,
  },
  type_aliases::type_id::TypeId,
};

impl ReplaceGenerics {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    // 对齐 cpp Instantiation.cpp:109-129：FunctionType 读取改走 log 的 pending 态
    // （txn_log_get_mutable），与 is_dirty_type_id 的 log 感知保持一致；
    // ExternType 一侧 cpp 本就是 plain get，保持不变。
    let log = self.base.base.log;
    // Safety: log 为 Substitution/Tarjan 构造注入的非空 *const TxnLog（进程级 empty 单例
    // 或会话活动 log），比 self 长寿；(*log) 取 &self 只读，txn_log_get_mutable 依 RTTI 命中
    // FunctionType 才返回可变视图裸指针（可能为 null，下一行判空），ty 为存活 arena 句柄。
    let ftv = unsafe { (*log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    if !ftv.is_null() {
      // Safety: 上一行 !ftv.is_null() 已判空，ftv 指向本次 log 视图内类型正确的 FunctionType；
      // 仅重建只读借用，单线程内无并存可变别名。
      let ftv_ref = unsafe { &*ftv };
      if ftv_ref.has_no_free_or_generic_types {
        return true;
      }

      return (!self.generics.is_empty() || !self.generic_packs.is_empty())
        && (ftv_ref.generics == self.generics)
        && (ftv_ref.generic_packs == self.generic_packs);
    }

    // Safety: 调用 unsafe fn get_optional_ty；ty 为存活 arena TypeId 句柄，其内部经 log/arena
    // 读取 ExternType 并返回裸指针（可能为 null，下一行判空），本行不解引用返回值。
    let et = unsafe { get_optional_ty::<ExternType, TypeId>(Some(ty)) };
    !et.is_null()
  }
}
