

/// # Safety
/// `create_singleton` 是登记给 Lua 的类型函数 `types.singleton`，经
/// `create_singleton_thunk` 作为 `lua_CFunction` 由 VM 调起。`l` 必须非空并指向
/// 该次真实 Lua 调用持有的存活 `lua_State`，且其调用栈自索引 1 起至少有一个实参
/// （函数体所有 `l.as_mut_ptr()`/`l` 操作均以索引 1 读取该实参）。`l.as_mut_ptr()` 只是同一对象在
/// `lua_state::LuaState` newtype 视图下的指针，对象身份与存活期不变。
use ulua_common::{fflag, functions::c_str::cstr_cow};
use ulua_vm::{functions::{lua_l_typename::lua_l_typename, lua_typename::lua_typename}, records::lua_state};
use crate::{enums::type_type_function_runtime::Type, functions::{alloc_type_user_data::alloc_type_user_data, throw_type_error::throw_type_error}, records::{arena_handle::{alias, alias_ref}, type_function_boolean_singleton::TypeFunctionBooleanSingleton, type_function_primitive_type::TypeFunctionPrimitiveType, type_function_singleton_type::TypeFunctionSingletonType, type_function_string_singleton::TypeFunctionStringSingleton}, type_aliases::{type_function_singleton_variant::TypeFunctionSingletonVariant, type_function_type_variant::TypeFunctionTypeVariant}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn create_singleton(l: &mut LuaState) -> i32 {

  if alias_ref(l.as_mut_ptr()).is_boolean(1) {
    let value = alias(l.as_mut_ptr()).check_boolean(1);
    // Safety: `l` 为存活 lua_State 且索引 1 实参在位（刚判定为布尔）；
    // `alloc_type_user_data` 在此状态上检查栈空间并压入新 userdata，满足其对
    // 有效 lua_State 的入参契约。
    unsafe {
      alloc_type_user_data(
        &mut *l,
        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
          variant: TypeFunctionSingletonVariant::V0(TypeFunctionBooleanSingleton { value }),
        }),
        false,
      )
    };

    return 1;
  }

  if alias_ref(l.as_mut_ptr()).is_string(1) {
    let value = alias(l.as_mut_ptr()).check_str(1);
    // Safety: 前置 `lua_type==String` 保证索引 1 是字符串，`check_str` 返回
    // 指向该存活 VM 字符串的切片；`alloc_type_user_data` 在存活 `l` 上压入 userdata。
    unsafe {
      alloc_type_user_data(
        &mut *l,
        TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
          variant: TypeFunctionSingletonVariant::V1(TypeFunctionStringSingleton {
            value: value.to_owned(),
          }),
        }),
        false,
      )
    };

    return 1;
  }

  if alias_ref(l.as_mut_ptr()).is_nil(1) {
    // Safety: `l` 为存活 lua_State（nil 分支仍持有有效状态），索引 1 实参在位；
    // `alloc_type_user_data` 在此状态上压入 NilType userdata。
    unsafe {
      alloc_type_user_data(
        &mut *l,
        TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType::new(Type::NilType)),
        false,
      )
    };

    return 1;
  }

  if fflag::LuauUdtfCreateSingletonFixErrorMessage.get() {
    // 上游修正后的消息：luaL_typename 按栈上值的实际类型取名
    // Safety: `lua_l_typename` 对存活 `l.as_mut_ptr()` 索引 1 恒返回静态 NUL 结尾串
    // （"no value" 或对象类型名），`cstr_cow` 因此有效且不接管所有权。
    let type_name = unsafe { cstr_cow(lua_l_typename(l.as_ptr(), 1)) };
    // Safety: `l.as_mut_ptr()` 存活；错误经 throw_type_error 收口，消息为其类型名（经
    // `format_args!` 安全转发）；`throw_type_error` 返回 `!`，此分支不再落到函数末尾。
    unsafe {
      throw_type_error(
        l.as_mut_ptr(),
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
    // Safety: 同修正分支——`l.as_mut_ptr()` 存活、错误经 throw_type_error 收口、消息为其类型名；
    // `throw_type_error` 发散返回 `!`。
    unsafe {
      throw_type_error(
        l.as_mut_ptr(),
        format_args!(
          "types.singleton: can't create singleton from `{}` type",
          type_name
        ),
      )
    }
  }
}
