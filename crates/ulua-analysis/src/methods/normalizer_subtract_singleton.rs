use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type,
  records::{
    boolean_singleton::BooleanSingleton, never_type::NeverType, normalized_type::NormalizedType,
    normalizer::Normalizer, primitive_type::PrimitiveType, singleton_type::SingletonType,
    string_singleton::StringSingleton,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn subtract_singleton(&mut self, here: &mut NormalizedType, ty: TypeId) {
    self.consume_fuel();

    // Safety: `self.builtin_types.as_ptr()` 是 Normalizer 构造期接线的 `NotNull<BuiltinTypes>` 裸指
    // 针（C++ `Normalizer(TypeArena*, BuiltinTypes* builtinTypes, ...)` 直接保存实参），非空、
    // 对齐且指向比本 Normalizer 长寿的共享单例；这里只建立只读共享借用，后续仅读取
    // `false_type`/`true_type`/`never_type` 三个 `Copy` 的 TypeId 常量字段，不改写该对象。
    // 本函数此后不再可变借用 self，故无别名冲突。
    let builtin_types = self.builtin_types.get();

    // 唯一调用方 `intersect_normal_with_ty` 先以 `get_type::get::<SingletonType>(follow(t)).is_some()`
    // 甄别分派才进入本函数（cpp 按类型 tag 分派后直接 get 的同前提），下转必命中。
    let stv =
      get_type::get::<SingletonType>(ty).expect("调用方已按 SingletonType 甄别后分派，下转必命中");

    if let Some(ss) = stv.variant.get_if::<StringSingleton>() {
      if here.strings.is_cofinite {
        here.strings.singletons.insert(ss.value.clone(), ty);
      } else {
        let it = here.strings.singletons.get_mut(&ss.value);
        if it.is_some() {
          here.strings.singletons.remove(&ss.value);
        }
      }
    } else if let Some(bs) = stv.variant.get_if::<BooleanSingleton>() {
      if get_type::get::<NeverType>(here.booleans).is_some() {
        // Nothing
      } else if let Some(prim) = get_type::get::<PrimitiveType>(here.booleans) {
        if prim.r#type == PrimitiveType::BOOLEAN {
          here.booleans = if bs.value {
            builtin_types.false_type
          } else {
            builtin_types.true_type
          };
        }
      } else if let Some(here_singleton) = get_type::get::<SingletonType>(here.booleans)
        .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      {
        // Crucial subtlety: ty (and thus bs) are the value that is being
        // negated out. We therefore reduce to never when the values match,
        // rather than when they differ.
        if bs.value == here_singleton.value {
          here.booleans = builtin_types.never_type;
        }
      } else {
        LUAU_ASSERT!(false);
      }
    } else {
      LUAU_ASSERT!(false);
    }
  }
}
