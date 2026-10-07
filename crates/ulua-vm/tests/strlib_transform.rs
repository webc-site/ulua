//! `string` 库单目变换/取子串/取字节簇的 C-API 行为契约（oracle：`cpp/VM/src/lstrlib.cpp`）。
//!
//! 迁移自 `src/functions/str_shared.rs` 的 `#[cfg(test)]` 单元（review.md §8 第一条：
//! 只测公开 API 的测试落 `tests/`）。`str_lower`/`str_upper`/`str_reverse`/`str_sub`/
//! `str_byte` 的输入→输出契约全部经 `string.*` 真实闭包在 `lua_pcall` 受保护帧内
//! 可观察，无须直调 `pub(crate)` 入口；`str_transform1` 骨架仍由前三例经各注册入口
//! 间接覆盖。期望值按 cpp 逐点位推算：`str_lower`（lstrlib.cpp:47）、`str_upper`
//! （:59）、`str_reverse`（:71）、`str_sub`（:440）、`str_byte`（:377，缺省单字节）。

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_openlibs::lua_l_openlibs, lua_pushlstring::lua_pushlstring_bytes,
    lua_tolstring::lua_tolstring_ref,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_multret::LUA_MULTRET},
};

#[path = "common/state.rs"]
mod state;

use state::State;

/// 实参形态：字节串（字符串实参）或整数（`str_sub` 的索引类数值实参）
#[derive(Debug)]
enum Arg<'a> {
  S(&'a [u8]),
  N(i32),
}

/// `string` 库用例入口：打开标准库并按 C-API 直接调用各臂
struct Str {
  vm: State,
}

impl Str {
  fn new() -> Self {
    let vm = State::new();
    // Safety: 测试并行运行下本 VM 由本用例独占；`lua_l_openlibs` 只初始化标准库表
    unsafe { lua_l_openlibs(&mut *vm.l) };
    Self { vm }
  }

  /// 压 `string.<name>` 与实参：栈形如 `[string 表(1), 函数(2), 实参(3..)]`
  fn setup(&self, name: &str, args: &[Arg<'_>]) {
    let l = self.vm.l;
    // Safety: `l` 为本用例独占的存活 VM；索引 1 为上一步压入的 `string` 表；
    // 字节实参借用覆盖整个用例作用域，`lua_pushlstring_bytes` 只界内拷贝；
    // `push_integer` 的整数实参恒为合法栈值
    unsafe {
      (*l).set_top(0);
      (*l).get_field_bytes(LUA_GLOBALSINDEX, b"string");
      (*l).get_field_str(1, name);
      for arg in args {
        match *arg {
          Arg::S(bytes) => lua_pushlstring_bytes(&mut *l, bytes),
          Arg::N(n) => (*l).push_integer(n),
        }
      }
    }
  }

  /// 栈 `idx` 槽的字节内容（错误消息读取专用）
  fn message_at(&self, idx: i32) -> String {
    // Safety: 仅读栈槽字节；非串时按 cpp `lua_tolstring` 的转换语义读
    unsafe {
      String::from_utf8_lossy(lua_tolstring_ref(self.vm.l, idx).unwrap_or_default()).into_owned()
    }
  }

  /// 以 `LUA_MULTRET` 调用 `string.<name>(args...)`，返回实际结果个数；抛错即中止
  #[track_caller]
  fn invoke(&self, name: &str, args: &[Arg<'_>]) -> i32 {
    self.setup(name, args);
    let l = self.vm.l;
    // Safety: 栈为 `[string 表, 函数, 实参...]`，`pcall` 在受保护帧内执行；
    // `LUA_MULTRET` 下结果自函数原索引 2 起连续摆放，个数 = 调用后栈顶 - 1
    unsafe {
      assert_eq!(
        (*l).pcall(args.len() as i32, LUA_MULTRET, 0),
        0,
        "string.{name}({args:?}) 不应抛错: {}",
        self.message_at(2)
      );
      (*l).get_top() - 1
    }
  }

  /// `string.<name>(args...)` 的唯一字符串返回值（结果数与串形态各自钉死）
  #[track_caller]
  fn string_result(&self, name: &str, args: &[Arg<'_>]) -> Vec<u8> {
    let count = self.invoke(name, args);
    assert_eq!(count, 1, "string.{name} 应恰好返回 1 个值");
    // Safety: 上两行已证结果恰 1 个，槽 2 即结果槽；`type_of` 只读类型
    unsafe {
      assert_eq!(
        (*self.vm.l).type_of(2),
        LuaType::String,
        "string.{name} 返回值应为字符串"
      );
      // 形态已钉为 String：`lua_tolstring_ref` 对串槽恒返回界内借用字节
      lua_tolstring_ref(self.vm.l, 2).unwrap_or_default().to_vec()
    }
  }
}

/// 单目变换骨架 `str_transform1`：lower/upper/reverse 取 1 号串实参、
/// 等长变换后压回（cpp lstrlib.cpp:47/59/71）
#[test]
fn str_transforms_match_cpp() {
  let strlib = Str::new();
  assert_eq!(
    strlib.string_result("lower", &[Arg::S(b"Hello, WORLD 123!")]),
    b"hello, world 123!"
  );
  assert_eq!(
    strlib.string_result("upper", &[Arg::S(b"Hello, world 123!")]),
    b"HELLO, WORLD 123!"
  );
  assert_eq!(
    strlib.string_result("reverse", &[Arg::S(b"Hello!")]),
    b"!olleH"
  );
}

/// `str_sub` 闭区间取子串（cpp lstrlib.cpp:440）与 `str_byte` 缺省单字节返回
/// 1 个整数（:377，对齐原单元对返回个数与值的双重断言）
#[test]
fn str_sub_and_byte_match_cpp() {
  let strlib = Str::new();
  assert_eq!(
    strlib.string_result("sub", &[Arg::S(b"abcdef"), Arg::N(2), Arg::N(4)]),
    b"bcd"
  );

  let count = strlib.invoke("byte", &[Arg::S(b"a")]);
  assert_eq!(count, 1, "单实参 string.byte 应恰好返回 1 个值");
  // Safety: 上一行已证结果恰 1 个，槽 2 为整数结果槽；`to_integer` 只读
  let byte = unsafe { (*strlib.vm.l).to_integer(2) };
  assert_eq!(byte, Some(i32::from(b'a')));
}
