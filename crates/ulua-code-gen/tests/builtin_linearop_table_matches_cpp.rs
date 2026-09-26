// builtin 三对骨架（load/check/store）宏表与 cpp 逐格对账。
// 权威源：cpp/CodeGen/src/IrTranslateBuiltins.cpp（下称 ITB）。
// 行 token 与生成体同源（见 crate::macros::impl_tables 的 builtin_linearop_impls 段），
// 本测试锁定「表 = cpp 语义」，即「生成函数 = cpp 语义」。

use ulua_code_gen::{
  enums::{ir_cmd::IrCmd, ir_const_kind::IrConstKind},
  functions::builtin_linearop::{BUILTIN_CHECK_ROWS, BUILTIN_LOAD_ROWS, BUILTIN_STORE_ROWS},
};
use ulua_vm::enums::lua_type::LuaType;

// ITB:38-44 `builtinLoadDouble`（LOAD_DOUBLE）、ITB:54-60 `builtinLoadInt64`（LOAD_INT64）
const LOAD_ORACLE: &[(&str, IrCmd)] = &[
  ("builtin_load_double", IrCmd::LoadDouble), // ITB:43 inst(IrCmd::LOAD_DOUBLE, arg)
  ("builtin_load_int_64", IrCmd::LoadInt64),  // ITB:59 inst(IrCmd::LOAD_INT64, arg)
];

// ITB:30-36 `builtinCheckDouble`、ITB:46-52 `builtinCheckInt64`
const CHECK_ORACLE: &[(&str, IrConstKind, LuaType)] = &[
  // ITB:33 constOp(arg).kind == IrConstKind::Double；ITB:35 loadAndCheckTag(.., LUA_TNUMBER, ..)
  ("builtin_check_double", IrConstKind::Double, LuaType::Number),
  // ITB:49 constOp(arg).kind == IrConstKind::Int64；ITB:51 loadAndCheckTag(.., LUA_TINTEGER, ..)
  ("builtin_check_int_64", IrConstKind::Int64, LuaType::Integer),
];

// store 尾：cpp 各翻译器成对出现的 `STORE_*, vmReg(ra)` + `STORE_TAG, constTag(..)`
const STORE_ORACLE: &[(&str, IrCmd, LuaType)] = &[
  // ITB:431-432（btest 尾）STORE_INT + LUA_TBOOLEAN
  (
    "builtin_store_bool_result",
    IrCmd::StoreInt,
    LuaType::Boolean,
  ),
  // ITB:1524-1525（integer.toNumber 整数路）STORE_INT64 + LUA_TINTEGER
  ("builtin_store_int_64", IrCmd::StoreInt64, LuaType::Integer),
  // ITB:921-922（string.len）STORE_DOUBLE + LUA_TNUMBER 无条件打标
  (
    "builtin_store_double_result",
    IrCmd::StoreDouble,
    LuaType::Number,
  ),
  // ITB:445-446（type）STORE_POINTER + LUA_TSTRING
  (
    "builtin_store_string_result",
    IrCmd::StorePointer,
    LuaType::String,
  ),
];

#[test]
fn load_table_matches_cpp() {
  assert_eq!(BUILTIN_LOAD_ROWS.len(), LOAD_ORACLE.len());
  for (row, want) in BUILTIN_LOAD_ROWS.iter().zip(LOAD_ORACLE) {
    assert_eq!(row.0, want.0);
    assert_eq!(row.1, want.1, "{} cmd 漂移", row.0);
  }
}

#[test]
fn check_table_matches_cpp() {
  assert_eq!(BUILTIN_CHECK_ROWS.len(), CHECK_ORACLE.len());
  for (row, want) in BUILTIN_CHECK_ROWS.iter().zip(CHECK_ORACLE) {
    assert_eq!(row.0, want.0);
    assert_eq!(row.1, want.1, "{} 常量种类漂移", row.0);
    assert_eq!(row.2, want.2, "{} 校验标签漂移", row.0);
  }
}

#[test]
fn store_table_matches_cpp() {
  assert_eq!(BUILTIN_STORE_ROWS.len(), STORE_ORACLE.len());
  for (row, want) in BUILTIN_STORE_ROWS.iter().zip(STORE_ORACLE) {
    assert_eq!(row.0, want.0);
    assert_eq!(row.1, want.1, "{} store cmd 漂移", row.0);
    assert_eq!(row.2, want.2, "{} 写入标签漂移", row.0);
  }
}

#[test]
fn table_names_are_unique_across_shapes() {
  // 三表签名互异（load 返回 / check 校验 / store 收尾），但同一函数名不得跨表重复注册
  let mut names: Vec<&str> = BUILTIN_LOAD_ROWS
    .iter()
    .map(|r| r.0)
    .chain(BUILTIN_CHECK_ROWS.iter().map(|r| r.0))
    .chain(BUILTIN_STORE_ROWS.iter().map(|r| r.0))
    .collect();
  let total = names.len();
  names.sort_unstable();
  names.dedup();
  assert_eq!(total, 8);
  assert_eq!(names.len(), total, "存在跨表重复函数名");
}
