use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{tms::TMS, value_view::ValueView},
  functions::{
    call_bin_tm::call_bin_tm, lua_g_aritherror::lua_g_aritherror, lua_v_tonumber::lua_v_tonumber,
    luai_numidiv::luai_numidiv, luai_nummod::luai_nummod,
  },
  macros::{
    luai_numadd::luai_numadd, luai_numdiv::luai_numdiv, luai_nummul::luai_nummul,
    luai_numpow::luai_numpow, luai_numsub::luai_numsub, luai_numunm::luai_numunm,
    setnvalue::setnvalue, setvvalue::setvvalue,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// Number 轴的 payload 读取（cpp `ttisnumber(o)` 判据 + `nvalue(o)` 读对的视图形态）：
/// 非数值槽返回 `None`，即 cpp 真值链的失败分支。`lua_v_tonumber` 成功返回的槽位
/// （原槽或写入的暂存槽）必为 Number 变体。
#[inline]
fn as_number(t: &TValue) -> Option<f64> {
  match ValueView::from_tvalue(t) {
    ValueView::Number(n) => Some(n),
    _ => None,
  }
}

/// cpp `luaV_doarithimpl`（`VM/src/lvmutils.cpp:564`）的浮点/向量部分：`op` 对应
/// `__add` 一类算术事件，两操作数可作数值时走标量快路径，否则退到元方法或算术错误。
///
/// §11 pass B（比较/算术簇）：`ttisvector! + vvalue!`、`ttisnumber! + nvalue!` 的
/// tag→payload 读链收敛为 [`ValueView`] match（变体即 tag）。向量分量以裸指针外溢是
/// 刻意的：`setvvalue!` 写回 `ra` 并可触发 GC，栈槽借用不能跨越；数值则以 `Option<f64>`
/// 带出，暂存槽指针不再外泄。
///
/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn lua_v_doarithimpl(
  l: *mut LuaState,
  ra: StkId,
  rb: *const TValue,
  rc: *const TValue,
  op: TMS,
) {
  // Safety: 契约保证 `l` 为存活调用帧、ra/rb/rc 可读/可写且对齐，块内 TM 调用与数值转换落在该帧栈界内
  unsafe {
    let mut tempb = TValue::default();
    let mut tempc = TValue::default();

    // cpp `ttisvector(o) ? vvalue(o) : nullptr`：判据与 payload 一并经视图取
    let vb = match ValueView::from_tvalue(&*rb) {
      ValueView::Vector(v) => v.as_ptr(),
      _ => null(),
    };
    let vc = match ValueView::from_tvalue(&*rc) {
      ValueView::Vector(v) => v.as_ptr(),
      _ => null(),
    };

    // 三条向量臂的 idiv 通道运算一致（f32 通道 → f64 取整 → f32），
    // 非捕获闭包为 Copy，可按值传给各处 `set_vec_binop`
    let idiv = |a: f32, b: f32| luai_numidiv(a as f64, b as f64) as f32;

    if !vb.is_null() && !vc.is_null() {
      match op {
        TMS::TmAdd => return set_vec_binop(ra, vb, vc, |a, b| a + b),
        TMS::TmSub => return set_vec_binop(ra, vb, vc, |a, b| a - b),
        TMS::TmMul => return set_vec_binop(ra, vb, vc, |a, b| a * b),
        TMS::TmDiv => return set_vec_binop(ra, vb, vc, |a, b| a / b),
        TMS::TmIDiv => return set_vec_binop(ra, vb, vc, idiv),
        // 一元取负：第二个通道指针不会被 `f` 读取，复用 vb
        TMS::TmUnm => return set_vec_binop(ra, vb, vb, |a, _| -a),
        _ => {}
      }
    } else if !vb.is_null() {
      // cpp `ttisnumber(rc) ? nvalue(rc) : luaV_tonumber(rc, &tempc)`：已是数值则短路，
      // 否则让字符串转数写入暂存槽，两侧数值统一经 `as_number` 读取
      let nc = as_number(&*rc)
        .or_else(|| lua_v_tonumber(rc, &mut tempc).and_then(|c_ptr| as_number(&*c_ptr)));
      if let Some(nc) = nc {
        let nc = nc as f32;
        // 标量广播到 4 通道
        let ncs = [nc; 4];
        match op {
          TMS::TmMul => return set_vec_binop(ra, vb, ncs.as_ptr(), |a, b| a * b),
          TMS::TmDiv => return set_vec_binop(ra, vb, ncs.as_ptr(), |a, b| a / b),
          TMS::TmIDiv => return set_vec_binop(ra, vb, ncs.as_ptr(), idiv),
          _ => {}
        }
      }
    } else if !vc.is_null() {
      let nb = as_number(&*rb)
        .or_else(|| lua_v_tonumber(rb, &mut tempb).and_then(|b_ptr| as_number(&*b_ptr)));
      if let Some(nb) = nb {
        let nb = nb as f32;
        // 标量广播到 4 通道
        let nbs = [nb; 4];
        match op {
          TMS::TmMul => return set_vec_binop(ra, nbs.as_ptr(), vc, |a, b| a * b),
          TMS::TmDiv => return set_vec_binop(ra, nbs.as_ptr(), vc, |a, b| a / b),
          TMS::TmIDiv => return set_vec_binop(ra, nbs.as_ptr(), vc, idiv),
          _ => {}
        }
      }
    }

    // cpp 两次 `luaV_tonumber` 都执行（其二可写暂存槽），再判断是否两值可用
    let nb = lua_v_tonumber(rb, &mut tempb).and_then(|b_res| as_number(&*b_res));
    let nc = lua_v_tonumber(rc, &mut tempc).and_then(|c_res| as_number(&*c_res));
    if let (Some(nb), Some(nc)) = (nb, nc) {
      match op {
        TMS::TmAdd => setnvalue!(ra, luai_numadd(nb, nc)),
        TMS::TmSub => setnvalue!(ra, luai_numsub(nb, nc)),
        TMS::TmMul => setnvalue!(ra, luai_nummul(nb, nc)),
        TMS::TmDiv => setnvalue!(ra, luai_numdiv(nb, nc)),
        TMS::TmIDiv => setnvalue!(ra, luai_numidiv(nb, nc)),
        TMS::TmMod => setnvalue!(ra, luai_nummod(nb, nc)),
        TMS::TmPow => setnvalue!(ra, luai_numpow(nb, nc)),
        TMS::TmUnm => setnvalue!(ra, luai_numunm(nb)),
        _ => LUAU_ASSERT!(false),
      }
    } else if call_bin_tm(l, rb, rc, ra, op) == 0 {
      lua_g_aritherror(l, rb, rc, op);
    }
  }
}

/// 向量逐通道二元运算辅助：对 4 通道按分量执行 `f` 并写入 `ra`。
/// `f` 经内联后与逐通道手写展开等价；一元运算（TmUnm）可对两个参数传同一指针。
///
/// # Safety
///
/// `ra` 必须为可写栈槽；`vb`、`vc` 必须各指向 4 个连续可读的 `f32`（向量 payload，
/// `add(0..=3)` 均落在对象内），一元运算可对二者传同一指针；越界读/写即 UB。
#[inline]
unsafe fn set_vec_binop(ra: StkId, vb: *const f32, vc: *const f32, f: impl Fn(f32, f32) -> f32) {
  // Safety: 契约保证 `ra` 为可写栈槽且 `vb`/`vc` 各指向 4 个连续可读 f32，setvvalue! 的 add(0..=3) 读取均落在向量 payload 界内
  unsafe {
    setvvalue!(
      ra,
      f(*vb.add(0), *vc.add(0)),
      f(*vb.add(1), *vc.add(1)),
      f(*vb.add(2), *vc.add(2)),
      f(*vb.add(3), *vc.add(3))
    );
  }
}

/// 8 个算术 tag-method 导出入口（C ABI 注册表用），各自转发到
/// `lua_v_doarithimpl` 并传入对应 `TMS`。
macro_rules! tm_exports {
  ($(($variant:ident, $snake:ident)),+ $(,)?) => {
    $(
      /// # Safety
      /// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
      pub unsafe extern "C-unwind" fn $snake(
        l: *mut LuaState,
        ra: StkId,
        rb: *const TValue,
        rc: *const TValue,
      ) {
        // Safety: 宏按 TMS 变体生成转发壳，参数原样透传给同契约的 `lua_v_doarithimpl`
        unsafe { lua_v_doarithimpl(l, ra, rb, rc, TMS::$variant) }
      }
    )+
  };
}

tm_exports! {
  (TmAdd, lua_v_doarithimpl_tm_add),
  (TmSub, lua_v_doarithimpl_tm_sub),
  (TmMul, lua_v_doarithimpl_tm_mul),
  (TmDiv, lua_v_doarithimpl_tm_div),
  (TmIDiv, lua_v_doarithimpl_tm_idiv),
  (TmMod, lua_v_doarithimpl_tm_mod),
  (TmPow, lua_v_doarithimpl_tm_pow),
  (TmUnm, lua_v_doarithimpl_tm_unm),
}

// §9.3：本模块的行为测试已迁至 `tests/doarith_vector_lanes.rs`，经 C ABI
// 注册表导出的 `lua_v_doarithimpl_tm_*` 八个入口驱动，src 内不再保留单元测试属性。
