//! 行为级回归测试：`get_builtin_function_id` 的内置函数识别表。
//!
//! 对照 oracle `cpp/Compiler/src/Builtins.cpp:70-387`（`getBuiltinFunctionId`）
//! 逐条枚举识别集合与返回 id：全局函数 :72-104（13 项）、math :106-174（33 项）、
//! bit32 :176-208（15 项）、string :210-220（4 项）、table :222-228（2 项）、
//! buffer :230-262（16 项 + 双旗标门控 :258-261 共 2 项）、vector :264-292
//!（13 项）、integer :294-370（37 项，受 `LuauIntegerFastcalls` 门控 :294），
//! 合计 135 项。断言三类行为：
//! 1. 全部表项在对应旗标环境下命中且 id 精确一致；
//! 2. 表外近邻假名（多尾缀、缺尾缀、大小写错、空名）一律未命中返回 -1；
//! 3. 实现侧三张 const 表的并集与本清单互为子集（条数 + 逐键 id 相等），
//!    任何一侧的增删/拼写漂移都会撕裂断言。

use ulua_ast::records::ast_name::AstName;
use ulua_common::{enums::luau_builtin_function::*, fflag, records::f_value::FValue};
use ulua_compiler::{
  functions::get_builtin_function_id::{
    BUFFER_FASTCALL_TABLE, INTEGER_TABLE, TABLE, get_builtin_function_id,
  },
  records::{builtin::Builtin, compile_options::CompileOptions},
};

/// 测试用例：`(库名, 方法名, 期望 id)`；库名/方法名以 NUL 结尾静态串存储
/// （`AstName::from_static` 的 C 串契约），空库名 `b""` 表示全局（空对象名）。
type Case = (&'static [u8], &'static [u8], LuauBuiltinFunction);

/// cpp:72-104 全局函数（13 项）。
const UNGATED: &[Case] = &[
  (b"", b"assert", LBF_ASSERT),
  (b"", b"type", LBF_TYPE),
  (b"", b"typeof", LBF_TYPEOF),
  (b"", b"rawset", LBF_RAWSET),
  (b"", b"rawget", LBF_RAWGET),
  (b"", b"rawequal", LBF_RAWEQUAL),
  (b"", b"rawlen", LBF_RAWLEN),
  (b"", b"unpack", LBF_TABLE_UNPACK),
  (b"", b"select", LBF_SELECT_VARARG),
  (b"", b"getmetatable", LBF_GETMETATABLE),
  (b"", b"setmetatable", LBF_SETMETATABLE),
  (b"", b"tonumber", LBF_TONUMBER),
  (b"", b"tostring", LBF_TOSTRING),
  // cpp:106-174 math（33 项）
  (b"math", b"abs", LBF_MATH_ABS),
  (b"math", b"acos", LBF_MATH_ACOS),
  (b"math", b"asin", LBF_MATH_ASIN),
  (b"math", b"atan2", LBF_MATH_ATAN2),
  (b"math", b"atan", LBF_MATH_ATAN),
  (b"math", b"ceil", LBF_MATH_CEIL),
  (b"math", b"cosh", LBF_MATH_COSH),
  (b"math", b"cos", LBF_MATH_COS),
  (b"math", b"deg", LBF_MATH_DEG),
  (b"math", b"exp", LBF_MATH_EXP),
  (b"math", b"floor", LBF_MATH_FLOOR),
  (b"math", b"fmod", LBF_MATH_FMOD),
  (b"math", b"frexp", LBF_MATH_FREXP),
  (b"math", b"ldexp", LBF_MATH_LDEXP),
  (b"math", b"log10", LBF_MATH_LOG10),
  (b"math", b"log", LBF_MATH_LOG),
  (b"math", b"max", LBF_MATH_MAX),
  (b"math", b"min", LBF_MATH_MIN),
  (b"math", b"modf", LBF_MATH_MODF),
  (b"math", b"pow", LBF_MATH_POW),
  (b"math", b"rad", LBF_MATH_RAD),
  (b"math", b"sinh", LBF_MATH_SINH),
  (b"math", b"sin", LBF_MATH_SIN),
  (b"math", b"sqrt", LBF_MATH_SQRT),
  (b"math", b"tanh", LBF_MATH_TANH),
  (b"math", b"tan", LBF_MATH_TAN),
  (b"math", b"clamp", LBF_MATH_CLAMP),
  (b"math", b"sign", LBF_MATH_SIGN),
  (b"math", b"round", LBF_MATH_ROUND),
  (b"math", b"lerp", LBF_MATH_LERP),
  (b"math", b"isnan", LBF_MATH_ISNAN),
  (b"math", b"isinf", LBF_MATH_ISINF),
  (b"math", b"isfinite", LBF_MATH_ISFINITE),
  // cpp:176-208 bit32（15 项）
  (b"bit32", b"arshift", LBF_BIT32_ARSHIFT),
  (b"bit32", b"band", LBF_BIT32_BAND),
  (b"bit32", b"bnot", LBF_BIT32_BNOT),
  (b"bit32", b"bor", LBF_BIT32_BOR),
  (b"bit32", b"bxor", LBF_BIT32_BXOR),
  (b"bit32", b"btest", LBF_BIT32_BTEST),
  (b"bit32", b"extract", LBF_BIT32_EXTRACT),
  (b"bit32", b"lrotate", LBF_BIT32_LROTATE),
  (b"bit32", b"lshift", LBF_BIT32_LSHIFT),
  (b"bit32", b"replace", LBF_BIT32_REPLACE),
  (b"bit32", b"rrotate", LBF_BIT32_RROTATE),
  (b"bit32", b"rshift", LBF_BIT32_RSHIFT),
  (b"bit32", b"countlz", LBF_BIT32_COUNTLZ),
  (b"bit32", b"countrz", LBF_BIT32_COUNTRZ),
  (b"bit32", b"byteswap", LBF_BIT32_BYTESWAP),
  // cpp:210-220 string（4 项）
  (b"string", b"byte", LBF_STRING_BYTE),
  (b"string", b"char", LBF_STRING_CHAR),
  (b"string", b"len", LBF_STRING_LEN),
  (b"string", b"sub", LBF_STRING_SUB),
  // cpp:222-228 table（2 项）
  (b"table", b"insert", LBF_TABLE_INSERT),
  (b"table", b"unpack", LBF_TABLE_UNPACK),
  // cpp:230-257 buffer 非门控段（16 项，写别名按 cpp :236/:242/:248 归并）
  (b"buffer", b"readi8", LBF_BUFFER_READI8),
  (b"buffer", b"readu8", LBF_BUFFER_READU8),
  (b"buffer", b"writei8", LBF_BUFFER_WRITEU8),
  (b"buffer", b"writeu8", LBF_BUFFER_WRITEU8),
  (b"buffer", b"readi16", LBF_BUFFER_READI16),
  (b"buffer", b"readu16", LBF_BUFFER_READU16),
  (b"buffer", b"writei16", LBF_BUFFER_WRITEU16),
  (b"buffer", b"writeu16", LBF_BUFFER_WRITEU16),
  (b"buffer", b"readi32", LBF_BUFFER_READI32),
  (b"buffer", b"readu32", LBF_BUFFER_READU32),
  (b"buffer", b"writei32", LBF_BUFFER_WRITEU32),
  (b"buffer", b"writeu32", LBF_BUFFER_WRITEU32),
  (b"buffer", b"readf32", LBF_BUFFER_READF32),
  (b"buffer", b"writef32", LBF_BUFFER_WRITEF32),
  (b"buffer", b"readf64", LBF_BUFFER_READF64),
  (b"buffer", b"writef64", LBF_BUFFER_WRITEF64),
  // cpp:264-292 vector（13 项，create → LBF_VECTOR）
  (b"vector", b"create", LBF_VECTOR),
  (b"vector", b"magnitude", LBF_VECTOR_MAGNITUDE),
  (b"vector", b"normalize", LBF_VECTOR_NORMALIZE),
  (b"vector", b"cross", LBF_VECTOR_CROSS),
  (b"vector", b"dot", LBF_VECTOR_DOT),
  (b"vector", b"floor", LBF_VECTOR_FLOOR),
  (b"vector", b"ceil", LBF_VECTOR_CEIL),
  (b"vector", b"abs", LBF_VECTOR_ABS),
  (b"vector", b"sign", LBF_VECTOR_SIGN),
  (b"vector", b"clamp", LBF_VECTOR_CLAMP),
  (b"vector", b"min", LBF_VECTOR_MIN),
  (b"vector", b"max", LBF_VECTOR_MAX),
  (b"vector", b"lerp", LBF_VECTOR_LERP),
];

/// cpp:294-370 integer 段（37 项），门控 `LuauIntegerFastcalls`（cpp:294）。
const INTEGER_FASTCALL: &[Case] = &[
  (b"integer", b"add", LBF_INTEGER_ADD),
  (b"integer", b"sub", LBF_INTEGER_SUB),
  (b"integer", b"mod", LBF_INTEGER_MOD),
  (b"integer", b"mul", LBF_INTEGER_MUL),
  (b"integer", b"div", LBF_INTEGER_DIV),
  (b"integer", b"idiv", LBF_INTEGER_IDIV),
  (b"integer", b"udiv", LBF_INTEGER_UDIV),
  (b"integer", b"rem", LBF_INTEGER_REM),
  (b"integer", b"urem", LBF_INTEGER_UREM),
  (b"integer", b"min", LBF_INTEGER_MIN),
  (b"integer", b"max", LBF_INTEGER_MAX),
  (b"integer", b"neg", LBF_INTEGER_NEG),
  (b"integer", b"create", LBF_INTEGER_CREATE),
  (b"integer", b"clamp", LBF_INTEGER_CLAMP),
  (b"integer", b"band", LBF_INTEGER_BAND),
  (b"integer", b"bor", LBF_INTEGER_BOR),
  (b"integer", b"bxor", LBF_INTEGER_BXOR),
  (b"integer", b"bnot", LBF_INTEGER_BNOT),
  (b"integer", b"btest", LBF_INTEGER_BTEST),
  (b"integer", b"bswap", LBF_INTEGER_BSWAP),
  (b"integer", b"lt", LBF_INTEGER_LT),
  (b"integer", b"le", LBF_INTEGER_LE),
  (b"integer", b"ult", LBF_INTEGER_ULT),
  (b"integer", b"ule", LBF_INTEGER_ULE),
  (b"integer", b"gt", LBF_INTEGER_GT),
  (b"integer", b"ge", LBF_INTEGER_GE),
  (b"integer", b"ugt", LBF_INTEGER_UGT),
  (b"integer", b"uge", LBF_INTEGER_UGE),
  (b"integer", b"lshift", LBF_INTEGER_LSHIFT),
  (b"integer", b"rshift", LBF_INTEGER_RSHIFT),
  (b"integer", b"arshift", LBF_INTEGER_ARSHIFT),
  (b"integer", b"lrotate", LBF_INTEGER_LROTATE),
  (b"integer", b"rrotate", LBF_INTEGER_RROTATE),
  (b"integer", b"countrz", LBF_INTEGER_COUNTRZ),
  (b"integer", b"countlz", LBF_INTEGER_COUNTLZ),
  (b"integer", b"extract", LBF_INTEGER_EXTRACT),
  (b"integer", b"tonumber", LBF_INTEGER_TONUMBER),
];

/// cpp:258-261 buffer 整型快调用段（2 项），双旗标门控
/// `LuauIntegerFastcalls && LuauIntegerBufferFastcalls`。
const BUFFER_FASTCALL: &[Case] = &[
  (b"buffer", b"readinteger", LBF_BUFFER_READINTEGER),
  (b"buffer", b"writeinteger", LBF_BUFFER_WRITEINTEGER),
];

/// RAII 旗标守卫：线程本地覆盖（`push_test_override`），Drop 弹回；
/// 与 ulua-unit-test 的 `ScopedFastFlag` 同一语义（libtest 并行线程互不串扰）。
struct ScopedBoolFlag(&'static FValue<bool>);

impl Drop for ScopedBoolFlag {
  fn drop(&mut self) {
    self.0.pop_test_override();
  }
}

fn set_flag(flag: &'static FValue<bool>, value: bool) -> ScopedBoolFlag {
  flag.push_test_override(value);
  ScopedBoolFlag(flag)
}

/// 纯字节键对照。
fn raw(name: &[u8]) -> &[u8] {
  name
}

/// 由 `(库名, 方法名)` 构造 `Builtin` 并查 id；空库名取空对象名（cpp
/// `Builtin{AstName(), name}` 的全局形态），`CompileOptions::default()`
/// 保证不触发 vectorCtor 回退段（cpp:372-384）。
fn lookup(object: &'static [u8], method: &'static [u8]) -> i32 {
  let builtin = Builtin {
    object: if object.is_empty() {
      AstName::default()
    } else {
      AstName::from_static(object)
    },
    method: AstName::from_static(method),
  };
  get_builtin_function_id(&builtin, &CompileOptions::default())
}

fn check_hits(list: &[Case]) {
  for &(object, method, id) in list {
    assert_eq!(lookup(object, method), id as i32, "表项漏命中: {object:?}");
  }
}

fn check_misses(list: &[Case]) {
  for &(object, method, _) in list {
    assert_eq!(lookup(object, method), -1, "门控关闭仍误命中: {object:?}");
  }
}

/// cpp oracle 的识别总条数：13+33+15+4+2+18+13+37 = 135。
const ORACLE_TOTAL: usize = 135;

#[test]
fn all_oracle_entries_hit_under_matching_flags() {
  let _integer = set_flag(&fflag::LuauIntegerFastcalls, true);
  let _buffer_integer = set_flag(&fflag::LuauIntegerBufferFastcalls, true);
  check_hits(UNGATED);
  check_hits(INTEGER_FASTCALL);
  check_hits(BUFFER_FASTCALL);
  assert_eq!(
    UNGATED.len() + INTEGER_FASTCALL.len() + BUFFER_FASTCALL.len(),
    ORACLE_TOTAL
  );
}

#[test]
fn ungated_entries_hit_with_flags_off() {
  let _integer = set_flag(&fflag::LuauIntegerFastcalls, false);
  let _buffer_integer = set_flag(&fflag::LuauIntegerBufferFastcalls, false);
  check_hits(UNGATED);
}

#[test]
fn gated_entries_miss_without_flags() {
  // 双旗标全关：integer 段与 buffer 整型段均不识别（cpp:258/:294 的 && 前置）。
  let _integer = set_flag(&fflag::LuauIntegerFastcalls, false);
  let _buffer_integer = set_flag(&fflag::LuauIntegerBufferFastcalls, false);
  check_misses(INTEGER_FASTCALL);
  check_misses(BUFFER_FASTCALL);

  // 仅开 LuauIntegerFastcalls：integer 段命中，buffer 段仍不命中（cpp:258 双旗标）。
  let _integer_on = set_flag(&fflag::LuauIntegerFastcalls, true);
  check_hits(INTEGER_FASTCALL);
  check_misses(BUFFER_FASTCALL);

  // 仅开 LuauIntegerBufferFastcalls：两段都不命中（integer 段需 :294 前置旗标）。
  drop(_integer);
  let _integer_off = set_flag(&fflag::LuauIntegerFastcalls, false);
  check_misses(INTEGER_FASTCALL);
  check_misses(BUFFER_FASTCALL);
}

#[test]
fn near_miss_names_are_rejected() {
  let _integer = set_flag(&fflag::LuauIntegerFastcalls, true);
  let _buffer_integer = set_flag(&fflag::LuauIntegerBufferFastcalls, true);
  const NEAR_MISSES: &[(&[u8], &[u8])] = &[
    (b"math", b"maxn"),           // cpp math 段无 maxn
    (b"table", b"foo"),           // 库存在、法不存在
    (b"", b"foo"),                // 全局段无 foo
    (b"MATH", b"abs"),            // 库名大小写错
    (b"math", b"ABS"),            // 法名大小写错
    (b"Math", b"abs"),            // 首字母大写库名
    (b"", b""),                   // 空法名（非空指针）
    (b"vector", b"creater"),      // create 多尾缀
    (b"integer", b"badd"),        // integer 段近邻
    (b"bit32", b"bands"),         // band 多尾缀
    (b"string", b"byt"),          // byte 缺尾缀
    (b"buffer", b"writei64"),     // 宽度假造
    (b"buffer", b"readinteger2"), // 门控段近邻
    (b"tablee", b"insert"),       // 库名多尾缀
  ];
  for &(object, method) in NEAR_MISSES {
    assert_eq!(lookup(object, method), -1, "近邻假名误命中: {object:?}");
  }
}

#[test]
fn null_method_returns_miss() {
  // cpp `getBuiltin` 永不产出空 method；端口以 null 法名早返回 -1。
  let builtin = Builtin {
    object: AstName::from_static(b"math"),
    method: AstName::default(),
  };
  assert_eq!(
    get_builtin_function_id(&builtin, &CompileOptions::default()),
    -1
  );
}

#[test]
fn implementation_tables_match_oracle_manifest() {
  // 双向对账：实现表每一键都在 cpp 清单里且 id 相等；总条数相等，
  // 从而任何一侧的增删或拼写漂移都撕裂本断言。
  let expected = |object: &[u8], method: &[u8]| -> Option<i32> {
    UNGATED
      .iter()
      .chain(INTEGER_FASTCALL.iter())
      .chain(BUFFER_FASTCALL.iter())
      .find(|&&(o, m, _)| raw(o) == object && raw(m) == method)
      .map(|&(_, _, id)| id as i32)
  };
  for &tables in &[TABLE, INTEGER_TABLE, BUFFER_FASTCALL_TABLE] {
    for &(object, method, id) in tables {
      assert_eq!(
        expected(object, method),
        Some(id as i32),
        "实现表条目未见于 cpp 清单或 id 漂移: {object:?} {method:?}"
      );
    }
  }
  assert_eq!(
    TABLE.len() + INTEGER_TABLE.len() + BUFFER_FASTCALL_TABLE.len(),
    ORACLE_TOTAL
  );
}
