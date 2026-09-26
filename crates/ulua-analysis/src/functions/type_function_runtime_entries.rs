//! `TypeFunctionRuntime.cpp` 中成对的静态 C 入口（read/write、parameters/returns
//! 孪生）的共享实现。各 `pub unsafe fn` 入口保持原签名不变（仍按函数指针注册进
//! `register_type_user_data`），仅把「消息前缀 + 读/写字段」两个分叉点参数化：
//! 诊断消息经 `format_args!("{prefix}: ...")` 拼出与原手写字面量逐字节一致的串。

use ulua_common::fflag;
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_pushnil::lua_pushnil},
  macros::lua_isnil::lua_isnil,
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
    push_table_indexer::push_table_indexer, push_type_pack::push_type_pack,
    throw_type_error::throw_type_error,
  },
  records::{
    type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};

/// 取 `type.readproperty`/`type.writeproperty` 的属性值并按 userdata 推栈
/// （C++ `readTableProp`/`writeTableProp` 共用骨架）。`read` 选择 `read_ty`
/// 还是 `write_ty`。
///
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：
/// VM 已把实参压入栈顶，本函数只借用不持有该地址；`prefix` 仅为诊断前缀字面量
/// （"type.readproperty"/"type.writeproperty"）。调用期间单线程独占 VM 栈与类型
/// 运行期数据；`tftt`/`tfst` 按 class-index 下转，`is_null()`/`is_none()` 分支内
/// `throw_type_error` 返回 `!` 不返回，故其后解引用合法。
pub(crate) unsafe fn get_table_prop(l: *mut LuaState, prefix: &str, read: bool) -> i32 {
  // Safety: `l` 同址重解释为 `lua_state`；`(*tftt).props`/`(*tfst).variant` 均
  // 在上方非空守卫之后只读访问，指向 VM 分配且本次调用内存活的对象。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
      throw_type_error(
        vm_l,
        format_args!("{prefix}: expected 2 arguments, but got {argument_count}"),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if tftt.is_null() {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: expected self to be either a table, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    let key = get_type_user_data(l, 2);
    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
    if tfst.is_null() {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: expected to be given a singleton type, but got {} instead",
          get_tag(l, key)
        ),
      );
    }

    let tfsst = (*tfst).variant.get_if_1();
    if tfsst.is_none() {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: expected to be given a string singleton type, but got {} instead",
          get_tag(l, key)
        ),
      );
    }

    // Safety: 上方 is_none 分支走 throw_type_error（返回 !，不返回至此）。
    let key_name = &tfsst
      .expect("is_none 分支经 throw_type_error(返回!)早退，至此必为 Some")
      .value;
    let prop = (*tftt).props.get(key_name);
    if prop.is_none() {
      lua_pushnil(vm_l);
      return 1;
    }

    // Safety: prop.is_none() 分支已提前 return。
    let prop_ty = if read {
      prop
        .expect("上方 is_none 分支已 return，至此必为 Some")
        .read_ty
    } else {
      prop
        .expect("上方 is_none 分支已 return，至此必为 Some")
        .write_ty
    };
    if let Some(prop_ty) = prop_ty {
      alloc_type_user_data(l, (*prop_ty).type_variant.clone(), false);
    } else {
      lua_pushnil(vm_l);
    }

    1
  }
}

/// 写 `type.setreadproperty`/`type.setwriteproperty` 的属性值（C++
/// `setReadTableProp`/`setWriteTableProp` 共用骨架）。`read` 决定改写
/// `read_ty` 还是 `write_ty`（含清空与「仅此一侧时整项移除/插入」分支）。
///
/// # Safety
/// 前置条件与 [`get_table_prop`] 相同（VM 回调契约、`prefix` 为诊断前缀）；
/// 本函数还会经 `get_mutable_type_function_type_id` 取可变指针改写 props，
/// 该写只经 VM 独占的 userdata 指针发生，单线程内无别名。
pub(crate) unsafe fn set_table_prop_rw(l: *mut LuaState, prefix: &str, read: bool) -> i32 {
  // Safety: 同上；`(*self_ty).frozen` 与 props 的 get/get_mut/remove 都在
  // `tftt` 非空守卫之后，指向本次调用内存活的 userdata。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if !(2..=3).contains(&argument_count) {
      throw_type_error(
        vm_l,
        format_args!("{prefix}: expected 2-3 arguments, but got {argument_count}"),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if tftt.is_null() {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: expected self to be a table, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    if fflag::LuauTypeFunctionSupportsFrozen.get() && (*self_ty).frozen {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
        ),
      );
    }

    let key = get_type_user_data(l, 2);
    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
    if tfst.is_null() {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: expected to be given a singleton type, but got {} instead",
          get_tag(l, key)
        ),
      );
    }

    let tfsst = (*tfst).variant.get_if_1();
    if tfsst.is_none() {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: expected to be given a string singleton type, but got {} instead",
          get_tag(l, key)
        ),
      );
    }

    // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
    let key_name = tfsst
      .expect("上方 throw_type_error(-> !) 已拦截 None 分支")
      .value
      .clone();

    if argument_count == 2 || lua_isnil!(vm_l, 3) {
      if let Some(existing) = (*tftt).props.get(&key_name) {
        let sole = if read {
          existing.is_read_only()
        } else {
          existing.is_write_only()
        };
        if sole {
          (*tftt).props.remove(&key_name);
        } else if let Some(prop) = (*tftt).props.get_mut(&key_name) {
          if read {
            prop.read_ty = None;
          } else {
            prop.write_ty = None;
          }
        }
      }

      return 0;
    }

    let value = get_type_user_data(l, 3);
    if let Some(prop) = (*tftt).props.get_mut(&key_name) {
      if read {
        prop.read_ty = Some(value);
      } else {
        prop.write_ty = Some(value);
      }
    } else {
      let prop = if read {
        TypeFunctionProperty::readonly(value)
      } else {
        TypeFunctionProperty::writeonly(value)
      };
      (*tftt).props.insert(key_name, prop);
    }

    0
  }
}

/// 取 `type.parent`（C++ `getReadParent`/`getWriteParent` 共用骨架，消息无分叉，
/// 仅 `read_parent`/`write_parent` 字段随 `read` 切换）。
///
/// # Safety
/// `l` 的契约同 [`get_table_prop`]；`tfct` 按 class-index 下转，`is_null()`
/// 分支内 `throw_type_error` 返回 `!` 不返回，其后 `read_parent`/`write_parent`
/// 解引用合法，命中 Some 时为 arena 存活 TypeId。
pub(crate) unsafe fn get_parent(l: *mut LuaState, read: bool) -> i32 {
  // Safety: `l` 同址重解释；`(*tfct)` 字段读取在非空守卫后，单线程串行。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      throw_type_error(
        vm_l,
        format_args!("type.parent: expected 1 arguments, but got {argument_count}"),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if tfct.is_null() {
      throw_type_error(
        vm_l,
        format_args!(
          "type.parent: expected self to be a class, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    let parent = if read {
      (*tfct).read_parent
    } else {
      (*tfct).write_parent
    };
    if let Some(parent) = parent {
      alloc_type_user_data(l, (*parent).type_variant.clone(), false);
    } else {
      lua_pushnil(vm_l);
    }

    1
  }
}

/// 取 `type.readindexer`/`type.writeindexer`（C++ `getReadIndexer`/
/// `getWriteIndexer` 共用骨架，仅消息前缀随 `prefix` 分叉）。
///
/// # Safety
/// `l` 的契约同 [`get_table_prop`]；`tftt`/`tfct` 按 class-index 下转，仅在
/// `!is_null()` 守卫分支解引用其 `indexer` 字段（借用存活至本次调用结束）；
/// 末尾错误分支 `throw_type_error` 返回 `!` 不返回。
pub(crate) unsafe fn get_indexer(l: *mut LuaState, prefix: &str) -> i32 {
  // Safety: `l` 同址重解释；`push_table_indexer` 借用守卫后的存活字段。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      throw_type_error(
        vm_l,
        format_args!("{prefix}: expected 1 arguments, but got {argument_count}"),
      );
    }

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tftt.is_null() {
      push_table_indexer(l, vm_l, &(*tftt).indexer);
      return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
      push_table_indexer(l, vm_l, &(*tfct).indexer);
      return 1;
    }

    throw_type_error(
      vm_l,
      format_args!(
        "{prefix}: expected self to be either a table or class, but got {} instead",
        get_tag(l, self_ty)
      ),
    );
  }
}

/// 取 `type.parameters`/`type.returns`（C++ `getFunctionParameters`/
/// `getFunctionReturns` 共用骨架）：仅消息前缀与 `arg_types`/`ret_types`
/// 字段随参数分叉，两者都是整包 push。
///
/// # Safety
/// `l` 的契约同 [`get_table_prop`]；`tfft` 按 class-index 下转，`is_null()`
/// 分支内 `throw_type_error` 返回 `!` 不返回，其后 `arg_types`/`ret_types`
/// 解引用合法，该类型函数数据存活于本次调用。
pub(crate) unsafe fn get_function_pack(l: *mut LuaState, prefix: &str, params: bool) -> i32 {
  // Safety: `l` 同址重解释；pack 字段在非空守卫后只读，单线程串行。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      throw_type_error(
        vm_l,
        format_args!("{prefix}: expected 1 arguments, but got {argument_count}"),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfft = get_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    if tfft.is_null() {
      throw_type_error(
        vm_l,
        format_args!(
          "{prefix}: expected self to be a function, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    let pack = if params {
      (*tfft).arg_types
    } else {
      (*tfft).ret_types
    };
    push_type_pack(l, pack);

    1
  }
}
