/// 对应 C++ 原生 `static int createTable(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:796`）。
use alloc::{collections::BTreeMap, string::String};

use ulua_vm::{functions::lua_l_typeerror_l::lua_l_typeerror_l, records::lua_state::LuaState};

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
    type_function_type_id::TypeFunctionTypeId, type_function_type_variant::TypeFunctionTypeVariant,
  },
};
pub(crate) fn create_table(l: &mut LuaState) -> i32 {
  // Safety: l 为 VM 调注册闭包传入的存活 lua_State；参数先经 lua_istable/lua_isnoneornil/
  // luaL_typeerror/throw_type_error 校验（格式串收敛在 throw_type_error 一处）；
  // lua_next 循环保守 -2 键/-1 值的栈序且每轮成对 lua_pop，getfield/pop 配平；
  // get_type_user_data 对非 type 栈槽先抛错，tfst/mt_table 命中 Some 后才读取，指向
  // type_arena 存活节点（bump 块地址不移动）；tfst.variant.get_if 是 tag 判别只读，
  // expect 前经 lua_check_tag 排除 None；optional_type_user_data/alloc_type_user_data 同族闭包前置满足。
  unsafe {
    lua_check_args!(l, > 3, "types.newtable: expected 0-3 arguments, but got {}");

    let mut props: BTreeMap<String, TypeFunctionProperty> = BTreeMap::new();

    if l.is_table(1) {
      l.push_nil();
      while l.next(1) {
        let key = get_type_user_data(l, -2);

        let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
        lua_check_tag!(
          l,
          tfst.is_none(),
          key,
          "types.newtable: expected to be given a singleton type, but got {} instead"
        );

        let tfst = tfst.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");
        let tfsst = tfst.variant.get_if::<TypeFunctionStringSingleton>();
        lua_check_tag!(
          l,
          tfsst.is_none(),
          key,
          "types.newtable: expected to be given a string singleton type, but got {} instead"
        );
        // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
        let tfsst = tfsst.expect("上方 throw_type_error(-> !) 已拦截 None 分支");

        if l.is_table(-1) {
          l.get_field_bytes(-1, FIELD_READ);
          let mut read_ty: Option<TypeFunctionTypeId> = None;
          if !l.is_nil(-1) {
            read_ty = Some(get_type_user_data(l, -1));
          }
          l.pop(1);

          l.get_field_bytes(-1, FIELD_WRITE);
          let mut write_ty: Option<TypeFunctionTypeId> = None;
          if !l.is_nil(-1) {
            write_ty = Some(get_type_user_data(l, -1));
          }
          l.pop(1);

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

        l.pop(1);
      }
    } else if !l.is_none_or_nil(1) {
      lua_l_typeerror_l(l.as_mut_ptr(), 1, "table");
    }

    let mut indexer: Option<TypeFunctionTableIndexer> = None;
    if l.is_table(2) {
      l.get_field_bytes(2, FIELD_INDEX);
      let key_type = get_type_user_data(l, -1);
      l.pop(1);

      l.get_field_bytes(2, FIELD_READ_RESULT);
      let value_type = get_type_user_data(l, -1);
      l.pop(1);

      indexer = Some(TypeFunctionTableIndexer::new(key_type, value_type));
    } else if !l.is_none_or_nil(2) {
      lua_l_typeerror_l(l.as_mut_ptr(), 2, "table");
    }

    let metatable = optional_type_user_data(l, 3);
    if let Some(mt) = metatable {
      let mt_table = get_type_function_type_id::<TypeFunctionTableType>(mt);
      lua_check_tag!(
        l,
        mt_table.is_none(),
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
