use crate::{
  records::{
    singleton_type::SingletonType, string_singleton::StringSingleton, r#type::Type,
    type_checker::TypeChecker,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId, type_variant::TypeVariant},
};

impl TypeChecker {
  pub fn singleton_type_string(&mut self, value: String) -> TypeId {
    let singleton = SingletonType::new(SingletonVariant::variant_t_enable_if_t_get_type_id_t(
      StringSingleton::new(value),
    ));
    let ty = Type::new(TypeVariant::Singleton(singleton));
    self.add_type(&ty)
  }
}
