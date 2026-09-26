//! Faithful port of `void registerTypesLibrary(lua_State* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1876-1914).

use ulua_vm::{
  functions::{lua_l_register::lua_l_register, lua_setfield::lua_setfield},
  macros::lua_pop::lua_pop,
  records::{lua_l_reg::LuaLReg, lua_state},
};

use crate::{
  functions::{
    create_any::create_any, create_boolean::create_boolean, create_buffer::create_buffer,
    create_function::create_function, create_generic::create_generic,
    create_intersection::create_intersection, create_negation::create_negation,
    create_never::create_never, create_number::create_number, create_optional::create_optional,
    create_singleton::create_singleton, create_string::create_string, create_table::create_table,
    create_thread::create_thread, create_union::create_union, create_unknown::create_unknown,
    deep_copy::deep_copy, lua_names::LIB_TYPES,
  },
  type_aliases::lua_state::LuaState,
};
macro_rules! type_lib_thunk {
  ($thunk:ident, $real:path) => {
    unsafe extern "C-unwind" fn $thunk(l: *mut ulua_vm::records::lua_state::LuaState) -> i32 {
      // Safety: Lua 只会在活跃 lua_State 的调用帧上回调已注册的 C 函数，故 `l`
      // 非空且指向存活的 VM 状态；analysis 侧 `LuaState` 是不透明镜像类型，
      // 与 vm 侧不透明 `lua_State` 间的指针转换不改变地址。`$real` 的前置条件
      // （存活 state、调用期独占其栈）由单线程类型函数 runtime 的调度保证。
      unsafe { $real(l as *mut LuaState) }
    }
  };
}

type_lib_thunk!(create_unknown_thunk, create_unknown);
type_lib_thunk!(create_never_thunk, create_never);
type_lib_thunk!(create_any_thunk, create_any);
type_lib_thunk!(create_boolean_thunk, create_boolean);
type_lib_thunk!(create_number_thunk, create_number);
type_lib_thunk!(create_string_thunk, create_string);
type_lib_thunk!(create_thread_thunk, create_thread);
type_lib_thunk!(create_buffer_thunk, create_buffer);

type_lib_thunk!(create_singleton_thunk, create_singleton);
type_lib_thunk!(create_negation_thunk, create_negation);
type_lib_thunk!(create_union_thunk, create_union);
type_lib_thunk!(create_intersection_thunk, create_intersection);
type_lib_thunk!(create_optional_thunk, create_optional);
type_lib_thunk!(create_table_thunk, create_table);
type_lib_thunk!(create_function_thunk, create_function);
type_lib_thunk!(deep_copy_thunk, deep_copy);
type_lib_thunk!(create_generic_thunk, create_generic);

/// 类型库字段构造函数指针（VM C 函数形状）。
type TypeLibCfunction = unsafe extern "C-unwind" fn(l: *mut lua_state::LuaState) -> i32;

/// `types` 库的「常量字段」表：调用即向栈压入对应 primitive 类型 userdata。
/// cpp 用 `luaL_Reg fields[]` 承载同一数据，这里直接是「NUL 结尾名字 + thunk」
/// 二元组：字段名只需交给 `lua_setfield`，无需构造 `luaL_Reg`，故省掉空名哨兵
/// 与逐项 `Option` 解包（原实现靠 `expect` 兜住「非空 name 项必有 func」）。
const TYPES_FIELDS: &[(&[u8], TypeLibCfunction)] = &[
  (b"unknown\0", create_unknown_thunk),
  (b"never\0", create_never_thunk),
  (b"any\0", create_any_thunk),
  (b"boolean\0", create_boolean_thunk),
  (b"number\0", create_number_thunk),
  (b"string\0", create_string_thunk),
  (b"thread\0", create_thread_thunk),
  (b"buffer\0", create_buffer_thunk),
];

/// `luaL_Reg methods[]`：注册为全局 `types` 库的构造函数。
const TYPES_METHODS: [LuaLReg; 9] = [
  LuaLReg::new(b"singleton", create_singleton_thunk),
  LuaLReg::new(b"negationof", create_negation_thunk),
  LuaLReg::new(b"unionof", create_union_thunk),
  LuaLReg::new(b"intersectionof", create_intersection_thunk),
  LuaLReg::new(b"optional", create_optional_thunk),
  LuaLReg::new(b"newtable", create_table_thunk),
  LuaLReg::new(b"newfunction", create_function_thunk),
  LuaLReg::new(b"copy", deep_copy_thunk),
  LuaLReg::new(b"generic", create_generic_thunk),
];

/// # Safety
/// `l` 必须指向存活的 lua_State，且在调用期间被本线程独占使用：本 crate 唯一
/// 调用点是 `TypeFunctionRuntime::prepare_state`，传入刚由 `lua_newstate` 创建、
/// 已 `set_type_function_environment` 与 `register_type_user_data` 初始化的
/// state；函数会向其栈推入/弹出类型 userdata 与 "types" 库表，故要求栈有可用
/// 余量且 state 未被其他持有者并发访问（runtime 为单线程构造路径）。
pub unsafe fn register_types_library(l: *mut LuaState) {
  let vm_l = l as *mut lua_state::LuaState;

  // luaL_register(l, "types", methods);
  // Safety: `vm_l` 即函数级 # Safety 中存活且独占的 lua_State（非空）；
  // `LIB_TYPES` 是静态 NUL 结尾字节串；`METHODS` 为常量数组，
  // 期间无人改写。
  unsafe { lua_l_register(vm_l, LIB_TYPES.as_ptr().cast(), &TYPES_METHODS) };

  // Set fields for type userdata
  // for (luaL_Reg* l = fields; l->name; l++) { l->func(L); lua_setfield(L, -2, l->name); }
  for (name, func) in TYPES_FIELDS {
    // Safety: thunk 要求存活且独占的 state，与调用点 `prepare_state` 对 `l` 的
    // 承诺一致；`name` 是静态 NUL 结尾字节串，thunk 已把该字段的类型 userdata 压入
    // 栈顶，"types" 表随后位于 -2。
    unsafe {
      func(vm_l);
      lua_setfield(vm_l, -2, name.as_ptr().cast());
    }
  }

  // lua_pop(l, 1);
  // Safety: 每个字段的 `lua_setfield` 已消费 thunk 压入的值，栈上仅剩
  // `lua_l_register` 留下的 "types" 表这一层额外槽位，弹 1 恢复调用方栈深；
  // `vm_l` 由函数级契约保证存活。
  unsafe { lua_pop(vm_l, 1) };
}
