use crate::{
  records::{function_type::FunctionType, instantiation::Instantiation},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Instantiation {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let log = self.base.base.log;
    // Safety: log 是 Tarjan 构造期（Substitution::substitution_new）收下的当前
    // 求解事务 TxnLog 地址（C++ `const TxnLog* log` 形参直译），比 Instantiation
    // 的使用期长寿；此处只重建 &TxnLog 共享借用做只读查询（pending 映射 +
    // arena 节点），*const 类型即承诺无经此指针的可变访问，单线程无别名冲突。
    let ftv = unsafe { (*log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    if !ftv.is_null() {
      // Safety: txn_log_get_mutable 的返回要么为 null，要么是 class-index 分派
      // 命中的 FunctionType 节点（type arena 或 log 内 pending 存储）的合法裸指针，
      // 两条路径的目标均比本次调用长寿；判空后只读 has_no_free_or_generic_types
      // 一个 bool 字段，无别名。
      if unsafe { (*ftv).has_no_free_or_generic_types } {
        return false;
      }
      return true;
    }
    false
  }

  pub fn is_dirty_type_pack_id(&self, _tp: TypePackId) -> bool {
    false
  }
}
