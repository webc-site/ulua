use ulua_common::LUAU_ASSERT;

use crate::{
  enums::{tms::TMS, value_view::ValueView},
  functions::{
    lua_h_getstr::lua_h_getstr, lua_h_new::lua_h_new, lua_h_setstr::lua_h_setstr,
    lua_s_newlstr::lua_s_newlstr,
  },
  macros::{
    lua_c_barrier::lua_c_barrier, lua_c_objbarrier::lua_c_objbarrier, setobj_2_class::setobj2class,
    setobj_2_t::setobj2t,
  },
  records::{
    global_state::global_State, lua_state::LuaState, luau_class::LuauClass, t_string::tstring,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// 调用方须保证：`l` 存活且处于受保护帧（建新串、扩容 metatable 可触发 GC/抛错）；`classobject` 为构造
/// 完整的存活类——`name` 已在 `memberstooffset` 注册且偏移落在静态成员区间
/// [numberofinstancemembers, numberofallmembers)，`staticmembers` 按该区间长度分配；`value` 为函数值。
/// 偏移越界即对 staticmembers 越界写。cpp lclass.cpp:344 `luaR_addclassmember`
pub(crate) unsafe fn lua_r_addclassmember(
  l: *mut LuaState,
  classobject: *mut LuauClass,
  name: *mut tstring,
  value: *mut TValue,
) {
  // Safety: 契约保证 `l` 存活且 class 相关指针指向存活 LuauClass，成员偏移落在已分配数组界内，写引用处均按协议补 luaC_barriert 写屏障
  unsafe {
    LUAU_ASSERT!(!(*classobject).staticmembers.is_null());

    let offset = lua_h_getstr((*classobject).memberstooffset, name);
    // tag 判定收敛为 ValueView 变体 match（§11 pass B）；payload 仍由该断言兜底的
    // `nvalue!` 读取，release 断言编译掉后行为与收敛前逐位一致
    LUAU_ASSERT!(matches!(
      ValueView::from_tvalue(&*offset),
      ValueView::Number(_)
    ));
    let offsetint = (*offset).as_number() as i32;
    LUAU_ASSERT!(
      offsetint >= (*classobject).numberofinstancemembers
        && offsetint < (*classobject).numberofallmembers
    );
    // gch.tt 一项走 GC 对象头轴（Function tag 与 UpVal 之分），非 ValueView 的
    // TValue tag 轴，保留原样；tag 轴判定收敛为 ValueView::Function 臂 match
    LUAU_ASSERT!(
      matches!(ValueView::from_tvalue(&*value), ValueView::Function(_))
        && (*(*value).value.gc).header().is_closure()
    );
    setobj2class!(
      l,
      (*classobject)
        .staticmembers
        .add((offsetint - (*classobject).numberofinstancemembers) as usize),
      value
    );
    lua_c_barrier!(l, classobject, value);

    (*classobject).hasuserinitinchain |= name == lua_s_newlstr(l, b"__init");

    // Only metamethods in the parser's allowlist are supported (see ALLOWED_METAMETHODS in Parser.cpp)
    let g: *mut global_State = (*l).global;
    let is_metamethod = name == lua_s_newlstr(l, b"__tostring")
      || (*g)
        .tmname
        .iter()
        .take(TMS::TmN as usize)
        .any(|&tmname| name == tmname);

    if is_metamethod {
      if (*classobject).instancemetatable.is_null() {
        (*classobject).instancemetatable = lua_h_new(l, 0, 1);
        lua_c_objbarrier!(l, classobject, (*classobject).instancemetatable);
      }
      let dest = lua_h_setstr(l, (*classobject).instancemetatable, name);
      setobj2t!(l, dest, value);
      lua_c_barrier!(l, (*classobject).instancemetatable, value);
    }
  }
}
