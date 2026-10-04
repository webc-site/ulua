/// 对应 C++ 原生 `static int getFunctionGenerics(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1464`）。
use ulua_vm::{
  functions::{lua_createtable::lua_createtable, lua_rawseti::lua_rawseti},
  records::lua_state::LuaState,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_tag::get_tag,
    get_type_function_runtime::{get_type_function_type_id, get_type_function_type_pack_id},
    get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_tag,
  records::{
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
  },
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};
pub(crate) fn get_function_generics(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活（经 `c_thunk!` 蹦床重建为独占 `&mut`）。`self_ty` 由 `get_type_user_data` 取得，`get_type_function_type_id::<
  // TypeFunctionFunctionType>` 按 RTTI class-index 下转；`tfft.is_none()` 分支内 `throw_type_error`
  // 返回 `!`（抛 Lua 错误不返回），故其后 `expect` 取回的引用由该 Some 证明蕴含，`(*el)`(arena
  // 存活 TypeId) 解引用合法。generic_packs 元素按构造均为 TypeFunctionGenericTypePack，未命中
  // 即契约违例、由 `expect` 确定性 panic。单线程串行执行，无并发别名。
  unsafe {
    let self_ty = get_type_user_data(l, 1);

    let tfft = get_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    lua_check_tag!(
      l,
      tfft.is_none(),
      self_ty,
      "type.generics: expected self to be a function, but got {} instead"
    );

    let tfft = tfft.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");

    lua_createtable(
      l.as_mut_ptr(),
      (tfft.generics.len() + tfft.generic_packs.len()) as i32,
      0,
    );

    let mut pos: i32 = 1;

    for el in &tfft.generics {
      alloc_type_user_data(l, (*(*el)).type_variant.clone(), false);
      lua_rawseti(l, -2, pos);
      pos += 1;
    }

    for el in &tfft.generic_packs {
      let gty = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(*el)
        .expect("generic_packs 按构造均为 TypeFunctionGenericTypePack，未命中即契约违例");
      alloc_type_user_data(
        l,
        TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
          is_named: gty.is_named,
          is_pack: true,
          name: gty.name.clone(),
        }),
        false,
      );
      lua_rawseti(l, -2, pos);
      pos += 1;
    }

    1
  }
}
