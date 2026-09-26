//! `luaD_rawrunprotected` 三臂集成测试（对照 cpp `ldo.cpp:124-159` 的
//! catch 结构，finding T2）：
//!
//! 1. `lua_exception` 载荷（`lua_d_throw` 的 longjmp 模拟）→ 原样恢复
//!    status，不触碰栈；
//! 2. `&str` / `String` 载荷（Rust 侧外部函数逸出的 panic，对应 cpp
//!    `catch (std::exception&)` 臂）→ `ErrRun` + 消息字符串压栈；
//! 3. 非字符串载荷 → `ErrRun` + `"unknown error"`；载荷含内嵌 NUL 时
//!    退到编译期 `const CStr` 兜底文案 `"invalid error message"`。

use core::{ffi::c_void, ptr::null_mut};
use std::{
  panic::{self, panic_any},
  ptr,
  sync::Once,
};

use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{
    lua_close::lua_close, lua_d_rawrunprotected_ldo::lua_d_rawrunprotected,
    lua_d_throw_ldo::lua_d_throw, lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate,
    lua_settop::lua_settop, lua_tolstring::lua_tolstring, lua_type::lua_type,
  },
  records::lua_state::LuaState,
};

/// 合成 panic 的消息前缀：本测试二进制内的静默钩子按此前缀过滤，
/// 避免受控 panic 在 stderr 制造噪音（真实 panic 仍照常打印）。
const SYNTHETIC: &str = "ulua_t2_synthetic";

/// 一次性安装「合成 panic 静默」钩子。必须早于首次
/// `install_lua_exception_panic_hook`（其链式包装取前一钩子）时生效；
/// 若竞争失败仅影响 stderr 噪音，不影响断言。
fn install_quiet_hook() {
  static ONCE: Once = Once::new();
  ONCE.call_once(|| {
    let prev = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
      let hit = |s: &str| s.starts_with(SYNTHETIC);
      let synthetic = info
        .payload()
        .downcast_ref::<&str>()
        .is_some_and(|s| hit(s))
        || info
          .payload()
          .downcast_ref::<String>()
          .is_some_and(|s| hit(s));
      if !synthetic {
        prev(info);
      }
    }));
  });
}

unsafe extern "C-unwind" fn f_ok(_l: *mut LuaState, _ud: *mut c_void) {}

unsafe extern "C-unwind" fn f_lua_exception(l: *mut LuaState, _ud: *mut c_void) {
  // `lua_d_throw` 已收口为安全函数：仅构造 panic 载荷并抛出，不解引用 `l`。
  lua_d_throw(l, LuaStatus::ErrSyntax as i32)
}

unsafe extern "C-unwind" fn f_panic_str(_l: *mut LuaState, _ud: *mut c_void) {
  panic!("{SYNTHETIC} str payload");
}

unsafe extern "C-unwind" fn f_panic_string(_l: *mut LuaState, _ud: *mut c_void) {
  let n = 42;
  panic!("{SYNTHETIC} fmt payload {n}");
}

unsafe extern "C-unwind" fn f_panic_unknown(_l: *mut LuaState, _ud: *mut c_void) {
  panic_any(7u8);
}

unsafe extern "C-unwind" fn f_panic_nul(_l: *mut LuaState, _ud: *mut c_void) {
  panic_any(String::from("ulua_t2_synthetic nul\0inside"));
}

/// 读栈顶：必须是字符串且内容等于 `want`，读完恢复到 `base`。
unsafe fn pop_string(l: *mut LuaState, base: i32, want: &str) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内
  // 取得/构造。本处经 C-ABI 垫片 `lua_tolstring` 取 NUL 结尾视图（size 出参传 NULL，
  // lua.h 约定），`CStr::from_ptr` 在首个 `\0` 截断——与旧行为逐字节一致。
  unsafe {
    assert_eq!(lua_type(l, -1), LuaType::String as i32, "top not string");
    let p = lua_tolstring(l, -1, ptr::null_mut());
    assert!(!p.is_null());
    let got = cstr_cow(p).into_owned();
    assert_eq!(got, want);
    lua_settop(l, base);
  }
}

#[test]
fn protected_call_three_arms() {
  install_quiet_hook();
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内
  // 取得/构造，至本块结束前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "luaL_newstate failed");
    let base = lua_gettop(l);

    // 正常返回：status 0，栈不变。
    assert_eq!(lua_d_rawrunprotected(l, Some(f_ok), null_mut()), 0);
    assert_eq!(lua_gettop(l), base);

    // 臂 1：lua_exception → 恢复其 status（ErrSyntax），不压错误对象。
    assert_eq!(
      lua_d_rawrunprotected(l, Some(f_lua_exception), null_mut()),
      LuaStatus::ErrSyntax as i32
    );
    assert_eq!(lua_gettop(l), base);

    // 臂 2：&str 载荷 → ErrRun + 原消息压栈。
    assert_eq!(
      lua_d_rawrunprotected(l, Some(f_panic_str), null_mut()),
      LuaStatus::ErrRun as i32
    );
    pop_string(l, base, &format!("{SYNTHETIC} str payload"));

    // 臂 2：String（fmt）载荷 → ErrRun + 格式化后的消息。
    assert_eq!(
      lua_d_rawrunprotected(l, Some(f_panic_string), null_mut()),
      LuaStatus::ErrRun as i32
    );
    pop_string(l, base, &format!("{SYNTHETIC} fmt payload 42"));

    // 臂 3：非字符串载荷 → ErrRun + "unknown error"。
    assert_eq!(
      lua_d_rawrunprotected(l, Some(f_panic_unknown), null_mut()),
      LuaStatus::ErrRun as i32
    );
    pop_string(l, base, "unknown error");

    // 臂 3 兜底：字符串载荷含内嵌 NUL，CString 构造失败 →
    // 编译期 const CStr 文案 "invalid error message"。
    assert_eq!(
      lua_d_rawrunprotected(l, Some(f_panic_nul), null_mut()),
      LuaStatus::ErrRun as i32
    );
    pop_string(l, base, "invalid error message");

    lua_close(l);
  }
}
