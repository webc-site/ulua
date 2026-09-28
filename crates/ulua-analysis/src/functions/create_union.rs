use crate::{
  macros::create_nary_variant,
  records::{
    type_function_never_type::TypeFunctionNeverType,
    type_function_union_type::TypeFunctionUnionType,
  },
};

create_nary_variant!(
  /// 对应 C++ 原生 `static int createUnion(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:653`）。
  create_union,
  TypeFunctionUnionType,
  TypeFunctionNeverType,
  Never,
  Union
);
