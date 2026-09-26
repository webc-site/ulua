use ulua_common::records::variant::Variant2;

use crate::{
  functions::{
    follow_type, get_type, is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
  },
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, extern_type::ExternType,
    function_type::FunctionType, metatable_type::MetatableType, never_type::NeverType,
    primitive_type::PrimitiveType, singleton_type::SingletonType, table_type::TableType,
    type_simplifier::TypeSimplifier, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl TypeSimplifier {
  pub fn basic_intersect_with_truthy(&self, target: TypeId) -> Option<TypeId> {
    // 契约：builtin_types 是构造期接线的 `Handle<BuiltinTypes>`（对应 C++
    // `const BuiltinTypes&` 成员），指向进程级长寿单例、恒非空且比
    // TypeSimplifier 长寿；借用只覆盖本函数体内对若干 Copy TypeId 字段的
    // 只读访问，无并存可变借用。
    let builtin_types = self.builtin_types.get();
    let target = follow_type::follow(target);

    if is_approximately_truthy_type(target) {
      return Some(target);
    }

    if is_approximately_falsy_type(target) {
      return Some(builtin_types.never_type);
    }

    if get_type::get::<UnknownType>(target).is_some() {
      return Some(builtin_types.truthy_type);
    }

    if get_type::get::<AnyType>(target).is_some() {
      // any = *error-type* | unknown, so truthy & any = *error-type* | truthy
      // 契约：self.arena 指向与 builtin_types.arena 同一 TypeArena 堆块（Box
      // 独占分配、地址稳定）；`get_mut` 对应 C++ 侧对同一 arena 的非常量使用
      // （const 成员句柄承载可写 arena 的既有模式）。此处物化的 &mut 只覆盖
      // arena 堆块、仅用于一次 add_type 追加 union 节点，同线程内无其它对该
      // arena 的并存借用；上面 builtin_types 的共享引用只读 BuiltinTypes 结构
      // 体字段，地址区间与 arena 堆块不相交，不构成别名冲突。
      let arena = self.arena.get_mut();
      return Some(arena.add_type(UnionType {
        options: alloc::vec![builtin_types.truthy_type, builtin_types.error_type],
      }));
    }

    if get_type::get::<NeverType>(target).is_some() || get_type::get::<ErrorType>(target).is_some()
    {
      return Some(target);
    }

    if get_type::get::<FunctionType>(target).is_some()
      || get_type::get::<TableType>(target).is_some()
      || get_type::get::<MetatableType>(target).is_some()
      || get_type::get::<ExternType>(target).is_some()
    {
      return Some(target);
    }

    if let Some(pt) = get_type::get::<PrimitiveType>(target) {
      return Some(match pt.r#type {
        PrimitiveType::NIL_TYPE => builtin_types.never_type,
        PrimitiveType::BOOLEAN => builtin_types.true_type,
        _ => target,
      });
    }

    if let Some(st) = get_type::get::<SingletonType>(target) {
      return Some(
        if st.variant == Variant2::V0(BooleanSingleton::new(false)) {
          builtin_types.never_type
        } else {
          target
        },
      );
    }

    None
  }
}
