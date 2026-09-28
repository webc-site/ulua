use crate::{
  macros::create_primordial, records::type_function_never_type::TypeFunctionNeverType,
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};

create_primordial!(
  /// 对应 C++ 原生 `static int createNever(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:471`）。
  create_never,
  TypeFunctionTypeVariant::Never(TypeFunctionNeverType::default())
);
