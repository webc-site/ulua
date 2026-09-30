//! `luauF_modf` 快速调用面契约（对照 cpp lbuiltins.cpp:330 `luauF_modf`：
//! `res = ip`、`res + 1 = fp` 的落栈次序与 C `modf` 语义；返回 -1 时
//! VM 回退慢路径，见 lvmexecute.cpp `FASTCALL1` 的 `n >= 0` 分支）。

use core::ptr::null_mut;

use ulua_vm::{
  enums::lua_type::LuaType, functions::luau_f_modf::luau_f_modf, macros::setnvalue::setnvalue,
  type_aliases::t_value::TValue,
};

/// 以 `LUAU_F_TABLE` 的调用约定直接驱动 `luau_f_modf`：
/// 只需两个 `TValue`（`lua_State` 与 `args` 在本实现里未被使用）。
fn call(a1: f64) -> Result<[f64; 2], i32> {
  let mut arg0 = TValue::default();
  let mut out = [TValue::default(); 2];

  unsafe {
    setnvalue!(&mut arg0, a1);

    let n = luau_f_modf(null_mut(), out.as_mut_ptr(), &mut arg0, 2, None, 1);

    if n < 0 {
      return Err(n);
    }

    assert_eq!(n, 2, "modf 固定返回两个结果");
    for slot in &out {
      assert_eq!(slot.tt, LuaType::Number as i32, "结果槽未写成 number");
    }

    Ok([out[0].value.n, out[1].value.n])
  }
}

/// 断言 `v` 是带 `negative` 符号的零（`== 0.0` 区分不了 ±0）。
fn assert_signed_zero(v: f64, negative: bool) {
  assert_eq!(v, 0.0, "期望零，实际 {v}");
  assert_eq!(v.is_sign_negative(), negative, "零的符号位不符");
}

#[test]
fn modf_writes_integer_then_fraction() {
  // 上游次序：res = ip，res + 1 = fp
  assert_eq!(call(3.5), Ok([3.0, 0.5]));
  assert_eq!(call(-3.5), Ok([-3.0, -0.5]));
  assert_eq!(call(0.5), Ok([0.0, 0.5]));
}

#[test]
fn modf_of_infinite_keeps_signed_zero_fraction() {
  let [ip, fp] = call(f64::INFINITY).unwrap();
  assert_eq!(ip, f64::INFINITY);
  assert_signed_zero(fp, false);

  let [ip, fp] = call(f64::NEG_INFINITY).unwrap();
  assert_eq!(ip, f64::NEG_INFINITY);
  assert_signed_zero(fp, true);
}

#[test]
fn modf_of_nan_yields_nan_pair() {
  let [ip, fp] = call(f64::NAN).unwrap();
  assert!(ip.is_nan() && fp.is_nan());
}

#[test]
fn modf_of_zero_keeps_sign() {
  let [ip, fp] = call(-0.0).unwrap();
  assert_signed_zero(ip, true);
  assert_signed_zero(fp, true);

  let [ip, fp] = call(0.0).unwrap();
  assert_signed_zero(ip, false);
  assert_signed_zero(fp, false);
}

#[test]
fn modf_of_integral_and_wide_values_is_exact() {
  let [ip, fp] = call(-7.0).unwrap();
  assert_eq!(ip, -7.0);
  assert_signed_zero(fp, true);

  // |x| >= 2^52 后 double 全是整数：`x - x.fract()` 式的写法会失真，
  // 截断值必须与输入逐位相同
  let large = 9_007_199_254_740_993.0_f64; // 2^53 + 1，舍入到 2^53
  let [ip, fp] = call(large).unwrap();
  assert_eq!(ip, large);
  assert_signed_zero(fp, false);
}

#[test]
fn modf_falls_back_on_unsupported_shape() {
  // 非 number 实参 / 结果数 > 2 / 无实参：返回 -1 让 VM 走慢路径
  let mut arg0 = TValue::default();
  let mut out = [TValue::default(); 3];

  unsafe {
    assert_eq!(
      luau_f_modf(null_mut(), out.as_mut_ptr(), &mut arg0, 2, None, 1),
      -1,
      "nil 实参不应走快速路径"
    );

    setnvalue!(&mut arg0, 1.5);
    assert_eq!(
      luau_f_modf(null_mut(), out.as_mut_ptr(), &mut arg0, 3, None, 1),
      -1,
      "要求 3 个结果时上游回退"
    );
    assert_eq!(
      luau_f_modf(null_mut(), out.as_mut_ptr(), &mut arg0, 2, None, 0),
      -1,
      "无实参时上游回退"
    );
  }
}
