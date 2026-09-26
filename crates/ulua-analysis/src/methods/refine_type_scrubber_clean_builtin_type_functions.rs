use crate::{
  functions::{follow_type, get_type},
  records::{
    intersection_type::IntersectionType, never_type::NeverType,
    refine_type_scrubber::RefineTypeScrubber, type_ids::TypeIds, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl RefineTypeScrubber {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    tp
  }

  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    // NOTE: this feels pretty similar to other places where we try to
    // filter over a set type, may be worth combining those in the future.
    // self.ctx 由 `RefineTypeScrubber::new` 以真实 `&mut` 借用接线为 `Handle`
    // （类型编码非空），指向类型函数归约会话存活的 TypeFunctionContext（solver/runtime
    // 持有，clean 遍历期不移动不释放），共享借用只读其字段。
    let ctx_ref = self.ctx.get();
    // Safety: ctx.builtins 是 TypeFunctionContext 构造期接线的 NonNull<BuiltinTypes>，
    // 指向检查会话不再写入的内置类型表，比 ctx 与本次 clean 长寿。
    let builtins = unsafe { ctx_ref.builtins.as_ref() };

    if let Some(ut) = get_type::get::<UnionType>(ty) {
      let mut new_options = TypeIds::new();
      for &option in &ut.options {
        let followed = follow_type::follow(option);
        if followed != self.needle && get_type::get::<NeverType>(followed).is_none() {
          new_options.insert_type_id(option);
        }
      }
      if new_options.empty() {
        builtins.never_type
      } else if new_options.size() == 1 {
        new_options.front()
      } else {
        // Safety: ctx.arena 是构造期接线的 NonNull<TypeArena>，指向会话存活 bump
        // arena；追加 UnionType 新节点只写自有块，块地址不移动，循环里借出的
        // ut.options 共享读取仍然有效；单线程串行下 add_type 的 &mut 重建无并存
        // 可变借用（本 pass 是唯一持有者）。
        unsafe {
          (*ctx_ref.arena.as_ptr()).add_type(UnionType {
            options: new_options.take(),
          })
        }
      }
    } else if let Some(it) = get_type::get::<IntersectionType>(ty) {
      let mut new_parts = TypeIds::new();
      for &part in &it.parts {
        let followed = follow_type::follow(part);
        if followed != self.needle && get_type::get::<UnknownType>(followed).is_none() {
          new_parts.insert_type_id(part);
        }
      }
      if new_parts.empty() {
        builtins.unknown_type
      } else if new_parts.size() == 1 {
        new_parts.front()
      } else {
        // Safety: 与 union 分支同一不变量——ctx.arena 构造期接线、指向会话存活
        // bump arena，IntersectionType 新节点追加不移动既有块，&mut 重建在本
        // 单线程串行 pass 中无并存可变借用。
        unsafe {
          (*ctx_ref.arena.as_ptr()).add_type(IntersectionType {
            parts: new_parts.take(),
          })
        }
      }
    } else if ty == self.needle {
      builtins.unknown_type
    } else {
      ty
    }
  }
}
