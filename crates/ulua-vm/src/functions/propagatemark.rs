//! Source: `VM/src/lgc.cpp` (lgc.cpp:504-590, hand-ported)

use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    clearstack::clearstack, shrinkstackprotected::shrinkstackprotected,
    traverseclass::traverseclass, traverseclosure::traverseclosure, traverseobject::traverseobject,
    traverseproto::traverseproto, traversestack::traversestack, traversetable::traversetable,
  },
  macros::{
    black_2_gray::black2gray, dummynode::dummynode, gc_satomic::GCSATOMIC,
    gc_spropagate::GCSPROPAGATE, gray_2_black::gray2black, isgray::isgray,
    size_cclosure::size_cclosure, size_lclosure::size_lclosure, sizenode::sizenode,
  },
  records::{
    call_info::CallInfo, closure::Closure, gc_object::GcViewMut, global_state::global_State,
    loc_var::LocVar, lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable,
    luau_class::LuauClass, luau_object::LuauObject, proto::Proto, t_string::tstring,
  },
  type_aliases::{instruction::Instruction, t_value::TValue},
};

/// # Safety
/// 仅在 gcstate 为 GCSpropagate/GCSatomic 且 `(*g).gray` 非空时调用（首行断言即解引用）：灰链头
/// 须为存活的 Table/Function/Thread/Proto/Class/Object 灰对象且 gclist 字段与当前扫描方向一致；
/// 类型混淆或对已回收对象调用即悬垂标记/越界遍历。返回本步 traversed 工作量（字节数）。
/// cpp lgc.cpp:541 `propagatemark`
// traverse one gray object, turning it to black.
// Returns `quantity' traversed.
pub(crate) unsafe fn propagatemark(g: *mut global_State) -> usize {
  // Safety: 契约保证 `g` 存活且 currentwhite 与灰队列遍历方向一致，块内逐个灰对象的引用遍历不越过对象界
  unsafe {
    let o = (*g).gray;
    LUAU_ASSERT!(isgray!(o));
    gray2black!(o);
    match (*o).as_view_mut() {
      Some(GcViewMut::Table(h)) => {
        let h_ptr = h as *mut LuaTable;
        (*g).gray = h.gclist;
        if traversetable(g, h_ptr) != 0 {
          // table is weak?
          black2gray!(o); // keep it gray
        }
        // cpp lgc.cpp:547：空哈希部（node == dummynode）计 0
        let hashsize = if h.node == dummynode.cast_mut() {
          0
        } else {
          sizenode!(h_ptr) as usize
        };
        size_of::<LuaTable>()
          + size_of::<TValue>() * h.sizearray as usize
          + size_of::<LuaNode>() * hashsize
      }
      Some(GcViewMut::Closure(cl)) => {
        let cl_ptr = cl as *mut Closure;
        (*g).gray = cl.gclist;
        traverseclosure(g, cl_ptr);
        if cl.is_c != 0 {
          size_cclosure(cl.nupvalues as i32)
        } else {
          size_lclosure(cl.nupvalues as usize)
        }
      }
      Some(GcViewMut::Thread(th)) => {
        let th_ptr = th as *mut LuaState;
        (*g).gray = th.gclist;

        let active = th.isactive || th_ptr == (*(*th_ptr).global).mainthread;

        traversestack(g, th_ptr);

        // active threads will need to be rescanned later to mark new stack writes so we mark them gray again
        if active {
          th.gclist = (*g).grayagain;
          (*g).grayagain = o;

          black2gray!(o);
        }

        // the stack needs to be cleared after the last modification of the thread state before sweep begins
        // if the thread is inactive, we might not see the thread in this cycle so we must clear it now
        if !active || (*g).gcstate as i32 == GCSATOMIC {
          clearstack(th_ptr);
        }

        // we could shrink stack at any time but we opt to do it during initial mark to do that just once per cycle
        if (*g).gcstate as i32 == GCSPROPAGATE {
          shrinkstackprotected(th_ptr);
        }

        size_of::<LuaState>()
          + size_of::<TValue>() * th.stacksize as usize
          + size_of::<CallInfo>() * th.size_ci as usize
      }
      Some(GcViewMut::Proto(p)) => {
        let p_ptr = p as *mut Proto;
        (*g).gray = p.gclist;
        traverseproto(g, p_ptr);

        size_of::<Proto>()
          + size_of::<Instruction>() * p.sizecode as usize
          + size_of::<*mut Proto>() * p.sizep as usize
          + size_of::<TValue>() * p.sizek as usize
          + p.sizelineinfo as usize
          + size_of::<LocVar>() * p.sizelocvars as usize
          + size_of::<*mut tstring>() * p.sizeupvalues as usize
          + p.sizetypeinfo as usize
      }
      Some(GcViewMut::Class(classobject)) => {
        let class_ptr = classobject as *mut LuauClass;
        (*g).gray = classobject.gclist;
        traverseclass(g, class_ptr);
        // We've traversed the "object" itself ...
        size_of::<LuauClass>()
                // ... plus the method closures, each a `TValue` wide ...
                + ((classobject.numberofallmembers - classobject.numberofinstancemembers)
                    as usize
                    * size_of::<TValue>())
                // ... plus a string pointer for each method or property, each a pointer wide.
                + (classobject.numberofallmembers as usize * size_of::<*mut tstring>())
      }
      Some(GcViewMut::Object(classinst)) => {
        let inst_ptr = classinst as *mut LuauObject;
        (*g).gray = classinst.gclist;
        traverseobject(g, inst_ptr);
        // We've traversed the instance ...
        size_of::<LuauObject>()
                // ... plus all of the instance fields.
                + classinst.numberofmembers as usize * size_of::<TValue>()
      }
      _ => {
        LUAU_ASSERT!(false);
        0
      }
    }
  }
}
