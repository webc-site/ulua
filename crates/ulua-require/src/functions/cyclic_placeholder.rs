//! cpp `Require/src/RequireImpl.cpp` 中循环依赖占位表一段的移植：
//! 共享占位元表（`__index`/`__newindex` 抛循环依赖错误）、`createPlaceholder`、
//! `lockPlaceholder`、`populatePlaceholder`，以及 `_CYCLIC_PLACEHOLDER_PROVIDED`
//! 标记的读写辅助。
//!
//! 本文件仅存真正的 `ulua-vm` C-API 边界裸指针：`LuaState` 不透明句柄（vm 全
//! crate 即此形态）、`lua_rawgetp`/`lua_rawsetp` 的注册表哨兵键（不透明地址身份）、
//! `extern "C-unwind"` 元方法闭包、边界上的 C 串指针互转。
//! 字节表键上的 findtable/getfield/setfield 三步样板一律经 `registry_table` 门面，
//! 本文件不再出现 `with_c_str`。
//! 每处均带 `# Safety` 契约；业务层不设自造指针。

use alloc::borrow::Cow;
use core::{
  ffi::{c_int, c_void},
  ptr::addr_of,
};

use ulua_common::functions::c_str::{cstr_bytes, cstr_cow};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_getmetatable::lua_getmetatable, lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean,
    lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue, lua_rawequal::lua_rawequal,
    lua_rawiter::lua_rawiter, lua_rawset::lua_rawset, lua_setfield::lua_setfield,
    lua_setmetatable::lua_setmetatable, lua_setreadonly::lua_setreadonly,
    lua_toboolean::lua_toboolean,
  },
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error, lua_newtable::lua_newtable,
    lua_pop::lua_pop, lua_pushcfunction::LUA_PUSHCFUNCTION, lua_pushliteral::lua_pushliteral,
    lua_rawgetp::lua_rawgetp, lua_rawsetp::lua_rawsetp, lua_registryindex::LUA_REGISTRYINDEX,
    lua_tostring::lua_tostring,
  },
  records::lua_state::LuaState,
};

use crate::functions::{
  cache_table_keys::{CYCLIC_PLACEHOLDER_PROVIDED_KEY, REQUIRED_CACHE_TABLE_KEY},
  registry_table::{get_table_field, push_registry_table, set_table_field},
  stack_index::to_absolute,
};

/// cpp `static char cyclicPlaceholderMetatableSentinel`：仅取其唯一地址作
/// 注册表键，值本身从不读写。
static CYCLIC_PLACEHOLDER_METATABLE_SENTINEL: u8 = 0;

/// 占位元表 `__metatable` 的锁定文案（cpp `lua_pushliteral`）；字节串切片入参，
/// 无 NUL 约定（驻留内容不含终止符）。
const METATABLE_LOCKED: &[u8] = b"The metatable is locked";
/// 两个元方法 C 闭包的调试名（cpp `debugname`，NUL 结尾字节串，
/// 仅在 `LUA_PUSHCFUNCTION` 收口点转 `*const c_char`）。
const INDEX_ERROR_NAME: &[u8] = b"CyclicDependencyIndexError\0";
const NEW_INDEX_ERROR_NAME: &[u8] = b"CyclicDependencyNewIndexError\0";
/// 元表字段名（NUL 结尾字节串，仅 `lua_setfield` 收口点转 C 指针）。
const INDEX_FIELD: &[u8] = b"__index\0";
const NEW_INDEX_FIELD: &[u8] = b"__newindex\0";
const METATABLE_FIELD: &[u8] = b"__metatable\0";
/// cpp 两条报错文案的公共尾巴（避免雷同字符串逐处复制）。
const CYCLIC_TAIL: &str = " because it has a cyclic dependency on its requiring module";
/// 非字符串键在报错文案中的占位名（cpp `key ? key : "unknown"`）。
const UNKNOWN_KEY: &str = "unknown";

/// 占位元表哨兵指针（cpp `&cyclicPlaceholderMetatableSentinel`）。
///
/// 本 crate 唯一的注册表键裸指针真边界：`lua_rawgetp`/`lua_rawsetp` 的键类型
/// 即不透明地址指针，只使用地址身份、从不解引用或经它写入，无纯 Rust
/// 替代（改字符串键会与用户经 `debug.getregistry()` 可见的 registry 键面冲突）。
/// 函数体本身是安全代码。
fn sentinel_ptr() -> *mut c_void {
  addr_of!(CYCLIC_PLACEHOLDER_METATABLE_SENTINEL)
    .cast_mut()
    .cast()
}

/// cpp `CyclicDependencyIndexError` / `CyclicDependencyNewIndexError` 的共同体：
/// 以 `lua_tostring` 归一栈 2 键（非字符串报 `unknown`），按 `action` 文案抛错。
/// 永不返回。
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈 1 为占位表、栈 2 为键（VM 调
/// `__index`/`__newindex` 元方法闭包的入栈约定）。
unsafe fn raise_cyclic_error(l: *mut LuaState, action: &str) -> ! {
  // Safety: l 是 VM 调元方法闭包时传入的存活 state；lua_tostring 对字符串槽返回
  // 有效 NUL 结尾指针、其余类型返回 null（块内判空后走兜底文案），非空分支的
  // cstr_cow 借用仅在本帧即时消费（格式化进错误消息），不跨后续栈操作存活。
  let key = unsafe {
    let key = lua_tostring!(l, 2);
    if key.is_null() {
      Cow::Borrowed(UNKNOWN_KEY)
    } else {
      cstr_cow(key)
    }
  };
  // Safety: l 为存活 state；luaL_error! 把 key 格式化进错误消息后抛错发散。
  unsafe {
    luaL_error!(
      l,
      "Cannot {} the exported field '{}'{}",
      action,
      key,
      CYCLIC_TAIL
    )
  }
}

/// cpp `CyclicDependencyIndexError`：访问占位表字段即报错。
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈 1 为占位表、栈 2 为键（由 VM 的 C 闭包
/// 调用契约保证）。
unsafe extern "C-unwind" fn cyclic_dependency_index_error(l: *mut LuaState) -> c_int {
  // Safety: l 满足 raise_cyclic_error 的 /// # Safety（__index 调用布局），且其
  // 抛错发散、不返回。
  unsafe { raise_cyclic_error(l, "access") }
}

/// cpp `CyclicDependencyNewIndexError`：写入占位表字段即报错。
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈 1 为占位表、栈 2 为键（由 VM 的 C 闭包
/// 调用契约保证）。
unsafe extern "C-unwind" fn cyclic_dependency_new_index_error(l: *mut LuaState) -> c_int {
  // Safety: 同 cyclic_dependency_index_error，__newindex 调用布局。
  unsafe { raise_cyclic_error(l, "set") }
}

/// cpp 元表方法登记的共同体：压入 C 闭包并设为栈顶表的字段
/// （`__index`/`__newindex` 两处同形样板的单点收口）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState` 且栈顶为表；`field`/`debug_name` 为静态
/// NUL 结尾串，`func` 静态存活（与 cpp 元方法注册同形态）。
unsafe fn set_meta_method(
  l: *mut LuaState,
  field: &[u8],
  func: unsafe extern "C-unwind" fn(*mut LuaState) -> c_int,
  debug_name: &[u8],
) {
  // Safety: l 与栈顶表由契约保证；LUA_PUSHCFUNCTION 只登记函数指针（静态存活），
  // lua_setfield 在调用内读完键内容并消费栈顶闭包，栈净变化 0。
  // field/debug_name 为 NUL 结尾静态字节串，仅在 C API 收口点转裸指针。
  unsafe {
    LUA_PUSHCFUNCTION(l, Some(func), debug_name.as_ptr().cast());
    lua_setfield(l, -2, field.as_ptr().cast());
  }
}

/// cpp `pushCyclicPlaceholderMetatable`：返回共享占位元表，首次使用时创建。
/// # Safety
/// `l` 必须指向存活的 `LuaState`；调用后栈上净增一个值（元表）。
unsafe fn push_cyclic_placeholder_metatable(l: *mut LuaState) {
  // Safety: l 为契约保证的存活 state；sentinel_ptr() 指向静态 u8 的地址（对齐/非空
  // 平凡成立），仅当注册表纯键使用、不解引用。命中即带表返回，未命中弹掉 nil 占位。
  unsafe {
    if lua_rawgetp(l, LUA_REGISTRYINDEX, sentinel_ptr()) != (LuaType::Nil as c_int) {
      return;
    }
    lua_pop(l, 1);
  }

  // Safety: l 存活；lua_newtable 压出新表，set_meta_method 按其自身契约登记
  // __index 元方法，栈顶恒为本元表。
  unsafe {
    lua_newtable(l);
    set_meta_method(
      l,
      INDEX_FIELD,
      cyclic_dependency_index_error,
      INDEX_ERROR_NAME,
    );
  }
  // Safety: 同上，登记 __newindex 与 __metatable 锁定文案；两串为静态数据，
  // 字段写入全经 VM API，结束后栈顶仍是元表。
  unsafe {
    set_meta_method(
      l,
      NEW_INDEX_FIELD,
      cyclic_dependency_new_index_error,
      NEW_INDEX_ERROR_NAME,
    );
    lua_pushliteral(l, METATABLE_LOCKED);
    lua_setfield(l, -2, METATABLE_FIELD.as_ptr().cast());
  }

  // Safety: 复制栈顶元表后以哨兵地址（纯键用途，不解引用）写入注册表，栈配平。
  unsafe {
    lua_pushvalue(l, -1);
    lua_rawsetp!(l, LUA_REGISTRYINDEX, sentinel_ptr());
  }
}

/// 对应 cpp `lockPlaceholder`：给占位表挂共享元表并冻结（readonly）。
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`idx` 处必须是表；调用后栈操作配平。
unsafe fn lock_placeholder(l: *mut LuaState, idx: c_int) {
  // Safety: l 与 idx 由 /// # Safety 保证；lua_gettop 在压栈前取栈顶（与 cpp
  // lua_absindex 的求值顺序一致——先归一索引，再压元表），
  // push_cyclic_placeholder_metatable 净压一元表后 lua_setmetatable/lua_setreadonly
  // 消费之，栈配平；无指针解引用。
  unsafe {
    let idx = to_absolute(idx, lua_gettop(l));
    push_cyclic_placeholder_metatable(l);
    lua_setmetatable(l, idx);
    lua_setreadonly(l, idx, 1);
  }
}

/// cpp `createPlaceholder` / `luarequire_createplaceholder`：
/// 以栈 2 槽的 cacheKey 建占位表并写入 `_MODULES` 缓存。
/// 由 CLI `load` 在模块字节码带 `LPF_USES_EXPORT` 时调用。
/// # Safety
/// `l` 必须指向存活的 `LuaState`，且栈 2 槽为 require 链路的 cacheKey 字符串。
pub unsafe fn luarequire_createplaceholder(l: *mut LuaState) {
  // Safety: l 为 CLI load 在 require 协程上调用时的存活 state；luaL_checkstring 对
  // 栈 2 的 cacheKey 校验类型并返回 NUL 结尾有效指针（否则抛错发散），真 FFI 入口
  // 经 cstr_bytes 门面一次性转字节串（读取发生在下次压栈前、串被调用帧保活）；
  // 其余栈操作走 VM API 与 registry_table 门面。
  unsafe {
    let cache_key = cstr_bytes(luaL_checkstring!(l, 2));

    lua_newtable(l);
    lock_placeholder(l, -1);

    push_registry_table(l, REQUIRED_CACHE_TABLE_KEY);
    lua_pushvalue(l, -2);
    set_table_field(l, -2, cache_key);
    lua_pop(l, 2);
  }
}

/// cpp `populatePlaceholder`：解冻占位表，raw 拷贝结果表全部字段与元表后
/// 重新冻结。
/// # Safety
/// `l` 必须指向存活的 `LuaState`；两索引处必须是表（占位表与模块结果表）。
pub(crate) unsafe fn populate_placeholder(
  l: *mut LuaState,
  placeholder_idx: c_int,
  result_idx: c_int,
) {
  // Safety: l/两索引由契约保证为存活 state 上的表槽；lua_gettop 是纯栈读，两索引
  // 在压栈前经同一栈顶归一（cpp 逐次 lua_absindex 之间同样无压弹，栈高一致故等价）。
  let top = unsafe { lua_gettop(l) };
  let placeholder_idx = to_absolute(placeholder_idx, top);
  let result_idx = to_absolute(result_idx, top);

  // Safety: 解冻后才能写入占位表；lua_setreadonly 只操作契约保证的表槽。
  unsafe { lua_setreadonly(l, placeholder_idx, 0) };

  // Safety: `lua_rawiter` 的迭代游标是纯 c_int、不携带指针；每次非 -1 返回时
  // 键值两槽已压栈（key 在 -2、value 在 -1），rawset 消费它们，栈回到循环入口形态。
  unsafe {
    let mut iter = 0;
    loop {
      iter = lua_rawiter(l, result_idx, iter);
      if iter == -1 {
        break;
      }
      lua_rawset(l, placeholder_idx);
    }
  }

  // Safety: 拷贝结果的元表（若有）——getmetatable 返回 0 时未压值需补 nil，
  // 否则值已在栈顶；setmetatable 消费栈顶值，栈配平。
  unsafe {
    if lua_getmetatable(l, result_idx) == 0 {
      lua_pushnil(l);
    }
    lua_setmetatable(l, placeholder_idx);
  }

  // Safety: 冻结填充完毕的占位表；契约保证的表槽上的纯 VM 栈操作。
  unsafe { lua_setreadonly(l, placeholder_idx, 1) };
}

/// cpp `isCached` 中"缓存值是否占位表"的判定：元表 === 哨兵注册的共享表。
/// 调用后栈高度不变（内部压弹配平）。
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`idx` 处为已入栈的值。
pub(crate) unsafe fn is_placeholder_at(l: *mut LuaState, idx: c_int) -> bool {
  // Safety: l 与 idx 由契约保证（idx 处已入栈值）；getmetatable/rawgetp/rawequal/pop
  // 仅栈操作；哨兵指针为静态生存期 u8 地址，纯键用途不解引用。
  unsafe {
    if lua_getmetatable(l, idx) != 1 {
      return false;
    }
    lua_rawgetp(l, LUA_REGISTRYINDEX, sentinel_ptr());
    let is_placeholder = lua_rawequal(l, -1, -2) == 1;
    lua_pop(l, 2); // 弹出两个元表
    is_placeholder
  }
}

/// cpp `isCached` 中的 provided 标记：`_CYCLIC_PLACEHOLDER_PROVIDED[cacheKey] = true`，
/// 表示占位表已实际交给循环 require 方。
/// # Safety
/// `l` 必须指向存活的 `LuaState`；栈操作配平。
pub(crate) unsafe fn mark_placeholder_provided(l: *mut LuaState, cache_key: &[u8]) {
  // Safety: l 为存活 state；provided_key 由 registry_table 门面按其内容补 NUL、
  // 仅门面闭包内存活，cache_key 则以纯字节形态入栈（findtable/pushboolean/
  // set_table_field/pop 栈配平）。
  unsafe {
    push_registry_table(l, CYCLIC_PLACEHOLDER_PROVIDED_KEY);
    lua_pushboolean(l, 1);
    set_table_field(l, -2, cache_key);
    lua_pop(l, 1);
  }
}

/// cpp `lua_requirecont` 开头：读取并清除 `_CYCLIC_PLACEHOLDER_PROVIDED[cacheKey]`。
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`cache_key` 为 cacheKey 字节串（门面按纯字节入栈）；栈操作配平。
pub(crate) unsafe fn consume_placeholder_provided(l: *mut LuaState, cache_key: &[u8]) -> bool {
  // Safety: l 为存活 state；cache_key 是调用方（lua_requirecont）本帧字节串的借用，
  // registry_table 门面把它按 ptr+len 形态即时压栈并消费，无跨调用指针；
  // 读值与弹栈在本块内配平，注册表留在栈顶交下方清标记块。
  let was_provided = unsafe {
    // 检查占位表是否已实际交给某个循环 require 方
    push_registry_table(l, CYCLIC_PLACEHOLDER_PROVIDED_KEY);
    get_table_field(l, -1, cache_key);
    let was_provided = lua_toboolean(l, -1) != 0;
    lua_pop(l, 1);
    was_provided
  };

  // Safety: 同上，cache_key 以纯字节形态入栈即时消费；pushnil/set_table_field/pop
  // 栈配平，结束后注册表弹空。
  unsafe {
    // 清掉 provided 标记，复位状态
    lua_pushnil(l);
    set_table_field(l, -2, cache_key);
    lua_pop(l, 1);
  }

  was_provided
}

/// cpp `lua_requirecont` provided 分支"取回缓存里的占位表"：结束后栈顶依次为
/// `_MODULES` 表与 `_MODULES[cacheKey]`（占位表）。
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`cache_key` 为字节串；净增两个栈槽。
pub(crate) unsafe fn push_cached_placeholder(l: *mut LuaState, cache_key: &[u8]) {
  // Safety: l 为存活 state；REQUIRED_CACHE_TABLE_KEY 由 registry_table 门面补 NUL 且
  // 仅门面闭包内使用，cache_key 以纯字节形态入栈；findtable + gettable 净增两槽由
  // 契约声明。
  unsafe {
    push_registry_table(l, REQUIRED_CACHE_TABLE_KEY);
    get_table_field(l, -1, cache_key);
  }
}
