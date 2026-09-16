use ulua_common::records::variant::Variant2;

use crate::records::{singleton_type::SingletonType, type_arena::TypeArena};

impl TypeArena {
  pub fn record_singleton_stats(&mut self, singleton: &SingletonType) {
    match &singleton.variant {
      Variant2::V0(_bool_singleton) => {
        self.bool_singletons_minted += 1;
      }
      Variant2::V1(str_singleton) => {
        self.str_singletons_minted += 1;
        if !str_singleton.value.is_empty() {
          self
            .unique_str_singletons_minted
            .insert(Some(str_singleton.value.clone()));
        }
      }
    }
  }
}
