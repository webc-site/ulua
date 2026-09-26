use core::ptr::null_mut;

use crate::{
  records::{pending_type::PendingType, pending_type_pack::PendingTypePack, txn_log::TxnLog},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  pub fn pending_type_id(&self, ty: TypeId) -> *mut PendingType {
    // This function will technically work if `this` is nullptr, but this
    // indicates a bug, so we explicitly assert.
    // (In Rust, `&self` is never null, so the C++ `LUAU_ASSERT(this != nullptr)`
    // has no analog here.)

    let mut current: *const TxnLog = self;
    while !current.is_null() {
      // Safety: current 初值来自 &self（必非空、对齐且存活），后续值是 parent 裸
      // 指针——TxnLog 派生（clone/构造）时从外层 log 接线，指向比本 log 更长寿的
      // 外层 TxnLog 或 null（链根，由循环守卫终止）；本轮迭代只重建只读借用查表。
      let cur = unsafe { &*current };
      if let Some(it) = cur.type_var_changes.find(&ty)
        && !it.dead
      {
        return it.as_ref() as *const PendingType as *mut PendingType;
      }
      current = cur.parent;
    }

    null_mut()
  }

  pub fn pending_type_pack_id(&self, tp: TypePackId) -> *mut PendingTypePack {
    // This function will technically work if `this` is nullptr, but this
    // indicates a bug, so we explicitly assert.
    // (In Rust, `&self` is never null, so the C++ `LUAU_ASSERT(this != nullptr)`
    // has no analog here.)

    let mut current: *const TxnLog = self;
    while !current.is_null() {
      // Safety: 同 pending_type_id——parent 在 TxnLog 派生/构造期从外层 log 接线，
      // 非空项必指向存活的外层 TxnLog（其寿命覆盖本查询），null 由循环守卫终止；
      // 只重建共享借用读取 type_pack_changes，无写路径。
      let cur = unsafe { &*current };
      if let Some(it) = cur.type_pack_changes.find(&tp) {
        return it.as_ref() as *const PendingTypePack as *mut PendingTypePack;
      }
      current = cur.parent;
    }

    null_mut()
  }
}
