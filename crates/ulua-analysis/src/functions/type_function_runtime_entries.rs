//! `TypeFunctionRuntime.cpp` 中成对的静态 C 入口（read/write、parameters/returns
//! 孪生）的共享实现。各入口保持函数名与守卫语义不变，仅把「消息前缀 + 读/写字段」
//! 两个分叉点参数化：诊断消息经 `format_args!("{prefix}: ...")` 拼出与原手写字面量
//! 逐字节一致的串。入口本身是 safe `fn`——孪生入口以 `&mut LuaState` 收形，真正的
//! C 边界在 `c_thunk!` 生成的 `unsafe extern "C-unwind"` thunk；本模块的 unsafe 仅
//! 收口在保留 `unsafe fn` 的 `push_type_pack`/`push_table_indexer` 调用点上
//! （二者解引用 arena 节点句柄，前提见其 `# Safety`）。

use ulua_common::fflag;
use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
    push_table_indexer::push_table_indexer, push_type_pack::push_type_pack,
    throw_type_error::throw_type_error,
  },
  records::{
    arena_handle::alias_ref, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
  },
};
/// 取 `type.readproperty`/`type.writeproperty` 的属性值并按 userdata 推栈
/// （C++ `readTableProp`/`writeTableProp` 共用骨架）。`read` 选择 `read_ty`
/// 还是 `write_ty`。
///
/// 内存安全前提：`l` 由 `&mut` 承载存活/独占；`tftt`/`tfst` 为 class-index 命中的
/// arena 存活节点借用（判据见 [`get_type_function_type_id`]），`prop_ty` 为
/// type_arena 句柄、经 `alias_ref` 只读取 variant。`throw_type_error` 与
/// `alloc_type_user_data` 已降级为 safe fn，调用点无 `unsafe` 块。
pub(crate) fn get_table_prop(l: &mut LuaState, prefix: &str, read: bool) -> i32 {
  let argument_count = l.get_top();
  if argument_count != 2 {
    throw_type_error(
      l,
      format_args!("{prefix}: expected 2 arguments, but got {argument_count}"),
    );
  }

  let self_ty = get_type_user_data(l, 1);
  let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
  if tftt.is_none() {
    let tag = get_tag(l, self_ty);
    throw_type_error(
      l,
      format_args!(
        "{prefix}: expected self to be either a table, but got {} instead",
        tag
      ),
    );
  }

  let key = get_type_user_data(l, 2);
  let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
  if tfst.is_none() {
    let tag = get_tag(l, key);
    throw_type_error(
      l,
      format_args!(
        "{prefix}: expected to be given a singleton type, but got {} instead",
        tag
      ),
    );
  }

  // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
  let tfsst = tfst
    .expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some")
    .variant
    .get_if_1();
  if tfsst.is_none() {
    let tag = get_tag(l, key);
    throw_type_error(
      l,
      format_args!(
        "{prefix}: expected to be given a string singleton type, but got {} instead",
        tag
      ),
    );
  }

  let tftt = tftt.expect("同上，is_none 分支经 throw_type_error(-> !) 早退");
  let key_name = &tfsst
    .expect("is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some")
    .value;
  let prop = tftt.props.get(key_name);
  if prop.is_none() {
    l.push_nil();
    return 1;
  }

  // is_none 分支已提前 return，至此必为 Some。
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
    alloc_type_user_data(l, alias_ref(prop_ty).type_variant.clone(), false);
  } else {
    l.push_nil();
  }

  1
}

/// 写 `type.setreadproperty`/`type.setwriteproperty` 的属性值（C++
/// `setReadTableProp`/`setWriteTableProp` 共用骨架）。`read` 决定改写
/// `read_ty` 还是 `write_ty`（含清空与「仅此一侧时整项移除/插入」分支）。
///
/// 内存安全前提：`l` 由 `&mut` 承载存活/独占；`tftt`/`tfst` 为 class-index 命中的
/// arena 存活节点借用（判据见 [`get_mutable_type_function_type_id`] 与
/// [`get_type_function_type_id`] 的函数头），`frozen` 标志经 `alias_ref` 只读。
/// `throw_type_error` 已降级为 safe fn，调用点无 `unsafe` 块。
pub(crate) fn set_table_prop_rw(l: &mut LuaState, prefix: &str, read: bool) -> i32 {
  let argument_count = l.get_top();
  if !(2..=3).contains(&argument_count) {
    throw_type_error(
      l,
      format_args!("{prefix}: expected 2-3 arguments, but got {argument_count}"),
    );
  }

  let self_ty = get_type_user_data(l, 1);
  let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);
  if tftt.is_none() {
    let tag = get_tag(l, self_ty);
    throw_type_error(
      l,
      format_args!(
        "{prefix}: expected self to be a table, but got {} instead",
        tag
      ),
    );
  }

  if fflag::LuauTypeFunctionSupportsFrozen.get() && alias_ref(self_ty).frozen {
    throw_type_error(
      l,
      format_args!(
        "{prefix}: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
      ),
    );
  }

  let key = get_type_user_data(l, 2);
  let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
  if tfst.is_none() {
    let tag = get_tag(l, key);
    throw_type_error(
      l,
      format_args!(
        "{prefix}: expected to be given a singleton type, but got {} instead",
        tag
      ),
    );
  }

  // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
  let tfsst = tfst
    .expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some")
    .variant
    .get_if_1();
  if tfsst.is_none() {
    let tag = get_tag(l, key);
    throw_type_error(
      l,
      format_args!(
        "{prefix}: expected to be given a string singleton type, but got {} instead",
        tag
      ),
    );
  }

  // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
  let tftt = tftt.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");
  let key_name = tfsst
    .expect("上方 throw_type_error(-> !) 已拦截 None 分支")
    .value
    .clone();

  if argument_count == 2 || l.is_nil(3) {
    if let Some(existing) = tftt.props.get(&key_name) {
      let sole = if read {
        existing.is_read_only()
      } else {
        existing.is_write_only()
      };
      if sole {
        tftt.props.remove(&key_name);
      } else if let Some(prop) = tftt.props.get_mut(&key_name) {
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
  if let Some(prop) = tftt.props.get_mut(&key_name) {
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
    tftt.props.insert(key_name, prop);
  }

  0
}

/// 取 `type.parent`（C++ `getReadParent`/`getWriteParent` 共用骨架，消息无分叉，
/// 仅 `read_parent`/`write_parent` 字段随 `read` 切换）。
///
/// 内存安全前提：`l` 由 `&mut` 承载；`tfct` 为 class-index 命中的 arena 存活节点
/// 借用；`parent` 为 type_arena 句柄，经 `alias_ref` 只读取 variant。本函数内
/// 调用点（`throw_type_error`/`alloc_type_user_data`）均已降级为 safe fn。
pub(crate) fn get_parent(l: &mut LuaState, read: bool) -> i32 {
  let argument_count = l.get_top();
  if argument_count != 1 {
    throw_type_error(
      l,
      format_args!("type.parent: expected 1 arguments, but got {argument_count}"),
    );
  }

  let self_ty = get_type_user_data(l, 1);
  let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
  if tfct.is_none() {
    let tag = get_tag(l, self_ty);
    throw_type_error(
      l,
      format_args!(
        "type.parent: expected self to be a class, but got {} instead",
        tag
      ),
    );
  }

  // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
  let tfct = tfct.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");
  let parent = if read {
    tfct.read_parent
  } else {
    tfct.write_parent
  };
  if let Some(parent) = parent {
    alloc_type_user_data(l, alias_ref(parent).type_variant.clone(), false);
  } else {
    l.push_nil();
  }

  1
}

/// 取 `type.readindexer`/`type.writeindexer`（C++ `getReadIndexer`/
/// `getWriteIndexer` 共用骨架，仅消息前缀随 `prefix` 分叉）。
///
/// 内存安全前提：`l` 由 `&mut` 承载；`indexer` 借用自 class-index 命中的 arena
/// 存活节点。仅 `push_table_indexer`（保留 `unsafe fn`，其内句柄解引用前提见其
/// `# Safety`）调用点保留 `unsafe` 块；`throw_type_error` 已降级为 safe fn。
pub(crate) fn get_indexer(l: &mut LuaState, prefix: &str) -> i32 {
  let argument_count = l.get_top();
  if argument_count != 1 {
    throw_type_error(
      l,
      format_args!("{prefix}: expected 1 arguments, but got {argument_count}"),
    );
  }

  let self_ty = get_type_user_data(l, 1);

  if let Some(tftt) = get_type_function_type_id::<TypeFunctionTableType>(self_ty) {
    // Safety: 保留 `unsafe fn` `push_table_indexer`；`indexer` 为 arena 存活节点字段，其内
    // 句柄的存活前提由该函数 `# Safety` 文档所述消费契约担保（同源守卫）。
    unsafe {
      push_table_indexer(l, &tftt.indexer);
    }
    return 1;
  }

  if let Some(tfct) = get_type_function_type_id::<TypeFunctionExternType>(self_ty) {
    // Safety: 同上，保留 `unsafe fn` `push_table_indexer` 对 extern 侧 indexer 字段。
    unsafe {
      push_table_indexer(l, &tfct.indexer);
    }
    return 1;
  }

  let tag = get_tag(l, self_ty);
  throw_type_error(
    l,
    format_args!(
      "{prefix}: expected self to be either a table or class, but got {} instead",
      tag
    ),
  );
}

/// 取 `type.parameters`/`type.returns`（C++ `getFunctionParameters`/
/// `getFunctionReturns` 共用骨架）：仅消息前缀与 `arg_types`/`ret_types`
/// 字段随参数分叉，两者都是整包 push。
///
/// 内存安全前提：`l` 由 `&mut` 承载；`tfft` 为 class-index 命中的 arena 存活节点
/// 借用；pack 字段是 type_pack_arena 句柄、连同 `l` 交给保留 `unsafe fn`
/// `push_type_pack`（其 `# Safety` 同源契约）。
pub(crate) fn get_function_pack(l: &mut LuaState, prefix: &str, params: bool) -> i32 {
  let argument_count = l.get_top();
  if argument_count != 1 {
    throw_type_error(
      l,
      format_args!("{prefix}: expected 1 arguments, but got {argument_count}"),
    );
  }

  let self_ty = get_type_user_data(l, 1);
  let tfft = get_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
  if tfft.is_none() {
    let tag = get_tag(l, self_ty);
    throw_type_error(
      l,
      format_args!(
        "{prefix}: expected self to be a function, but got {} instead",
        tag
      ),
    );
  }

  // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
  let tfft = tfft.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");
  let pack = if params {
    tfft.arg_types
  } else {
    tfft.ret_types
  };
  // Safety: 保留 `unsafe fn` `push_type_pack`，pack 为本次调用期存活的 arena 句柄。
  unsafe {
    push_type_pack(l, pack);
  }

  1
}
