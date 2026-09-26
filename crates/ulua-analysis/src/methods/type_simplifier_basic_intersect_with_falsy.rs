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
  pub fn basic_intersect_with_falsy(&self, target: TypeId) -> Option<TypeId> {
    // 契约：`builtin_types` 是构造期由 `simplify_union`/`simplify_intersection` 从调用方
    // 传入的 `Handle<BuiltinTypes>`（C++ `NotNull<BuiltinTypes>` 形参），非空、
    // 对齐且指向比本 simplifier 长寿的会话单例。这里只建只读共享借用，后续全部是
    // `never/false/true/nil/error/falsy/any` 等 `Copy` TypeId 字段的读取，不改写该对象。
    let builtin_types = self.builtin_types.get();
    let target = follow_type::follow(target);

    if is_approximately_truthy_type(target) {
      return Some(builtin_types.never_type);
    }

    if is_approximately_falsy_type(target) {
      return Some(target);
    }

    if get_type::get::<NeverType>(target).is_some() || get_type::get::<ErrorType>(target).is_some()
    {
      return Some(target);
    }

    if get_type::get::<AnyType>(target).is_some() {
      // any = *error-type* | unknown, so falsy & any = *error-type* | falsy
      // 契约：`arena` 字段是构造期由调用方传入的 `Handle<TypeArena>`，其源头
      // 为 `NonNull::new(&mut arena).unwrap()`/`&mut arena as *mut _` 之类的独占借用。
      // 单次 simplify 调用链在单线程内串行执行，本函数此刻除 `builtin_types`（不同对象）
      // 外没有其它存活的 `&TypeArena`，故 `get_mut` 物化的可变借用不产生别名冲突；
      // `add_type` 走 bump 分配器，块地址不移动，返回的 UnionType 句柄在 arena 存活期内有效。
      let arena = self.arena.get_mut();
      return Some(arena.add_type(UnionType {
        options: alloc::vec![builtin_types.falsy_type, builtin_types.error_type],
      }));
    }

    if get_type::get::<UnknownType>(target).is_some() {
      return Some(builtin_types.falsy_type);
    }

    if get_type::get::<FunctionType>(target).is_some()
      || get_type::get::<TableType>(target).is_some()
      || get_type::get::<MetatableType>(target).is_some()
      || get_type::get::<ExternType>(target).is_some()
    {
      return Some(builtin_types.never_type);
    }

    if let Some(pt) = get_type::get::<PrimitiveType>(target) {
      return Some(match pt.r#type {
        PrimitiveType::NIL_TYPE => builtin_types.nil_type,
        PrimitiveType::BOOLEAN => builtin_types.false_type,
        _ => builtin_types.never_type,
      });
    }

    if let Some(st) = get_type::get::<SingletonType>(target) {
      return Some(
        if st.variant == Variant2::V0(BooleanSingleton::new(false)) {
          builtin_types.false_type
        } else {
          builtin_types.never_type
        },
      );
    }

    None
  }
}
