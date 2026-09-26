use core::ptr::{eq, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{tms::TMS, value_view::ValueView},
  functions::{call_t_mres::call_t_mres, get_comp_tm::get_comp_tm, luai_veceq::luai_veceq},
  macros::{
    gcvalue::gcvalue, l_isfalse::l_isfalse, luai_inteq::luai_inteq, luai_numeq::luai_numeq,
    ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// cpp `luaV_equalval`（`VM/src/lvmutils.cpp:451`）：同 tag 两值的 `==` 语义。
///
/// §11 pass B（比较/算术簇）：cpp 的 `switch (ttype(t1))` 在入口断言两侧同 tag 的前提下
/// 收敛为 [`ValueView`] 成对变体 match——变体即 tag，`nvalue!`/`lvalue!`/`vvalue!`/
/// `bvalue!`/`pvalue!`/`lightuserdatatag!`/`uvalue!`/`classvalue!`/`objectvalue!`/`hvalue!`
/// 的双侧读链由视图直接带出。`Class`/`Object` 因有真实 payload 读取而是一级变体；
/// cpp `default:` 的 GC 对象同一性比较（String/Function/Thread/Buffer 等未列出 tag，以及
/// Proto/Upval 等内部 tag）保持 `gcvalue!` 形状：视图变体不带 union 基址 payload。
///
/// 寿命契约与 [`ValueView`] 相同：视图借用只活在 match 内，元方法调用 `call_t_mres`
/// （可 GC、可搬栈）在 match 之外执行。
///
/// # Safety
/// `l` 必须指向存活 `lua_State`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn lua_v_equalval(l: *mut LuaState, t1: *const TValue, t2: *const TValue) -> i32 {
  // Safety: 视图构造与各读取宏均只读已初始化、tag 有效的 TValue 槽；`luai_veceq` 按
  // value+extra 连续布局读满分量，`call_t_mres` 走标准 TM 调用协议
  unsafe {
    let tm: *const TValue;
    LUAU_ASSERT!(ttype!(t1) == ttype!(t2));

    match (ValueView::from_tvalue(&*t1), ValueView::from_tvalue(&*t2)) {
      (ValueView::Nil, ValueView::Nil) => return 1,
      (ValueView::Number(a), ValueView::Number(b)) => return luai_numeq(a, b) as i32,
      (ValueView::Integer(a), ValueView::Integer(b)) => return luai_inteq(a, b) as i32,
      (ValueView::Vector(a), ValueView::Vector(b)) => {
        return luai_veceq(a.as_ptr(), b.as_ptr()) as i32;
      }
      // `bvalue!` 原样 i32 比较（cpp 注释 "true must be 1"），不折算 bool
      (ValueView::Boolean(a), ValueView::Boolean(b)) => return (a == b) as i32,
      (
        ValueView::LightUserdata {
          pointer: p1,
          tag: tag1,
        },
        ValueView::LightUserdata {
          pointer: p2,
          tag: tag2,
        },
      ) => return (p1 == p2 && tag1 == tag2) as i32,
      (ValueView::Userdata(u1), ValueView::Userdata(u2)) => {
        tm = get_comp_tm(l, (*u1).metatable, (*u2).metatable, TMS::TmEq);
        if tm.is_null() {
          return eq(u1, u2) as i32;
        }
      }
      (ValueView::Class(c1), ValueView::Class(c2)) => return eq(c1, c2) as i32,
      (ValueView::Object(o1), ValueView::Object(o2)) => {
        // 与 metatable 同规则：比较两侧实例类的 instancemetatable 上的 __eq
        // （`lclass` 为空对应 cpp 同址解引用前的既有判空加固，行为不变）
        let mt1 = if (*o1).lclass.is_null() {
          null_mut()
        } else {
          (*(*o1).lclass).instancemetatable
        };
        let mt2 = if (*o2).lclass.is_null() {
          null_mut()
        } else {
          (*(*o2).lclass).instancemetatable
        };
        tm = get_comp_tm(l, mt1, mt2, TMS::TmEq);
        if tm.is_null() {
          return eq(o1, o2) as i32;
        }
      }
      (ValueView::Table(h1), ValueView::Table(h2)) => {
        tm = get_comp_tm(l, h1.metatable, h2.metatable, TMS::TmEq);
        if tm.is_null() {
          return eq(h1, h2) as i32;
        }
      }
      // cpp `default: return gcvalue(t1) == gcvalue(t2)`：其余 collectable tag
      // （String/Function/Thread/Buffer）与 Proto/Upval 等内部 tag 按 GC 对象同一性
      // 比较。tag 不等已由入口 LUAU_ASSERT 排除（cpp 亦以断言为前提，其 release 形态
      // 在该态下按 t1 的 tag 误读 t2 payload，本臂的 union 基址比较不新增行为）
      _ => return eq(gcvalue!(t1), gcvalue!(t2)) as i32,
    }

    call_t_mres(l, (*l).top, tm, t1, t2);
    if !l_isfalse!((*l).top) { 1 } else { 0 }
  }
}

/// # Safety
/// `l` 必须指向存活 `lua_State`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub unsafe extern "C-unwind" fn lua_v_equalval_export(
  l: *mut LuaState,
  t1: *const TValue,
  t2: *const TValue,
) -> i32 {
  // Safety: 导出壳原样转发同契约 `lua_v_equalval`；t1/t2 为存活对齐 TValue 指针
  unsafe { lua_v_equalval(l, t1, t2) }
}
