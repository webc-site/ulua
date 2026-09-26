use core::{
  ffi::{c_char, c_void},
  ptr::{NonNull, null},
};

use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_error::lua_error, lua_gettop::lua_gettop, lua_settop::lua_settop,
    lua_tolightuserdata::lua_tolightuserdata, lua_tolstring::lua_tolstring_ref,
    lua_touserdata::lua_touserdata, lua_yield::lua_yield,
  },
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error,
    lua_upvalueindex::lua_upvalueindex,
  },
  records::lua_state::LuaState,
};

use crate::{
  enums::status_require_impl::Status,
  functions::{
    c_str_prefix::push_c_str,
    check_registered_modules::check_registered_modules,
    lua_requirecont::{K_REQUIRE_STACK_VALUES, lua_requirecont},
    resolve_require::resolve_require,
  },
  records::luarequire_configuration::luarequire_Configuration,
};

/// cpp `lua_requireinternal` 取配置的三步样板（`lua_touserdata` → cast → 判空 →
/// 解引用）在本 crate 的唯一收口：`NonNull` 证明非空后才重建共享引用，缺失返回 `None`
/// 交由调用方 `luaL_error` 发散；返回的引用只用于读取回调字段（配置初始化后无人再写）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`idx` 处须是 `pushrequireclosureinternal` placement
/// 构造、被闭包 upvalue 持有（与闭包同寿命）的 `luarequire_Configuration` userdata。
unsafe fn borrowed_configuration<'cfg>(
  l: *mut LuaState,
  idx: i32,
) -> Option<&'cfg luarequire_Configuration> {
  // Safety: 契约保证 idx 处是与 `l` 同帧存活的配置 userdata（lua_touserdata 只取该槽地址、
  // 不移动栈），`Option` 已证非空后即指向已初始化对象，可安全重建共享引用
  unsafe {
    lua_touserdata(l, idx).map(|r| NonNull::from(r).cast::<luarequire_Configuration>().as_ref())
  }
}

/// `resolve_and_push` 的阶段结果：驱动主链决定直接返回、抛错还是继续装载。
enum ResolvePhase {
  /// 命中缓存，缓存值已留在栈顶，调用方直接 `return 1`
  Cached,
  /// 导航/解析报错，错误串已压栈，调用方须 `lua_error` 发散
  ErrorReported,
  /// 解析成功，cacheKey/chunkname/loadname 三段已压栈，继续装载
  Resolved,
}

/// cpp `lua_requireinternal` 的开栈阶段：把栈归一为 1 个实参，取出配置共享引用、
/// 转手 ctx 与 path（NUL 结尾串指针）及其字节视图。返回值即 cpp 序幕的四个局部量
/// （`config`/`ctx`/`path`/`std::string path`），tuple 形态避免为四个 Copy 值造记录。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；upvalue(1)/(2) 须分别为
/// `pushrequireclosureinternal` 建立的配置 userdata 与 ctx，栈顶为 require 路径
/// 参数（由 C 闭包调用约定保证）。
unsafe fn open_require_frame<'cfg>(
  l: *mut LuaState,
) -> (
  &'cfg luarequire_Configuration,
  *mut c_void,
  *const c_char,
  &'cfg [u8],
) {
  // Safety: 纯栈操作，把闭包帧归一为 require 路径 1 个实参（对应 cpp lua_settop(L, 1)）。
  unsafe { lua_settop(l, 1) };

  // Safety: borrowed_configuration 按自身契约以 `NonNull` 证明非空后构造只读借用，
  // 缺失即 luaL_error! 发散；ctx 仅作转手存储的 lightuserdata 裸指针（本 crate 不
  // 解引用）；path 由 luaL_checkstring! 校验为字符串并返回 NUL 结尾有效指针，
  // path_bytes 对应 cpp `std::string path(luaL_checkstring(L, 1))`：经 cstr_bytes 门面
  // 取首个 NUL 前的纯字节，零拷贝不校验 UTF-8（旧 to_string_lossy 会换 U+FFFD）。
  unsafe {
    let Some(config) = borrowed_configuration(l, lua_upvalueindex(1)) else {
      luaL_error!(l, "unable to find require configuration");
    };
    let ctx = lua_tolightuserdata(l, lua_upvalueindex(2));
    let path = luaL_checkstring!(l, 1);
    let path_bytes = cstr_bytes(path);

    (config, ctx, path, path_bytes)
  }
}

/// cpp `lua_requireinternal` 的解析阶段：`resolve_require` 后按其状态把结果三段（或
/// 错误串）压栈，返回控制流信号。栈副作用与被拆前的内联块逐字一致。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`config`/`ctx`/两条字节串由开栈阶段
/// （[`open_require_frame`]）在同一帧内备好，本阶段仅转手传给 `resolve_require`
/// 与 `push_c_str`。
unsafe fn resolve_and_push(
  config: &luarequire_Configuration,
  l: *mut LuaState,
  ctx: *mut c_void,
  requirer_chunkname: &[u8],
  path_bytes: &[u8],
) -> ResolvePhase {
  // Safety: 全部栈副作用来自 resolve_require（命中时值留栈顶）与 push_c_str（推本地字节串），
  // 二者按各自契约操作 l；本函数不解引用任何裸指针。
  let resolved_require = unsafe { resolve_require(config, l, ctx, requirer_chunkname, path_bytes) };

  match resolved_require.status {
    // cpp 命中缓存路径：is_cached 已把值留在栈顶，无需再压
    Status::Cached => ResolvePhase::Cached,
    Status::ErrorReported => {
      unsafe { push_c_str(l, &resolved_require.error) };
      ResolvePhase::ErrorReported
    }
    _ => {
      unsafe {
        push_c_str(l, &resolved_require.cache_key);
        push_c_str(l, &resolved_require.chunkname);
        push_c_str(l, &resolved_require.loadname);
      };
      ResolvePhase::Resolved
    }
  }
}

/// `load_or_yield` 的阶段结果：`Yielded` 携带 `lua_yield` 返回值直接回给调用方，
/// `Loaded` 表示同步装载完成、交回 continuation 收尾。
enum LoadOutcome {
  Yielded(i32),
  Loaded,
}

/// cpp `lua_requireinternal` 的装载阶段：取栈上 chunkname/loadname 的 C 视图，经
/// `config.load` 回调装载；回调返回 -1 表示协程挂起，校验栈未被改动后 `lua_yield`，
/// 否则交回调用方接续 continuation。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState` 且栈为 `K_REQUIRE_STACK_VALUES` 布局（-2/-1 为
/// chunkname/loadname、其下为 cacheKey）；`ctx`/`path` 为同帧存活的 lightuserdata 与
/// NUL 结尾 VM 串指针，仅转手传给 `config.load`（validate_config 确认存在的回调）。
unsafe fn load_or_yield(
  l: *mut LuaState,
  config: &luarequire_Configuration,
  ctx: *mut c_void,
  path: *const c_char,
  stack_values: i32,
) -> LoadOutcome {
  // Safety: l 存活且 -2/-1 为字符串槽（resolve 阶段刚压入），`lua_tolstring_ref`
  // 返回指向被栈槽持有的 VM 串的切片，取首字节地址即 NUL 结尾指针（VM 串恒有终止
  // NUL）；两调用均为只读栈访问，不占栈位。`None`（不可达）折算回 null，与旧形态
  // 经 C 出参交出的指针一致——本调用不消费长度，收口后连 len sink 都不再需要。
  let (chunkname, loadname) = unsafe {
    (
      lua_tolstring_ref(l, -2).map_or(null(), |s| s.as_ptr().cast::<c_char>()),
      lua_tolstring_ref(l, -1).map_or(null(), |s| s.as_ptr().cast::<c_char>()),
    )
  };

  let Some(load) = config.load else {
    // Safety: l 存活，luaL_error! 抛错发散（load 存在性已由 validate_config 前置校验，
    // 此分支为防御性收口）
    unsafe {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: load"
      )
    };
  };

  // Safety: config.load 的三个串参（path/chunkname/loadname）在本帧调用窗口内均存活，
  // ctx 为转手存储的 lightuserdata 裸指针（本函数不解引用）。
  let num_results = unsafe { load(l.cast(), ctx, path, chunkname, loadname) };

  if num_results == -1 {
    // Safety: lua_gettop 是纯栈读；挂起路径先复核栈未被改动（不一致即 luaL_error!
    // 发散），lua_yield 由协程状态机接续。
    unsafe {
      if lua_gettop(l) != stack_values {
        luaL_error!(l, "stack cannot be modified when require yields");
      }
      LoadOutcome::Yielded(lua_yield(l, 0))
    }
  } else {
    LoadOutcome::Loaded
  }
}

/// cpp `lua_requireinternal` 的解析阶段：先查已注册模块缓存（命中即与解析出
/// 缓存同一落点：值留在栈顶、调用方直接 `return 1`），未命中再走
/// `resolve_and_push`。
///
/// # Safety
/// 与 [`resolve_and_push`] 同：`l` 指向存活 `LuaState`，`config`/`ctx`/两条字节串
/// 由开栈阶段在同一帧内备好，本阶段仅转手传给各门面。
unsafe fn resolve_or_cached(
  config: &luarequire_Configuration,
  l: *mut LuaState,
  ctx: *mut c_void,
  requirer_chunkname: &[u8],
  path_bytes: &[u8],
) -> ResolvePhase {
  // Safety: check_registered_modules 收口在 registry_table::cache_hit 门面（按其契约
  // 命中时值留栈顶），未命中自行配平；其后 resolve_and_push 全栈副作用按自身契约。
  unsafe {
    if check_registered_modules(l, path_bytes) {
      return ResolvePhase::Cached;
    }
    resolve_and_push(config, l, ctx, requirer_chunkname, path_bytes)
  }
}

/// `lua_requireinternal` 的装载收尾阶段（cpp 主链尾）：复核 resolve 后的栈高，
/// 交 `config.load` 装载并接续 continuation。
///
/// # Safety
/// `l` 指向存活 `LuaState` 且 resolve 阶段已完成（栈为 K_REQUIRE_STACK_VALUES 布局）；
/// `config`/`ctx`/`path` 为同帧开栈阶段备好的只读配置、转手 lightuserdata 与
/// NUL 结尾串指针。
unsafe fn load_frame_and_continue(
  l: *mut LuaState,
  config: &luarequire_Configuration,
  ctx: *mut c_void,
  path: *const c_char,
) -> i32 {
  // Safety: lua_gettop/LUAU_ASSERT 是纯栈读复核；load_or_yield/lua_requirecont
  // 各按自身契约操作本帧栈。
  unsafe {
    let stack_values = lua_gettop(l);
    ulua_common::LUAU_ASSERT!(stack_values == K_REQUIRE_STACK_VALUES);

    match load_or_yield(l, config, ctx, path, stack_values) {
      LoadOutcome::Yielded(ret) => ret,
      LoadOutcome::Loaded => lua_requirecont(l, LuaStatus::Ok as i32),
    }
  }
}

/// # Safety
/// `l` 必须指向存活的 `LuaState`；upvalue(1)/(2) 须分别为配置指针与 ctx，
/// 栈顶为 require 路径参数（由 C 闭包调用约定保证）。
/// `requirer_chunkname` 为 requirer chunkname 字节串，由真 FFI 入口经 `cstr_bytes`
/// 门面一次性取得。
pub(crate) unsafe fn lua_requireinternal(l: *mut LuaState, requirer_chunkname: &[u8]) -> i32 {
  // Safety: l 是 VM 调 require 闭包时传入的当前有效状态（fn /// # Safety）；开栈阶段
  // （open_require_frame）内部各步按其契约取配置/ctx/path 并转字节串；
  // resolve_or_cached 与 load_frame_and_continue 两阶段各自带契约、内部只操作 VM 栈，
  // lua_error 由 VM 状态机接续。
  unsafe {
    let (config, ctx, path, path_bytes) = open_require_frame(l);

    match resolve_or_cached(config, l, ctx, requirer_chunkname, path_bytes) {
      ResolvePhase::Cached => return 1,
      ResolvePhase::ErrorReported => lua_error(l),
      ResolvePhase::Resolved => {}
    }

    load_frame_and_continue(l, config, ctx, path)
  }
}
