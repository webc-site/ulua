use crate::{
  records::substitution::Substitution,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Substitution {
  pub(crate) fn replace_type_id(&mut self, ty: TypeId) -> TypeId {
    // Safety: self.base.log 是 Substitution 构造注入的非空 TxnLog 指针（进程级 empty 单例
    // 或会话活动 log，均比本 substitution 长寿）；follow_type_id 取 &self 只读遍历 parent 链，
    // 返回遍历期内存活的 arena TypeId，重建共享解引用不产生 &/&mut 冲突。
    let ty = unsafe { (*self.base.log).follow_type_id(ty) };
    match self.new_types.find(&ty) {
      Some(prev_ty) => *prev_ty,
      None => ty,
    }
  }

  pub(crate) fn replace_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    // Safety: 同上——self.base.log 为构造注入的非空 TxnLog（empty 单例或会话活动 log，
    // 比本对象长寿）；follow_type_pack_id 取 &self 只读遍历 parent 链，返回存活 arena 类型包句柄。
    let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    if let Some(prev_tp) = self.new_packs.find(&tp) {
      *prev_tp
    } else {
      tp
    }
  }
}
