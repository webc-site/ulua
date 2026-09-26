use core::option::Option;

use crate::{
  macros::create_primordial, records::type_function_unknown_type::TypeFunctionUnknownType,
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};

create_primordial!(
  /// 对应 C++ 原生 `static int createUnknown(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:462`）。
  create_unknown,
  TypeFunctionTypeVariant::Unknown(TypeFunctionUnknownType {
    _unused: Option::None,
  })
);
