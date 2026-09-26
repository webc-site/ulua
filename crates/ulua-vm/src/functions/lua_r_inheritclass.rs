//! Source: `VM/src/lclass.cpp:195`（`luaR_inheritclass`，注册辅助为 lclass.cpp:158）

use core::{
  mem::size_of,
  ptr::{copy, copy_nonoverlapping, null_mut},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    c_slice, cstr_cow, lua_h_clone::lua_h_clone, lua_h_getstr::lua_h_getstr,
    lua_h_setstr::lua_h_setstr, lua_m_realloc::lua_m_realloc_,
  },
  macros::{
    getstr::getstr, lua_c_barrier::lua_c_barrier, lua_c_objbarrier::lua_c_objbarrier,
    lua_g_runerror::lua_g_runerror, lua_m_newarray::luaM_newarray, setnvalue::setnvalue,
    setobj_2_class::setobj2class,
  },
  records::{lua_state::LuaState, luau_class::LuauClass, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 指针须有效且指向存活对象；offset/static_member_offset 必须落在已分配区间内。
#[inline]
unsafe fn lua_r_registerstaticmember(
  l: *mut LuaState,
  class_object: *mut LuauClass,
  member_name: *mut tstring,
  val: *const TValue,
  static_member_offset: i32,
) {
  // Safety: 契约保证 child/parent 为存活开放类、静态成员偏移落在已分配数组界内，写引用处补写屏障
  unsafe {
    let dest = (*class_object)
      .staticmembers
      .add(static_member_offset as usize);
    setobj2class!(l, dest, val);
    lua_c_barrier!(l, class_object, dest);

    // We also need to add an entry to the memberstooffset table for this member
    let offset_in_child_int = (*class_object).numberofinstancemembers + static_member_offset;
    let offset_val = lua_h_setstr(l, (*class_object).memberstooffset, member_name);
    setnvalue!(offset_val, offset_in_child_int as f64);
    lua_c_barrier!(l, (*class_object).memberstooffset, offset_val);

    // And add it to offsettomember
    *(*class_object)
      .offsettomember
      .add(offset_in_child_int as usize) = member_name;
  }
}

/// 将 `child` 原地改写为继承 `parent` 后的类（cpp lclass.cpp:195：void、就地
/// 改写 child 的 super/成员数组/数量字段；旧版"另建新类返回"系对 oracle 的
/// 偏差，已对齐——NEWCLASS 臂传入的 child 是本帧克隆，就地改写正是 cpp
/// "克隆后改克隆"协议的落点）。
///
/// # Safety
/// 调用方须保证：`l` 存活且处于受保护帧（分配失败与 runerror 均经 `l` 抛出）；
/// `child`/`parent` 为存活类对象——二者的 `offsettomember`/`staticmembers` 均按
/// 各自 numberof* 字段分配、`memberstooffset` 已注册全部成员名。`child` 会被
/// 本函数重建成员数组（原数组经 luaM_realloc_/丢弃退役），调用方须保证 `child`
/// 已被寄存器/栈持根（NEWCLASS 臂在调用前 setclassvalue 入 ra）。
pub(crate) unsafe fn lua_r_inheritclass(
  l: *mut LuaState,
  child: *mut LuauClass,
  parent: *mut LuauClass,
) {
  // Safety: 契约保证 l/child/parent 存活且字段一致，数组分配/重分配在受保护帧内
  unsafe {
    if !(*parent).isopen {
      lua_g_runerror!(
        l,
        "Non-open class '{}' cannot be extended",
        cstr_cow(getstr((*parent).name))
      );
    }

    // Next, check for illegal instance member overrides
    if (*parent).numberofinstancemembers > 0 {
      // offsettomember 是 `[*mut tstring]` 成员名数组，idx 仅游标 → 切片只读迭代
      for &member_name in c_slice(
        (*parent).offsettomember,
        (*parent).numberofinstancemembers as usize,
      ) {
        let existing = lua_h_getstr((*child).memberstooffset, member_name);
        if !(*existing).is_nil() {
          lua_g_runerror!(
            l,
            "Cannot override instance member '{}' of parent class '{}' in child class '{}'",
            cstr_cow(getstr(member_name)),
            cstr_cow(getstr((*parent).name)),
            cstr_cow(getstr((*child).name))
          );
        }
      }
    }

    (*child).super_ = parent;
    lua_c_objbarrier!(l, child, parent);

    (*child).hasuserinitinchain = (*parent).hasuserinitinchain;

    let child_declared_static_members =
      (*child).numberofallmembers - (*child).numberofinstancemembers;
    let mut child_static_members = (*child).staticmembers;
    let mut child_offset_to_member = (*child).offsettomember;

    (*child).staticmembers = null_mut();
    (*child).offsettomember = null_mut();

    let parent_inst = (*parent).numberofinstancemembers;
    if parent_inst > 0 {
      // Bump every member offset in child->memberstooffset up by
      // parent->numberofinstancemembers (even static members are shifted up),
      // and then add the parent's instance members to child->memberstooffset.
      // 游标化：数组本体 child_offset_to_member 在两个循环内不被改写（仅其
      // 指向的 memberstooffset 表值被写），共享切片借用与表写无别名冲突
      for &member_name in c_slice(child_offset_to_member, (*child).numberofallmembers as usize) {
        let offset_in_child = lua_h_setstr(l, (*child).memberstooffset, member_name);
        setnvalue!(
          offset_in_child,
          (*offset_in_child).as_number() + parent_inst as f64
        );
      }

      for (idx, &member_name) in c_slice((*parent).offsettomember, parent_inst as usize)
        .iter()
        .enumerate()
      {
        let offset_in_child = lua_h_setstr(l, (*child).memberstooffset, member_name);
        // idx 即源串偏移数据本身（cpp `setnvalue(..., idx)`），enumerate 保序等价
        setnvalue!(offset_in_child, idx as f64);
        lua_c_barrier!(l, (*child).memberstooffset, offset_in_child);
      }
    }

    // Count how many static members we'll actually need to copy from parent,
    // ie non-overridden ones（自 parent 实例段起只看静态成员；lua_h_getstr 为纯读
    // 无副作用，计数改 filter+count 链，遍历序与抛错点与原循环逐位一致）
    let static_names = c_slice(
      (*parent).offsettomember.add(parent_inst as usize),
      ((*parent).numberofallmembers - parent_inst).max(0) as usize,
    );
    let num_static_members_to_copy: u32 = static_names
      .iter()
      .filter(|&&member_name| (*lua_h_getstr((*child).memberstooffset, member_name)).is_nil())
      .count() as u32;

    let original_child_all_members = (*child).numberofallmembers;
    let new_numberof_all_members =
      original_child_all_members + parent_inst + num_static_members_to_copy as i32;

    // Resize childOffsetToMember appropriately
    if new_numberof_all_members > original_child_all_members {
      if original_child_all_members == 0 {
        child_offset_to_member =
          luaM_newarray!(l, new_numberof_all_members, *mut tstring, (*child).memcat);
      } else {
        child_offset_to_member = lua_m_realloc_(
          l,
          child_offset_to_member.cast::<u8>(),
          original_child_all_members as usize * size_of::<*mut tstring>(),
          new_numberof_all_members as usize * size_of::<*mut tstring>(),
          (*child).memcat,
        )
        .cast::<*mut tstring>();
      }
    }

    // Make room for parent instance members（同区重叠搬移，须 copy 而非
    // copy_nonoverlapping）
    copy(
      child_offset_to_member,
      child_offset_to_member.add(parent_inst as usize),
      original_child_all_members as usize,
    );

    // Copy parent instance members to the beginning of childOffsetToMember
    copy_nonoverlapping(
      (*parent).offsettomember,
      child_offset_to_member,
      parent_inst as usize,
    );

    (*child).offsettomember = child_offset_to_member;
    (*child).numberofallmembers = new_numberof_all_members;

    // Resize child->staticmembers appropriately
    if num_static_members_to_copy > 0 {
      if child_declared_static_members == 0 {
        child_static_members = luaM_newarray!(
          l,
          num_static_members_to_copy as i32,
          TValue,
          (*child).memcat
        );
      } else {
        child_static_members = lua_m_realloc_(
          l,
          child_static_members.cast::<u8>(),
          child_declared_static_members as usize * size_of::<TValue>(),
          (child_declared_static_members + num_static_members_to_copy as i32) as usize
            * size_of::<TValue>(),
          (*child).memcat,
        )
        .cast::<TValue>();
      }
    }

    (*child).staticmembers = child_static_members;
    (*child).numberofinstancemembers += parent_inst;

    // Copy static members from parent that aren't overridden in child.
    // rel 即 parent 静态成员数组下标（原 `idx - parent_inst` 的差值语义）；
    // 读侧全程为 parent 数组的共享切片，写侧只落 child 的数组/表——两对象分配互不
    // 重叠（cpp 同款前提），无别名冲突
    let mut num_static_members_copied: i32 = 0;
    for (rel, &member_name) in static_names.iter().enumerate() {
      // This lookup duplicates the one we did earlier, when we counted how
      // many static members we needed to copy.（cpp 同款重复查表并注明）
      let existing = lua_h_getstr((*child).memberstooffset, member_name);
      if (*existing).is_nil() {
        // This static member isn't declared in the child, so we need to copy
        // it over from the parent
        let static_member_offset_in_child =
          child_declared_static_members + num_static_members_copied;

        let parent_val = (*parent).staticmembers.add(rel);

        lua_r_registerstaticmember(
          l,
          child,
          member_name,
          parent_val,
          static_member_offset_in_child,
        );

        num_static_members_copied += 1;
      }
    }

    LUAU_ASSERT!(num_static_members_copied == num_static_members_to_copy as i32);

    // Copy instance metatable（lclass.cpp:335-341）：父类实例元表（含 __tostring
    // 等元方法与 __index 挂接）克隆进子类——child 自身尚无实例元表
    if !(*parent).instancemetatable.is_null() {
      LUAU_ASSERT!((*child).instancemetatable.is_null());
      (*child).instancemetatable = lua_h_clone(l, (*parent).instancemetatable);
      lua_c_objbarrier!(l, child, (*child).instancemetatable);
    }
  }
}
