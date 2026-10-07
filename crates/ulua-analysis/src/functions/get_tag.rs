/// 取类型函数运行期句柄的类型标签文本（cpp `getTag`，静态字面量、零分配）。
///
/// cpp 为逐个变体各写一次 `get<TypeFunctionPrimitiveType>(ty)`，本实现只做一次
/// 下转再按 variant 分派；标签全部是 `'static` 字面量，故返回 `&'static str`
/// 而非 `String`（调用点仅用于拼错误消息与 `lua_pushlstring`）。
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};
use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::type_type_function_runtime::Type,
  functions::{
    get_type_function_runtime::get_type_function_type_id, throw_type_error::throw_type_error,
  },
  records::{
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};
pub(crate) fn get_tag(l: &mut LuaState, ty: TypeFunctionTypeId) -> &'static str {
  if let Some(primitive) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty) {
    match primitive.r#type {
      Type::NilType => return "nil",
      Type::Boolean => return "boolean",
      Type::Number => return "number",
      // `integer` 仅在 LuauIntegerType2 打开时可命名；关闭时与 cpp 一样落到下面的兜底报错。
      Type::Integer if fflag::LuauIntegerType2.get() => return "integer",
      Type::String => return "string",
      Type::Thread => return "thread",
      Type::Buffer => return "buffer",
      // 唯一可到达的是「flag 关闭的 Integer」：不 return，继续走后面的判定链，
      // 与 cpp 逐条 else-if 全不命中后落入兜底报错一致。
      _ => {}
    }
  }

  // 非 primitive 类别：按 cpp 的判定顺序逐个 RTTI 下转，命中即返回。
  if get_type_function_type_id::<TypeFunctionUnknownType>(ty).is_some() {
    return "unknown";
  }
  if get_type_function_type_id::<TypeFunctionNeverType>(ty).is_some() {
    return "never";
  }
  if get_type_function_type_id::<TypeFunctionAnyType>(ty).is_some() {
    return "any";
  }
  if get_type_function_type_id::<TypeFunctionSingletonType>(ty).is_some() {
    return "singleton";
  }
  if get_type_function_type_id::<TypeFunctionNegationType>(ty).is_some() {
    return "negation";
  }
  if get_type_function_type_id::<TypeFunctionUnionType>(ty).is_some() {
    return "union";
  }
  if get_type_function_type_id::<TypeFunctionIntersectionType>(ty).is_some() {
    return "intersection";
  }
  if get_type_function_type_id::<TypeFunctionTableType>(ty).is_some() {
    return "table";
  }
  if get_type_function_type_id::<TypeFunctionFunctionType>(ty).is_some() {
    return "function";
  }
  if get_type_function_type_id::<TypeFunctionExternType>(ty).is_some() {
    return "extern";
  }
  if get_type_function_type_id::<TypeFunctionGenericType>(ty).is_some() {
    return "generic";
  }

  LUAU_ASSERT!(false);
  throw_type_error(
    l,
    format_args!("VM encountered unexpected type variant when determining tag"),
  );
}
