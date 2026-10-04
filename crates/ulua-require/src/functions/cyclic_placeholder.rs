//! cpp `Require/src/RequireImpl.cpp` 中循环依赖占位表一段的移植：
//! 共享占位元表（`__index`/`__newindex` 抛循环依赖错误）、`createPlaceholder`、
//! `lockPlaceholder`、`populatePlaceholder`。`_CYCLIC_PLACEHOLDER_PROVIDED`
//! 标记的读写已并入 `registry_table` 的 mark 门面（set/take 两式）。
//!
//! 本文件对 `&mut LuaState` 操作，全部栈读写走 vm 的安全方法；仅两处保留
//! 最小 `unsafe`（各带 `# Safety`）：登记元方法 C 闭包（VM 回调契约）与
//! `luaL_error` 抛错入口。
//!
//! DELIBERATE DEVIATION：cpp 用静态变量 `&cyclicPlaceholderMetatableSentinel`
//! 的裸地址作注册表 `lua_rawgetp/rawsetp` 键；Rust 侧改以本 crate 私有的注册表
//! 字符串键 `PLACEHOLDER_METATABLE_KEY` 承载同一「全局唯一共享元表」身份，
//! 免掉业务层的裸指针哨兵。解释器内部行为（占位表识别、循环报错、缓存读写）
//! 逐项一致；仅 `debug.getregistry()` 下该内部槽的可见形态从 lightuserdata
//! 地址变为隐藏命名的字符串键。

use ulua_common::functions::c_str::cstr;
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::lua_rawiter::lua_rawiter,
  macros::{lua_l_error::luaL_error, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

use crate::functions::{
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY, push_str::c_str_prefix_owned,
  registry_table::cache_stack_value, stack_index::to_absolute,
};

/// 共享占位元表在注册表中的私有字符串键（替代 cpp 哨兵地址键，见文件头
/// `DELIBERATE DEVIATION` 说明）。
const PLACEHOLDER_METATABLE_KEY: &[u8] = b"_LUAU_CYCLIC_PLACEHOLDER_METATABLE";

/// 占位元表 `__metatable` 的锁定文案（cpp `lua_pushliteral`）；字节串切片入参，
/// 无 NUL 约定（驻留内容不含终止符）。
const METATABLE_LOCKED: &[u8] = b"The metatable is locked";
/// 两个元方法 C 闭包的调试名（cpp `debugname`，NUL 结尾静态字节串，
/// 仅在 `push_c_function` 收口点转 C 指针）。
const INDEX_ERROR_NAME: &[u8] = b"CyclicDependencyIndexError\0";
const NEW_INDEX_ERROR_NAME: &[u8] = b"CyclicDependencyNewIndexError\0";
/// 元表字段名（NUL 结尾字节串，仅 `lua_setfield` 收口点转 C 指针）。
const INDEX_FIELD: &[u8] = b"__index";
const NEW_INDEX_FIELD: &[u8] = b"__newindex";
const METATABLE_FIELD: &[u8] = b"__metatable";
/// cpp 两条报错文案的公共尾巴（避免雷同字符串逐处复制）。
const CYCLIC_TAIL: &str = " because it has a cyclic dependency on its requiring module";
/// 非字符串键在报错文案中的占位名（cpp `key ? key : "unknown"`）。
const UNKNOWN_KEY: &str = "unknown";

/// cpp `CyclicDependencyIndexError` / `CyclicDependencyNewIndexError` 的共同体：
/// 以 `to_str` 归一栈 2 键（非字符串报 `unknown`），按 `action` 文案抛错。
/// 永不返回。
fn raise_cyclic_error(l: &mut LuaState, action: &str) -> ! {
  // r16-p28 锚定形：抛错文案先取 owned 快照解耦窗口借用，其后 `l.as_mut_ptr()` 重建可用
  let key = l.to_str(2).unwrap_or(UNKNOWN_KEY).to_owned();
  luaL_error!(
    l,
    "Cannot {} the exported field '{}'{}",
    action,
    key,
    CYCLIC_TAIL
  )
}

/// cpp `CyclicDependencyIndexError`：访问占位表字段即报错。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈 1 为占位表、栈 2 为键（由 VM 的 C 闭包
/// 调用契约保证）。
unsafe extern "C-unwind" fn cyclic_dependency_index_error(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 l 为 VM 回调帧的存活独占 state，重建借用不与其他别名冲突。
  raise_cyclic_error(unsafe { &mut *l }, "access")
}

/// cpp `CyclicDependencyNewIndexError`：写入占位表字段即报错。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈 1 为占位表、栈 2 为键（由 VM 的 C 闭包
/// 调用契约保证）。
unsafe extern "C-unwind" fn cyclic_dependency_new_index_error(l: *mut LuaState) -> i32 {
  // Safety: 同 cyclic_dependency_index_error，__newindex 调用布局。
  raise_cyclic_error(unsafe { &mut *l }, "set")
}

/// cpp 元表方法登记的共同体：压入 C 闭包并设为栈顶表的字段
/// （`__index`/`__newindex` 两处同形样板的单点收口）。
///
/// `func` 须为静态存活的 VM 元方法闭包；`debug_name` 为 NUL 结尾静态字节串。
fn set_meta_method(l: &mut LuaState, field: &[u8], func: LuaCFunction, debug_name: &'static [u8]) {
  // Safety: func 静态存活（仅登记函数指针）；debug_name 为 NUL 结尾静态串，满足
  // push_c_function 闭包存活期内有效契约（净压一值，set_field_bytes 随后消费）。
  unsafe {
    l.push_c_function(func, cstr(debug_name));
  }
  l.set_field_bytes(-2, field);
}

/// cpp `pushCyclicPlaceholderMetatable`：返回共享占位元表，首次使用时创建。
/// 调用后栈净增一个值（元表）。
fn push_cyclic_placeholder_metatable(l: &mut LuaState) {
  // 命中即带表返回；未命中弹掉 nil 占位后创建。
  if l.get_field_bytes(LUA_REGISTRYINDEX, PLACEHOLDER_METATABLE_KEY) != (LuaType::Nil as i32) {
    return;
  }
  l.pop(1);

  l.new_table();
  set_meta_method(
    l,
    INDEX_FIELD,
    Some(cyclic_dependency_index_error),
    INDEX_ERROR_NAME,
  );
  set_meta_method(
    l,
    NEW_INDEX_FIELD,
    Some(cyclic_dependency_new_index_error),
    NEW_INDEX_ERROR_NAME,
  );
  l.push_bytes(METATABLE_LOCKED);
  l.set_field_bytes(-2, METATABLE_FIELD);

  // 以字符串键登记共享元表并把元表留在栈顶（对应 cpp lua_pushvalue + lua_rawsetp）。
  l.push_value(-1);
  l.set_field_bytes(LUA_REGISTRYINDEX, PLACEHOLDER_METATABLE_KEY);
}

/// 对应 cpp `lockPlaceholder`：给占位表挂共享元表并冻结（readonly）。
fn lock_placeholder(l: &mut LuaState, idx: i32) {
  // 与 cpp lua_absindex 的求值顺序一致：先归一索引，再压元表。
  let idx = to_absolute(idx, l.get_top());
  push_cyclic_placeholder_metatable(l);
  l.set_metatable(idx);
  l.set_readonly(idx, true);
}

/// cpp `createPlaceholder` / `luarequire_createplaceholder`：
/// 以栈 2 槽的 cacheKey 建占位表并写入 `_MODULES` 缓存。
/// 由 CLI `load` 在模块字节码带 `LPF_USES_EXPORT` 时调用。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，且栈 2 槽为 require 链路的 cacheKey 字符串。
pub unsafe fn luarequire_createplaceholder(l: *mut LuaState) {
  // Safety: 契约保证 l 为 CLI load 在 require 协程上调用时的存活 state，入口一次
  // 重建独占借用；cacheKey 拷为本地 Vec 后即不再借出 VM 内存。
  let l = unsafe { &mut *l };

  let cache_key = c_str_prefix_owned(l.check_bytes(2));

  l.new_table();
  lock_placeholder(l, -1);

  cache_stack_value(l, REQUIRED_CACHE_TABLE_KEY, &cache_key, -1);
  // 弹掉占位表本体：与 cpp createPlaceholder 末尾 lua_pop(L, 2) 的净效果一致
  // （cache_stack_value 内部已弹掉缓存表）。
  l.pop(1);
}

/// cpp `populatePlaceholder`：解冻占位表，raw 拷贝结果表全部字段与元表后
/// 重新冻结。
pub(crate) fn populate_placeholder(l: &mut LuaState, placeholder_idx: i32, result_idx: i32) {
  let top = l.get_top();
  let placeholder_idx = to_absolute(placeholder_idx, top);
  let result_idx = to_absolute(result_idx, top);

  // 解冻后才能写入占位表
  l.set_readonly(placeholder_idx, false);

  // 迭代游标是纯 i32：每次非 -1 返回时键值两槽已压栈（key 在 -2、value 在 -1），
  // raw_set 消费它们，栈回到循环入口形态。
  // Safety: lua_rawiter 按 vm C-API 约定在存活 state 上迭代 result_idx 处的表，
  // 索引为调用方契约保证的同一帧表槽，无指针解引用。
  let mut cursor = 0;
  loop {
    cursor = lua_rawiter(l, result_idx, cursor);
    if cursor < 0 {
      break;
    }
    l.raw_set(placeholder_idx);
  }

  // 拷贝结果的元表（若有）：无元表时补 nil 占位，保证 setmetatable 总有值可消费
  if !l.get_metatable(result_idx) {
    l.push_nil();
  }
  l.set_metatable(placeholder_idx);

  // 冻结填充完毕的占位表
  l.set_readonly(placeholder_idx, true);
}

/// cpp `isCached` 中"缓存值是否占位表"的判定：元表 === 登记的共享表。
/// 调用后栈高度不变（内部压弹配平）。
pub(crate) fn is_placeholder_at(l: &mut LuaState, idx: i32) -> bool {
  if !l.get_metatable(idx) {
    return false;
  }
  l.get_field_bytes(LUA_REGISTRYINDEX, PLACEHOLDER_METATABLE_KEY);
  let is_placeholder = l.raw_equal(-1, -2);
  l.pop(2); // 弹出两个元表
  is_placeholder
}
