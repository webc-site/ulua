//! `luauF_sqrt` 快速调用面契约（对照 cpp lbuiltins.cpp:393-403 `luauF_sqrt`：
//! 数值单参快速调用，返回 1 槽；非数值或异常结果需求回退 -1 慢路径）。

use core::ptr::null_mut;

use ulua_vm::{
  enums::lua_type::LuaType, functions::luau_f_sqrt::luau_f_sqrt, macros::setnvalue::setnvalue,
  type_aliases::t_value::TValue,
};

fn call_sqrt(a1: f64) -> Result<f64, i32> {
  let mut arg0 = TValue::default();
  let mut out = [TValue::default(); 1];

  unsafe {
    setnvalue!(&mut arg0, a1);

    let n = luau_f_sqrt(null_mut(), out.as_mut_ptr(), &mut arg0, 1, None, 1);

    if n < 0 {
      return Err(n);
    }

    assert_eq!(n, 1, "sqrt 固定返回 1 个结果");
    assert_eq!(out[0].tt, LuaType::Number as i32, "结果槽未写成 number");

    Ok(out[0].value.n)
  }
}

#[test]
fn sqrt_computes_exact_value() {
  assert_eq!(call_sqrt(4.0), Ok(2.0));
  assert_eq!(call_sqrt(9.0), Ok(3.0));
  assert_eq!(call_sqrt(0.0), Ok(0.0));
  assert_eq!(call_sqrt(100.0), Ok(10.0));
}

#[test]
fn sqrt_handles_ieee_corner_cases() {
  assert_eq!(call_sqrt(f64::INFINITY), Ok(f64::INFINITY));
  let neg_zero = call_sqrt(-0.0).unwrap();
  assert_eq!(neg_zero, 0.0);
  assert!(neg_zero.is_sign_negative());

  let nan_res = call_sqrt(-1.0).unwrap();
  assert!(nan_res.is_nan());
}

#[test]
fn sqrt_falls_back_when_not_number() {
  let mut arg0 = TValue::default(); // default 为 nil
  let mut out = [TValue::default(); 1];

  unsafe {
    let n = luau_f_sqrt(null_mut(), out.as_mut_ptr(), &mut arg0, 1, None, 1);
    assert_eq!(n, -1, "非数值必须回退到慢路径");
  }
}

#[test]
fn sqrt_falls_back_when_more_results_needed() {
  let mut arg0 = TValue::default();
  let mut out = [TValue::default(); 2];

  unsafe {
    setnvalue!(&mut arg0, 4.0);
    // nresults = 2（调用方要求两个结果，需要慢路径补 nil）
    let n = luau_f_sqrt(null_mut(), out.as_mut_ptr(), &mut arg0, 2, None, 1);
    assert_eq!(n, -1, "nresults > 1 必须回退到慢路径");
  }
}
