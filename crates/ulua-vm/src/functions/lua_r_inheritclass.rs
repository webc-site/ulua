//! Node: `cxx:Function:Luau.VM:VM/src/lclass.cpp:156:luaR_inheritclass`

use core::{ffi::CStr, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    getcurrenv::getcurrenv,
    lua_h_clone::lua_h_clone,
    lua_h_getstr::lua_h_getstr,
    lua_h_new::lua_h_new,
    lua_h_setstr::lua_h_setstr,
    lua_r_newclass::{lua_r_newblankclass, lua_r_setupconstructor},
  },
  macros::{
    getstr::getstr, lua_c_barrier::luaC_barrier, lua_c_objbarrier::luaC_objbarrier,
    lua_g_runerror::luaG_runerror, lua_m_newarray::luaM_newarray, setnvalue::setnvalue,
    setobj_2_class::setobj2class, ttisnil::ttisnil,
  },
  records::{lua_state::lua_State, luau_class::LuauClass, t_string::tstring},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn lua_r_registerstaticmember(
  l: *mut lua_State,
  class_object: *mut LuauClass,
  member_name: *mut tstring,
  val: *const TValue,
  offset: u32,
  static_member_offset: u32,
) {
  unsafe {
    let dest = (*class_object)
      .staticmembers
      .add(static_member_offset as usize);
    setobj2class!(l, dest, val);
    luaC_barrier!(l, class_object, dest);

    *(*class_object).offsettomember.add(offset as usize) = member_name;

    let offset_val = lua_h_setstr(l, (*class_object).memberstooffset, member_name);
    setnvalue!(offset_val, offset as f64);
    luaC_barrier!(l, (*class_object).memberstooffset, offset_val);
  }
}

pub(crate) unsafe fn lua_r_inheritclass(
  l: *mut lua_State,
  child: *const LuauClass,
  parent: *mut LuauClass,
) -> *mut LuauClass {
  unsafe {
    if !(*parent).isopen {
      luaG_runerror!(
        l,
        "Non-open class '{}' cannot be extended",
        CStr::from_ptr(getstr((*parent).name)).to_string_lossy()
      );
    }

    if (*parent).numberofinstancemembers > 0 {
      for idx in 0..(*parent).numberofinstancemembers as u32 {
        let member_name = *(*parent).offsettomember.add(idx as usize);
        let existing = lua_h_getstr((*child).memberstooffset, member_name);
        if !existing.is_null() && !ttisnil!(existing) {
          luaG_runerror!(
            l,
            "Cannot override instance member '{}' of parent class '{}' in child class '{}'",
            CStr::from_ptr(getstr(member_name)).to_string_lossy(),
            CStr::from_ptr(getstr((*parent).name)).to_string_lossy(),
            CStr::from_ptr(getstr((*child).name)).to_string_lossy()
          );
        }
      }
    }

    let new_class = lua_r_newblankclass(l, (*child).name, (*child).isopen);
    (*new_class).super_ = parent;

    let mut num_static_members_to_copy: u32 = 0;
    for idx in (*parent).numberofinstancemembers as u32..(*parent).numberofallmembers as u32 {
      let member_name = *(*parent).offsettomember.add(idx as usize);
      let existing = lua_h_getstr((*child).memberstooffset, member_name);
      if existing.is_null() || ttisnil!(existing) {
        num_static_members_to_copy += 1;
      }
    }

    let num_members = (*child).numberofallmembers as u32
      + (*parent).numberofinstancemembers as u32
      + num_static_members_to_copy;

    (*new_class).offsettomember =
      luaM_newarray!(l, num_members as i32, *mut tstring, (*new_class).memcat);
    (*new_class).numberofallmembers = num_members as i32;

    (*new_class).memberstooffset = lua_h_new(l, 0, num_members as i32);
    luaC_objbarrier!(l, new_class, (*new_class).memberstooffset);

    (*new_class).hasuserinitinchain = (*parent).hasuserinitinchain;

    let mut offset: u32 = 0;

    if (*parent).numberofinstancemembers > 0 {
      while offset < (*parent).numberofinstancemembers as u32 {
        let member_name = *(*parent).offsettomember.add(offset as usize);
        *(*new_class).offsettomember.add(offset as usize) = member_name;
        let val = lua_h_setstr(l, (*new_class).memberstooffset, member_name);
        setnvalue!(val, offset as f64);
        luaC_barrier!(l, (*new_class).memberstooffset, val);
        offset += 1;
      }
    }

    if (*child).numberofinstancemembers > 0 {
      for idx in 0..(*child).numberofinstancemembers as u32 {
        let member_name = *(*child).offsettomember.add(idx as usize);
        *(*new_class).offsettomember.add(offset as usize) = member_name;
        let val = lua_h_setstr(l, (*new_class).memberstooffset, member_name);
        setnvalue!(val, offset as f64);
        luaC_barrier!(l, (*new_class).memberstooffset, val);
        offset += 1;
      }
    }

    (*new_class).staticmembers = luaM_newarray!(
      l,
      (num_members - offset) as i32,
      TValue,
      (*new_class).memcat
    );
    (*new_class).numberofinstancemembers = offset as i32;

    let mut num_static_members_copied: u32 = 0;
    for idx in (*parent).numberofinstancemembers as u32..(*parent).numberofallmembers as u32 {
      let member_name = *(*parent).offsettomember.add(idx as usize);
      let existing = lua_h_getstr((*child).memberstooffset, member_name);
      if existing.is_null() || ttisnil!(existing) {
        let parent_val = (*parent)
          .staticmembers
          .add((idx - (*parent).numberofinstancemembers as u32) as usize);
        lua_r_registerstaticmember(
          l,
          new_class,
          member_name,
          parent_val,
          offset,
          num_static_members_copied,
        );
        offset += 1;
        num_static_members_copied += 1;
      }
    }

    for idx in (*child).numberofinstancemembers as u32..(*child).numberofallmembers as u32 {
      let member_name = *(*child).offsettomember.add(idx as usize);
      let child_val = (*child)
        .staticmembers
        .add((idx - (*child).numberofinstancemembers as u32) as usize);
      lua_r_registerstaticmember(
        l,
        new_class,
        member_name,
        child_val,
        offset,
        num_static_members_copied,
      );
      offset += 1;
      num_static_members_copied += 1;
    }

    LUAU_ASSERT!(
      num_static_members_copied
        == num_static_members_to_copy
          + ((*child).numberofallmembers as u32 - (*child).numberofinstancemembers as u32)
    );

    if !(*parent).instancemetatable.is_null() {
      (*new_class).instancemetatable = lua_h_clone(l, (*parent).instancemetatable);
      luaC_objbarrier!(l, new_class, (*new_class).instancemetatable);
    } else {
      (*new_class).instancemetatable = null_mut();
    }

    lua_r_setupconstructor(l, new_class, getcurrenv(l));

    new_class
  }
}
