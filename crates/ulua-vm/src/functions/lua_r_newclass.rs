use core::{
  mem::size_of,
  ptr::{null_mut, write_bytes},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{lua_type::LuaType, value_view::ValueView},
  functions::{
    c_slice, c_slice_mut, cstr_cow, lua_d_call::lua_d_call, lua_f_new_cclosure::lua_f_new_cclosure,
    lua_gettop::lua_gettop, lua_h_getstr::lua_h_getstr, lua_m_newgco::lua_m_newgco,
    lua_s_newlstr::lua_s_newlstr, lua_v_gettable::lua_v_gettable,
  },
  macros::{
    classvalue::classvalue, getstr::getstr, lua_c_barrier::lua_c_barrier, lua_c_init::luaC_init,
    lua_d_checkstack::luaD_checkstack, lua_l_error::luaL_error, lua_m_newarray::luaM_newarray,
    setclassvalue::setclassvalue, setclvalue::setclvalue, setnilvalue::setnilvalue, setobj::setobj,
    setobj_2_s::setobj_2_s, setobjectvalue::setobjectvalue, setsvalue::setsvalue,
  },
  records::{
    lua_state::LuaState, lua_table::LuaTable, luau_class::LuauClass, luau_object::LuauObject,
    t_string::tstring,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// 仅作为 C 闭包入口经 luau_precall 调用：`(*l).ci` 须为该闭包的存活帧且闭包 upval[0] 为类值、
/// `(*l).base..(*l).top` 为可读参数窗口；栈尾空间由内部 luaD_checkstack 扩容保证，`lua_d_call`
/// 可能抛错，须在受保护帧内。cpp lclass.cpp:374 `luaR_constructobject`
pub(crate) unsafe extern "C-unwind" fn lua_r_constructobject(l: *mut LuaState) -> i32 {
  unsafe {
    let cl = (*(*(*l).ci).func).as_closure_ptr();
    let classobject = classvalue!(&(*cl).inner.c.upvals[0]);

    let self_obj = lua_m_newgco(l, size_of::<LuauObject>(), (*l).activememcat) as *mut LuauObject;
    // 清零初始化：LuauObject 为 repr(C) POD 记录，全零即各字段的合法空值；
    // self_obj 为刚按 LuauObject 大小分配的存活内存
    write_bytes(self_obj.cast::<u8>(), 0, size_of::<LuauObject>());
    luaC_init!(l, self_obj, LuaType::Object as i32);

    let class = &*classobject;
    let obj = &mut *self_obj;
    obj.lclass = classobject;
    obj.members = luaM_newarray!(l, class.numberofinstancemembers, TValue, (*l).activememcat);
    obj.numberofmembers = class.numberofinstancemembers;

    for member in c_slice_mut(obj.members, class.numberofinstancemembers as usize) {
      setnilvalue!(member);
    }

    let init_key = lua_s_newlstr(l, b"__init");
    let init_index = lua_h_getstr(class.memberstooffset, init_key);
    // tag 判定收敛为 ValueView 变体 match（§11 pass B）；payload 仍由该断言兜底的
    // `nvalue!` 读取——release 断言编译掉后行为与收敛前逐位一致，不新增分支
    LUAU_ASSERT!(
      !init_index.is_null() && !matches!(ValueView::from_tvalue(&*init_index), ValueView::Nil)
    );
    let init_offset = (*init_index).as_number() as i32 - class.numberofinstancemembers;
    let init_function = class.staticmembers.add(init_offset as usize);

    let numargs = (*l).top.offset_from((*l).base) as i32;

    // Put self onto the stack to ensure that it unconditionally survives GC during execution of __init.
    setobjectvalue!(l, (*l).top, self_obj);
    (*l).top = (*l).top.add(1);

    luaD_checkstack!(l, 2 + numargs);

    let args_base = (*l).top;
    setobj_2_s!(l, (*l).top, init_function);
    (*l).top = (*l).top.add(1);

    setobjectvalue!(l, (*l).top, self_obj);
    (*l).top = (*l).top.add(1);

    // 批量拷贝参数（TValue 为 Copy 的 POD 值，切片 copy 与 cpp 逐元素 setobj2s 语义一致；
    // base..base+numargs 与 top 之后不重叠，边界由上方 luaD_checkstack 保证）
    let arg_count = numargs as usize;
    c_slice_mut((*l).top, arg_count).copy_from_slice(c_slice((*l).base, arg_count));
    (*l).top = (*l).top.add(arg_count);

    lua_d_call(l, args_base, 0);

    1
  }
}

/// # Safety
/// 同经 C 闭包调用约定：`(*l).ci`.func 为该闭包且 upval[0] 为类值、`(*l).base..base+2` 可读
/// （首参须是本类的 Object 实例，否则走抛错路径）、栈顶另有 ≥1 空闲槽承接 `lua_v_gettable` 临时值；
/// 各抛错路径经 `luaL_error` 不返回，须在受保护帧内。cpp lclass.cpp:421 `luaR_defaultcreateobject`
pub(crate) unsafe extern "C-unwind" fn lua_r_defaultcreateobject(l: *mut LuaState) -> i32 {
  unsafe {
    let cl = (*(*(*l).ci).func).as_closure_ptr();
    let classobject = classvalue!(&(*cl).inner.c.upvals[0]);
    let class_name = getstr((*classobject).name);

    if (*classobject).hasuserinitinchain {
      luaL_error!(
        l,
        "Class {} must define a constructor because it is derived from a class that defines one",
        cstr_cow(class_name)
      );
    }

    let numargs = lua_gettop(l);
    if numargs != 2 {
      luaL_error!(
        l,
        "The constructor of {} must be called with 2 arguments.  Got {}",
        cstr_cow(class_name),
        numargs
      );
    }

    // `ttisobject! + objectvalue!` 链收敛为 ValueView::Object 臂：tag 判定与 payload
    // 提取同臂完成，非 Object 首参走原抛错路径（`luaL_error` 不返回作 let-else 臂）
    let ValueView::Object(classinst) = ValueView::from_tvalue(&*(*l).base) else {
      luaL_error!(
        l,
        "{}.__init must be called with an instance of the class as its first argument",
        cstr_cow(class_name)
      );
    };
    LUAU_ASSERT!(!classinst.is_null());

    if (*classinst).lclass != classobject {
      let inst_class_name = getstr((*(*classinst).lclass).name);
      luaL_error!(
        l,
        "Cannot call {}.__init on an instance of class {}",
        cstr_cow(class_name),
        cstr_cow(inst_class_name)
      );
    }

    let prop_slot = 1;

    setnilvalue!((*l).top);
    (*l).top = (*l).top.add(1);

    let inst_members = (*classinst).members;
    let offsettomember = (*classobject).offsettomember;
    // 下标为实例成员槽号（members/offsettomember 的属性偏移）；循环体内 luaV_gettable
    // 可经 __index 再入读写同一实例成员区，保留索引寻址、不持有跨调用的 &mut 切片
    for idx in 0..(*classobject).numberofinstancemembers as usize {
      let mut key = TValue::default();
      setsvalue!(l, &mut key, *offsettomember.add(idx));
      lua_v_gettable(l, (*l).base.add(prop_slot), &mut key, (*l).top.sub(1));
      setobj!(l, inst_members.add(idx), (*l).top.sub(1));
      lua_c_barrier!(l, classinst, inst_members.add(idx));
    }

    (*l).top = (*l).top.sub(1);

    0
  }
}

/// # Safety
/// 调用方须保证：`l` 存活且处于受保护帧（建闭包/新串可触发 GC 与抛错）；`classobject` 为
/// staticmembers 与 memberstooffset 均已初始化的存活类（若 `new`/`__init` 已注册，其偏移须落在
/// 静态成员区间内）；`env` 为存活环境表。cpp lclass.cpp:42 `luaR_setupconstructor`
pub(crate) unsafe fn lua_r_setupconstructor(
  l: *mut LuaState,
  classobject: *mut LuauClass,
  env: *mut LuaTable,
) {
  // Safety: 契约保证 `l`/`classobject`/`env` 存活，构造器闭包与 new 名串注册仅触及类静态区与 env 表的合法槽位
  unsafe {
    let new_key = lua_s_newlstr(l, b"new");
    let constructor = lua_f_new_cclosure(l, 1, env);
    let ctor_c = &mut (*constructor).inner.c;
    ctor_c.f = Some(lua_r_constructobject);
    ctor_c.debugname = c"luaR_constructobject".as_ptr();
    setclassvalue!(l, &mut ctor_c.upvals[0], classobject);
    ctor_c.cont = None;

    let inst_members = (*classobject).numberofinstancemembers;
    let staticmembers = (*classobject).staticmembers;

    let offset_value = lua_h_getstr((*classobject).memberstooffset, new_key);
    // memberstooffset 的值经 loadsafe/luaR_inheritclass 各写入点均为 `setnvalue!`
    // （Number/Nil 两态）：原 `!ttisnil!` 守卫 + `nvalue!` 读链收敛为
    // ValueView::Number 臂 match
    if !offset_value.is_null()
      && let ValueView::Number(offset_double) = ValueView::from_tvalue(&*offset_value)
    {
      LUAU_ASSERT!(
        offset_double >= inst_members as f64
          && offset_double < (*classobject).numberofallmembers as f64
      );
      let dest = staticmembers.add((offset_double as i32 - inst_members) as usize);
      setclvalue!(l, dest, constructor);
      lua_c_barrier!(l, classobject, dest);
    }

    let default_ctor = lua_f_new_cclosure(l, 1, env);
    let default_ctor_c = &mut (*default_ctor).inner.c;
    default_ctor_c.f = Some(lua_r_defaultcreateobject);
    default_ctor_c.debugname = c"luaR_defaultcreateobject".as_ptr();
    setclassvalue!(l, &mut default_ctor_c.upvals[0], classobject);
    default_ctor_c.cont = None;

    let init_key = lua_s_newlstr(l, b"__init");
    let init_index = lua_h_getstr((*classobject).memberstooffset, init_key);
    // 同上：`__init` 偏移读链收敛为 ValueView::Number 臂 match
    if !init_index.is_null()
      && let ValueView::Number(init_offset) = ValueView::from_tvalue(&*init_index)
    {
      let dest = staticmembers.add((init_offset as i32 - inst_members) as usize);
      setclvalue!(l, dest, default_ctor);
      lua_c_barrier!(l, classobject, dest);
    }
  }
}

/// # Safety
/// 调用方须保证：`l` 存活（lua_m_newgco OOM 时经 `l` 抛 ErrMem，需受保护帧）、`name` 为存活 TString；
/// 返回值各成员数组字段为 null，必须经 lua_r_newclass/inheritclass 填充后才可交付使用。
/// cpp lclass.cpp:18 `luaR_newblankclass`
pub(crate) unsafe fn lua_r_newblankclass(
  l: *mut LuaState,
  name: *mut tstring,
  isopen: bool,
) -> *mut LuauClass {
  // Safety: 契约保证 `l` 存活且 name 为存活串；lua_m_newgco/luaC_init! 按类大小分配并挂入 GC 链
  unsafe {
    let classobject = lua_m_newgco(l, size_of::<LuauClass>(), (*l).activememcat) as *mut LuauClass;
    luaC_init!(l, classobject, LuaType::Class as i32);
    let co = &mut *classobject;
    co.name = name;
    co.super_ = null_mut();
    co.staticmembers = null_mut();
    co.memberstooffset = null_mut();
    co.offsettomember = null_mut();
    co.instancemetatable = null_mut();
    co.numberofinstancemembers = 0;
    co.numberofallmembers = 0;
    co.isopen = isopen;
    co.hasuserinitinchain = false;
    classobject
  }
}

/// # Safety
/// 仅由 luau_load 在类字节码载入期调用：`(*(*l).global).gc_threshold` 须已被置 usize::MAX 冻结 GC
/// 至类构造完成（否则静态区/成员表可被回收）；`memberstooffset`/`offsettomember` 为已按
/// numberofinstancemembers+numberofstaticmembers 分配并填充完的存活表/数组；`l` 存活且处于受保护帧。
/// cpp lclass.cpp:89 `luaR_newclass`
///
/// `envt` 是 `new`/`__init` 两个 C 闭包的环境表，对应 cpp `lclass.h:17-25` 的第 7 个
/// 形参：`luau_load` 带非当前环境时必须绑到该环境，而不是 `L->gt`。
pub(crate) unsafe fn lua_r_newclass(
  l: *mut LuaState,
  name: *mut tstring,
  memberstooffset: *mut LuaTable,
  offsettomember: *mut *mut tstring,
  numberofinstancemembers: i32,
  numberofstaticmembers: i32,
  envt: *mut LuaTable,
) -> *mut LuauClass {
  // Safety: 契约保证 `l` 存活、gc_threshold 处于本函数前置断言的暂停态，新类字段填充与成员注册均在分配界内
  unsafe {
    let global = (*l).global;
    LUAU_ASSERT!((*global).gc_threshold == usize::MAX);

    let classobject = lua_r_newblankclass(l, name, false);
    let co = &mut *classobject;

    co.staticmembers = luaM_newarray!(l, numberofstaticmembers, TValue, co.memcat);
    // Safety:staticmembers 刚按 numberofstaticmembers 分配完成，全部元素可写。
    for member in c_slice_mut(co.staticmembers, numberofstaticmembers as usize) {
      setnilvalue!(member);
    }

    co.memberstooffset = memberstooffset;
    co.offsettomember = offsettomember;

    co.numberofinstancemembers = numberofinstancemembers;
    co.numberofallmembers = numberofinstancemembers + numberofstaticmembers;

    lua_r_setupconstructor(l, classobject, envt);

    classobject
  }
}
