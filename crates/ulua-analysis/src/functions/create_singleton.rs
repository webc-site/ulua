

/// 类型函数 `types.singleton`：按索引 1 实参的运行时种类压入对应 singleton userdata。
/// `l` 的存活与本次调用独占前提已由 `&mut LuaState` 接收者类型承载；本函数经
/// `create_singleton_thunk` 在 C-ABI 边界由 VM 调起，函数体只在读栈与写栈之间切换。
use ulua_common::{fflag, functions::c_str::cstr_cow};
use ulua_vm::functions::{lua_l_typename::lua_l_typename, lua_typename::lua_typename};
use crate::{enums::type_type_function_runtime::Type, functions::{alloc_type_user_data::alloc_type_user_data, throw_type_error::throw_type_error}, records::{type_function_boolean_singleton::TypeFunctionBooleanSingleton, type_function_primitive_type::TypeFunctionPrimitiveType, type_function_singleton_type::TypeFunctionSingletonType, type_function_string_singleton::TypeFunctionStringSingleton}, type_aliases::{type_function_singleton_variant::TypeFunctionSingletonVariant, type_function_type_variant::TypeFunctionTypeVariant}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn create_singleton(l: &mut LuaState) -> i32 {

  if l.is_boolean(1) {
    let value = l.check_boolean(1);
    // Safety: `l` 为存活 lua_State 且索引 1 实参在位（刚判定为布尔）；
    // `alloc_type_user_data` 在此状态上检查栈空间并压入新 userdata，满足其对
    // 有效 lua_State 的入参契约。
    unsafe {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
          variant: TypeFunctionSingletonVariant::V0(TypeFunctionBooleanSingleton { value }),
        }),
        false,
      )
    };

    return 1;
  }

  if l.is_string(1) {
    // cpp `std::string value = luaL_checkstring(L, 1)`：与 oracle 同形地先落地拥有值，
    // 借用窗不跨过 `alloc_type_user_data` 的写栈。
    let value = l.check_str(1).to_owned();
    // Safety: 前置 `lua_type==String` 保证索引 1 是字符串；`alloc_type_user_data` 在存活
    // `l` 上压入 userdata。
    unsafe {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
          variant: TypeFunctionSingletonVariant::V1(TypeFunctionStringSingleton {
            value,
          }),
        }),
        false,
      )
    };

    return 1;
  }

  if l.is_nil(1) {
    // Safety: `l` 为存活 lua_State（nil 分支仍持有有效状态），索引 1 实参在位；
    // `alloc_type_user_data` 在此状态上压入 NilType userdata。
    unsafe {
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType::new(Type::NilType)),
        false,
      )
    };

    return 1;
  }

  if fflag::LuauUdtfCreateSingletonFixErrorMessage.get() {
    // 上游修正后的消息：luaL_typename 按栈上值的实际类型取名
    // Safety: `lua_l_typename` 对存活 `l` 索引 1 恒返回静态 NUL 结尾串
    // （"no value" 或对象类型名），`cstr_cow` 因此有效且不接管所有权。
    let type_name = unsafe { cstr_cow(lua_l_typename(&*l, 1)) };
    // Safety: `l` 存活；错误经 throw_type_error 收口，消息为其类型名（经
    // `format_args!` 安全转发）；`throw_type_error` 返回 `!`，此分支不再落到函数末尾。
    unsafe {
      throw_type_error(
        l,
        format_args!(
          "types.singleton: can't create a singleton from a {}",
          type_name
        ),
      )
    }
  } else {
    // 上游遗留消息：lua_typename 收到的是类型常量而非栈索引，
    // C++ 原样传 1（LUA_TNIL），故恒为 "nil"，忠实保留
    // Safety: `lua_typename(_, 1)` 把 1 当作类型常量 LUA_TNIL 索引静态表
    // `TYPENAMES_C`，返回其静态 NUL 结尾串，`cstr_cow` 有效。
    let type_name = unsafe { cstr_cow(lua_typename(l.as_mut_ptr(), 1)) };
    // Safety: 同修正分支——`l` 存活、错误经 throw_type_error 收口、消息为其类型名；
    // `throw_type_error` 发散返回 `!`。
    unsafe {
      throw_type_error(
        l,
        format_args!(
          "types.singleton: can't create singleton from `{}` type",
          type_name
        ),
      )
    }
  }
}
