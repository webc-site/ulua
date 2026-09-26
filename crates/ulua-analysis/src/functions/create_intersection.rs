use crate::{
  macros::create_nary_variant,
  records::{
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_unknown_type::TypeFunctionUnknownType,
  },
};

create_nary_variant!(
  /// 对应 C++ 原生 `static int createIntersection(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:685`）。
  create_intersection,
  TypeFunctionIntersectionType,
  TypeFunctionUnknownType,
  Unknown,
  Intersection
);
