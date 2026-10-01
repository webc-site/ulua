//! Faithful port of `void registerTypesLibrary(lua_State* l)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1876-1914).

use ulua_vm::{
  functions::lua_l_register::lua_l_register_bytes,
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
  macros::c_thunk,
  records::arena_handle::alias,
  type_aliases::lua_state::LuaState,
};

c_thunk!(create_unknown_thunk, create_unknown);
c_thunk!(create_never_thunk, create_never);
c_thunk!(create_any_thunk, create_any);
c_thunk!(create_boolean_thunk, create_boolean);
c_thunk!(create_number_thunk, create_number);
c_thunk!(create_string_thunk, create_string);
c_thunk!(create_thread_thunk, create_thread);
c_thunk!(create_buffer_thunk, create_buffer);

c_thunk!(create_singleton_thunk, create_singleton);
c_thunk!(create_negation_thunk, create_negation);
c_thunk!(create_union_thunk, create_union);
c_thunk!(create_intersection_thunk, create_intersection);
c_thunk!(create_optional_thunk, create_optional);
c_thunk!(create_table_thunk, create_table);
c_thunk!(create_function_thunk, create_function);
c_thunk!(deep_copy_thunk, deep_copy);
c_thunk!(create_generic_thunk, create_generic);

/// 类型库字段构造函数指针（VM C 函数形状）。
type TypeLibCfunction = unsafe extern "C-unwind" fn(l: *mut lua_state::LuaState) -> i32;

/// `types` 库的「常量字段」表：调用即向栈压入对应 primitive 类型 userdata。
/// cpp 用 `luaL_Reg fields[]` 承载同一数据，这里直接是「NUL 结尾名字 + thunk」
/// 二元组：字段名只需交给 `lua_setfield`，无需构造 `luaL_Reg`，故省掉空名哨兵
/// 与逐项 `Option` 解包（原实现靠 `expect` 兜住「非空 name 项必有 func」）。
const TYPES_FIELDS: &[(&[u8], TypeLibCfunction)] = &[
  (b"unknown", create_unknown_thunk),
  (b"never", create_never_thunk),
  (b"any", create_any_thunk),
  (b"boolean", create_boolean_thunk),
  (b"number", create_number_thunk),
  (b"string", create_string_thunk),
  (b"thread", create_thread_thunk),
  (b"buffer", create_buffer_thunk),
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
pub(crate) unsafe fn register_types_library(l: *mut LuaState) {
  let vm_l = l as *mut lua_state::LuaState;

  // luaL_register(l, "types", methods);
  // Safety: `vm_l` 即函数级 # Safety 中存活且独占的 lua_State（非空）；
  // `LIB_TYPES` 是静态 NUL 结尾字节串；`METHODS` 为常量数组，
  // 期间无人改写。
  unsafe { lua_l_register_bytes(vm_l, Some(LIB_TYPES), &TYPES_METHODS) };

  // Set fields for type userdata
  // for (luaL_Reg* l = fields; l->name; l++) { l->func(L); lua_setfield(L, -2, l->name); }
  for (name, func) in TYPES_FIELDS {
    // Safety: thunk 要求存活且独占的 state，与调用点 `prepare_state` 对 `l` 的
    // 承诺一致；`name` 是静态 NUL 结尾字节串，thunk 已把该字段的类型 userdata 压入
    // 栈顶，"types" 表随后位于 -2。
    unsafe {
      func(vm_l);
      (*vm_l).set_field_bytes(-2, name);
    }
  }

  // lua_pop(l, 1);
  alias(vm_l).pop(1);
}
