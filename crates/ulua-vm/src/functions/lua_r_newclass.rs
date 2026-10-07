use core::{mem::size_of, ptr};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{lua_type::LuaType, value_view::ValueView},
  functions::{
    c_slice, c_slice_mut, cstr_cow, lua_d_call::lua_d_call, lua_f_new_cclosure::lua_f_new_cclosure,
    lua_h_getstr::lua_h_getstr, lua_m_newgco::lua_m_newgco, lua_s_newlstr::lua_s_newlstr,
    lua_v_gettable::lua_v_gettable,
  },
  macros::{
    classvalue::classvalue, getstr::getstr, lua_c_barrier::lua_c_barrier, lua_c_init::luaC_init,
    lua_d_checkstack::luaD_checkstack, lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn,
    lua_m_newarray::luaM_newarray, lua_o_nilobject::LUA_O_NILOBJECT, setclassvalue::setclassvalue,
    setclvalue::setclvalue, setnilvalue::setnilvalue, setobj::setobj, setobj_2_s::setobj_2_s,
    setobjectvalue::setobjectvalue, setsvalue::setsvalue,
  },
  records::{
    lua_state::LuaState, lua_table::LuaTable, luau_class::LuauClass, luau_object::LuauObject,
    slot::Slot, t_string::tstring,
  },
  type_aliases::t_value::TValue,
};

/// 仅作为 C 闭包入口经 luau_precall 调用（调用序契约，正确性而非内存安全——`l` 的存活与独占由
/// `&mut LuaState` 承载）：`(*l).ci` 须为该闭包的存活帧且闭包 upval[0] 为类值、
/// `(*l).base..(*l).top` 为可读参数窗口；栈尾空间由内部 luaD_checkstack 扩容保证，`lua_d_call`
/// 可能抛错，须在受保护帧内。cpp lclass.cpp:374 `luaR_constructobject`
///
/// r16-v22 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载。体内保留的裸解引用点
/// 全部落在 `(*(*(*l).ci).func)` 帧现读（ci 链无门面，r13-w1b lua_v_call_tm 同款保留判例）、
/// `(*l).activememcat`（GC 分配类目字段读数）与参数拷贝源窗 `(*l).base`（帧窗基址裸读）三处
/// r13-w1c 逐点定性保留面，故本体仍由一个 `unsafe { … }` 块整体覆盖——块界与既有保留面严格对齐，
/// 未新增也未收窄任何授权窗。
pub(crate) fn lua_r_constructobject(l: &mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为存活 C 闭包帧，self/func/args 压栈均落在 checkstack 后的界内槽
  //
  // r13-w1c 逐点定性（w6d 口径保留面）：`(*(*(*l).ci).func)` 帧现读（ci 链无门面，
  // r13-w1b lua_v_call_tm 同款保留判例）、`(*l).activememcat` 两处（GC 分配类目字段
  // 读数，非栈顶算术覆盖面）与参数拷贝源窗 `(*l).base`（帧窗基址裸读）均无既有
  // 门面，保留。收编共六点：栈顶槽距读数经 get_top 门面，self 写入/args 窗基址/
  // func 写入/self 续写/参数目的窗共五处顶槽裸读经 top_slot(0) 边界原语
  // （抬顶落笔系 r12-w7a2 既有 advance_top，位点时序全数不变）。
  //
  // r16-v22 收形：形参转 `&mut LuaState`，收形后对仍收裸形的核心（`lua_m_newgco`/`lua_d_call`）
  // 转调采用一次 `&mut *l` 就地重建（借用窗仅在当句内），未跨调用持有；`lua_s_newlstr` 已收形，
  // 直接透传 `l`。`(*l).activememcat`/`(*l).base` 帧窗读点通过 `&mut` 的隐式 deref 保持原位语义，
  // 时序与位点均无变化。
  unsafe {
    let cl = (*(*l.ci).func).as_closure_ptr();
    let classobject = classvalue!(&(*cl).inner.c.upvals[0]);

    let self_obj =
      lua_m_newgco(l.as_mut_ptr(), size_of::<LuauObject>(), l.activememcat) as *mut LuauObject;
    // 类型化清零初始化：LuauObject 为 repr(C) POD 记录，`Default` 即各字段的合法
    // 空值（指针 null、标量 0）；`ptr::write` 落在刚按 LuauObject 大小分配的存活
    // 内存上，替代原先按字节 `write_bytes` 的无类型清零
    ptr::write(self_obj, LuauObject::default());
    luaC_init!(l.as_mut_ptr(), self_obj, LuaType::Object as i32);

    let class = &*classobject;
    let obj = &mut *self_obj;
    obj.lclass = classobject;
    obj.members = luaM_newarray!(l, class.numberofinstancemembers, TValue, l.activememcat);
    obj.numberofmembers = class.numberofinstancemembers;

    for member in c_slice_mut(obj.members, class.numberofinstancemembers as usize) {
      setnilvalue!(member);
    }

    let init_key = lua_s_newlstr(l, b"__init");
    // B2-2a 任务B：getstr 折叠 Option<Slot> 后在边界还原哨兵裸形——下方契约断言
    // 与 `as_number` 读链保持原形（miss 兜底读 nil 哨兵行为逐位一致）
    let init_index =
      lua_h_getstr(&*class.memberstooffset, init_key).map_or(LUA_O_NILOBJECT, |s| s.as_const_ptr());
    // tag 判定收敛为 ValueView 变体 match（§11 pass B）；payload 仍由该断言兜底的
    // `nvalue!` 读取——release 断言编译掉后行为与收敛前逐位一致，不新增分支
    LUAU_ASSERT!(
      !init_index.is_null() && !matches!(ValueView::from_tvalue(&*init_index), ValueView::Nil)
    );
    let init_offset = (*init_index).as_number() as i32 - class.numberofinstancemembers;
    // 静态成员窗偏移经 c_slice 收口（形制同簇内 :48/:291 先例）：下标越界由裸指针
    // UB 降为 panic，界内性由类构造不变量兜底（`__init` 注册偏移必落在静态区）
    let static_count = (class.numberofallmembers - class.numberofinstancemembers) as usize;
    let init_function: *const TValue =
      &c_slice(class.staticmembers, static_count)[init_offset as usize];

    // r13-w1c 收编：栈顶帧槽距读数落既有 get_top 门面——其本体 `slot_distance(base, top)`
    // 即被替代式 `(*l).top.offset_from((*l).base) as i32` 的同址同宽镜像
    // （cpp `L->top - L->base` 读数形，位点现读不变）
    let numargs = (*l).get_top();

    // Put self onto the stack to ensure that it unconditionally survives GC during execution of __init.
    // r12-w7a2 收编：本函数体「写已界内槽后裸抬顶」点位（self 槽写入 + checkstack
    // 后 func/self/args 三段压栈）统一经 advance_top 原语——setobj/setobjectvalue/
    // 切片拷贝均不移动栈、不写 top 场，原语现读场与被替代式同址同宽；
    // 扩容（checkstack）与 lua_d_call 的时序照旧先行
    // r13-w1c 收编：同段的写窗基址裸读 `(*l).top` 统一经 top_slot(0) 边界原语
    // （off=0 即保留顶槽，镜像 cpp `L->top` 读数形；各点位原位替换，读数序不变）
    setobjectvalue!(l, (*l).top_slot(0), self_obj);
    (*l).advance_top(1);

    luaD_checkstack!(l, 2 + numargs);

    // 收编：args 窗基址绑定经 top_slot(0)（checkstack 扩容先行之后现读，
    // 与被替代点位同位同值）
    let args_base = (*l).top_slot(0);
    setobj_2_s!(l, (*l).top_slot(0), init_function);
    (*l).advance_top(1);

    setobjectvalue!(l, (*l).top_slot(0), self_obj);
    (*l).advance_top(1);

    // 批量拷贝参数（TValue 为 Copy 的 POD 值，切片 copy 与 cpp 逐元素 setobj2s 语义一致；
    // base..base+numargs 与 top 之后不重叠，边界由上方 luaD_checkstack 保证）
    // 收编：目的窗基址经 top_slot(0)；源窗基址 `(*l).base` 为帧窗裸读——base 无既有
    // 门面/原语（r13-w1b 判例：base 落笔与读数属帧建立面，定性保留），保留
    let arg_count = numargs as usize;
    c_slice_mut((*l).top_slot(0), arg_count).copy_from_slice(c_slice(l.base, arg_count));
    (*l).advance_top(arg_count);

    lua_d_call(l.as_mut_ptr(), args_base, 0);

    1
  }
}

lua_lib_fn!(pub(crate) fn lua_r_constructobject @ref, lua_r_constructobject_arm);

/// 同经 C 闭包调用约定（调用序契约，正确性而非内存安全——`l` 的存活与独占由 `&mut LuaState`
/// 承载）：`(*l).ci`.func 为该闭包且 upval[0] 为类值、`(*l).base..base+2` 可读
/// （首参须是本类的 Object 实例，否则走抛错路径）、栈顶另有 ≥1 空闲槽承接 `lua_v_gettable` 临时值；
/// 各抛错路径经 `luaL_error` 不返回，须在受保护帧内。cpp lclass.cpp:421 `luaR_defaultcreateobject`
///
/// r16-v22 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载。体内保留的裸解引用点
/// 全部落在 `(*(*(*l).ci).func)` 帧现读（同 constructobject 保留判例）与两处 `(*l).base` 帧窗
/// 基址裸读（首参校验、gettable 实参窗）——r13-w1c 已逐点定性保留面，故本体仍由一个
/// `unsafe { … }` 块整体覆盖，块界与既有保留面对齐。
pub(crate) fn lua_r_defaultcreateobject(l: &mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为存活 C 闭包帧、首参为本类 Object 实例、栈顶 ≥1 空闲槽
  //
  // r13-w1c 逐点定性（w6d 口径保留面）：`(*(*(*l).ci).func)` 帧现读（同 constructobject
  // 保留判例）与两处 `(*l).base` 帧窗基址裸读（首参校验、gettable 实参窗）无既有
  // 门面，保留。收编共二点：循环内临时槽读/写两侧 `(*l).top.sub(1)` 均经
  // top_slot(-1) 边界原语（每轮 luaV_gettable 再入后现读位点不变，值恒等）；
  // push_nil/rewind_top/get_top 三点系 r12-w7a2 与 B2-0 既有门面，本票不动其形制。
  unsafe {
    let cl = (*(*l.ci).func).as_closure_ptr();
    let classobject = classvalue!(&(*cl).inner.c.upvals[0]);
    let class_name = getstr((*classobject).name);

    if (*classobject).hasuserinitinchain {
      luaL_error!(
        l,
        "Class {} must define a constructor because it is derived from a class that defines one",
        cstr_cow(class_name)
      );
    }

    let numargs = (*l).get_top();
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
    let ValueView::Object(classinst) = ValueView::from_tvalue(&*l.base) else {
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

    // 栈顶保留槽写入收口为 B2-0 push 族的 pub 面 `push_nil`（即
    // `reserved_top_slot`+`incr_top` 形）：本帧契约已保证栈顶 ≥1 空闲槽，
    // 其前置 `ensure_stack_space(1)` 恒为 no-op，抛错/校验序逐位不变
    (*l).push_nil();

    let inst_members = (*classinst).members;
    let offsettomember = (*classobject).offsettomember;
    // 成员名数组为只读视图（循环内不改写该数组本体），以 enumerate 取代裸 `add(idx)`；
    // 循环体内 luaV_gettable 可经 __index 再入读写实例成员区，故 `inst_members` 与栈槽
    // 仍按迭代时点从 `l` 现读寻址，不持有跨调用的可变借用
    for (idx, &member_name) in c_slice(
      offsettomember,
      (*classobject).numberofinstancemembers as usize,
    )
    .iter()
    .enumerate()
    {
      let mut key = TValue::default();
      setsvalue!(l, &mut key, member_name);
      lua_v_gettable(
        l.as_mut_ptr(),
        // 实参窗 base..base+numargs（numargs 已于上方校验为 2）经 c_slice 界内下标
        // 收口 `base.add(prop_slot)`；luaV_gettable 对 `t` 句柄只走读面，from_ref 合法。
        // base 侧裸读为帧窗基址（无既有门面，r13-w1b 判例定性保留）
        Slot::from_ref(&c_slice(l.base, numargs as usize)[prop_slot]),
        Slot::from_mut(&mut key),
        // r13-w1c 收编：顶下临时槽读数经 top_slot(-1) 边界原语（同位现读、值恒等；
        // 形制同 lua_l_pushresult/lua_setlocal 既有判例）
        Slot::from_raw((*l).top_slot(-1)),
      );
      // 实例成员窗偏移经 c_slice 收口：每轮在 luaV_gettable 再入之后重取切片，
      // 仍不持跨调用的可变借用（上方注释纪律不变），越界 UB 降 panic
      let member_dst: *mut TValue = &mut c_slice_mut(
        inst_members,
        (*classobject).numberofinstancemembers as usize,
      )[idx];
      // r13-w1c 收编：消费临时槽的源侧读数同上经 top_slot(-1)（再入后现读位点不变）
      setobj!(l, member_dst, (*l).top_slot(-1));
      lua_c_barrier!(l, classinst, member_dst);
    }

    // r12-w7a2 收编：弹回临时槽的裸场域回落经 rewind_top 原语（循环内 luaV_gettable
    // 再入之后现读场，与被替代式同址同宽；返回值计数 0 的收尾时序不变）
    (*l).rewind_top(1);

    0
  }
}

lua_lib_fn!(pub(crate) fn lua_r_defaultcreateobject @ref, lua_r_defaultcreateobject_arm);

/// 建 constructor/default_ctor 闭包并挂回类的内部例程（cpp lclass.cpp:42 `luaR_setupconstructor`）。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 承载）：调用方须保证
/// `l` 处于受保护帧（建闭包/新串可触发 GC 与抛错）；`classobject` 为 staticmembers 与
/// memberstooffset 均已初始化的存活类（若 `new`/`__init` 已注册，其偏移须落在静态成员区间内）；
/// `env` 为存活环境表。
///
/// r16-v22 收形：首参转 `&mut LuaState`——存活与独占由类型承载；体内仍存的两处 `(*classobject)`
/// 类对象字段读、`(*constructor)`/`(*default_ctor)` 闭包字段写、以及 `(*memberstooffset)`
/// 哈希表读为 GC 头/类静态区/闭包 upval 面的裸解引用，无既有门面，保留——`unsafe { … }` 块
/// 边界与 r13-w1c 判例的保留面对齐；`lua_s_newlstr` 收 `&mut` 直接透传 `l`，
/// `lua_f_new_cclosure` 仍收裸形，一处一次 `l.as_mut_ptr()` 就地重建（借用窗仅在当句内）。
pub(crate) fn lua_r_setupconstructor(
  l: &mut LuaState,
  classobject: *mut LuauClass,
  env: *mut LuaTable,
) {
  // SAFETY: 契约保证 `l`/`classobject`/`env` 存活，构造器闭包与 new 名串注册仅触及类静态区与 env 表的合法槽位
  unsafe {
    let new_key = lua_s_newlstr(l, b"new");
    let constructor = lua_f_new_cclosure(l, 1, env);
    let ctor_c = &mut (*constructor).inner.c;
    ctor_c.f = Some(lua_r_constructobject_arm);
    // debugname 经 intern 复制锚定为 TString（cpp 存静态字面量，此处内容等价；
    // 新闭包为白、挂引用免写屏障，存活由 traverseclosure 的 debugname 标记边保证）
    ctor_c.debugname = lua_s_newlstr(l, b"luaR_constructobject");
    setclassvalue!(l, &mut ctor_c.upvals[0], classobject);
    ctor_c.cont = None;

    let inst_members = (*classobject).numberofinstancemembers;
    let staticmembers = (*classobject).staticmembers;
    // 静态成员窗长 = 全部成员数 − 实例成员数（newclass 按静态数分配、all=inst+static）
    let static_count = ((*classobject).numberofallmembers - inst_members) as usize;

    // B2-2a 任务B：同借用窗口即时读判定——Option<Slot> 原生收口；原 `!is_null()`
    // 守卫对恒非空 sentinel/cpp getstr 本就恒真，折叠后由 None 臂（miss）统一落空。
    // memberstooffset 的值经 loadsafe/luaR_inheritclass 各写入点均为 `setnvalue!`
    // （Number/Nil 两态）：Number 臂经 `.get()` 只读视图取值，读形逐位一致
    if let Some(offset_value) = lua_h_getstr(&*(*classobject).memberstooffset, new_key)
      && let ValueView::Number(offset_double) = ValueView::from_tvalue(offset_value.get())
    {
      LUAU_ASSERT!(
        offset_double >= inst_members as f64
          && offset_double < (*classobject).numberofallmembers as f64
      );
      let dest: *mut TValue = &mut c_slice_mut(staticmembers, static_count)
        [(offset_double as i32 - inst_members) as usize];
      setclvalue!(l, dest, constructor);
      lua_c_barrier!(l, classobject, dest);
    }

    let default_ctor = lua_f_new_cclosure(l, 1, env);
    let default_ctor_c = &mut (*default_ctor).inner.c;
    default_ctor_c.f = Some(lua_r_defaultcreateobject_arm);
    // 同上：intern 锚定
    default_ctor_c.debugname = lua_s_newlstr(l, b"luaR_defaultcreateobject");
    setclassvalue!(l, &mut default_ctor_c.upvals[0], classobject);
    default_ctor_c.cont = None;

    let init_key = lua_s_newlstr(l, b"__init");
    // 同上：`__init` 偏移读链原生 Option<Slot> 收口，None 臂承接 miss
    if let Some(init_index) = lua_h_getstr(&*(*classobject).memberstooffset, init_key)
      && let ValueView::Number(init_offset) = ValueView::from_tvalue(init_index.get())
    {
      let dest: *mut TValue =
        &mut c_slice_mut(staticmembers, static_count)[(init_offset as i32 - inst_members) as usize];
      setclvalue!(l, dest, default_ctor);
      lua_c_barrier!(l, classobject, dest);
    }
  }
}

/// 分配一个尚未填充成员数组的空类对象（cpp lclass.cpp:18 `luaR_newblankclass`）。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 承载）：调用方须保证
/// `l` 处于受保护帧（lua_m_newgco OOM 时经 `l` 抛 ErrMem）；`name` 为存活 TString；返回值
/// 各成员数组字段为 null，必须经 lua_r_newclass/inheritclass 填充后才可交付使用。
///
/// r16-v22 收形：首参转 `&mut LuaState`——存活与独占由类型承载；体内 r13-w1c 唯一保留面
/// `(*l).activememcat`（GC 分配类目字段读数）现经 `&mut` 的隐式 deref 保持原位；
/// `lua_m_newgco`/`luaC_init!` 仍收裸形，一次 `l.as_mut_ptr()` 就地重建（借用窗仅在当句内）。
pub(crate) fn lua_r_newblankclass(
  l: &mut LuaState,
  name: *mut tstring,
  isopen: bool,
) -> *mut LuauClass {
  // SAFETY: 契约保证 `l` 存活且 name 为存活串；lua_m_newgco/luaC_init! 按类大小分配并挂入 GC 链
  //
  // r13-w1c 逐点定性（w6d 口径保留面）：本体唯一 `(*l).` 点位为 `(*l).activememcat`
  // GC 分配类目字段读数，非栈顶算术/非 API 门面覆盖面，无既有门面，保留。
  unsafe {
    let classobject =
      lua_m_newgco(l.as_mut_ptr(), size_of::<LuauClass>(), l.activememcat) as *mut LuauClass;
    // 类型化清零先于头初始化：`Default` 即各字段的合法空值（指针 null、标量 0），
    // `luaC_init!` 随后只覆写 tt/marked/memcat 三字段的头，最终态与原逐字段写一致
    ptr::write(
      classobject,
      LuauClass {
        name,
        isopen,
        ..Default::default()
      },
    );
    luaC_init!(l.as_mut_ptr(), classobject, LuaType::Class as i32);
    classobject
  }
}

/// 按已填充完的成员名数组与 name→offset 表建立类对象，并挂上 `new`/`__init`
/// constructor 闭包（cpp lclass.cpp:89 `luaR_newclass`）。
///
/// # Safety（外部前提；`l` 的存活与独占已由 `&mut LuaState` 承载，不再列入契约）
/// 调用方须保证：`l` 处于受保护帧（新类分配与 lua_r_setupconstructor 建闭包/新串可触发 GC
/// 与抛错）；`(*l.global).gc_threshold` 须已被置 usize::MAX 冻结 GC 至类构造完成
/// （否则静态区/成员表可被回收）；`memberstooffset`/`offsettomember` 为已按
/// numberofinstancemembers+numberofstaticmembers 分配并填充完的存活表/数组。
///
/// `envt` 是 `new`/`__init` 两个 C 闭包的环境表，对应 cpp `lclass.h:17-25` 的第 7 个
/// 形参：`luau_load` 带非当前环境时必须绑到该环境，而不是 `L->gt`。
///
/// r16-v27 收形：外层首参 `*mut LuaState` → `&mut LuaState`——`l` 的存活与独占交由类型承载，
/// 实测唯一真实调用点 `loadsafe.rs` 的 class shape 分支就地以一次 `&mut *l` 桥接（借用窗止于
/// 当句）。本函数**仍为 `unsafe fn`**：体内前提由调用方给出的真实裸指针面（`global_State`
/// 链基址读数、`(*global).gc_threshold` GC 冻结前置断言）与 `luaM_newarray!` 分配均无既有
/// 门面，判形须由签名屏障承载，形制对齐 r16-v21 `lua_touserdatatagged_ref`、r16-v25
/// `shrinkstackprotected` 的「收形不降屏障」判例；本票只收 `l` 一枚形，不改任何 unsafe 授权窗。
///
/// r16-v22 曾保留外层 `*mut LuaState` 形（其时把 doc 名指的 `lua_pushunsigned.rs`/
/// `lua_pcallyieldable.rs` 计为消费方）；v27 复核实测二者均为 doc 名指，真实调用点仅
/// `loadsafe.rs` 一处且属战役 territory，`luau_execute.rs` 只消费 `lua_r_cloneclass`
/// （并发热区，本票未触碰），故外层收形不再撞车。
pub(crate) unsafe fn lua_r_newclass(
  l: &mut LuaState,
  name: *mut tstring,
  memberstooffset: *mut LuaTable,
  offsettomember: *mut *mut tstring,
  numberofinstancemembers: i32,
  numberofstaticmembers: i32,
  envt: *mut LuaTable,
) -> *mut LuauClass {
  // SAFETY: 契约保证 `l` 处于受保护帧、gc_threshold 处于本函数前置断言的暂停态，新类字段填充与成员注册均在分配界内
  // （`l` 的存活与独占由 `&mut LuaState` 承载）
  //
  // r13-w1c 逐点定性（w6d 口径保留面）：本体唯一 `(*l).` 点位为 `(*l).global`
  // global_State 链基址读数（r13-w1b resume_finish 同款保留判例），不属栈顶门面/
  // 边界原语覆盖面，保留。r16-v27 收形后该点位经 `&mut` 的隐式 deref 等价写作 `l.global`
  // （同址同宽、时序不变），深层 `(*global).gc_threshold` 断言仍为裸读。
  unsafe {
    let global = l.global;
    LUAU_ASSERT!((*global).gc_threshold == usize::MAX);

    let classobject = lua_r_newblankclass(l, name, false);
    let co = &mut *classobject;

    // r16-v27：`luaM_newarray!`/其展开的 `lua_m_new`、`lua_m_toobig` 仍收裸形，
    // 一次 `l.as_mut_ptr()` 就地重建（借用窗止于当句），未新增授权面
    co.staticmembers = luaM_newarray!(l.as_mut_ptr(), numberofstaticmembers, TValue, co.memcat);
    // SAFETY:staticmembers 刚按 numberofstaticmembers 分配完成，全部元素可写。
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
