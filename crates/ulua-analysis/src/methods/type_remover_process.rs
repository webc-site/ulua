use alloc::vec::Vec;

use crate::{
  functions::{follow_type, get_mutable_type, get_type},
  records::{
    intersection_type::IntersectionType, never_type::NeverType, r#type::Type, type_ids::TypeIds,
    type_remover::TypeRemover, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl TypeRemover {
  /// C++ `void TypeRemover::process(TypeId item)` (Generalization.cpp:669-719).
  pub fn process(&mut self, item: TypeId) {
    let item = follow_type::follow(item);

    // If we've already visited this item, or it's outside our arena, then
    // do not try to mutate it.
    if self.seen.contains(&item)
      // Safety: item 经 follow_type_id 归一后恒为非空 TypeId 裸句柄，指向类型
      // arena 存活节点（bump 分配、地址稳定）；仅读 owning_arena 身份字段做归属
      // 值比较，不构造引用、不写节点。
      || unsafe { (*item).owning_arena } != self.arena.get().arena_id
      // Safety: 同上，item 非空存活；persistent 是按值读出的 bool 字段。
      || unsafe { (*item).persistent }
    {
      return;
    }
    self.seen.insert(item);

    if let Some(ut) = get_mutable_type::get_mutable::<UnionType>(item) {
      let options: Vec<TypeId> = ut.options.clone();
      let old_size = ut.options.len();
      let mut new_options = TypeIds::new();
      for option in options {
        self.process(option);
        let option = follow_type::follow(option);
        if option != self.needle && get_type::get::<NeverType>(option).is_none() && option != item {
          new_options.insert_type_id(option);
        }
      }
      if old_size != new_options.size() {
        if new_options.empty() {
          emplace_bound_type(item, self.builtin_never_type());
        } else if new_options.size() == 1 {
          let first = new_options.front();
          emplace_bound_type(item, first);
        } else {
          let taken = new_options.take();
          // Safety: self.arena 是构造期接线的非空 TypeArena 裸句柄（C++
          // NotNull<TypeArena>），活过整个 process；ut 的最后一次使用止于上方
          // options 快照，此处 &mut 再借用与递归 process 的 arena 借用窗口串行
          // （递归对 item 本身被 seen 挡下，不再改写本节点），add_type 只追加
          // bump arena 新节点，地址稳定。
          let new_ty = self.arena.get_mut().add_type(UnionType { options: taken });
          emplace_bound_type(item, new_ty);
        }
      }
      return;
    }

    if let Some(it) = get_mutable_type::get_mutable::<IntersectionType>(item) {
      let parts: Vec<TypeId> = it.parts.clone();
      let old_size = it.parts.len();
      let mut new_parts = TypeIds::new();
      for part in parts {
        self.process(part);
        let part = follow_type::follow(part);
        if part != self.needle && get_type::get::<UnknownType>(part).is_none() && part != item {
          new_parts.insert_type_id(part);
        }
      }
      if old_size != new_parts.size() {
        if new_parts.empty() {
          emplace_bound_type(item, self.builtin_unknown_type());
        } else if new_parts.size() == 1 {
          let first = new_parts.front();
          emplace_bound_type(item, first);
        } else {
          let taken = new_parts.take();
          // Safety: 与 union 分支对称——self.arena 构造期接线非空且活过 process；
          // it 的借用止于上方 parts 快照，递归对 item 由 seen 挡下，此处 &mut
          // 再借用窗口内无并存别名，add_type 仅追加地址稳定的 bump arena 节点。
          let new_ty = self
            .arena
            .get_mut()
            .add_type(IntersectionType { parts: taken });
          emplace_bound_type(item, new_ty);
        }
      }
    }
  }

  #[inline]
  fn builtin_never_type(&self) -> TypeId {
    // Safety: self.builtin_types 是构造期注入的非空 BuiltinTypes 会话单例裸句柄
    // （C++ NotNull<BuiltinTypes>），比本 TypeRemover 长寿；只读拷贝 never_type 句柄。
    self.builtin_types.get_mut().never_type
  }

  #[inline]
  fn builtin_unknown_type(&self) -> TypeId {
    // Safety: 同上——非空 BuiltinTypes 单例，只读拷贝 unknown_type 句柄。
    self.builtin_types.get_mut().unknown_type
  }
}

/// C++ `emplaceType<BoundType>(asMutable(item), bound_to)`: replace the type's
/// variant in place with a `BoundType` pointing at `bound_to`.
fn emplace_bound_type(item: TypeId, bound_to: TypeId) {
  // Safety: 本私有函数仅由 process 在 item 通过归属校验（owning_arena ==
  // self.arena.get().arena_id）、
  // 非 persistent、未重复访问三重过滤后调用：item 指向本 arena 可变且存活的
  // 类型节点（TypeId 即 C++ asMutable 惯用裸句柄，bump arena 地址稳定）。
  // const→mut 再借用处所有 get_mutable/递归借用窗口均已结束，单线程串行写入
  // 无并存别名；bound_to 为同 arena 的存活句柄，仅按值存入 variant。
  unsafe {
    let m = item as *mut Type;
    (*m).ty = TypeVariant::Bound(bound_to);
  }
}
