use crate::{functions::is_metamethod_type_infer::is_metamethod, type_aliases::name_type::Name};

/// cpp `isMetamethod(Name&)`（ConstraintGenerator.cpp:2281）：与 TypeInfer 侧的
/// 同名静态函数名单逐字相同，故判定共用 `functions::metamethod_names::METAMETHODS`。
pub fn is_metamethod_mut(name: &Name) -> bool {
  is_metamethod(name)
}
