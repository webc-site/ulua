//! Source: `VM/src/lclass.cpp:122`
//!
//! cpp `luaR_cloneclass`：按 `classobject` 克隆出新鲜类对象——proto 常量表中
//! 的类形状被标 readonly，NEWCLASS 每次执行必须克隆后再改写（isopen/继承/
//! 增员都只落在克隆上），不得写穿共享常量形状。

use core::ptr::copy_nonoverlapping;

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{
    getcurrenv::getcurrenv,
    lua_h_clone::lua_h_clone,
    lua_r_newclass::{lua_r_newblankclass, lua_r_setupconstructor},
  },
  macros::{iswhite::iswhite, lua_m_newarray::luaM_newarray, obj_2_gco::obj2gco},
  records::{lua_state::LuaState, luau_class::LuauClass, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// 分配并返回一个与 `classobject` 成员相同的新类对象
/// （cpp lclass.cpp:122 `luaR_cloneclass`）。
///
/// # Safety
/// `l` 须存活且处于受保护帧（分配失败经 `l` 抛 ErrMem）；`classobject` 为
/// 存活且成员数组成员按 numberof* 分配完整的类。新类为白色对象，克隆写入
/// 无需写屏障（cpp lclass.cpp:129 同断言）。
pub(crate) unsafe fn lua_r_cloneclass(
  l: *mut LuaState,
  classobject: *mut LuauClass,
) -> *mut LuauClass {
  // Safety: 前置契约保证 l/类对象存活，克隆构造与 lua_h_clone 均在受保护帧内
  unsafe {
    let newclass = lua_r_newblankclass(l, (*classobject).name, (*classobject).isopen);

    // newclass was just allocated, so it is white and none of the writes below
    // need a write barrier.（cpp lclass.cpp:129 同款断言）
    LUAU_ASSERT!(iswhite!(obj2gco!(newclass)));

    (*newclass).super_ = (*classobject).super_;
    (*newclass).hasuserinitinchain = (*classobject).hasuserinitinchain;

    let numallmembers = (*classobject).numberofallmembers;
    let numstaticmembers = numallmembers - (*classobject).numberofinstancemembers;

    // The name->offset mapping is fixed when the class shape is built and is
    // never mutated afterwards (shapes in a Proto's constant table are
    // additionally marked readonly), so the clone shares it rather than paying
    // for a table copy on every class definition that executes.
    (*newclass).memberstooffset = (*classobject).memberstooffset;

    (*newclass).offsettomember = luaM_newarray!(l, numallmembers, *mut tstring, (*newclass).memcat);
    copy_nonoverlapping(
      (*classobject).offsettomember,
      (*newclass).offsettomember,
      numallmembers as usize,
    );

    (*newclass).numberofallmembers = numallmembers;

    (*newclass).staticmembers = luaM_newarray!(l, numstaticmembers, TValue, (*newclass).memcat);
    copy_nonoverlapping(
      (*classobject).staticmembers,
      (*newclass).staticmembers,
      numstaticmembers as usize,
    );

    (*newclass).numberofinstancemembers = (*classobject).numberofinstancemembers;

    if !(*classobject).instancemetatable.is_null() {
      (*newclass).instancemetatable = lua_h_clone(l, (*classobject).instancemetatable);
    }

    lua_r_setupconstructor(l, newclass, getcurrenv(l));

    newclass
  }
}
