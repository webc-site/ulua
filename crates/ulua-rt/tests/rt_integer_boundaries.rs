//! Integer-conversion boundary tests for the f64-backed `Value::Integer` /
//! `Value::Number` reconstruction and the 128-bit integer conversions.
//!
//! Luau stores every number as an `f64`; ulua-rt reconstructs the
//! `Integer`/`Number` split by testing whether the `f64` is an exact, in-range
//! whole number. The boundary is subtle: `i64::MAX as f64` rounds UP to 2^63,
//! which is one past the largest representable `i64`, so `2^63` must NOT be
//! presented as an integer (a `<= i64::MAX as f64` check would let it through
//! and saturate it to `i64::MAX` on the cast). These tests pin the corrected
//! behavior:
//!
//! * `2^63` (exactly representable in `f64`, out of `i64` range) -> `Number`,
//!   and `coerce_integer` -> `None` (matching Lua's `math.tointeger`).
//! * `9223372036854774784` (2^63 - 1024, the largest exact `i64` in `f64`)
//!   -> `Integer`.
//! * `-2^63` (`i64::MIN`, exactly representable) -> `Integer`.
//! * 128-bit conversions reject floats beyond the target's range instead of
//!   silently saturating (`1e300 as i128`, `-1 as u128`).

use ulua_rt::{Integer, Number, Value, prelude::*};

#[test]
fn test_value_from_stack_integer_boundary() -> Result<()> {
  let lua = Lua::new();

  // 2^63：f64 可精确表示，但超出 i64 范围 -> 必须是 Number，不是 Integer。
  let v = lua.load("return 9223372036854775808").eval::<Value>()?;
  assert!(
    matches!(v, Value::Number(_)),
    "2^63 must be Number, got {v:?}"
  );

  // 2^63 - 1024：f64 中可精确表示的最大 i64 -> Integer。
  let v = lua.load("return 9223372036854774784").eval::<Value>()?;
  assert_eq!(v, Value::Integer(9223372036854774784));

  // -2^63（i64::MIN）：f64 可精确表示 -> Integer。
  let v = lua.load("return -9223372036854775808").eval::<Value>()?;
  assert_eq!(v, Value::Integer(i64::MIN));

  // 2^62：普通大整数 -> Integer。
  let v = lua.load("return 4611686018427387904").eval::<Value>()?;
  assert_eq!(v, Value::Integer(4611686018427387904));
  Ok(())
}

#[test]
fn test_coerce_integer_boundary() -> Result<()> {
  let lua = Lua::new();

  // 2^63：超出 i64 -> None（对齐 Lua math.tointeger）。
  assert_eq!(
    lua.coerce_integer(lua.load("return 2^63").eval::<Value>()?)?,
    None
  );
  // 2^63 - 1024：可精确表示 -> Some。
  assert_eq!(
    lua.coerce_integer(lua.load("return 9223372036854774784").eval::<Value>()?)?,
    Some(9223372036854774784)
  );
  // -2^63：i64::MIN -> Some(i64::MIN)。
  assert_eq!(
    lua.coerce_integer(lua.load("return -2^63").eval::<Value>()?)?,
    Some(i64::MIN)
  );
  // 1e300：超出 i64 -> None。
  assert_eq!(
    lua.coerce_integer(lua.load("return 1e300").eval::<Value>()?)?,
    None
  );
  // 2^63 - 512：超出 i64 且 f64 间距 1024，向下取整后是 2^63 - 1024？不，
  // 2^63 - 512 无法被 f64 精确表示，实际存储为最近的 f64（2^63 - 1024）。
  // 精确可表示时才为 Integer。
  let v = lua.load("return 9223372036854775296").eval::<Value>()?;
  // 9223372036854775296 = 2^63 - 512，f64 间距 1024 -> 取整到 2^63。
  assert!(matches!(v, Value::Number(_)), "got {v:?}");
  Ok(())
}

#[test]
fn test_i128_overflow_rejected() -> Result<()> {
  let lua = Lua::new();

  // 1e300 超出 i128（约 1.7e38）-> 报错而非静默饱和。
  assert!(i128::from_lua(lua.load("return 1e300").eval::<Value>()?, &lua).is_err());
  // u128 同样拒绝。
  assert!(u128::from_lua(lua.load("return 1e300").eval::<Value>()?, &lua).is_err());
  // 负数对 u128 必须拒绝（`-1 as u128` 会饱和为 0）。
  assert!(u128::from_lua(Value::Integer(-1), &lua).is_err());
  // 范围内的 128 位整数正常转换。
  assert_eq!(
    i128::from_lua(lua.load("return 1e30").eval::<Value>()?, &lua)?,
    1e30 as i128
  );
  // 128 位整数值经 f64 往返（有损，见 mlua 对齐说明）。
  let v = 1234567890123456789i128.into_lua(&lua)?;
  assert_eq!(v, Value::Number(1234567890123456789i128 as f64));
  Ok(())
}

#[test]
fn test_integer_round_trip_boundaries() -> Result<()> {
  let lua = Lua::new();

  // i64::MAX / i64::MIN 字面量（f64 精度内）往返。
  let max: Integer = lua.load("return 9223372036854774784").eval()?;
  assert_eq!(max, 9223372036854774784);
  let min: Integer = lua.load("return -9223372036854775808").eval()?;
  assert_eq!(min, i64::MIN);

  // ulua 的 Luau 基础库没有 math.maxinteger/mininteger（读取为 nil，
  // Lua 5.3+ 概念）。
  let v = lua.load("return math.maxinteger").eval::<Value>().unwrap();
  assert!(
    v.is_nil(),
    "math.maxinteger must be absent (nil), got {v:?}"
  );

  // 2^53（f64 精确整数边界）-> Integer。
  let v = lua.load("return 2^53").eval::<Value>()?;
  assert_eq!(v, Value::Integer(9007199254740992));

  // 非整数浮点 -> Number。
  let v = lua.load("return 1.5").eval::<Value>()?;
  assert_eq!(v, Value::Number(1.5));
  Ok(())
}

#[test]
fn test_vec_from_lua_out_of_range_rejected() -> Result<()> {
  let lua = Lua::new();

  // 表中放 2^63，Vec<i64> 转换必须拒绝（不能静默饱和）。
  let t = lua.load("return {9223372036854775808}").eval::<Value>()?;
  assert!(Vec::<i64>::from_lua(t, &lua).is_err());

  // 正常序列转换。
  let t = lua.load("return {1, 2, 3}").eval::<Value>()?;
  assert_eq!(Vec::<i64>::from_lua(t, &lua)?, vec![1, 2, 3]);
  Ok(())
}

#[test]
fn test_number_type_aliases() {
  // Integer/Number 别名与 mlua 一致（i64/f64）。
  fn assert_i64(_: i64) {}
  fn assert_f64(_: f64) {}
  assert_i64(0 as Integer);
  assert_f64(0 as Number);
}

#[test]
fn test_raw_insert_remove_bounds() -> Result<()> {
  let lua = Lua::new();

  // 对齐 C++ `table.insert`/`table.remove`：越界索引报错。
  let t = lua.create_sequence_from([1, 2, 3])?;
  // insert 允许 [1, n+1]；n+2 越界。
  assert!(t.raw_insert(0, 0).is_err());
  assert!(t.raw_insert(5, 0).is_err());
  // remove 允许 [1, n]；0 和 n+1 越界。
  assert!(t.raw_remove(0).is_err());
  assert!(t.raw_remove(4).is_err());
  // 合法边界：insert(n+1) 追加，remove(n) 弹出。
  t.raw_insert(4, 4)?;
  assert_eq!(t.raw_get::<i64>(4)?, 4);
  assert_eq!(t.raw_remove(4)?, Value::Integer(4));
  // 空表 remove -> nil。
  let empty = lua.create_table();
  assert!(empty.raw_remove(1)?.is_nil());
  Ok(())
}
