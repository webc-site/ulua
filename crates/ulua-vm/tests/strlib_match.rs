//! string 库匹配/替换簇的 C-API 行为契约（oracle：`cpp/VM/src/lstrlib.cpp`）。
//!
//! 覆盖 `MatchState` 偏移化 + 切片门面改造后最容易走偏的边界：
//! 1. `off == src.len()`/`off == pat.len()` 的串尾读（cpp 直读终止 NUL，门面折算为 0）；
//! 2. `str_find_aux`（lstrlib.cpp:663）的 `init` 钳位与 `^` 锚定跳格；
//! 3. `match`（:428）的 `%f`/`%1`/`()`、`matchbalance`（:338）的 `%b`、`classend`（:215）报错路径；
//! 4. `add_s`（:765）替换串末字节 `%` 的终止 NUL 读；
//! 5. `gmatch_aux`（:730）空匹配右移一格。
//!
//! 全部经 `string.*` 真实闭包在 `lua_pcall` 受保护帧内调用，期望值按 cpp 逐点位推算。

use core::ffi::{CStr, c_char};

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_getfield::lua_getfield, lua_l_openlibs::lua_l_openlibs, lua_pcall::lua_pcall,
    lua_pushlstring::lua_pushlstring, lua_pushvalue::lua_pushvalue, lua_settop::lua_settop,
    lua_tointegerx::lua_tointegerx, lua_tolstring::lua_tolstring_ref, lua_type::lua_type,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
};

#[path = "common/state.rs"]
mod state;

use state::State;

/// 栈槽值的三种形态：串 / 整数 / nil（`find` 的位置、`match` 的捕获、`gsub` 的计数各归其类）
#[derive(Debug, Clone, PartialEq)]
enum Val {
  Str(String),
  Int(i64),
  Nil,
}

impl Val {
  fn s(text: &str) -> Val {
    Val::Str(text.to_owned())
  }

  fn i(n: i64) -> Val {
    Val::Int(n)
  }
}

/// `string` 库用例入口：打开标准库并按 C-API 直接调用各臂
struct Str {
  vm: State,
}

impl Str {
  fn new() -> Self {
    let vm = State::new();
    // Safety: 测试并行运行下本 VM 由本用例独占；`lua_l_openlibs` 只初始化标准库表
    unsafe { lua_l_openlibs(vm.l) };
    Self { vm }
  }

  /// 压 `string.<name>` 与字符串实参：栈形如 `[string 表(1), 函数(2), 实参(3..)]`
  fn setup(&self, name: &CStr, args: &[&[u8]]) {
    let l = self.vm.l;
    // Safety: `l` 为本用例独占的存活 VM；`c"..."` 字面量恒 NUL 结尾；索引 1 为上一步
    // 压入的 `string` 表。字节实参借用覆盖整个用例作用域，`lua_pushlstring` 只界内拷贝
    unsafe {
      lua_settop(l, 0);
      lua_getfield(l, LUA_GLOBALSINDEX, c"string".as_ptr());
      lua_getfield(l, 1, name.as_ptr());
      for arg in args {
        lua_pushlstring(l, arg.as_ptr().cast::<c_char>(), arg.len());
      }
    }
  }

  /// 压 `string.<member>` 当实参（函数替换臂用，要求槽 1 已是 `string` 表）
  fn push_member_arg(&self, name: &CStr) {
    // Safety: 槽 1 由 `setup` 压入的 `string` 表占据
    unsafe { lua_getfield(self.vm.l, 1, name.as_ptr()) };
  }

  /// 读栈 `idx` 槽：先 `lua_type` 分派，避免 `lua_tolstring` 把数值槽就地改写成串
  fn value_at(&self, idx: i32) -> Val {
    let l = self.vm.l;
    // Safety: 测试独占 VM，`idx` 由调用侧保证为本次调用结果区内的合法索引
    unsafe {
      let t = lua_type(l, idx);
      if t == LuaType::Nil as i32 || t == LuaType::None as i32 {
        Val::Nil
      } else if t == LuaType::String as i32 {
        Val::Str(self.message_at(idx))
      } else {
        let n = lua_tointegerx(l, idx)
          .unwrap_or_else(|| panic!("槽 {idx} 既非 nil/串也不是整数: {}", self.message_at(idx)));
        Val::Int(i64::from(n))
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

  /// 调用槽 2 的函数 + 其后 `nargs` 个实参并收 `nresults` 个结果；抛错时返回错误消息
  fn invoke(&self, nargs: i32, nresults: i32) -> Result<Vec<Val>, String> {
    let l = self.vm.l;
    // Safety: 栈为 `[string 表, 函数, 实参...]`，`lua_pcall` 在受保护帧内执行；
    // 结果自函数原索引 2 起
    unsafe {
      if lua_pcall(l, nargs, nresults, 0) != 0 {
        return Err(self.message_at(2));
      }
      Ok((0..nresults).map(|k| self.value_at(2 + k)).collect())
    }
  }

  /// `string.<name>(args...)` 的成功断言
  #[track_caller]
  fn expect(&self, name: &CStr, args: &[&[u8]], nresults: i32, want: &[Val]) {
    self.setup(name, args);
    assert_eq!(
      self.invoke(args.len() as i32, nresults),
      Ok(want.to_vec()),
      "string.{}({:?}) 行为与 cpp 推算不符",
      name.to_string_lossy(),
      args
    );
  }

  /// `string.<name>(args...)` 的抛错断言：消息须以 cpp `luaL_error` 文案结尾
  /// （前缀是本用例无 chunkname 时 `lWhere` 的位置串，不断言）
  #[track_caller]
  fn expect_error(&self, name: &CStr, args: &[&[u8]], cpp_message: &str) {
    self.setup(name, args);
    let message = match self.invoke(args.len() as i32, 1) {
      Err(message) => message,
      Ok(values) => panic!(
        "string.{}({:?}) 应抛错，实际返回 {:?}",
        name.to_string_lossy(),
        args,
        values
      ),
    };
    assert!(
      message.ends_with(cpp_message),
      "string.{} 错误消息 {:?} 未以 cpp 文案 {:?} 结尾",
      name.to_string_lossy(),
      message,
      cpp_message
    );
  }

  /// `string.gmatch(src, pat)` 迭代到底的全部结果（空匹配按 cpp 右移一格）
  fn gmatch_all(&self, src: &[u8], pat: &[u8]) -> Vec<Val> {
    self.setup(c"gmatch", &[src, pat]);
    let l = self.vm.l;
    // Safety: 栈为 `[string 表, 实参, 实参]`，首次 pcall 产出迭代闭包于槽 2；
    // 其后每轮把闭包副本压到槽 3 调用，结果读自槽 3
    unsafe {
      assert_eq!(
        lua_pcall(l, 2, 1, 0),
        0,
        "gmatch 建闭包失败: {}",
        self.message_at(2)
      );
      let mut out = Vec::new();
      loop {
        lua_pushvalue(l, 2);
        assert_eq!(
          lua_pcall(l, 0, 1, 0),
          0,
          "gmatch 迭代抛错: {}",
          self.message_at(3)
        );
        let value = self.value_at(3);
        lua_settop(l, 2); // 弹出结果，回到 [string 表, 闭包]
        if value == Val::Nil {
          return out;
        }
        out.push(value);
      }
    }
  }
}

/// `str_find_aux` 的 plain 分支（lstrlib.cpp:679-686）：`init` 钳位与 `off == len`
/// 串尾哨兵。`find("abc","",4)`：init=4 == ls+1 放行，空 needle 命中偏移 3 → 4,3
#[test]
fn find_plain_init_and_end_sentinel() {
  let strlib = Str::new();
  // init=8 → 起点偏移 7（'o'）→ cpp `s2 - s + 1` = 8，`s2 - s + lp` = 8
  strlib.expect(
    c"find",
    &[b"hello world", b"o", b"8"],
    2,
    &[Val::i(8), Val::i(8)],
  );
  strlib.expect(c"find", &[b"abc", b"", b"4"], 2, &[Val::i(4), Val::i(3)]);
  // init > ls + 1 → cpp :671-674 直接 pushnil
  strlib.expect(c"find", &[b"abc", b"", b"5"], 1, &[Val::Nil]);
  strlib.expect(c"find", &[b"abc", b"c", b"5"], 1, &[Val::Nil]);
  // 负 init 走 posrelat（cpp:22）：posrelat(-3, 5) = 3 → 从偏移 2 起找
  strlib.expect(
    c"find",
    &[b"hello", b"o", b"-3"],
    2,
    &[Val::i(5), Val::i(5)],
  );
  // 第 4 槽 plain=true：`[b]` 按字面量子串查找（cpp :676 走 lmemfind 分支），
  // 命中的是源串里那 3 个字节本身，而非字符类
  strlib.expect(
    c"find",
    &[b"a[b]c", b"[b]", b"1", b"1"],
    2,
    &[Val::i(2), Val::i(4)],
  );
  // 同一 pattern 非 plain 时是字符类：`[b]` 命中偏移 2 的 'b' → 3,3
  strlib.expect(
    c"find",
    &[b"a[b]c", b"[b]", b"1"],
    2,
    &[Val::i(3), Val::i(3)],
  );
}

/// `str_find_aux` 的模式分支：`^` 锚定跳格（cpp `if (anchor) { p++; lp--; }`）、
/// 游标游走与 `push_captures`（cpp:625）
#[test]
fn find_pattern_anchor_and_captures() {
  let strlib = Str::new();
  strlib.expect(
    c"find",
    &[b"hello world", b"(%w+) (%w+)", b"1"],
    4,
    &[Val::i(1), Val::i(11), Val::s("hello"), Val::s("world")],
  );
  // 锚定后模式为空串：`while (s1++ < ms.src_end && !anchor)` 只试偏移 0 一处
  strlib.expect(c"find", &[b"abc", b"^", b"1"], 2, &[Val::i(1), Val::i(0)]);
  strlib.expect(c"find", &[b"abc", b"^b", b"1"], 1, &[Val::Nil]);
  // 空源串 + 空模式：`match` 立即命中，返回 1,0
  strlib.expect(c"find", &[b"", b"", b"1"], 2, &[Val::i(1), Val::i(0)]);
}

/// `match`（:428）的捕获形态：位置捕获 `()`（`CAP_POSITION` → `init + 1`）、
/// 回填 `%1`（`match_capture` :417 的源偏移切片比对）
#[test]
fn match_captures_position_and_backref() {
  let strlib = Str::new();
  strlib.expect(c"match", &[b"abc", b"()b"], 1, &[Val::i(2)]);
  // `match` 返回捕获本身（`push_captures` :625），整窗位置只在 `find` 里出
  strlib.expect(c"match", &[b"aa", b"(a)%1"], 1, &[Val::s("a")]);
  // `find` 的整窗 + 捕获：cpp 先 push (s1-s+1)/(res-s)，再 push_captures(NULL, NULL)
  strlib.expect(
    c"find",
    &[b"aa", b"(a)%1"],
    3,
    &[Val::i(1), Val::i(2), Val::s("a")],
  );
  strlib.expect(c"match", &[b"ab", b"(a)%1"], 1, &[Val::Nil]);
  strlib.expect(
    c"match",
    &[b"hello world", b"(%w+)%s(%w+)"],
    2,
    &[Val::s("hello"), Val::s("world")],
  );
  // 空源串空模式：整窗长度为 0，串尾哨兵由门面的终止 NUL 语义覆盖
  strlib.expect(c"match", &[b"", b""], 1, &[Val::s("")]);
  strlib.expect(c"match", &[b"", b".+"], 1, &[Val::Nil]);
  // check_capture（:198）越界序号：cpp 文案 "invalid capture index %2"
  strlib.expect_error(c"match", &[b"abc", b"(a)%2"], "invalid capture index %2");
}

/// `matchbalance`（:338）的 `%b`：配对扫描止于串尾；畸形 `%b` 报缺参
#[test]
fn match_balance_percent_b() {
  let strlib = Str::new();
  strlib.expect(c"match", &[b"(ab)cde", b"%b()"], 1, &[Val::s("(ab)")]);
  strlib.expect(c"match", &[b"(a(b))x", b"%b()"], 1, &[Val::s("(a(b))")]);
  strlib.expect(c"match", &[b"(ab", b"%b()"], 1, &[Val::Nil]);
  strlib.expect_error(
    c"match",
    &[b"x", b"%b("],
    "malformed pattern (missing arguments to '%b')",
  );
}

/// `match` 的 `%f` frontier 分支（cpp `previous = (s == ms->src_init) ? '\0' : *(s - 1)`）：
/// 偏移 0 处 previous 取 0，串尾 `cur == src.len()` 读终止 NUL
#[test]
fn match_frontier_percent_f() {
  let strlib = Str::new();
  // frontier 在偏移 1 命中（previous '=' 不在类内、*s='a' 在类内），`%a+` 吞 "abc"
  // → 整窗 [1, 4) → cpp `s1 - s + 1` = 2、`res - s` = 4
  strlib.expect(
    c"find",
    &[b"=abc=", b"%f[%a]%a+"],
    2,
    &[Val::i(2), Val::i(4)],
  );
  // 源串首字符即字母：previous = 0 不在类内、*s 在类内 → 命中偏移 1
  strlib.expect(c"find", &[b"abc", b"%f[%a]%a+"], 2, &[Val::i(1), Val::i(3)]);
  strlib.expect_error(
    c"match",
    &[b"abc", b"%fa"],
    "missing '[' after '%f' in pattern",
  );
}

/// `classend`（:215）的两条畸形模式报错路径（读终止 NUL 前先判界）
#[test]
fn classend_malformed_patterns() {
  let strlib = Str::new();
  strlib.expect_error(c"match", &[b"a", b"%"], "malformed pattern (ends with '%')");
  strlib.expect_error(c"match", &[b"a", b"["], "malformed pattern (missing ']')");
  // `%]` 转义允许 `]` 出现在类内：cpp `if (*(p++) == L_ESC && p < ms->p_end) p++;`
  strlib.expect(c"match", &[b"]", b"[%]]"], 1, &[Val::s("]")]);
}

/// `add_s`（:765）：`%0` 整窗、`%%` 字面量、末字节 `%` 读替换串终止 NUL 报错、
/// `%1` 走 `push_onecapture` + `lua_l_addvalue`
#[test]
fn gsub_replacement_arms() {
  let strlib = Str::new();
  strlib.expect(
    c"gsub",
    &[b"abc", b"b", b"x%0y"],
    2,
    &[Val::s("axbyc"), Val::i(1)],
  );
  strlib.expect(
    c"gsub",
    &[b"abc", b"b", b"%%"],
    2,
    &[Val::s("a%c"), Val::i(1)],
  );
  strlib.expect(
    c"gsub",
    &[b"abc", b"(b)", b"<%1>"],
    2,
    &[Val::s("a<b>c"), Val::i(1)],
  );
  strlib.expect(
    c"gsub",
    &[b"hello", b"l", b"-"],
    2,
    &[Val::s("he--o"), Val::i(2)],
  );
  // 空源串：max_s 默认 srcl+1 = 1，空匹配一次后 `src_off < src.len()` 不成立即 break
  strlib.expect(c"gsub", &[b"", b"", b"X"], 2, &[Val::s("X"), Val::i(1)]);
  strlib.expect_error(
    c"gsub",
    &[b"abc", b"b", b"%"],
    "invalid use of '%' in replacement string",
  );
}

/// `str_gsub`（:831）的 `^` 锚定与 `add_value`（:796）函数替换臂
#[test]
fn gsub_anchor_and_function_replacement() {
  let strlib = Str::new();
  strlib.expect(
    c"gsub",
    &[b"abc", b"^a", b"X"],
    2,
    &[Val::s("Xbc"), Val::i(1)],
  );
  strlib.expect(
    c"gsub",
    &[b"abc", b"^b", b"X"],
    2,
    &[Val::s("abc"), Val::i(0)],
  );
  strlib.expect(
    c"gsub",
    &[b"abc", b"(b)", b"[%0]"],
    2,
    &[Val::s("a[b]c"), Val::i(1)],
  );

  // 函数替换：`string.gsub("abc", "(b)", string.upper)` → "aBc", 1
  strlib.setup(c"gsub", &[b"abc", b"(b)"]);
  strlib.push_member_arg(c"upper");
  assert_eq!(
    strlib.invoke(3, 2),
    Ok(vec![Val::s("aBc"), Val::i(1)]),
    "函数替换臂（push_captures + lua_call）行为不符"
  );
}

/// `gmatch_aux`（:730）：游标以偏移存于 upvalue 3，`e == src` 的空匹配右移一格
#[test]
fn gmatch_iterates_including_empty_matches() {
  let strlib = Str::new();
  assert_eq!(
    strlib.gmatch_all(b"a-b-c", b"[^-]+"),
    vec![Val::s("a"), Val::s("b"), Val::s("c")]
  );
  // 空模式：cpp `for (src = s + pos; src <= ms.src_end; src++)` + 空匹配右移，
  // 长度 5 的源串共 6 个空串结果（偏移 0..=5，即 `off == src.len()` 也在内）
  assert_eq!(strlib.gmatch_all(b"hello", b""), vec![Val::s(""); 6]);
  // 串尾空匹配：`a-` 在 "a-" 之后仍有一格空匹配
  assert_eq!(
    strlib.gmatch_all(b"a-", b"[a-]*"),
    vec![Val::s("a-"), Val::s("")]
  );
}
