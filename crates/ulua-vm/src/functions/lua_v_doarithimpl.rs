use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{tms::TMS, value_view::ValueView},
  functions::{
    call_bin_tm::call_bin_tm, lua_g_aritherror::lua_g_aritherror, lua_v_tonumber::lua_v_tonumber,
    luai_numidiv::luai_numidiv, luai_nummod::luai_nummod,
  },
  macros::{
    lua_vector_size::LUA_VECTOR_SIZE, luai_numadd::luai_numadd, luai_numdiv::luai_numdiv,
    luai_nummul::luai_nummul, luai_numpow::luai_numpow, luai_numsub::luai_numsub,
    luai_numunm::luai_numunm, setnvalue::setnvalue, setvvalue::setvvalue,
  },
  records::{lua_state::LuaState, slot::Slot},
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

/// 向量 payload 的分量窗口：把视图带出的 `LUA_VECTOR_SIZE` 分量切片拷成定长 4 槽
/// 数组（r12 R-D 切片形，与 `vector_shared::vector_components` 同一窗口论证）。
/// 3 分量构建下第 4 位恒 `+0.0` 且**不触碰**槽内存（栈上一个 vector 只占
/// `LUA_VECTOR_SIZE` 个分量位，`get(3)` 落 None 补常量，与收口前不读 `.add(3)` 等值）。
#[inline]
fn lanes(v: &[f32; LUA_VECTOR_SIZE as usize]) -> [f32; 4] {
  [v[0], v[1], v[2], v.get(3).copied().unwrap_or(0.0)]
}

/// cpp `luaV_doarithimpl`（`VM/src/lvmutils.cpp:564`）的浮点/向量部分：`op` 对应
/// `__add` 一类算术事件，两操作数可作数值时走标量快路径，否则退到元方法或算术错误。
///
/// §11 pass B（比较/算术簇）+ r12 R-D 切片形收口：`ttisvector! + vvalue!`、
/// `ttisnumber! + nvalue!` 的 tag→payload 读链收敛为 [`ValueView`] match（变体即
/// tag）。向量分量经 [`lanes`] 拷成 `Option<[f32; 4]>` 定长分量数组（cpp null 哨兵的
/// Option 归一，review §2）：数组为拷贝出的 owned 值，读写窗口与栈槽借用彻底解耦，
/// 写回 `ra` 经 `setvvalue!` 触发 GC 亦不与之相关；数值则以 `Option<f64>` 带出，
/// 暂存槽指针不再外泄。
///
/// # Safety
/// `l` 必须指向存活 `LuaState`；`ra` 为可写结果栈槽句柄，`rb`/`rc` 引用保证可读/对齐；
/// TM 调用协议（res 槽、栈余量）与「扩容先行、借用后派生」不变量由调用侧管辖：
/// `call_bin_tm` 可搬栈，句柄/引用所指槽在调用期间不得迁移。
pub(crate) unsafe fn lua_v_doarithimpl(
  l: *mut LuaState,
  ra: Slot<'_>,
  rb: &TValue,
  rc: &TValue,
  op: TMS,
) {
  // SAFETY: 契约保证 `l` 为存活调用帧、ra 可写且 rb/rc 可读对齐，块内 TM 调用与数值转换落在该帧栈界内
  unsafe {
    // 句柄落回裸槽视图一处：`ra` 在本体内仅作 `setnvalue!`/`setvvalue!`/`call_bin_tm`
    // 的写侧槽地址，`as_ptr` 为 `inline(always)` 指针读出，与原 `StkId` 形参同址同宽度
    // （§9.4）
    let ra = ra.as_ptr();

    let mut tempb = TValue::default();
    let mut tempc = TValue::default();

    // cpp `ttisvector(o) ? vvalue(o) : nullptr`：判据与 payload 一并经视图取，
    // 分量当场拷成定长数组；null 哨兵收敛为 `Option`（review §2）
    let vb = match ValueView::from_tvalue(rb) {
      ValueView::Vector(v) => Some(lanes(v)),
      _ => None,
    };
    let vc = match ValueView::from_tvalue(rc) {
      ValueView::Vector(v) => Some(lanes(v)),
      _ => None,
    };

    // 三条向量臂的 idiv 通道运算一致（f32 通道 → f64 取整 → f32），
    // 非捕获闭包为 Copy，可按值传给各处 `set_vec_binop`
    let idiv = |a: f32, b: f32| luai_numidiv(a as f64, b as f64) as f32;

    if let (Some(vb), Some(vc)) = (vb, vc) {
      match op {
        TMS::TmAdd => return set_vec_binop(ra, vb, vc, |a, b| a + b),
        TMS::TmSub => return set_vec_binop(ra, vb, vc, |a, b| a - b),
        TMS::TmMul => return set_vec_binop(ra, vb, vc, |a, b| a * b),
        TMS::TmDiv => return set_vec_binop(ra, vb, vc, |a, b| a / b),
        TMS::TmIDiv => return set_vec_binop(ra, vb, vc, idiv),
        // 一元取负：第二个分量数组不会被 `f` 读取（只取 `a`），复用 vb
        TMS::TmUnm => return set_vec_binop(ra, vb, vb, |a, _| -a),
        _ => {}
      }
    } else if let Some(vb) = vb {
      // cpp `ttisnumber(rc) ? nvalue(rc) : luaV_tonumber(rc, &tempc)`：已是数值则短路，
      // 否则让字符串转数写入暂存槽，两侧数值统一经 `as_number` 读取
      let nc = as_number(rc).or_else(|| lua_v_tonumber(rc, &mut tempc).and_then(as_number));
      if let Some(nc) = nc {
        let nc = nc as f32;
        // 标量广播到 4 通道（分量数组形，与向量臂同路 `set_vec_binop`）
        let ncs = [nc; 4];
        match op {
          TMS::TmMul => return set_vec_binop(ra, vb, ncs, |a, b| a * b),
          TMS::TmDiv => return set_vec_binop(ra, vb, ncs, |a, b| a / b),
          TMS::TmIDiv => return set_vec_binop(ra, vb, ncs, idiv),
          _ => {}
        }
      }
    } else if let Some(vc) = vc {
      let nb = as_number(rb).or_else(|| lua_v_tonumber(rb, &mut tempb).and_then(as_number));
      if let Some(nb) = nb {
        let nb = nb as f32;
        // 标量广播到 4 通道（分量数组形，与向量臂同路 `set_vec_binop`）
        let nbs = [nb; 4];
        match op {
          TMS::TmMul => return set_vec_binop(ra, nbs, vc, |a, b| a * b),
          TMS::TmDiv => return set_vec_binop(ra, nbs, vc, |a, b| a / b),
          TMS::TmIDiv => return set_vec_binop(ra, nbs, vc, idiv),
          _ => {}
        }
      }
    }

    // cpp 两次 `luaV_tonumber` 都执行（其二可写暂存槽），再判断是否两值可用
    let nb = lua_v_tonumber(rb, &mut tempb).and_then(as_number);
    let nc = lua_v_tonumber(rc, &mut tempc).and_then(as_number);
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
/// `f` 经内联后与逐通道手写展开等价；一元运算（TmUnm）可对两个分量数组传同一值。
///
/// 第 4 通道在 3 分量构建下是 [`lanes`] 补出的常量 `+0.0`，而 `setvvalue!` 的 `$w`
/// 以闭包惰性传入、只在 `LUA_VECTOR_SIZE == 4` 门内求值（macros/setvvalue 的
/// 越界防御 rationale 原样成立），故该 lane 的 `f` 连调用都不发生，与收口前
/// `.add(3)` 读取被门消除逐位等价。
///
/// # Safety
///
/// `ra` 必须为可写栈槽（解引用窗口止于 `setvvalue!` 体内）；`vb`/`vc` 为
/// [`lanes`] 拷出的 owned 分量数组，无内存安全前提。
#[inline]
unsafe fn set_vec_binop(ra: StkId, vb: [f32; 4], vc: [f32; 4], f: impl Fn(f32, f32) -> f32) {
  // SAFETY: 契约保证 `ra` 为可写栈槽，setvvalue! 的 lane0..=2 写入与门内 lane3 写入均落在该槽 TValue 内
  unsafe {
    setvvalue!(
      ra,
      f(vb[0], vc[0]),
      f(vb[1], vc[1]),
      f(vb[2], vc[2]),
      f(vb[3], vc[3])
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
        // SAFETY: 宏按 TMS 变体生成转发壳；导出签名与符号不动，在边界显式重建
        // 可写槽句柄与只读借用后转调同契约的 `lua_v_doarithimpl`，解引用窗口止于本调用
        unsafe { lua_v_doarithimpl(l, Slot::from_raw(ra), &*rb, &*rc, TMS::$variant) }
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
