//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:504:propagatemark`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:504-590, hand-ported)

use core::{ffi::c_int, mem::size_of};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    clearstack::clearstack, shrinkstackprotected::shrinkstackprotected,
    traverseclass::traverseclass, traverseclosure::traverseclosure, traverseobject::traverseobject,
    traverseproto::traverseproto, traversestack::traversestack, traversetable::traversetable,
  },
  macros::{
    black_2_gray::black2gray, gc_satomic::GCSATOMIC, gc_spropagate::GCSPROPAGATE, gco_2_cl::gco2cl,
    gco_2_class::gco2class, gco_2_h::gco2h, gco_2_object::gco2object, gco_2_p::gco2p,
    gco_2_th::gco2th, gray_2_black::gray2black, isgray::isgray, size_cclosure::size_cclosure,
    size_lclosure::size_lclosure, sizenode::sizenode,
  },
  records::{
    call_info::CallInfo, closure::Closure, global_state::global_State, loc_var::LocVar,
    lua_node::LuaNode, lua_table::LuaTable, luau_class::LuauClass, luau_object::LuauObject,
    proto::Proto, t_string::tstring,
  },
  type_aliases::{instruction::Instruction, lua_state::lua_State, t_value::TValue},
};

// traverse one gray object, turning it to black.
// Returns `quantity' traversed.
pub(crate) unsafe fn propagatemark(g: *mut global_State) -> usize {
  unsafe {
    let o = (*g).gray;
    LUAU_ASSERT!(isgray!(o));
    gray2black!(o);
    match (*o).gch.tt as i32 {
      t if t == LuaType::Table as i32 => {
        let h = gco2h!(o) as *const _ as *mut LuaTable;
        (*g).gray = (*h).gclist;
        if traversetable(g, h) != 0 {
          // table is weak?
          black2gray!(o); // keep it gray
        }
        size_of::<LuaTable>()
          + size_of::<TValue>() * (*h).sizearray as usize
          + size_of::<LuaNode>() * sizenode!(h) as usize
      }
      t if t == LuaType::Function as i32 => {
        let cl = gco2cl!(o) as *const _ as *mut Closure;
        (*g).gray = (*cl).gclist;
        traverseclosure(g, cl);
        if (*cl).is_c != 0 {
          size_cclosure((*cl).nupvalues as c_int)
        } else {
          size_lclosure((*cl).nupvalues as usize)
        }
      }
      t if t == LuaType::Thread as i32 => {
        let th = gco2th!(o) as *const _ as *mut lua_State;
        (*g).gray = (*th).gclist;

        let active = (*th).isactive || th == (*(*th).global).mainthread;

        traversestack(g, th);

        // active threads will need to be rescanned later to mark new stack writes so we mark them gray again
        if active {
          (*th).gclist = (*g).grayagain;
          (*g).grayagain = o;

          black2gray!(o);
        }

        // the stack needs to be cleared after the last modification of the thread state before sweep begins
        // if the thread is inactive, we might not see the thread in this cycle so we must clear it now
        if !active || (*g).gcstate as i32 == GCSATOMIC {
          clearstack(th);
        }

        // we could shrink stack at any time but we opt to do it during initial mark to do that just once per cycle
        if (*g).gcstate as i32 == GCSPROPAGATE {
          shrinkstackprotected(th);
        }

        size_of::<lua_State>()
          + size_of::<TValue>() * (*th).stacksize as usize
          + size_of::<CallInfo>() * (*th).size_ci as usize
      }
      t if t == LuaType::Proto as i32 => {
        let p = gco2p!(o) as *const _ as *mut Proto;
        (*g).gray = (*p).gclist;
        traverseproto(g, p);

        size_of::<Proto>()
          + size_of::<Instruction>() * (*p).sizecode as usize
          + size_of::<*mut Proto>() * (*p).sizep as usize
          + size_of::<TValue>() * (*p).sizek as usize
          + (*p).sizelineinfo as usize
          + size_of::<LocVar>() * (*p).sizelocvars as usize
          + size_of::<*mut tstring>() * (*p).sizeupvalues as usize
          + (*p).sizetypeinfo as usize
      }
      t if t == LuaType::Class as i32 => {
        let classobject = gco2class!(o) as *const _ as *mut LuauClass;
        (*g).gray = (*classobject).gclist;
        traverseclass(g, classobject);
        // We've traversed the "object" itself ...
        size_of::<LuauClass>()
                // ... plus the method closures, each a `TValue` wide ...
                + (((*classobject).numberofallmembers - (*classobject).numberofinstancemembers)
                    as usize
                    * size_of::<TValue>())
                // ... plus a string pointer for each method or property, each a pointer wide.
                + ((*classobject).numberofallmembers as usize * size_of::<*mut tstring>())
      }
      t if t == LuaType::Object as i32 => {
        let classinst = gco2object!(o) as *const _ as *mut LuauObject;
        (*g).gray = (*classinst).gclist;
        traverseobject(g, classinst);
        // We've traversed the instance ...
        size_of::<LuauObject>()
                // ... plus all of the instance fields.
                + (*classinst).numberofmembers as usize * size_of::<TValue>()
      }
      _ => {
        LUAU_ASSERT!(false);
        0
      }
    }
  }
}
