use core::{
  ffi::{CStr, c_int},
  mem::size_of,
  ptr::{null_mut, write_bytes},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    c_slice_mut, lua_d_call::lua_d_call, lua_f_new_cclosure::lua_f_new_cclosure,
    lua_gettop::lua_gettop, lua_h_getstr::lua_h_getstr, lua_l_error_l::lua_l_error_l,
    lua_m_newgco::luaM_newgco_, lua_s_newlstr::lua_s_newlstr, lua_v_gettable::lua_v_gettable,
  },
  macros::{
    classvalue::classvalue, clvalue::clvalue, getstr::getstr, lua_c_barrier::luaC_barrier,
    lua_c_init::luaC_init, lua_d_checkstack::luaD_checkstack, lua_m_newarray::luaM_newarray,
    nvalue::nvalue, objectvalue::objectvalue, setclassvalue::setclassvalue, setclvalue::setclvalue,
    setnilvalue::setnilvalue, setobj::setobj, setobj_2_s::setobj2s, setobjectvalue::setobjectvalue,
    setsvalue::setsvalue, ttisnil::ttisnil, ttisobject::ttisobject,
  },
  records::{
    lua_state::lua_State, lua_table::LuaTable, luau_class::LuauClass, luau_object::LuauObject,
    t_string::tstring,
  },
  type_aliases::t_value::TValue,
};

pub(crate) unsafe extern "C-unwind" fn lua_r_constructobject(l: *mut lua_State) -> c_int {
  unsafe {
    let cl = clvalue!((*(*l).ci).func);
    let classobject = &mut **classvalue!(&(*cl).inner.c.upvals[0]) as *mut LuauClass;

    let self_obj = luaM_newgco_(l, size_of::<LuauObject>(), (*l).activememcat) as *mut LuauObject;
    write_bytes(self_obj as *mut u8, 0, size_of::<LuauObject>());
    luaC_init!(l, self_obj, LuaType::Object as c_int);
    (*self_obj).lclass = classobject;
    (*self_obj).members = luaM_newarray!(
      l,
      (*classobject).numberofinstancemembers,
      TValue,
      (*l).activememcat
    );
    (*self_obj).numberofmembers = (*classobject).numberofinstancemembers;

    for member in c_slice_mut(
      (*self_obj).members,
      (*classobject).numberofinstancemembers as usize,
    ) {
      setnilvalue!(member);
    }

    let init_key = lua_s_newlstr(l, c"__init".as_ptr() as *const _, 6);
    let init_index = lua_h_getstr((*classobject).memberstooffset, init_key);
    LUAU_ASSERT!(!init_index.is_null() && !ttisnil!(init_index));
    let init_offset = nvalue!(init_index) as i32 - (*classobject).numberofinstancemembers;
    let init_function = (*classobject).staticmembers.add(init_offset as usize);

    let numargs = (*l).top.offset_from((*l).base) as c_int;

    // Put self onto the stack to ensure that it unconditionally survives GC during execution of __init.
    setobjectvalue!(l, (*l).top, self_obj);
    (*l).top = (*l).top.add(1);

    luaD_checkstack!(l, 2 + numargs);

    let args_base = (*l).top;
    setobj2s!(l, (*l).top, init_function);
    (*l).top = (*l).top.add(1);

    setobjectvalue!(l, (*l).top, self_obj);
    (*l).top = (*l).top.add(1);

    for i in 0..numargs as usize {
      setobj2s!(l, (*l).top, (*l).base.add(i));
      (*l).top = (*l).top.add(1);
    }

    lua_d_call(l, args_base, 0);

    1
  }
}

pub(crate) unsafe extern "C-unwind" fn lua_r_defaultcreateobject(l: *mut lua_State) -> c_int {
  unsafe {
    let cl = clvalue!((*(*l).ci).func);
    let classobject = &mut **classvalue!(&(*cl).inner.c.upvals[0]) as *mut LuauClass;

    if (*classobject).hasuserinitinchain {
      lua_l_error_l(
        l,
        c"Class %s must define a constructor because it is derived from a class that defines one"
          .as_ptr(),
        core::format_args!(
          "Class {} must define a constructor because it is derived from a class that defines one",
          CStr::from_ptr(getstr((*classobject).name)).to_string_lossy()
        ),
      );
    }

    let numargs = lua_gettop(l);
    if numargs != 2 {
      lua_l_error_l(
        l,
        c"The constructor of %s must be called with 2 arguments.  Got %d".as_ptr(),
        core::format_args!(
          "The constructor of {} must be called with 2 arguments.  Got {}",
          CStr::from_ptr(getstr((*classobject).name)).to_string_lossy(),
          numargs
        ),
      );
    }

    if !ttisobject!((*l).base) {
      lua_l_error_l(
        l,
        c"%s.__init must be called with an instance of the class as its first argument".as_ptr(),
        core::format_args!(
          "{}.__init must be called with an instance of the class as its first argument",
          CStr::from_ptr(getstr((*classobject).name)).to_string_lossy()
        ),
      );
    }

    let classinst = &mut **objectvalue!((*l).base) as *mut LuauObject;
    LUAU_ASSERT!(!classinst.is_null());

    if (*classinst).lclass != classobject {
      lua_l_error_l(
        l,
        c"Cannot call %s.__init on an instance of class %s".as_ptr(),
        core::format_args!(
          "Cannot call {}.__init on an instance of class {}",
          CStr::from_ptr(getstr((*classobject).name)).to_string_lossy(),
          CStr::from_ptr(getstr((*(*classinst).lclass).name)).to_string_lossy()
        ),
      );
    }

    let prop_slot = 1;

    setnilvalue!((*l).top);
    (*l).top = (*l).top.add(1);

    for idx in 0..(*classobject).numberofinstancemembers as usize {
      let mut key = TValue::default();
      setsvalue!(l, &mut key, *(*classobject).offsettomember.add(idx));
      lua_v_gettable(l, (*l).base.add(prop_slot), &mut key, (*l).top.sub(1));
      setobj!(l, (*classinst).members.add(idx), (*l).top.sub(1));
      luaC_barrier!(l, classinst, (*classinst).members.add(idx));
    }

    (*l).top = (*l).top.sub(1);

    0
  }
}

pub(crate) unsafe fn lua_r_setupconstructor(
  l: *mut lua_State,
  classobject: *mut LuauClass,
  env: *mut LuaTable,
) {
  unsafe {
    let new_key = lua_s_newlstr(l, c"new".as_ptr() as *const _, 3);
    let constructor = lua_f_new_cclosure(l, 1, env);
    let constructor_c = core::ptr::addr_of_mut!((*constructor).inner.c);
    (*constructor_c).f = Some(lua_r_constructobject);
    (*constructor_c).debugname = c"luaR_constructobject".as_ptr();
    setclassvalue!(l, &mut (*constructor_c).upvals[0], classobject);
    (*constructor_c).cont = None;

    let offset_value = lua_h_getstr((*classobject).memberstooffset, new_key);
    if !offset_value.is_null() && !ttisnil!(offset_value) {
      let offset_double = nvalue!(offset_value);
      LUAU_ASSERT!(
        offset_double >= (*classobject).numberofinstancemembers as f64
          && offset_double < (*classobject).numberofallmembers as f64
      );
      let offset = offset_double as i32 - (*classobject).numberofinstancemembers;
      setclvalue!(
        l,
        (*classobject).staticmembers.add(offset as usize),
        constructor
      );
      luaC_barrier!(
        l,
        classobject,
        (*classobject).staticmembers.add(offset as usize)
      );
    }

    let default_ctor = lua_f_new_cclosure(l, 1, env);
    let default_ctor_c = core::ptr::addr_of_mut!((*default_ctor).inner.c);
    (*default_ctor_c).f = Some(lua_r_defaultcreateobject);
    (*default_ctor_c).debugname = c"luaR_defaultcreateobject".as_ptr();
    setclassvalue!(l, &mut (*default_ctor_c).upvals[0], classobject);
    (*default_ctor_c).cont = None;

    let init_key = lua_s_newlstr(l, c"__init".as_ptr() as *const _, 6);
    let init_index = lua_h_getstr((*classobject).memberstooffset, init_key);
    if !init_index.is_null() && !ttisnil!(init_index) {
      let init_offset = nvalue!(init_index) as i32 - (*classobject).numberofinstancemembers;
      setclvalue!(
        l,
        (*classobject).staticmembers.add(init_offset as usize),
        default_ctor
      );
      luaC_barrier!(
        l,
        classobject,
        (*classobject).staticmembers.add(init_offset as usize)
      );
    }
  }
}

pub(crate) unsafe fn lua_r_newblankclass(
  l: *mut lua_State,
  name: *mut tstring,
  isopen: bool,
) -> *mut LuauClass {
  unsafe {
    let classobject = luaM_newgco_(l, size_of::<LuauClass>(), (*l).activememcat) as *mut LuauClass;
    luaC_init!(l, classobject, LuaType::Class as c_int);
    (*classobject).name = name;
    (*classobject).super_ = null_mut();
    (*classobject).staticmembers = null_mut();
    (*classobject).memberstooffset = null_mut();
    (*classobject).offsettomember = null_mut();
    (*classobject).instancemetatable = null_mut();
    (*classobject).numberofinstancemembers = 0;
    (*classobject).numberofallmembers = 0;
    (*classobject).isopen = isopen;
    (*classobject).hasuserinitinchain = false;
    classobject
  }
}

pub(crate) unsafe fn lua_r_newclass(
  l: *mut lua_State,
  name: *mut tstring,
  memberstooffset: *mut LuaTable,
  offsettomember: *mut *mut tstring,
  numberofinstancemembers: i32,
  numberofstaticmembers: i32,
) -> *mut LuauClass {
  unsafe {
    let global = (*l).global;
    LUAU_ASSERT!((*global).gc_threshold == usize::MAX);

    let classobject = lua_r_newblankclass(l, name, false);

    (*classobject).staticmembers =
      luaM_newarray!(l, numberofstaticmembers, TValue, (*classobject).memcat);
    for i in 0..numberofstaticmembers as usize {
      setnilvalue!((*classobject).staticmembers.add(i));
    }

    (*classobject).memberstooffset = memberstooffset;
    (*classobject).offsettomember = offsettomember;

    (*classobject).numberofinstancemembers = numberofinstancemembers;
    (*classobject).numberofallmembers = numberofinstancemembers + numberofstaticmembers;

    lua_r_setupconstructor(l, classobject, (*l).gt);

    classobject
  }
}
