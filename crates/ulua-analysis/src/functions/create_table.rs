use alloc::{collections::BTreeMap, string::String};

use ulua_vm::{
  functions::{
    lua_getfield::lua_getfield, lua_gettop::lua_gettop, lua_l_typeerror_l::lua_l_typeerror_l,
    lua_next::lua_next, lua_pushnil::lua_pushnil,
  },
  macros::{
    lua_isnil::lua_isnil, lua_isnoneornil::lua_isnoneornil, lua_istable::lua_istable,
    lua_pop::lua_pop,
  },
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
    lua_names::{FIELD_INDEX, FIELD_READ, FIELD_READ_RESULT, FIELD_WRITE},
    optional_type_user_data::optional_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::{lua_check_args, lua_check_tag},
  records::{
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int createTable(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:796`）。
pub unsafe fn create_table(l: *mut LuaState) -> i32 {
  // Safety: l 为 VM 调注册闭包传入的存活 lua_State；参数先经 lua_istable/lua_isnoneornil/
  // luaL_typeerror/throw_type_error 校验（格式串收敛在 throw_type_error 一处）；
  // lua_next 循环保守 -2 键/-1 值的栈序且每轮成对 lua_pop，getfield/pop 配平；
  // get_type_user_data 对非 type 栈槽先抛错，tfst/mt_table 判 is_null 后才解引用，指向
  // type_arena 存活节点（bump 块地址不移动）；(*tfst).variant.get_if 是 tag 判别只读，
  // unwrap 前已排除 None；optional_type_user_data/alloc_type_user_data 同族闭包前置满足。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, > 3, "types.newtable: expected 0-3 arguments, but got {}");

    let mut props: BTreeMap<String, TypeFunctionProperty> = BTreeMap::new();

    if lua_istable!(vm_l, 1) {
      lua_pushnil(vm_l);
      while lua_next(vm_l, 1) != 0 {
        let key = get_type_user_data(l, -2);

        let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
        lua_check_tag!(
          vm_l,
          tfst.is_null(),
          l,
          key,
          "types.newtable: expected to be given a singleton type, but got {} instead"
        );

        let tfsst = (*tfst).variant.get_if::<TypeFunctionStringSingleton>();
        lua_check_tag!(
          vm_l,
          tfsst.is_none(),
          l,
          key,
          "types.newtable: expected to be given a string singleton type, but got {} instead"
        );
        // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
        let tfsst = tfsst.expect("上方 throw_type_error(-> !) 已拦截 None 分支");

        if lua_istable!(vm_l, -1) {
          lua_getfield(vm_l, -1, FIELD_READ.as_ptr().cast());
          let mut read_ty: Option<TypeFunctionTypeId> = None;
          if !lua_isnil!(vm_l, -1) {
            read_ty = Some(get_type_user_data(l, -1));
          }
          lua_pop(vm_l, 1);

          lua_getfield(vm_l, -1, FIELD_WRITE.as_ptr().cast());
          let mut write_ty: Option<TypeFunctionTypeId> = None;
          if !lua_isnil!(vm_l, -1) {
            write_ty = Some(get_type_user_data(l, -1));
          }
          lua_pop(vm_l, 1);

          let key_name = &tfsst.value;
          props.insert(key_name.clone(), TypeFunctionProperty { read_ty, write_ty });
        } else {
          let value = get_type_user_data(l, -1);
          let key_name = &tfsst.value;
          props.insert(
            key_name.clone(),
            TypeFunctionProperty {
              read_ty: Some(value),
              write_ty: Some(value),
            },
          );
        }

        lua_pop(vm_l, 1);
      }
    } else if !lua_isnoneornil!(vm_l, 1) {
      lua_l_typeerror_l(vm_l, 1, "table");
    }

    let mut indexer: Option<TypeFunctionTableIndexer> = None;
    if lua_istable!(vm_l, 2) {
      lua_getfield(vm_l, 2, FIELD_INDEX.as_ptr().cast());
      let key_type = get_type_user_data(l, -1);
      lua_pop(vm_l, 1);

      lua_getfield(vm_l, 2, FIELD_READ_RESULT.as_ptr().cast());
      let value_type = get_type_user_data(l, -1);
      lua_pop(vm_l, 1);

      indexer = Some(TypeFunctionTableIndexer::new(key_type, value_type));
    } else if !lua_isnoneornil!(vm_l, 2) {
      lua_l_typeerror_l(vm_l, 2, "table");
    }

    let metatable = optional_type_user_data(l, 3);
    if let Some(mt) = metatable {
      let mt_table = get_type_function_type_id::<TypeFunctionTableType>(mt);
      lua_check_tag!(
        vm_l,
        mt_table.is_null(),
        l,
        mt,
        "types.newtable: expected to be given a table type as a metatable, but got {} instead"
      );
    }

    alloc_type_user_data(
      l,
      TypeFunctionTypeVariant::Table(TypeFunctionTableType {
        props,
        indexer,
        metatable,
      }),
      false,
    );
    1
  }
}
