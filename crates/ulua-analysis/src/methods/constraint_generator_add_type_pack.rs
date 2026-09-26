use alloc::vec::Vec;

use crate::{
  records::{
    arena_id::ArenaId, constraint_generator::ConstraintGenerator, type_pack::TypePack,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
impl ConstraintGenerator {
  pub fn add_type_pack(&mut self, head: Vec<TypeId>, tail: Option<TypePackId>) -> TypePackId {
    if head.is_empty() {
      if let Some(tail) = tail {
        tail
      } else {
        // Safety: self.builtin_types.as_ptr() 为构造期接线的会话级 *mut BuiltinTypes（非空、比本
        // generator 长寿，指向内建单例），此处仅按值只读其 empty_type_pack 句柄字段。
        self.builtin_types.get().empty_type_pack
      }
    } else {
      let pack = TypePack::new(head, tail);
      let pack_var = TypePackVar {
        ty: TypePackVariant::TypePack(pack),
        persistent: false,
        owning_arena: ArenaId::NONE,
      };

      // Safety: self.arena.as_ptr() 是构造期注入的非空 *mut TypeArena 且比本次检查长寿；
      // add_type_pack_t 是 arena 独占追加（&mut self），pack_var 为按值移交的新节点，
      // 返回 arena 分配的存活 TypePackId。
      self.arena.get_mut().add_type_pack_t(pack_var)
    }
  }
}
