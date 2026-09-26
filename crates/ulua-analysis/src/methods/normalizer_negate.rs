use crate::{
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type, get_type,
  },
  records::{
    any_type::AnyType, intersection_type::IntersectionType, negation_type::NegationType,
    never_type::NeverType, normalizer::Normalizer, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn negate(&mut self, mut there: TypeId) -> TypeId {
    self.consume_fuel();

    there = follow_type::follow(there);

    if get_type::get::<AnyType>(there).is_some() {
      there
    } else if get_type::get::<UnknownType>(there).is_some() {
      // Safety: self.builtin_types.as_ptr() 是构造期接线的非空 BuiltinTypes 会话单例
      // （C++ NotNull），只读拷贝 never_type 句柄；there 为存活归一句柄。
      self.builtin_types.get().never_type
    } else if get_type::get::<NeverType>(there).is_some() {
      // Safety: 同上——非空单例只读拷贝 unknown_type 句柄。
      self.builtin_types.get().unknown_type
    } else if let Some(ntv) = get_type::get::<NegationType>(there) {
      ntv.ty
    } else if let Some(utv) = get_type::get::<UnionType>(there) {
      let mut parts = Vec::new();
      // C++ `for (TypeId option : utv)`——UnionTypeIterator 展平嵌套 union
      // 并 follow,裸遍历 options 会漏掉嵌套成员。
      for option in begin_union_type(utv) {
        parts.push(self.negate(option));
      }
      // 契约：self.arena 构造/接线期已注入且活过本次 negate；utv 的共享借用经
      // begin_union_type 消费完后不再使用，递归内对 arena 的可变借用窗口均已
      // 随各自返回结束——wired_arena_mut 的 &mut 再借用为串行窗口，无并存别名；
      // TypedAllocator 只追加，节点地址稳定，已有句柄不失效。
      self.wired_arena_mut().add_type(IntersectionType { parts })
    } else if let Some(itv) = get_type::get::<IntersectionType>(there) {
      let mut options = Vec::new();
      // C++ `for (TypeId part : itv)`——IntersectionTypeIterator 同理。
      for part in begin_intersection_type(itv) {
        options.push(self.negate(part));
      }
      // Safety: 与 union 分支对称——arena 经 wired_arena_mut 断言已接线，itv
      // 借用与递归的可变窗口串行无并存，add_type 仅追加地址稳定节点。
      self.wired_arena_mut().add_type(UnionType { options })
    } else {
      there
    }
  }
}
