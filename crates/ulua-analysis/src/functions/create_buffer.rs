use crate::{
  enums::type_type_function_runtime::Type, macros::create_primordial,
  records::type_function_primitive_type::TypeFunctionPrimitiveType,
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};

create_primordial!(
  /// 对应 C++ 原生 `static int createBuffer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:532`）。
  create_buffer,
  TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType::new(Type::Buffer))
);
