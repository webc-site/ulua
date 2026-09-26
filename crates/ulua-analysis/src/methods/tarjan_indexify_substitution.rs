use core::ptr::null_mut;

use crate::{
  records::{tarjan::Tarjan, tarjan_node::TarjanNode},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Tarjan {
  pub(crate) fn indexify_type_id(&mut self, ty: TypeId) -> (i32, bool) {
    // Safety: self.log 由 clear_tarjan/reset_state 在每轮 substitute 前接线，取值只
    // 可能是调用方存活的 &TxnLog 或 TxnLog::empty() 进程级单例指针，均非空且比本
    // 遍历长寿；follow_type_id 为 &self 只读解析，返回仍指向 arena 存活 Type。
    let ty = unsafe { (*self.log).follow_type_id(ty) };

    if let Some(&index) = self.type_to_index.find(&ty) {
      (index, false)
    } else {
      let index = self.nodes.len() as i32;
      self.type_to_index.try_insert(ty, index);
      self.nodes.push(TarjanNode {
        ty,
        tp: null_mut(),
        on_stack: false,
        dirty: false,
        lowlink: index,
      });
      (index, true)
    }
  }

  pub(crate) fn indexify_type_pack_id(&mut self, mut tp: TypePackId) -> (i32, bool) {
    // Safety: 同 TypeId 版——self.log 为上轮接线保留的非空存活 TxnLog 句柄，
    // follow_type_pack_id 只读沿日志链解析，返回仍指向 arena 存活 TypePack。
    tp = unsafe { (*self.log).follow_type_pack_id(tp) };

    if let Some(&index) = self.pack_to_index.find(&tp) {
      (index, false)
    } else {
      let index = self.nodes.len() as i32;

      self.pack_to_index.try_insert(tp, index);

      self.nodes.push(TarjanNode {
        ty: null_mut(),
        tp,
        on_stack: false,
        dirty: false,
        lowlink: index,
      });

      (index, true)
    }
  }
}
