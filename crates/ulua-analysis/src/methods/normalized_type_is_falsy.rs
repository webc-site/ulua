use crate::{
  enums::normalized_part::NormalizedPart,
  functions::{get_singleton_type::get_singleton_type, get_type},
  records::{
    boolean_singleton::BooleanSingleton, normalized_type::NormalizedType,
    singleton_type::SingletonType,
  },
};

impl NormalizedType {
  pub fn is_falsy(&self) -> bool {
    let mut has_a_false = false;
    if let Some(singleton_ptr) = get_type::get::<SingletonType>(self.booleans)
      && let Some(boolean_ptr) = get_singleton_type::<BooleanSingleton>(singleton_ptr)
    {
      has_a_false = !boolean_ptr.value;
    }

    // 允许名单含 booleans：bool 部件只允许存 `false` 单例（上面已判），原判定链
    // 因此不否定 `hasBooleans()`；nils 是正部件，integers 自带旗标门控。
    (has_a_false || self.has_nils())
      && !self.has_parts_other_than(&[NormalizedPart::Nils, NormalizedPart::Booleans])
  }
}
