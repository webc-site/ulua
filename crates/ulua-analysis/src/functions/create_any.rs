use crate::{
  macros::create_primordial, records::type_function_any_type::TypeFunctionAnyType,
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};

create_primordial!(
  /// 对应 C++ 原生 `static int createAny(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:480`）。
  create_any,
  TypeFunctionTypeVariant::Any(TypeFunctionAnyType::default())
);
