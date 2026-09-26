use crate::{
  records::{
    singleton_type::SingletonType, string_singleton::StringSingleton, r#type::Type,
    type_checker::TypeChecker,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId, type_variant::TypeVariant},
};

impl TypeChecker {
  pub fn singleton_type_bool(&mut self, value: bool) -> TypeId {
    // builtin_types 为 Handle（NonNull 编码非空）持有的会话级单例，get() 物化
    // 只读借用取其 Copy 的 true/false 常量 TypeId。
    if value {
      self.builtin_types.get().true_type
    } else {
      self.builtin_types.get().false_type
    }
  }

  pub fn singleton_type_string(&mut self, value: String) -> TypeId {
    let singleton = SingletonType::new(SingletonVariant::V1(StringSingleton::new(value)));
    let ty = Type::new(TypeVariant::Singleton(singleton));
    self.add_type(&ty)
  }
}
