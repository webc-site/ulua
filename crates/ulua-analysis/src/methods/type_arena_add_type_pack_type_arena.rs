use crate::{
  functions::as_mutable_type_pack::as_mutable_type_pack,
  records::{type_arena::TypeArena, type_pack::TypePack, type_pack_var::TypePackVar},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeArena {
  pub fn add_type_pack_t<T>(&mut self, tp: T) -> TypePackId
  where
    T: Into<TypePackVar>,
  {
    self.add_type_pack_type_pack_var(tp.into())
  }

  pub fn add_type_pack_initializer_list_type_id(&mut self, types: &[TypeId]) -> TypePackId {
    let tp = TypePack::from_vec(types.to_vec());
    let allocated = self.type_packs.allocate(TypePackVar::from(tp));
    // Safety: `allocated` 是 self.type_packs（TypedAllocator bump 块分配器）刚返回
    // 的槽位——非空、对齐、可写，且块地址一经分配永不移动；句柄尚未返回给任何
    // 调用方，此刻 `self` 的 &mut 独占就是该对象的唯一存活借用，写 owning_arena
    // 不与任何其他别名冲突。as_mutable_type_pack 是同一地址的恒等指针转换。
    unsafe {
      (*as_mutable_type_pack(allocated)).owning_arena = self.arena_id;
    }
    allocated
  }

  pub fn add_type_pack_vector_type_id_optional_type_pack_id(
    &mut self,
    types: Vec<TypeId>,
    tail: Option<TypePackId>,
  ) -> TypePackId {
    let tp = TypePack::new(types, tail);
    // 安全等价：转发到 safe 的 add_type_pack_type_pack_var（其内部完成
    // allocate + owning_arena 接线的同一证成路径），本方法不再需要 unsafe。
    self.add_type_pack_type_pack_var(TypePackVar::from(tp))
  }

  pub fn add_type_pack_type_pack(&mut self, tp: TypePackVar) -> TypePackId {
    // 安全等价：与 add_type_pack_type_pack_var 逐行同体（allocate + 写 owning_arena），
    // 收敛到同一 safe 实现，消除重复 unsafe 站点。
    self.add_type_pack_type_pack_var(tp)
  }

  pub fn add_type_pack_type_pack_var(&mut self, tp: TypePackVar) -> TypePackId {
    let allocated = self.type_packs.allocate(tp);
    // Safety: `allocated` 由 bump 块分配器刚返回——非空、对齐、可写，块地址永不
    // 移动；pack 句柄在写完 owning_arena 前未外泄，`self` 的 &mut 是该 arena 及其
    // 新槽位的唯一借用者，写回不与任何别名冲突。as_mutable_type_pack 恒等转换。
    unsafe {
      (*as_mutable_type_pack(allocated)).owning_arena = self.arena_id;
    }
    allocated
  }
}
