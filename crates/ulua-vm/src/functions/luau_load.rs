use core::{ffi::c_void, ptr::null_mut};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_rawrunprotected_ldo::lua_d_rawrunprotected, lua_pushstring::lua_pushstring},
  macros::{lua_c_check_gc::lua_c_check_gc, lua_memerrmsg::LUA_MEMERRMSG},
  methods::load_context_run::LoadContextRun,
  records::{
    load_context::LoadContext, lua_state::LuaState, scoped_set_gc_threshold::ScopedSetGcThreshold,
    temp_buffer::TempBuffer,
  },
};

/// # Safety
/// 输入字节流缓冲与读游标必须位于本次加载的串数据界内，长度参数覆盖所有被读字段。
// trait 方法不能声明 `extern "C"`,以 C ABI 包装转发(供 Pfunc 回调使用)
unsafe extern "C-unwind" fn load_context_run(l: *mut LuaState, ud: *mut c_void) {
  // Safety: `LoadContext::run` 以调用者契约保证的存活 `l` 与非空 `ud`（字节码 LoadInfo）执行
  unsafe { <LoadContext<'_> as LoadContextRun>::run(l, ud) }
}

/// 反序列化字节码并压入可调用原型。
///
/// `chunkname` 为源名（cpp 同款 `=`/`@` 前缀约定，长度即 `.len()`，无 NUL 依赖）；
/// `data` 为字节码切片，长度由切片自身携带。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，且满足 C++ 参考实现（lvmload.cpp `lua_load`）的前置条件。
pub unsafe fn luau_load(l: *mut LuaState, chunkname: &str, data: &[u8], env: i32) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧、输入串数据全程可读；反序列化期间 GC 暂停，新建对象暂未挂根仍属本调用可达
  unsafe {
    // we will allocate a fair amount of memory so check GC before we do
    lua_c_check_gc!(l);

    // pause GC for the duration of deserialization - some objects we're creating aren't rooted
    let mut pause_gc = ScopedSetGcThreshold {
      global: null_mut(),
      original_threshold: 0,
    };
    pause_gc.scoped_set_gc_threshold_global_state_usize((*l).global, usize::MAX);

    let mut ctx = LoadContext {
      strings: TempBuffer::new(),
      protos: TempBuffer::new(),
      chunkname,
      data,
      env,
      result: 0,
    };

    let status = lua_d_rawrunprotected(
      l,
      Some(load_context_run),
      &mut ctx as *mut LoadContext<'_> as *mut c_void,
    );

    // load 只能成功或 OOM，其它错误都由 loadsafe 内部收口
    let result = if status == LuaStatus::ErrMem as i32 {
      lua_pushstring(l, LUA_MEMERRMSG.as_ptr().cast());
      1
    } else {
      load_return_code(status, ctx.result)
    };

    drop(pause_gc);

    result
  }
}

/// `lua_d_rawrunprotected` 的 status 到 `luau_load` 返回码的映射。
///
/// 上游 `cpp/VM/src/lvmload.cpp:832` 只有一句
/// `LUAU_ASSERT(status == LUA_OK || status == LUA_ERRMEM)`，release 编译掉后就变成
/// 「非 ERRMEM 一律取 `ctx.result`」，而 `ctx.result` 的初值是 0（成功）。上游成立
/// 的前提是反序列化除 OOM 外不抛任何东西；Rust 侧 `lua_d_rawrunprotected` 用
/// `catch_unwind` 收口，任何逃逸 panic 都会被映射成 `ErrRun`（并由
/// `luaG_pusherror` 把消息压栈），此时照抄上游会把「栈顶是错误串」的调用伪装成
/// 成功返回 0。
///
/// 因此只承认 `Ok` 一种 status 可以沿用 loadsafe 的结论；其余 status（`ErrRun`、
/// 以及理论上可达的 `Yield`/`ErrSyntax`/`ErrErr`/`Break`）都按错误返回 1，
/// 与 `luau_load` 既有的「非 0 即栈顶为错误串」契约同形态，不新增错误码。
/// 逃逸 panic 在 `catch_unwind` 前已由默认 panic hook 打到 stderr，
/// 无需在此再断一次。
fn load_return_code(status: i32, result: i32) -> i32 {
  if status == LuaStatus::Ok as i32 {
    result
  } else {
    1
  }
}

// 留证：`load_return_code` 是私有状态码映射胶水（cpp lvmload.cpp 无对应导出面），
// 「非 Ok/ErrMem 的 status 不得回落到 ctx.result 初值 0」的失败模式只有 panic
// 逃逸路径能触发，公开 `luau_load` 入口（含 tests/load_malformed.rs）无从构造
// ErrRun/Break 等 status，外测退化为只重测 Ok 分支。
#[cfg(test)]
mod tests {
  use super::*;

  /// 只有 `Ok` 才允许把 loadsafe 的结论透传给调用方
  #[test]
  fn load_return_code_only_follows_ok() {
    assert_eq!(load_return_code(LuaStatus::Ok as i32, 0), 0);
    assert_eq!(load_return_code(LuaStatus::Ok as i32, 1), 1);
  }

  /// 非 `Ok`/`ErrMem` 的 status（逃逸 panic 收口成 `ErrRun`）不得回落到
  /// `ctx.result` 的初值 0，必须以错误码返回
  #[test]
  fn load_return_code_rejects_unverified_status() {
    for status in [
      LuaStatus::Yield,
      LuaStatus::ErrRun,
      LuaStatus::ErrSyntax,
      LuaStatus::ErrErr,
      LuaStatus::Break,
    ] {
      assert_eq!(
        load_return_code(status as i32, 0),
        1,
        "{status:?} 不能当成功返回"
      );
    }
  }

  /// status 不在枚举取值内（脏数据）同样按错误处理
  #[test]
  fn load_return_code_rejects_unknown_status() {
    assert_eq!(load_return_code(42, 0), 1);
    assert_eq!(load_return_code(-1, 0), 1);
  }
}
