use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::type_type_function_runtime::Type,
  functions::{
    get_tag::get_tag, get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data, push_string::push_string,
    throw_type_error::throw_type_error,
  },
  macros::{lua_check_args, lua_check_tag},
  records::{
    type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
  },
};
pub(crate) fn get_singleton_value(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活（经 `c_thunk!` 蹦床重建为独占 `&mut`）。
  // `tfpt`/`tfst` 按 class-index 下转：`tfpt` 命中 Some 分支才读 `r#type`；`tfst` 的
  // `is_none()` 分支内 `throw_type_error` 返回 `!` 不返回，故其后 `expect` 取回的引用由该
  // Some 证明蕴含，`get_if::<..>()` 命中的子借用随该存活对象存在。末尾 `throw_type_error`
  // 亦不返回。单线程串行，无并发别名。
  unsafe {
    lua_check_args!(l, != 1, "type.value: expected 1 argument, but got {}");

    let self_ty = get_type_user_data(l, 1);
    if let Some(tfpt) = get_type_function_type_id::<TypeFunctionPrimitiveType>(self_ty) {
      lua_check_tag!(
        l,
        tfpt.r#type != Type::NilType,
        self_ty,
        "type.value: expected self to be a singleton, but got {} instead"
      );

      l.push_nil();
      return 1;
    }

    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(self_ty);
    lua_check_tag!(
      l,
      tfst.is_none(),
      self_ty,
      "type.value: expected self to be a singleton, but got {} instead"
    );

    let tfst = tfst.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");

    if let Some(tfbst) = tfst.variant.get_if::<TypeFunctionBooleanSingleton>() {
      l.push_boolean(tfbst.value);
      return 1;
    }

    if let Some(tfsst) = tfst.variant.get_if::<TypeFunctionStringSingleton>() {
      push_string(l, &tfsst.value);
      return 1;
    }

    let tag = get_tag(l, self_ty);
    throw_type_error(
      l,
      format_args!("type.value: can't call `value` method on `{}` type", tag),
    );
  }
}
